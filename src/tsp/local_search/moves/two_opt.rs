use crate::{data::ProblemData, solution::Solution};

pub fn best_improvement(solution: &mut Solution, data: &ProblemData) -> bool {
    let mut best_delta = 0;
    let mut best_i = 0;
    let mut best_j = 0;

    let route = &solution.routes[0];

    for i in 1..route.path.len() - 2 {
        let node_i = route.path[i];
        let prev_node_i = route.path[i - 1];

        for j in i + 2..route.path.len() - 1 - (i == 1) as usize {
            let node_j = route.path[j];
            let next_node_j = route.path[j + 1];

            let delta = -data.cost(prev_node_i, node_i)
                + data.cost(prev_node_i, node_j)
                + data.cost(node_i, next_node_j)
                - data.cost(node_j, next_node_j);

            #[cfg(debug_assertions)]
            {
                test_objective(solution, delta, i, j);
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

        route.path[best_i..=best_j].reverse();
        route.objective += best_delta;

        solution.total_objective = route.objective;

        debug_assert_eq!(solution.invalid_cost_routes(), vec![]);

        return true;
    }
    false
}

#[cfg(debug_assertions)]
fn test_objective(solution: &Solution, delta: i32, i: usize, j: usize) {
    let move_objective = solution.total_objective + delta;

    let mut solution_after_move = solution.clone();

    solution_after_move.routes[0].path[i..=j].reverse();

    let expected_objective = solution.calculate_cost();

    assert_eq!(
        move_objective, expected_objective,
        "Invalid two-opt evaluation at ({i}, {j})! Expected: {expected_objective}, but got {move_objective}",
    );
}
