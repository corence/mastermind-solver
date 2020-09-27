
use crate::code::*;
use super::Solver;

pub struct Random {
}

impl Random {
    pub fn new() -> Self {
        Random {
        }
    }
}

impl Solver for Random {
    fn name(&self) -> &str {
        "random"
    }

    fn solve(&mut self) -> Code {
        Code::with_length(0)
    }
}

