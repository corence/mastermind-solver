
pub mod algorithm;
mod node;

use crate::attempt::*;
use crate::code::*;
use self::algorithm::*;

fn solve(algorithm: &mut Algorithm, max_attempt_count: usize, solution: &Code) -> Option<Code> {
    for _attempt_number in 0..max_attempt_count {
        if let Some(candidate) = algorithm.generate_candidate() {
            let score = compute_score(&candidate, solution);
            println!("recording guess: {:?}", (&candidate, score));
            if score.black == solution.len() {
                return Some(candidate);
            }
            algorithm.record_attempt(&candidate, score);
        } else {
            break;
        }
    }

    None
}
