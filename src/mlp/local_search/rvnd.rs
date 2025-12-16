use rand::{Rng, rng};

use pyo3::prelude::*;

use crate::{
    data::ProblemData,
    mlp::{self, SubseqLocalSearch},
    neighbourhood::{MLP_BS_F2O_BOO123, Move},
    solution::{PySolution, Solution},
};

pub struct Rvnd<'a> {
    data: &'a ProblemData,
}

impl<'a> Rvnd<'a> {
    pub fn new(data: &'a ProblemData) -> Self {
        Self { data }
    }
}

impl<'a> mlp::SubseqLocalSearch for Rvnd<'a> {
    fn apply(
        &self,
        solution: &mut Solution,
        subseq_matrix: &mut mlp::SubseqMatrix,
        moves: &[Move],
    ) {
        let mut active_moves = moves.to_vec();

        while !active_moves.is_empty() {
            let move_idx = rng().random_range(0..active_moves.len());
            let selected_move = active_moves[move_idx];

            let improved = match selected_move {
                Move::BestSwapMlp => {
                    mlp::swap::best_improvement(solution, subseq_matrix, self.data)
                }
                Move::FirstTwoOptMlp => {
                    mlp::two_opt::first_improvement(solution, subseq_matrix, self.data)
                }
                Move::BestTwoOptMlp => {
                    mlp::two_opt::best_improvement(solution, subseq_matrix, self.data)
                }
                Move::BestOrOptMlp(block_size) => {
                    mlp::or_opt::best_improvement(solution, block_size, subseq_matrix, self.data)
                }
                _ => false,
            };
            if improved {
                active_moves = moves.to_vec();
            } else {
                active_moves.swap_remove(move_idx);
            }
        }
        debug_assert_eq!(solution.invalid_latency_routes(), vec![]);
    }
}

#[pyclass(name = "MlpRvnd")]
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

        let mut rs_solution = Solution::from_py(&py_solution, &self.data);

        let mut subseq_matrix = mlp::SubseqMatrix::new(&self.data);

        subseq_matrix.update(&rs_solution);

        let rvnd = mlp::Rvnd::new(&self.data);

        rvnd.apply(&mut rs_solution, &mut subseq_matrix, MLP_BS_F2O_BOO123);

        Ok(rs_solution.into())
    }
}
