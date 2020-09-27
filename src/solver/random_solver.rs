
use crate::code::*;
use super::Solver;

pub struct RandomSolver {
}

impl RandomSolver {
    pub fn new() -> Self {
        RandomSolver {
        }
    }
}

impl Solver for RandomSolver {
    fn solve(&mut self) -> Code {
        Code::with_length(0)
    }
}

