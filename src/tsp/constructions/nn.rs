use crate::{data::ProblemData, metaheuristic::Construction, solution::Solution};

use rand::Rng;

#[derive(Debug)]
pub struct NearestNeighbour<'a> {
    data: &'a ProblemData,
    alpha_bests: &'a Vec<f32>,
}

impl<'a> NearestNeighbour<'a> {
    pub fn new(data: &'a ProblemData, alpha_bests: &'a Vec<f32>) -> Self {
        Self { alpha_bests, data }
    }
}

impl<'a> Construction for NearestNeighbour<'a> {
    fn solve(&self) -> Solution<'_> {
        let alpha_idx = rand::rng().random_range(0..self.alpha_bests.len());
        let alpha = self.alpha_bests[alpha_idx];

        let mut solution = Solution::new(self.data);

        let route = &mut solution.routes[0];

        route.path.push(0);

        let mut candidate_list: Vec<usize> = (1..self.data.dimension()).collect();
        let mut current_node = 0;

        while !candidate_list.is_empty() {
            candidate_list.sort_unstable_by(|a, b| {
                self.data
                    .cost(current_node, *a)
                    .cmp(&self.data.cost(current_node, *b))
            });
            let restricted_candidate_list_size =
                (((candidate_list.len()) as f32 * alpha).ceil() as usize).max(1);

            let node_idx = rand::rng().random_range(0..restricted_candidate_list_size);
            let node = candidate_list[node_idx];

            route.path.push(node);
            current_node = node;

            candidate_list.swap_remove(node_idx);
        }
        route.path.push(0);
        route.objective = route.calculate_cost();

        solution.total_objective = route.objective;

        debug_assert!(solution.invalid_cost_routes().is_empty());

        solution
    }
}

pub mod py {
    use super::*;

    use pyo3::prelude::*;
    use pyo3::types::PyDict;

    #[pyclass]
    pub struct TspNearestNeighbour {
        data: ProblemData,
    }

    #[pymethods]
    impl TspNearestNeighbour {
        #[new]
        #[pyo3(signature = (data))]
        pub fn new(data: ProblemData) -> Self {
            Self { data }
        }

        pub fn solve(&self, py: Python, alpha_bests: Vec<f32>) -> PyResult<Py<PyAny>> {
            let nn = NearestNeighbour::new(&self.data, &alpha_bests);
            let solution = nn.solve();

            let dict = PyDict::new(py);

            dict.set_item("route", solution.routes[0].path.clone())?;
            dict.set_item("objective", solution.calculate_cost())?;

            Ok(dict.into())
        }
    }
}
