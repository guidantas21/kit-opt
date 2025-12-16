use crate::{
    data::ProblemData,
    metaheuristic::Perturbation,
    solution::{PySolution, Solution},
};
use rand::Rng;

use pyo3::prelude::*;

#[derive(Debug)]
pub struct DoubleBridge {
    max_block_size: usize,
}

impl DoubleBridge {
    pub fn new(data: &ProblemData) -> Self {
        let max_block_size = (data.dimension() as f64 / 10.).ceil() as usize;
        Self { max_block_size }
    }
}

impl Perturbation for DoubleBridge {
    fn apply(&self, solution: &mut Solution) {
        let mut rng = rand::rng();

        let block_i_size: usize = rng.random_range(2..=self.max_block_size);
        let block_j_size: usize = rng.random_range(2..=self.max_block_size);

        let route = &mut solution.routes[0];

        let space = route.path.len() - 2 - block_i_size - block_j_size;

        let separation = rng.random_range(0..=space);
        let offset = rng.random_range(0..=space - separation);

        let head_i_idx = 1 + offset;
        let tail_j_idx = offset + block_i_size + separation + block_j_size;

        route.path[head_i_idx..=tail_j_idx].reverse();
        route.path[head_i_idx..head_i_idx + block_j_size].reverse();
        route.path[head_i_idx + block_j_size..head_i_idx + block_j_size + separation].reverse();
        route.path[head_i_idx + block_j_size + separation
            ..head_i_idx + block_j_size + separation + block_i_size]
            .reverse();

        route.objective = route.calculate_latency();

        solution.total_objective = route.objective;

        debug_assert_eq!(solution.invalid_latency_routes(), vec![]);
    }
}

#[pyclass(name = "MlpDoubleBridge")]
pub struct PyDoubleBridge {
    data: ProblemData,
}

#[pymethods]
impl PyDoubleBridge {
    #[new]
    #[pyo3(signature = (data))]
    pub fn new(data: ProblemData) -> Self {
        Self { data }
    }

    pub fn solve(&self, py: Python, solution_handle: Py<PySolution>) -> PyResult<PySolution> {
        let py_solution = solution_handle.borrow(py);

        let db = DoubleBridge::new(&self.data);
        let mut rs_solution = Solution::from_py(&py_solution, &self.data);

        db.apply(&mut rs_solution);

        Ok(rs_solution.into())
    }
}
