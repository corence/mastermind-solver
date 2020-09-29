
use crate::attempt::*;
use crate::code::*;
use super::*;
use super::node::*;

pub struct TreeWithSurrender {
    tree: Node,
    available_colors: Vec<Color>,
    attempts: Vec<Attempt>,
}

impl TreeWithSurrender {
    pub fn new(available_colors: &Vec<Color>, solution_length: usize) -> Self {
        TreeWithSurrender {
            tree: Node::new(Code::with_length(solution_length)),
            available_colors: available_colors.clone(),
            attempts: Vec::new(),
        }
    }
}

impl Solver for TreeWithSurrender {
    fn name(&self) -> &str {
        "tree_with_surrender"
    }

    fn record_attempt(&mut self, attempt: &Code, score: Score) {
        self.attempts.push(Attempt::new(attempt.clone(), score));
    }

    fn generate_candidate(&mut self) -> Option<Code> {
        let max_attempt_generations = 10000;

        for attempt_generation in 0..max_attempt_generations {
            if let Selection(attempt) = self.tree.select_code(&self.attempts, &self.available_colors, 2) {
                return Some(attempt);
            }
        }

        println!("generations exhausted; tree size: {}; generations: {}", self.tree.code_count(), max_attempt_generations);
        None
    }
}

