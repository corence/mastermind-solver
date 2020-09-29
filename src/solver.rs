
mod node;
mod random;
mod tree_with_surrender;

use crate::Score;
use crate::code::*;
use std::collections::BTreeMap;

pub type SolverConstructor = fn (&Vec<Color>, usize) -> Box<dyn Solver>;

pub trait Solver {
    fn generate_candidate(&mut self) -> Option<Code>;
    fn name(&self) -> &str;
    fn record_attempt(&mut self, attempt: &Code, score: Score);
}

pub fn solver_constructors() -> BTreeMap<String, SolverConstructor> {
    let mut result: BTreeMap<String, SolverConstructor> = BTreeMap::new();
    let mut constructors: Vec<(&str, SolverConstructor)> = vec![
        ("random", |available_colors, solution_length| Box::new(random::Random::new(available_colors, solution_length))),
        ("tree_with_surrender", |available_colors, solution_length| Box::new(tree_with_surrender::TreeWithSurrender::new(available_colors, solution_length))),
    ];

    for (name, constructor) in constructors {
        result.insert(name.to_string(), constructor);
    }

    result
}
