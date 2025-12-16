pub mod data;
pub mod gils;
pub mod metaheuristic;
pub mod mlp;
pub mod neighbourhood;
pub mod route;
pub mod solution;
pub mod tsp;

use pyo3::prelude::*;

pub use crate::data::ProblemData;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn kit_opt(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ProblemData>()?;
    m.add_class::<tsp::PyCheapestInsertion>()?;
    m.add_class::<tsp::PyNearestNeighbour>()?;

    m.add_class::<tsp::PyRvnd>()?;

    m.add_class::<tsp::PyDoubleBridge>()?;

    m.add_class::<tsp::PyGilsRvnd>()?;

    m.add_class::<mlp::PyCheapestInsertion>()?;
    m.add_class::<mlp::PyNearestNeighbour>()?;

    m.add_class::<mlp::PyRvnd>()?;

    m.add_class::<mlp::PyDoubleBridge>()?;

    m.add_class::<mlp::PyGilsRvnd>()?;

    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

    Ok(())
}
