use pyo3::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Move {
    BestSwap,
    BestTwoOpt,
    BestOrOpt(usize),

    BestSwapMlp,
    BestTwoOptMlp,
    FirstTwoOptMlp,
    BestOrOptMlp(usize),
}

pub static TSP_BS_B2O_BOO123: &[Move] = &[
    Move::BestSwap,
    Move::BestTwoOpt,
    Move::BestOrOpt(1),
    Move::BestOrOpt(2),
    Move::BestOrOpt(3),
];

pub static MLP_BS_F2O_BOO123: &[Move] = &[
    Move::BestSwapMlp,
    Move::FirstTwoOptMlp,
    Move::BestOrOptMlp(1),
    Move::BestOrOptMlp(2),
    Move::BestOrOptMlp(3),
];
