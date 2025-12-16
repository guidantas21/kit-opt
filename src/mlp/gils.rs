use pyo3::prelude::*;

use crate::{
    data::ProblemData,
    metaheuristic::{Construction, Perturbation},
    mlp::{self, CheapestInsertion, DoubleBridge, Rvnd},
    neighbourhood::{MLP_BS_F2O_BOO123, Move},
    solution::{PySolution, Solution},
};

#[derive(Debug)]
pub struct Gils<'a, C, LS, P> {
    data: &'a ProblemData,
    construction: C,
    perturbation: P,
    local_search: LS,
}

impl<'a, C, LS, P> Gils<'a, C, LS, P>
where
    C: Construction,
    LS: mlp::SubseqLocalSearch,
    P: Perturbation,
{
    pub fn new(data: &'a ProblemData, construction: C, local_search: LS, perturbation: P) -> Self {
        Self {
            data,
            construction,
            local_search,
            perturbation,
        }
    }

    pub fn solve(
        &self,
        num_grasp_iters: usize,
        num_ils_iters: usize,
        moves: &[Move],
    ) -> Solution<'_> {
        let mut global_best = Solution::new(self.data);
        global_best.total_objective = i32::MAX;

        for _grasp_iter in 0..num_grasp_iters {
            let mut subseq_matrix = mlp::SubseqMatrix::new(self.data);

            let mut solution = self.construction.solve();
            let mut local_best = solution.clone();

            subseq_matrix.update_range(&solution, 0, solution.routes[0].path.len() - 1);

            self.local_search
                .apply(&mut solution, &mut subseq_matrix, moves);

            if solution.total_objective < local_best.total_objective {
                local_best = solution.clone();
            }
            let mut ils_iter = 0;

            while ils_iter <= num_ils_iters {
                solution = local_best.clone();
                self.perturbation.apply(&mut solution);

                subseq_matrix.update(&solution);

                self.local_search
                    .apply(&mut solution, &mut subseq_matrix, moves);

                if solution.total_objective < local_best.total_objective {
                    local_best = solution.clone();
                    ils_iter = 0;
                } else {
                    ils_iter += 1;
                }
            }
            if local_best.total_objective < global_best.total_objective {
                global_best = local_best.clone();
            }
        }
        global_best
    }
}

#[pyclass(name = "MlpGilsRvnd")]
pub struct PyGilsRvnd {
    data: ProblemData,
}

#[pymethods]
impl PyGilsRvnd {
    #[new]
    #[pyo3(signature = (data))]
    pub fn new(data: ProblemData) -> Self {
        Self { data }
    }

    pub fn solve(&self, num_grasp_iters: usize, num_ils_iters: usize) -> PyResult<PySolution> {
        let construction = CheapestInsertion::new(&self.data);
        let local_search = Rvnd::new(&self.data);
        let perturbation = DoubleBridge::new(&self.data);

        let gils_rvnd = Gils::new(&self.data, construction, local_search, perturbation);
        let solution = gils_rvnd.solve(num_grasp_iters, num_ils_iters, MLP_BS_F2O_BOO123);

        Ok(solution.into())
    }
}
