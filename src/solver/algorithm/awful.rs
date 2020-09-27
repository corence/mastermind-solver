
use crate::code::*;
use super::Algorithm;

pub struct Awful {
}

impl Awful {
    pub fn new(available_colors: &Vec<Color>, solution_length: usize) -> Self {
        Awful {
        }
    }
}

impl Algorithm for Awful {
    fn name(&self) -> &str {
        "awful"
    }
}

