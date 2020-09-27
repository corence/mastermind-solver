
use crate::code::*;
use super::Algorithm;

pub struct Awful {
}

impl Awful {
    pub fn new() -> Self {
        Awful {
        }
    }
}

impl Algorithm for Awful {
    fn name(&self) -> &str {
        "awful"
    }
}

