use crate::{mlp, neighbourhood::Move, solution::Solution};

pub trait SubseqLocalSearch {
    fn apply(&self, solution: &mut Solution, subseq_matrix: &mut mlp::SubseqMatrix, moves: &[Move]);
}
