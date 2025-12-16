use crate::data::ProblemData;

use pyo3::prelude::*;

#[derive(Debug, Clone)]
pub struct Route<'a> {
    id: usize,
    data: &'a ProblemData,
    pub path: Vec<usize>,
    pub objective: i32,
}

impl<'a> Route<'a> {
    pub fn new(data: &'a ProblemData, id: usize) -> Self {
        Self {
            id,
            data,
            path: Vec::new(),
            objective: 0,
        }
    }

    pub fn from_path(data: &'a ProblemData, id: usize, path: Vec<usize>, objective: i32) -> Self {
        Self {
            id,
            data,
            path,
            objective,
        }
    }

    #[inline(always)]
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn calculate_cost(&self) -> i32 {
        let mut cost = 0;

        for i in 0..self.path.len() - 1 {
            cost += self.data.cost(self.path[i], self.path[i + 1]);
        }
        cost
    }

    pub fn calculate_latency(&self) -> i32 {
        let mut latency = 0;

        for i in 0..self.path.len() - 1 {
            latency +=
                (self.path.len() - 1 - i) as i32 * self.data.cost(self.path[i], self.path[i + 1]);
        }
        latency
    }

    pub fn check_cost(&self) -> bool {
        let mut valid = true;

        let real_cost = self.calculate_cost();

        if self.objective != real_cost {
            valid = false;
            eprintln!(
                "Invalid objective: Expected {} but found {}!",
                real_cost, self.objective
            );
        }
        valid
    }

    pub fn check_latency(&self) -> bool {
        let mut valid = true;

        let real_latency = self.calculate_latency();

        if self.objective != real_latency {
            valid = false;
            eprintln!(
                "Invalid objective: Expected {} but found {}!",
                real_latency, self.objective
            );
        }
        valid
    }
}

#[pyclass(name = "Route")]
#[derive(Clone)]
pub struct PyRoute {
    #[pyo3(get)] // Exposes .id to Python
    pub id: usize,
    #[pyo3(get)] // Exposes .path to Python
    pub path: Vec<usize>,
    #[pyo3(get)] // Exposes .objective to Python
    pub objective: i32,
}

impl<'a> From<Route<'a>> for PyRoute {
    fn from(route: Route<'a>) -> Self {
        // 1. Extract scalar values (copy)
        // We must do this before moving 'route.path' because moving invalidates 'route'
        let id = route.id();
        let objective = route.objective;

        // 2. Move the heavy vector (Partial Move)
        // Since 'path' is public, we can move it out even if other fields are private.
        // This is ZERO-COPY (much faster than cloning).
        let path = route.path;

        Self {
            id,
            path,
            objective,
        }
    }
}
