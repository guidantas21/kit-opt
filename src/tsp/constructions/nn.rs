use crate::route::PyRoute;
use crate::solution::PySolution;
use crate::{data::ProblemData, metaheuristic::Construction, solution::Solution};

use pyo3::prelude::*;
use pyo3::types::PyDict;

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

        debug_assert_eq!(solution.invalid_cost_routes(), vec![]);

        solution
    }
}

#[pyclass(name = "TspNearestNeighbour")]
pub struct PyNearestNeighbour {
    data: ProblemData,
}

#[pymethods]
impl PyNearestNeighbour {
    #[new]
    #[pyo3(signature = (data))]
    pub fn new(data: ProblemData) -> Self {
        Self { data }
    }

    pub fn solve(&self, alpha_bests: Vec<f32>) -> PyResult<PySolution> {
        let nn = NearestNeighbour::new(&self.data, &alpha_bests);
        let rs_solution = nn.solve();

        let py_routes: Vec<PyRoute> = rs_solution
            .routes
            .iter()
            .map(|r| PyRoute {
                id: r.id(),
                path: r.path.clone(),
                objective: r.objective,
            })
            .collect();

        Ok(PySolution {
            routes: py_routes,
            total_objective: rs_solution.calculate_cost(),
        })
    }
}
