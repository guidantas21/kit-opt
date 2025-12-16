use rand::{Rng, rng};

use crate::solution::PySolution;
use crate::{
    data::ProblemData, metaheuristic::LocalSearch, neighbourhood::Move, solution::Solution, tsp,
};

use crate::neighbourhood::TSP_BS_B2O_BOO123;

use pyo3::prelude::*;

pub struct Rvnd<'a> {
    data: &'a ProblemData,
}

impl<'a> Rvnd<'a> {
    pub fn new(data: &'a ProblemData) -> Self {
        Self { data }
    }
}

impl<'a> LocalSearch for Rvnd<'a> {
    fn apply(&self, solution: &mut Solution, moves: &[Move]) {
        let mut active_moves = moves.to_vec();

        while !active_moves.is_empty() {
            let move_idx = rng().random_range(0..active_moves.len());
            let selected_move = active_moves[move_idx];

            let improved = match selected_move {
                Move::BestSwap => tsp::swap::best_improvement(solution, self.data),
                Move::BestTwoOpt => tsp::two_opt::best_improvement(solution, self.data),
                Move::BestOrOpt(block_size) => {
                    tsp::or_opt::best_improvement(solution, block_size, self.data)
                }
                _ => false,
            };
            if improved {
                active_moves = moves.to_vec();
            } else {
                active_moves.swap_remove(move_idx);
            }
        }
        debug_assert_eq!(solution.invalid_cost_routes(), vec![]);
    }
}

#[pyclass(name = "TspRvnd")]
pub struct PyRvnd {
    data: ProblemData,
}

#[pymethods]
impl PyRvnd {
    #[new]
    #[pyo3(signature = (data))]
    pub fn new(data: ProblemData) -> Self {
        Self { data }
    }

    pub fn solve(&self, py: Python, solution_handle: Py<PySolution>) -> PyResult<PySolution> {
        let py_solution = solution_handle.borrow(py);

        let rvnd = Rvnd::new(&self.data);
        let mut rs_solution = Solution::from_py(&py_solution, &self.data);

        rvnd.apply(&mut rs_solution, TSP_BS_B2O_BOO123);

        Ok(rs_solution.into())
    }
}
