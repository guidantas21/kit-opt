use crate::{data::ProblemData, route::Route};

#[derive(Debug, Clone)]
pub struct Solution<'a> {
    data: &'a ProblemData,
    pub routes: Vec<Route<'a>>,
    pub total_objective: i32,
}

impl<'a> Solution<'a> {
    pub fn new(data: &'a ProblemData) -> Self {
        let mut routes = Vec::new();

        for id in 0..data.num_vehicles() {
            routes.push(Route::new(id, data));
        }

        Self {
            data,
            routes,
            total_objective: 0,
        }
    }

    pub fn calculate_total_objective(&self) -> i32 {
        let mut objective = 0;

        for route in &self.routes {
            objective += route.objective;
        }
        objective
    }

    pub fn calculate_cost(&self) -> i32 {
        let mut cost = 0;

        for route in &self.routes {
            cost += route.calculate_cost();
        }
        cost
    }

    pub fn calculate_update_cost(&mut self) -> i32 {
        let mut cost = 0;

        for route in &mut self.routes {
            route.objective = route.calculate_cost();
            cost += route.objective;
        }
        cost
    }

    pub fn calculate_update_latency(&mut self) -> i32 {
        let mut latency = 0;

        for route in &mut self.routes {
            route.objective = route.calculate_latency();
            latency += route.objective;
        }
        latency
    }

    pub fn invalid_cost_routes(&self) -> Vec<(usize, i32)> {
        let mut invalid_routes = Vec::new();

        for route in &self.routes {
            if !route.check_cost() {
                invalid_routes.push((route.id(), route.calculate_cost()));
            }
        }
        invalid_routes
    }

    pub fn invalid_latency_routes(&self) -> Vec<(usize, i32)> {
        let mut invalid_routes = Vec::new();

        for route in &self.routes {
            if !route.check_latency() {
                invalid_routes.push((route.id(), route.calculate_latency()));
            }
        }
        invalid_routes
    }
}
