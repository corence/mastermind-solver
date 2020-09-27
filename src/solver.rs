
pub mod algorithm;
mod node;

use crate::attempt::*;
use crate::code::*;
use self::node::*;
use self::algorithm::*;
use std::collections::HashMap;

fn solve(algorithm: &mut Algorithm, max_attempt_count: usize, solution: &Code, available_colors: &Vec<Color>) -> Option<Code> {
    let mut guesses = Vec::new();
    let mut tree = Node::new(Code::with_length(solution.len()));
    let max_attempt_generations = 10000;

    println!("Solution: {:?}", solution);

    for _attempt_number in 0..max_attempt_count {
        let mut attempted = false;
        for attempt_generation in 0..max_attempt_generations {
            if let Selection(attempt) = tree.select_code(&guesses, available_colors, 2) {
                let score = compute_score(&attempt, solution);
                let guess = Guess::new(attempt, score);
                println!("recording guess: {:?}; tree size: {}; generations: {}", guess, tree.code_count(), attempt_generation);
                if score.black == solution.len() {
                    return Some(guess.code());
                }
                guesses.push(guess);
                attempted = true;
            }
        }

        if !attempted {
            println!("generations exhausted; tree size: {}; generations: {}", tree.code_count(), max_attempt_generations);
            break;
        }
    }

    None
}
