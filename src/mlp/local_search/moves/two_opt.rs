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
        for j in i + 2..route.path.len() - 1 - (i == 1) as usize {
            let subseq = subseq_matrix
                .get(0, i - 1)
                .concatenate(data, subseq_matrix.get(j, i))
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

        route.path[best_i..=best_j].reverse();
        route.objective = best_objective;

        solution.total_objective = route.objective;

        debug_assert_eq!(solution.invalid_latency_routes(), vec![]);

        subseq_matrix.update_range(solution, best_i, best_j);

        return true;
    }
    false
}

pub fn first_improvement(
    solution: &mut Solution,
    subseq_matrix: &mut SubseqMatrix,
    data: &ProblemData,
) -> bool {
    let route = &solution.routes[0];

    for i in 1..route.path.len() - 2 {
        for j in i + 2..route.path.len() - 1 - (i == 1) as usize {
            let subseq = subseq_matrix
                .get(0, i - 1)
                .concatenate(data, subseq_matrix.get(j, i))
                .concatenate(data, subseq_matrix.get(j + 1, route.path.len() - 1));

            #[cfg(debug_assertions)]
            {
                test_objective(solution.clone(), subseq.cost, i, j);
            }
            if subseq.cost < solution.total_objective {
                let route = &mut solution.routes[0];

                route.path[i..=j].reverse();
                route.objective = subseq.cost;

                solution.total_objective = route.objective;

                debug_assert_eq!(solution.invalid_latency_routes(), vec![]);

                subseq_matrix.update_range(solution, i, j);

                return true;
            }
        }
    }
    false
}

#[cfg(debug_assertions)]
fn test_objective(mut solution: Solution, move_objective: i32, i: usize, j: usize) {
    solution.routes[0].path[i..=j].reverse();

    solution.total_objective = solution.routes[0].calculate_latency();

    assert_eq!(
        move_objective, solution.total_objective,
        "Invalid two-opt evaluation at ({i}, {j})! Expected: {}, but got {move_objective}",
        solution.total_objective
    );
}
