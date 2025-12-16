use std::i32;

use crate::{data::ProblemData, mlp::subseq::SubseqMatrix, solution::Solution};

pub fn best_improvement(
    solution: &mut Solution,
    block_size: usize,
    subseq_matrix: &mut SubseqMatrix,
    data: &ProblemData,
) -> bool {
    let mut best_objective = i32::MAX;
    let mut best_i = 0;
    let mut best_j = 0;

    let route = &solution.routes[0];

    let skip_swap_case = (block_size == 1) as usize;

    for i in 1..route.path.len() - block_size {
        for j in 1..i - skip_swap_case {
            let subseq = subseq_matrix
                .get(0, j - 1)
                .concatenate(data, subseq_matrix.get(i, i + block_size - 1))
                .concatenate(data, subseq_matrix.get(j, i - 1))
                .concatenate(
                    data,
                    subseq_matrix.get(i + block_size, route.path.len() - 1),
                );
            #[cfg(debug_assertions)]
            {
                test_objective(solution.clone(), subseq.cost, block_size, i, j);
            }
            if subseq.cost < best_objective {
                best_objective = subseq.cost;
                best_i = i;
                best_j = j;
            }
        }

        for j in i + block_size + skip_swap_case..route.path.len() - 1 {
            let subseq = subseq_matrix
                .get(0, i - 1)
                .concatenate(data, subseq_matrix.get(i + block_size, j))
                .concatenate(data, subseq_matrix.get(i, i + block_size - 1))
                .concatenate(data, subseq_matrix.get(j + 1, route.path.len() - 1));

            #[cfg(debug_assertions)]
            {
                test_objective(solution.clone(), subseq.cost, block_size, i, j);
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

        let mut start = best_i;
        let mut end = best_j;

        if best_i < best_j {
            route.path[start..=end].rotate_left(block_size);
        } else {
            start = best_j;
            end = best_i + block_size - 1;

            route.path[start..=end].rotate_right(block_size);
        }
        route.objective = best_objective;
        solution.total_objective = route.objective;

        subseq_matrix.update_range(solution, start, end);

        debug_assert_eq!(solution.invalid_latency_routes(), vec![]);

        return true;
    }
    false
}

#[cfg(debug_assertions)]
fn test_objective(
    mut solution: Solution,
    move_objective: i32,
    block_size: usize,
    i: usize,
    j: usize,
) {
    if i < j {
        solution.routes[0].path[i..=j].rotate_left(block_size);
    } else {
        solution.routes[0].path[j..=i + block_size - 1].rotate_right(block_size);
    }
    solution.total_objective = solution.routes[0].calculate_latency();

    assert_eq!(
        move_objective, solution.total_objective,
        "Invalid or-opt (block size of {block_size}) evaluation at ({i}, {j})! Expected: {}, but got {}",
        solution.total_objective, move_objective
    );
}
