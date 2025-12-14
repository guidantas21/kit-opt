use crate::{moves::Move, solution::Solution};

pub trait Construction {
    fn solve(&self) -> Solution<'_>;
}

pub trait LocalSearch {
    fn apply(&self, solution: &mut Solution, moves: &[Move]);
}

pub trait Perturbation {
    fn apply(&self, solution: &mut Solution);
}
