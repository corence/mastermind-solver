
use crate::random_index::*;
use std::fmt;

pub type Color = u8;

#[derive(Clone)]
pub struct Code {
    pub code: Vec<Color>,
}

impl Code {
    pub fn new(code: Vec<Color>) -> Self {
        Code {
            code
        }
    }

    pub fn with_length(length: usize) -> Code {
        let mut code = Vec::new();
        code.resize(length, b'.');
        Self::new(code)
    }

    pub fn from_str(s: &str) -> Code {
        Self::new(s.as_bytes().to_vec())
    }

    pub fn generate(length: usize, available_colors: &Vec<Color>) -> Code {
        let mut code = Vec::new();

        for _ in 0..length {
            let index = select_random_index(&available_colors).unwrap();
            code.push(available_colors[index]);
        }

        Self::new(code)
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }
}

impl fmt::Debug for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(std::str::from_utf8(&self.code).unwrap())
    }
}
