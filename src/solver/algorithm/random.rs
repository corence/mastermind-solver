
use crate::code::*;
use super::Algorithm;

pub struct Random {
}

impl Random {
    pub fn new() -> Self {
        Random {
        }
    }
}

impl Algorithm for Random {
    fn name(&self) -> &str {
        "random"
    }
}

