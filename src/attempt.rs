
use crate::code::*;
use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq,Eq,Clone,Copy)]
pub struct Score {
    pub num_dots: usize,
    pub black: usize,
    pub white: usize,
}

impl fmt::Debug for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
         .field(&self.num_dots)
         .field(&self.black)
         .field(&self.white)
         .finish()
    }
}

#[derive(Debug)]
pub struct Attempt {
    code: Code,
    score: Score,
}

impl Attempt {
    pub fn new(code: Code, score: Score) -> Self {
        Attempt {
            code,
            score,
        }
    }

    pub fn code(&self) -> Code {
        self.code.clone()
    }

    pub fn matches(&self, code: &Code) -> bool {
        let score = compute_score(&self.code, code);

        if score.black + score.num_dots < self.score.black + self.score.num_dots {
            return false;
        }

        if score.white + score.num_dots < self.score.white + self.score.num_dots {
            return false;
        }

        if score.black > self.score.black {
            return false;
        }

        if score.white > self.score.white {
            return false;
        }

        //println!("contender {:?} {:?} challenges {:?} {:?}", code, score, self.code, self.score);

        true
    }
}

pub fn compute_score(a: &Code, b: &Code) -> Score {
    let mut black = 0;

    let mut a_counts = HashMap::new();
    let mut b_counts = HashMap::new();

    for (ac, bc) in a.code.iter().zip(b.code.iter()) {
        if ac == bc {
            black += 1;
        }

        *a_counts.entry(ac).or_insert(0) += 1;
        *b_counts.entry(bc).or_insert(0) += 1;
    }

    let mut white = 0;
    for (ac, a_count) in &a_counts {
        if **ac == '.' {
            continue;
        }
        let b_count = *b_counts.get(*ac).unwrap_or(&0);
        white += std::cmp::min(*a_count, b_count);
    }

    let num_dots = *a_counts.get(&'.').unwrap_or(&0) + *b_counts.get(&'.').unwrap_or(&0);
    Score {
        num_dots,
        black,
        white,
    }
}

