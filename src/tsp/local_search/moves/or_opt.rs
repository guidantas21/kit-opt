use crate::{data::ProblemData, solution::Solution};

pub fn best_improvement(solution: &mut Solution, block_size: usize, data: &ProblemData) -> bool {
    let mut best_delta = 0;
    let mut best_i = 0;
    let mut best_j = 0;

    let route = &solution.routes[0];

    let skip_swap_case = (block_size == 1) as usize;
    for i in 1..route.path.len() - block_size {
        let head_i = route.path[i];
        let prev_head_i = route.path[i - 1];

        let tail_i = route.path[i + block_size - 1];
        let next_tail_i = route.path[i + block_size];

        let static_delta = data.cost(prev_head_i, next_tail_i)
            - data.cost(prev_head_i, head_i)
            - data.cost(tail_i, next_tail_i);

        for j in 1..i - skip_swap_case {
            let node_j = route.path[j];
            let prev_node_j = route.path[j - 1];

            let delta = static_delta - data.cost(prev_node_j, node_j)
                + data.cost(prev_node_j, head_i)
                + data.cost(tail_i, node_j);

            #[cfg(debug_assertions)]
            {
                test_objective(solution, delta, block_size, i, j);
            }
            if delta < best_delta {
                best_delta = delta;
                best_i = i;
                best_j = j;
            }
        }

        for j in i + block_size + skip_swap_case..route.path.len() - 1 {
            let node_j = route.path[j];
            let next_node_j = route.path[j + 1];

            let delta = static_delta - data.cost(node_j, next_node_j)
                + data.cost(node_j, head_i)
                + data.cost(tail_i, next_node_j);

            #[cfg(debug_assertions)]
            {
                test_objective(solution, delta, block_size, i, j);
            }
            if delta < best_delta {
                best_delta = delta;
                best_i = i;
                best_j = j;
            }
        }
    }
    if best_delta < 0 {
        let route = &mut solution.routes[0];

        if best_i < best_j {
            route.path[best_i..=best_j].rotate_left(block_size);
        } else {
            route.path[best_j..=best_i + block_size - 1].rotate_right(block_size);
        }
        route.objective += best_delta;

        solution.total_objective = route.objective;

        debug_assert_eq!(solution.invalid_cost_routes(), vec![]);

        return true;
    }
    false
}

#[cfg(debug_assertions)]
fn test_objective(solution: &Solution, delta: i32, block_size: usize, i: usize, j: usize) {
    let move_objective = solution.total_objective + delta;

    let mut solution_after_move = solution.clone();

    if i < j {
        solution_after_move.routes[0].path[i..=j].rotate_left(block_size);
    } else {
        solution_after_move.routes[0].path[j..=i + block_size - 1].rotate_right(block_size);
    }
    let expected_objective = solution_after_move.calculate_cost();

    assert_eq!(
        move_objective, expected_objective,
        "Invalid or-opt (block size of {block_size}) evaluation at ({i}, {j})! Expected: {}, but got {}",
        expected_objective, move_objective
    );
}
