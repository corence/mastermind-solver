
use crate::attempt::*;
use crate::code::*;
use crate::random_index::*;
use super::Solver;

pub struct Random {
    available_colors: Vec<Color>,
    solution_length: usize,
}

impl Random {
    pub fn new(available_colors: &Vec<Color>, solution_length: usize) -> Self {
        Random {
            available_colors: available_colors.clone(),
            solution_length,
        }
    }
}

impl Solver for Random {
    fn name(&self) -> &str {
        "random"
    }

    fn record_attempt(&mut self, _: &Code, _: Score) {
        // ignore attempts for this silly solver
    }

    fn generate_candidate(&mut self) -> Option<Code> {
        let mut code = vec![];

        for _ in 0..self.solution_length {
            code.push(self.available_colors[select_random_index(&self.available_colors).unwrap()]);
        }

        Some(Code { code })
    }
}

