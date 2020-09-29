
mod random;
mod tree_with_surrender;

use crate::Score;
use crate::code::*;
use std::collections::BTreeMap;

pub type AlgorithmConstructor = fn (&Vec<Color>, usize) -> Box<dyn Algorithm>;

pub trait Algorithm {
    fn generate_candidate(&mut self) -> Option<Code>;
    fn name(&self) -> &str;
    fn record_attempt(&mut self, attempt: &Code, score: Score);
}

pub fn algorithm_constructors() -> BTreeMap<String, AlgorithmConstructor> {
    let mut result: BTreeMap<String, AlgorithmConstructor> = BTreeMap::new();
    let mut constructors: Vec<(&str, AlgorithmConstructor)> = vec![
        ("random", |available_colors, solution_length| Box::new(random::Random::new(available_colors, solution_length))),
        ("tree_with_surrender", |available_colors, solution_length| Box::new(tree_with_surrender::TreeWithSurrender::new(available_colors, solution_length))),
    ];

    for (name, constructor) in constructors {
        result.insert(name.to_string(), constructor);
    }

    result
}
