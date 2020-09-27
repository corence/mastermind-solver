
mod awful;
mod random;

use crate::code::*;
use std::collections::HashMap;

pub type AlgorithmConstructor = fn (&Vec<Color>, usize) -> Box<dyn Algorithm>;

pub trait Algorithm {
    fn name(&self) -> &str;
}

pub fn algorithm_constructors() -> HashMap<String, AlgorithmConstructor> {
    let mut result: HashMap<String, AlgorithmConstructor> = HashMap::new();
    let mut constructors: Vec<(&str, AlgorithmConstructor)> = vec![
        ("random", |available_colors, solution_length| Box::new(random::Random::new(available_colors, solution_length))),
        ("awful", |available_colors, solution_length| Box::new(awful::Awful::new(available_colors, solution_length))),
    ];

    for (name, constructor) in constructors {
        result.insert(name.to_string(), constructor);
    }

    result
}
