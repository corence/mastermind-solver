
use crate::code::*;
use super::Algorithm;

pub struct Random {
}

impl Random {
    pub fn new(available_colors: &Vec<Color>, solution_length: usize) -> Self {
        Random {
        }
    }
}

impl Algorithm for Random {
    fn name(&self) -> &str {
        "random"
    }
}

