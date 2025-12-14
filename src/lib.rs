mod data;
mod gils;
mod metaheuristic;
mod moves;
mod route;
mod solution;
mod tsp;

use pyo3::prelude::*;

pub use crate::data::ProblemData;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn kit_opt(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ProblemData>()?;
    m.add_class::<tsp::TspNearestNeighbour>()?;
    m.add_class::<tsp::TspCheapestInsertion>()?;

    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

    Ok(())
}
