use std::i32;

use crate::{data::ProblemData, mlp::subseq::SubseqMatrix, solution::Solution};

pub fn best_improvement(
    solution: &mut Solution,
    subseq_matrix: &mut SubseqMatrix,
    data: &ProblemData,
) -> bool {
    let mut best_objective = i32::MAX;
    let mut best_i = 0;
    let mut best_j = 0;

    let route = &solution.routes[0];

    for i in 1..route.path.len() - 2 {
        for j in i + 1..route.path.len() - 1 {
            let mut subseq = subseq_matrix
                .get(0, i - 1)
                .concatenate(data, subseq_matrix.get(j, j));

            if i + 1 < j {
                subseq = subseq.concatenate(data, subseq_matrix.get(i + 1, j - 1));
            }
            subseq = subseq
                .concatenate(data, subseq_matrix.get(i, i))
                .concatenate(data, subseq_matrix.get(j + 1, route.path.len() - 1));

            #[cfg(debug_assertions)]
            {
                test_objective(solution.clone(), subseq.cost, i, j);
            }
            if subseq.cost < best_objective {
                best_objective = subseq.cost;
                best_i = i;
                best_j = j;
            }
        }
    }
    if best_objective < solution.total_objective {
        let route = &mut solution.routes[0];

        route.path.swap(best_i, best_j);
        route.objective = best_objective;

        solution.total_objective = route.objective;

        debug_assert_eq!(solution.invalid_latency_routes(), vec![]);

        subseq_matrix.update_range(solution, best_i, best_j);

        return true;
    }
    false
}

#[cfg(debug_assertions)]
fn test_objective(mut solution: Solution, move_objective: i32, i: usize, j: usize) {
    solution.routes[0].path.swap(i, j);

    solution.total_objective = solution.routes[0].calculate_latency();

    assert_eq!(
        move_objective, solution.total_objective,
        "Invalid swap evaluation at ({i}, {j})! Expected: {}, but got {move_objective}",
        solution.total_objective
    );
}
