
mod random;
mod awful;

use std::collections::HashMap;

pub trait Algorithm {
    fn name(&self) -> &str;
}

pub fn algorithms() -> HashMap<String, Box<dyn Algorithm>> {
    let mut result: HashMap<String, Box<dyn Algorithm>> = HashMap::new();
    let mut algorithms: Vec<Box<dyn Algorithm>> = vec![Box::new(random::Random::new()), Box::new(awful::Awful::new())];

    for algorithm in algorithms {
        result.insert(algorithm.name().to_string(), algorithm);
    }

    result
}
