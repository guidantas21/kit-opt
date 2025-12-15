use crate::data::ProblemData;

use pyo3::prelude::*;

// #[derive(Debug, Clone)]
// pub struct Route<'a> {
//     id: usize,
//     data: &'a ProblemData,
//     pub path: Vec<usize>,
//     pub objective: i32,
// }

// impl<'a> Route<'a> {
//     pub fn new(data: &'a ProblemData, id: usize) -> Self {
//         Self {
//             id,
//             data,
//             path: Vec::new(),
//             objective: 0,
//         }
//     }

//     pub fn from_path(data: &'a ProblemData, id: usize, path: Vec<usize>, objective: i32) -> Self {
//         Self {
//             id,
//             data,
//             path: Vec::new(),
//             objective: 0,
//         }
//     }

//     #[inline(always)]
//     pub fn id(&self) -> usize {
//         self.id
//     }

//     pub fn calculate_cost(&self) -> i32 {
//         let mut cost = 0;

//         for i in 0..self.path.len() - 1 {
//             cost += self.data.cost(self.path[i], self.path[i + 1]);
//         }
//         cost
//     }

//     pub fn calculate_latency(&self) -> i32 {
//         let mut latency = 0;

//         for i in 0..self.path.len() - 1 {
//             latency +=
//                 (self.path.len() - 1 - i) as i32 * self.data.cost(self.path[i], self.path[i + 1]);
//         }
//         latency
//     }

//     pub fn check_cost(&self) -> bool {
//         let mut valid = true;

//         let real_cost = self.calculate_cost();

//         if self.objective != real_cost {
//             valid = false;
//             eprintln!(
//                 "Invalid objective: Expected {} but found {}!",
//                 real_cost, self.objective
//             );
//         }
//         valid
//     }

//     pub fn check_latency(&self) -> bool {
//         let mut valid = true;

//         let real_latency = self.calculate_latency();

//         if self.objective != real_latency {
//             valid = false;
//             eprintln!(
//                 "Invalid objective: Expected {} but found {}!",
//                 real_latency, self.objective
//             );
//         }
//         valid
//     }
// }

#[derive(Debug, Clone)]
pub struct Route {
    id: usize,
    pub path: Vec<usize>,
    pub objective: i32,
}

impl Route {
    pub fn new(id: usize, path: Vec<usize>, objective: i32) -> Self {
        Self {
            id,
            path: Vec::new(),
            objective: 0,
        }
    }

    #[inline(always)]
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn calculate_cost(&self, data: &ProblemData) -> i32 {
        let mut cost = 0;

        for i in 0..self.path.len() - 1 {
            cost += data.cost(self.path[i], self.path[i + 1]);
        }
        cost
    }

    pub fn calculate_latency(&self, data: &ProblemData) -> i32 {
        let mut latency = 0;

        for i in 0..self.path.len() - 1 {
            latency += (self.path.len() - 1 - i) as i32 * data.cost(self.path[i], self.path[i + 1]);
        }
        latency
    }

    pub fn check_cost(&self, data: &ProblemData) -> bool {
        let mut valid = true;

        let real_cost = self.calculate_cost(data);

        if self.objective != real_cost {
            valid = false;
            eprintln!(
                "Invalid objective: Expected {} but found {}!",
                real_cost, self.objective
            );
        }
        valid
    }

    pub fn check_latency(&self, data: &ProblemData) -> bool {
        let mut valid = true;

        let real_latency = self.calculate_latency(data);

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
