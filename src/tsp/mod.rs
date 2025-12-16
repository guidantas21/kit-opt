pub mod constructions;
pub mod local_search;
pub mod perturbations;

pub use constructions::{
    ci::{CheapestInsertion, PyCheapestInsertion},
    nn::{NearestNeighbour, PyNearestNeighbour},
};

pub use local_search::{PyRvnd, or_opt, swap, two_opt};

pub use perturbations::double_brigde::{DoubleBridge, PyDoubleBridge};

use pyo3::prelude::*;

use crate::{
    ProblemData, gils::Gils, neighbourhood::TSP_BS_B2O_BOO123, solution::PySolution,
    tsp::local_search::Rvnd,
};

#[pyclass(name = "TspGilsRvnd")]
pub struct PyGilsRvnd {
    data: ProblemData,
}

#[pymethods]
impl PyGilsRvnd {
    #[new]
    #[pyo3(signature = (data))]
    pub fn new(data: ProblemData) -> Self {
        Self { data }
    }

    pub fn solve(&self, num_grasp_iters: usize, num_ils_iters: usize) -> PyResult<PySolution> {
        let construction = CheapestInsertion::new(&self.data);
        let local_search = Rvnd::new(&self.data);
        let perturbation = DoubleBridge::new(&self.data);

        let gils_rvnd = Gils::new(&self.data, construction, local_search, perturbation);
        let solution = gils_rvnd.solve(num_grasp_iters, num_ils_iters, TSP_BS_B2O_BOO123);

        Ok(solution.into())
    }
}
