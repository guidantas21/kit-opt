use rand::{Rng, rng};

use crate::{
    data::ProblemData,
    metaheuristic::LocalSearch,
    neighbourhood::Move,
    solution::{self, Solution},
    tsp,
};

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

pub mod py {
    use crate::neighbourhood::TSP_BS_B2O_BOO123;
    use crate::solution::py_solution;

    use super::*;

    use pyo3::prelude::*;
    use pyo3::types::PyDict;

    #[pyclass]
    pub struct TspRvnd {
        data: ProblemData,
    }

    #[pymethods]
    impl TspRvnd {
        #[new]
        #[pyo3(signature = (data))]
        pub fn new(data: ProblemData) -> Self {
            Self { data }
        }

        pub fn solve(
            &self,
            py: Python,
            initial_solution: Py<py_solution::Solution>,
        ) -> PyResult<Py<py_solution::Solution>> {
            let rvnd = Rvnd::new(&self.data);

            let solution = rvnd.apply(initial_solution, TSP_BS_B2O_BOO123);

            Ok(solution)
        }
    }

    impl TspRvnd {
        pub fn to_solution(&self) -> Solution {}
    }
}
