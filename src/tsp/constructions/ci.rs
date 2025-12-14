use crate::data::ProblemData;
use crate::metaheuristic::Construction;
use crate::route::Route;
use crate::solution::Solution;

use rand::Rng;

struct Insertion {
    pub candidate_idx: usize,
    pub removed_edge_pos: usize,
    pub delta: i32,
}

impl Insertion {
    pub fn new(candidate_idx: usize, removed_edge_pos: usize, delta: i32) -> Self {
        Self {
            candidate_idx,
            removed_edge_pos,
            delta,
        }
    }
}

#[derive(Debug)]
pub struct CheapestInsertion<'a> {
    data: &'a ProblemData,
}

impl<'a> CheapestInsertion<'a> {
    pub fn new(data: &'a ProblemData) -> Self {
        Self { data }
    }

    pub fn init_solution(&self) -> (Solution<'_>, Vec<usize>) {
        let mut rng = rand::rng();

        let mut candidate_list: Vec<usize> = (1..self.data.dimension()).collect();

        let mut solution = Solution::new(self.data);
        let route = &mut solution.routes[0];

        route.path.push(0);

        while route.path.len() < 3 {
            let new_node_idx = rng.random_range(0..candidate_list.len());
            let new_node = candidate_list[new_node_idx];

            route.objective += self.data.cost(route.path[route.path.len() - 1], new_node);

            route.path.push(new_node);
            candidate_list.swap_remove(new_node_idx);
        }
        route.objective += self.data.cost(route.path[route.path.len() - 1], 0);
        route.path.push(0);

        solution.total_objective = route.objective;

        debug_assert!(solution.invalid_cost_routes().is_empty());

        (solution, candidate_list)
    }

    fn compute_insertions(&self, route: &Route, candidate_list: &[usize]) -> Vec<Insertion> {
        let mut insertions = Vec::new();

        for (candidate_idx, candidate) in candidate_list.iter().enumerate() {
            for edge_idx in 0..route.path.len() - 1 {
                let new_node = *candidate;

                let node_i = route.path[edge_idx];
                let node_j = route.path[edge_idx + 1];

                let delta = self.data.cost(node_i, new_node) + self.data.cost(new_node, node_j)
                    - self.data.cost(node_i, node_j);

                insertions.push(Insertion::new(candidate_idx, edge_idx, delta));
            }
        }
        insertions
    }
}

impl<'a> Construction for CheapestInsertion<'a> {
    fn solve(&self) -> Solution<'_> {
        let mut rng = rand::rng();
        let (mut solution, mut candidate_list) = self.init_solution();

        let route = &mut solution.routes[0];

        while !candidate_list.is_empty() {
            let mut insertions = self.compute_insertions(route, &candidate_list);
            insertions.sort_unstable_by_key(|i| i.delta);

            let alpha = rng.random::<f32>();
            let num_best_insertions = (alpha * insertions.len() as f32).ceil() as usize;

            let insertion_idx: usize = rng.random_range(0..num_best_insertions);
            let insertion = &insertions[insertion_idx];

            let new_node = candidate_list[insertion.candidate_idx];

            route.path.insert(insertion.removed_edge_pos + 1, new_node);
            route.objective += insertion.delta;

            candidate_list.swap_remove(insertion.candidate_idx);
        }
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
    pub struct TspCheapestInsertion {
        data: ProblemData,
    }

    #[pymethods]
    impl TspCheapestInsertion {
        #[new]
        #[pyo3(signature = (data))]
        pub fn new(data: ProblemData) -> Self {
            Self { data }
        }

        pub fn solve(&self, py: Python) -> PyResult<Py<PyAny>> {
            let ci = CheapestInsertion::new(&self.data);
            let solution = ci.solve();

            let dict = PyDict::new(py);

            dict.set_item("route", solution.routes[0].path.clone())?;
            dict.set_item("objective", solution.calculate_cost())?;

            Ok(dict.into())
        }
    }
}
