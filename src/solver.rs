
mod random;
mod awful;

use crate::code::*;
use std::collections::HashMap;

pub trait Solver {
    fn solve(&mut self) -> Code;
    fn name(&self) -> &str;
}

pub fn generate_solver_directory() -> HashMap<String, Box<dyn Solver>> {
    let mut result: HashMap<String, Box<dyn Solver>> = HashMap::new();

    for solver in vec![random::Random::new()] {
        result.insert(solver.name().to_string(), Box::new(solver));
    }

    //result.insert("random".to_string(), Box::new(random::Random::new()));

    result
}
