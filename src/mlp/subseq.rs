use crate::{data::ProblemData, solution::Solution};

#[derive(Debug, Clone, Default)]
pub struct Subseq {
    pub duration: i32,
    pub cost: i32,
    pub delay: i32,
    pub first: usize,
    pub last: usize,
}

impl Subseq {
    #[inline(always)]
    pub fn concatenate(&self, data: &ProblemData, other: &Subseq) -> Self {
        let time = data.cost(self.last, other.first);

        Subseq {
            duration: self.duration + time + other.duration,
            delay: self.delay + other.delay,
            cost: self.cost + other.delay * (self.duration + time) + other.cost,
            first: self.first,
            last: other.last,
        }
    }
}

#[derive(Debug)]
pub struct SubseqMatrix<'a> {
    data: &'a ProblemData,
    pub matrix: Vec<Subseq>,
}

impl<'a> SubseqMatrix<'a> {
    pub fn new(data: &'a ProblemData) -> Self {
        let matrix = vec![Subseq::default(); (data.dimension() + 1).pow(2)];

        Self { data, matrix }
    }

    #[inline(always)]
    pub fn get(&self, i: usize, j: usize) -> &Subseq {
        let dim = self.data.dimension() + 1;
        &self.matrix[i * dim + j]
    }

    #[inline(always)]
    pub fn get_mut(&mut self, i: usize, j: usize) -> &mut Subseq {
        let dim = self.data.dimension() + 1;
        &mut self.matrix[i * dim + j]
    }

    pub fn update(&mut self, solution: &Solution) {
        let route = &solution.routes[0];

        for i in 0..route.path.len() {
            self.get_mut(i, i).delay = (i > 0) as i32;
            self.get_mut(i, i).first = route.path[i];
            self.get_mut(i, i).last = route.path[i];
        }

        for i in 0..route.path.len() {
            for j in i + 1..route.path.len() {
                *self.get_mut(i, j) = self.get(i, j - 1).concatenate(self.data, self.get(j, j));
            }
        }

        for i in (0..route.path.len()).rev() {
            for j in (0..i).rev() {
                *self.get_mut(i, j) = self.get(i, j + 1).concatenate(self.data, self.get(j, j));
            }
        }
    }

    pub fn update_range(&mut self, solution: &Solution, start: usize, end: usize) {
        let route = &solution.routes[0];

        for i in start..=end {
            self.get_mut(i, i).delay = (i > 0) as i32;
            self.get_mut(i, i).first = route.path[i];
            self.get_mut(i, i).last = route.path[i];
        }
        for i in 0..=end {
            for j in start.max(i + 1)..route.path.len() {
                *self.get_mut(i, j) = self.get(i, j - 1).concatenate(self.data, self.get(j, j));
            }
        }

        for i in (start..route.path.len()).rev() {
            for j in (0..(end + 1).min(i)).rev() {
                *self.get_mut(i, j) = self.get(i, j + 1).concatenate(self.data, self.get(j, j));
            }
        }
    }
}
