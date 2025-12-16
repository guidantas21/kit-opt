use crate::{
    data::ProblemData,
    metaheuristic::Construction,
    solution::{PySolution, Solution},
    tsp,
};

use pyo3::prelude::*;

#[derive(Debug)]
pub struct CheapestInsertion<'a> {
    tsp_ci: tsp::CheapestInsertion<'a>,
}

impl<'a> CheapestInsertion<'a> {
    pub fn new(data: &'a ProblemData) -> Self {
        Self {
            tsp_ci: tsp::CheapestInsertion::new(data),
        }
    }
}

impl<'a> Construction for CheapestInsertion<'a> {
    fn solve(&self) -> Solution<'_> {
        let mut solution = self.tsp_ci.solve();

        solution.routes[0].objective = solution.routes[0].calculate_latency();
        solution.total_objective = solution.routes[0].objective;

        solution
    }
}

#[pyclass(name = "MlpCheapestInsertion")]
pub struct PyCheapestInsertion {
    data: ProblemData,
}

#[pymethods]
impl PyCheapestInsertion {
    #[new]
    #[pyo3(signature = (data))]
    pub fn new(data: ProblemData) -> Self {
        Self { data }
    }

    pub fn solve(&self) -> PyResult<PySolution> {
        let ci = CheapestInsertion::new(&self.data);
        let solution = ci.solve();

        Ok(solution.into())
    }
}
