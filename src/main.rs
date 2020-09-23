
use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::fmt;

type Color = char;

#[derive(Clone)]
struct Code {
    code: Vec<Color>,
}

impl Code {
    fn new(code: Vec<Color>) -> Self {
        Code {
            code
        }
    }

    fn with_length(length: usize) -> Code {
        let mut code = Vec::new();
        code.resize(length, '.');
        Self::new(code)
    }

    fn from_str(s: &str) -> Code {
        Self::new(s.chars().collect())
    }

    fn generate(length: usize, available_colors: &Vec<Color>) -> Code {
        let mut code = Vec::new();

        for _ in 0..length {
            let index = select_random_index(&available_colors).unwrap();
            code.push(available_colors[index]);
        }

        Self::new(code)
    }

    fn len(&self) -> usize {
        self.code.len()
    }
}

impl fmt::Debug for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.code.iter().collect::<String>())
    }
}


#[derive(PartialEq,Eq,Clone,Copy)]
struct Score {
    num_dots: usize,
    black: usize,
    white: usize,
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

fn main() {
    solve(4, &Code::from_str("abca"), &("abc".chars().collect()));
    println!();
    let ten_chars = "rygbovxcfp".chars().collect();
    solve(12, &Code::generate(6, &ten_chars), &ten_chars);
    println!();
    solve(12, &Code::from_str("rvcrgfgb"), &ten_chars);
    println!();
    solve(12, &Code::generate(10, &ten_chars), &ten_chars);
}

fn select_random_index<T>(vec: &Vec<T>) -> Option<usize> {
    if vec.len() > 0 {
        let index = thread_rng().gen_range(0, vec.len());
        Some(index)
    } else {
        None
    }
}

fn select_random_matching_index<T: Eq>(vec: &Vec<T>, target: T) -> Option<usize> {
    let mut indexes = Vec::new();
    for (i, element) in vec.iter().enumerate() {
        if *element == target {
            indexes.push(i);
        }
    }

    select_random_index(&indexes).map(|index| indexes[index])
}

fn solve_and_bounce_back_from_tree(max_attempt_count: usize, max_attempt_generations: usize, solution: &Code, available_colors: &Vec<Color>) {
    let mut guesses = Vec::new();
    let mut tree = Node::new(Code::with_length(solution.len()));

    println!("Solution: {:?}", solution);

    for _attempt_number in 0..max_attempt_count {
        let mut attempted = false;
        for attempt_generation in 0..max_attempt_generations {
            if let Selection(attempt) = tree.select_code(&guesses, available_colors, 2) {
                let score = compute_score(&attempt, solution);
                let guess = Guess::new(attempt, score);
                println!("recording guess: {:?}; tree size: {}; generations: {}", guess, tree.code_count(), attempt_generation);
                guesses.push(guess);
                if score.black == solution.len() {
                    return;
                }
                attempted = true;
            }
        }

        if !attempted {
            println!("generations exhausted; tree size: {}; generations: {}", tree.code_count(), max_attempt_generations);
            break;
        }
    }
}

#[allow(dead_code)]
fn solve_without_tree(max_attempt_count: usize, max_attempt_generations: usize, solution: &Code, available_colors: &Vec<Color>) {
    let mut guesses = Vec::new();

    println!("Solution: {:?}", solution);

    for _attempt_number in 0..max_attempt_count {
        let mut attempted = false;
        for attempt_generation in 0..max_attempt_generations {
            if let Some(attempt) = try_select_code(solution.len(), &guesses, available_colors) {
                let score = compute_score(&attempt, solution);
                let guess = Guess::new(attempt, score);
                println!("recording guess: {:?}; generations: {}", guess, attempt_generation);
                guesses.push(guess);
                if score.black == solution.len() {
                    return;
                }
                attempted = true;
            }
        }

        if !attempted {
            break;
        }
    }
}

#[allow(dead_code)]
fn solve_with_one_tree(max_attempt_count: usize, solution: &Code, available_colors: &Vec<Color>) {
    let mut guesses = Vec::new();
    let mut tree = Node::new(Code::with_length(solution.len()));

    println!("Solution: {:?}", solution);

    for _attempt_number in 0..max_attempt_count {
        if let Selection(attempt) = tree.select_code(&guesses, available_colors, 0) {
            let score = compute_score(&attempt, solution);
            let guess = Guess::new(attempt, score);
            println!("recording guess: {:?}; tree size: {}", guess, tree.code_count());
            guesses.push(guess);
            if score.black == solution.len() {
                return;
            }
        } else {
            break;
        }
    }
}

#[allow(dead_code)]
fn solve_with_tree_per_attempt(max_attempt_count: usize, solution: &Code, available_colors: &Vec<Color>) {
    let mut guesses = Vec::new();

    println!("Solution: {:?}", solution);

    for _attempt_number in 0..max_attempt_count {
        let mut tree = Node::new(Code::with_length(solution.len()));

        if let Selection(attempt) = tree.select_code(&guesses, available_colors, 0) {
            let score = compute_score(&attempt, solution);
            let guess = Guess::new(attempt, score);
            println!("recording guess: {:?}; tree size: {}", guess, tree.code_count());
            guesses.push(guess);
            if score.black == solution.len() {
                return;
            }
        } else {
            break;
        }
    }
}

fn solve(max_attempt_count: usize, solution: &Code, available_colors: &Vec<Color>) {
    solve_and_bounce_back_from_tree(max_attempt_count, 1000000, solution, available_colors);
}

fn compute_score(a: &Code, b: &Code) -> Score {
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

#[derive(Debug)]
struct Guess {
    code: Code,
    score: Score,
}

#[derive(Debug)]
enum NodeChildren {
    Expanded(Vec<Node>),
    NoChildren,
    Unexpanded,
}
use NodeChildren::*;

struct Node {
    code: Code,
    code_count: usize,
    children: NodeChildren,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
         .field("code", &self.code)
         .field("children", &self.children)
         .finish()
    }
}

impl Guess {
    fn new(code: Code, score: Score) -> Self {
        Guess {
            code,
            score,
        }
    }

    fn matches(&self, code: &Code) -> bool {
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

#[allow(dead_code)]
fn try_select_code(length: usize, previous_guesses: &Vec<Guess>, available_colors: &Vec<Color>) -> Option<Code> {
    let mut attempt = Code::with_length(length);
    let mut open_indexes = (0..length).collect::<Vec<usize>>();
    open_indexes.shuffle(&mut thread_rng());

    while let Some(index) = open_indexes.pop() {
        let color = available_colors[select_random_index(&available_colors).unwrap()];
        attempt.code[index] = color;

        if previous_guesses.iter().any(|guess| !guess.matches(&attempt)) {
            return None;
        }
    }

    Some(attempt)
}

enum SelectionResult {
    Selection(Code),
    DeadBranch,
    Surrender(usize),
}
use SelectionResult::*;

impl Node {
    fn new(code: Code) -> Self {
        Node {
            code,
            children: Unexpanded,
            code_count: 1,
        }
    }

    fn code_count(&self) -> usize {
        self.code_count
    }

    fn select_code(&mut self, previous_guesses: &Vec<Guess>, available_colors: &Vec<Color>, surrender_depth_on_dead_branch: usize) -> SelectionResult {
        // given a node,
        //   a) does it contradict any guesses? if so:
        //     delete it from its parent
        //     set the parent as the current node
        //     goto e (or if easier, just goto a)
        //   b) has it been expanded? if not, expand it now
        //   c) does it have children? 
        //     d) no: return its code
        //     e) yes: select a random child and make that the new node

        if previous_guesses.iter().any(|guess| !guess.matches(&self.code)) {
            return DeadBranch;
        }

        self.expand(available_colors);

        match &mut self.children {
            NoChildren => Selection(self.code.clone()),
            Unexpanded => panic!("ok but what"),
            Expanded(children) => {
                while children.len() > 0 {
                    let index = select_random_index(&children).unwrap();
                    self.code_count -= children[index].code_count();
                    let result = children[index].select_code(previous_guesses, available_colors, surrender_depth_on_dead_branch);

                    match result {
                        Selection(_) => {
                            self.code_count += children[index].code_count();
                            return result;
                        },
                        DeadBranch => {
                            children.remove(index);
                            if surrender_depth_on_dead_branch > 0 {
                                return Surrender(surrender_depth_on_dead_branch - 1);
                            }
                        },
                        Surrender(0) => {
                            self.code_count += children[index].code_count();
                        }
                        Surrender(depth) => {
                            self.code_count += children[index].code_count();
                            return Surrender(depth - 1);
                        }
                    }
                }
                DeadBranch
            },
        }
    }

    fn expand(&mut self, available_colors: &Vec<Color>) {
        if let Unexpanded = self.children {
            self.children = match select_random_matching_index(&self.code.code, '.') {
                None => NoChildren,
                Some(index) => {
                    let mut children = Vec::new();
                    for color in available_colors {
                        let mut new_code = self.code.clone();
                        new_code.code[index] = *color;
                        children.push(Node::new(new_code));
                        self.code_count += 1;
                    }
                    Expanded(children)
                },
            };
        }
    }
}