mod data;
mod gils;
mod metaheuristic;
mod moves;
mod route;
mod solution;
mod tsp;

use pyo3::prelude::*;

use crate::data::ProblemData;
use crate::tsp::constructions::nn::TspNearestNeighbour;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn kit_opt(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ProblemData>()?;
    m.add_class::<TspNearestNeighbour>()?;

    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

    Ok(())
}
