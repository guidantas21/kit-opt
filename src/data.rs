use pyo3::prelude::*;
use std::path::PathBuf;

#[derive(Debug, Clone)]
enum Problem {
    MLP,
    TSP,
    CVRPTW,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct ProblemData {
    instance_name: Option<String>,
    instance_path: Option<PathBuf>,
    problem: Problem,
    dimension: usize,
    costs: Vec<i32>,
    bounds: (i32, i32),
    num_vehicles: usize,
    vehicle_capacity: usize,
    service_time: i32,
    coords: Vec<(i32, i32)>,
    tws: Vec<(i32, i32)>,
}

#[pymethods]
impl ProblemData {
    #[new]
    #[pyo3(signature = (costs, bounds, instance_name=None))]
    pub fn new_tsp(costs: Vec<i32>, bounds: (i32, i32), instance_name: Option<String>) -> Self {
        Self {
            instance_name,
            instance_path: None,
            problem: Problem::TSP,
            dimension: costs.len().isqrt(),
            costs,
            bounds,
            num_vehicles: 1,
            vehicle_capacity: 1,
            service_time: 0,
            coords: vec![],
            tws: vec![],
        }
    }

    #[getter]
    #[inline(always)]
    pub fn dimension(&self) -> usize {
        self.dimension
    }

    #[getter]
    #[inline(always)]
    pub fn num_vehicles(&self) -> usize {
        self.num_vehicles
    }

    #[getter]
    #[inline(always)]
    pub fn instance_name(&self) -> Option<String> {
        self.instance_name.clone()
    }

    #[getter]
    #[inline(always)]
    pub fn bounds(&self) -> (i32, i32) {
        self.bounds
    }

    #[inline(always)]
    pub fn cost(&self, i: usize, j: usize) -> i32 {
        self.costs[i * self.dimension + j]
    }

    pub fn __repr__(&self) -> String {
        format!(
            "ProblemData(dimension={}, instance_name={:?}, bounds={:?})",
            self.dimension, self.instance_name, self.bounds
        )
    }
}
