mod data;
mod metaheuristic;
mod moves;
mod route;
mod solution;

use pyo3::prelude::*;

use crate::data::ProblemData;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn kit_opt(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add your ProblemData class
    m.add_class::<ProblemData>()?;

    // Add the example function
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

    Ok(())
}
