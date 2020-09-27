
use crate::code::*;
use super::Solver;

pub struct Awful {
}

impl Awful {
    pub fn new() -> Self {
        Awful {
        }
    }
}

impl Solver for Awful {
    fn name(&self) -> &str {
        "awful"
    }

    fn solve(&mut self) -> Code {
        Code::with_length(0)
    }
}

