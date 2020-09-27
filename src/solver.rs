
mod random_solver;

use crate::code::*;
use std::collections::HashMap;

pub trait Solver {
    fn solve(&mut self) -> Code;
}

pub fn generate_solver_directory() -> HashMap<String, Box<dyn Solver>> {
    let mut result: HashMap<String, Box<dyn Solver>> = HashMap::new();

    result.insert("random".to_string(), Box::new(random_solver::RandomSolver::new()));

    result
}
