use crate::{
    data::ProblemData, metaheuristic::Construction, metaheuristic::LocalSearch,
    metaheuristic::Perturbation, neighbourhood::Move, solution::Solution,
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
    LS: LocalSearch,
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
            let mut solution = self.construction.solve();
            let mut local_best = solution.clone();

            self.local_search.apply(&mut solution, moves);

            if solution.total_objective < local_best.total_objective {
                local_best = solution.clone();
            }
            let mut ils_iter = 0;
            while ils_iter <= num_ils_iters {
                solution = local_best.clone();
                self.perturbation.apply(&mut solution);

                self.local_search.apply(&mut solution, moves);

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
