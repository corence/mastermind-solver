
mod attempt;
mod code;
mod random_index;
mod solver;

use crate::attempt::*;
use crate::code::*;
use crate::random_index::*;
use crate::solver::*;
use crate::solver::algorithm::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::BTreeMap;
use std::env;
use std::fmt;

fn parse_charset(charset: &str) -> Result<Vec<Color>, String> {
    let mut available_colors: Vec<Color> = charset.chars().collect();
    if let Some(pos) = available_colors.iter().position(|c| *c == '.') {
        return Err(format!("charset cannot contain period character"));
    }

    let input_length = available_colors.len();
    available_colors.sort();
    available_colors.dedup();

    if available_colors.len() != input_length {
        return Err(format!("charset cannot contain duplicate characters"));
    }

    Ok(available_colors)
}

fn parse_solution(solution_type: &str, second_argument: &str, available_colors: &Vec<Color>) -> Result<Code, String> {
    match solution_type {
        "generate" => {
            if let Ok(solution_length) = second_argument.parse::<usize>() {
                if solution_length > 10 {
                    Err(format!("okay but that's too crazy: your length shouldn't go over 10 i mean"))
                } else {
                    Ok(Code::generate(solution_length, available_colors))
                }
            } else {
                Err(format!("couldn't parse the length for the generated solution length"))
            }
        },
        "input" => {
            let solution = second_argument;
            let illegal_colors: Vec<Color> = solution.chars()
                .filter(|sc| available_colors.iter().position(|ac| *ac == *sc).is_none())
                .collect();

            if !illegal_colors.is_empty() {
                Err(format!("okay but the solution contains characters not in the charset: {:?}", illegal_colors))
            } else {
                Ok(Code::from_str(solution))
            }
        },
        _ => Err(format!("solution type must be 'generate' or 'input', but was: '{}'", solution_type)),
    }
}

fn parse_algorithm(name: &str, algorithm_constructors: &mut BTreeMap<String, AlgorithmConstructor>) -> Result<AlgorithmConstructor, String> {
    if let Some(constructor) = algorithm_constructors.remove(name) {
        Ok(constructor)
    } else {
        Err(format!("unrecognized algorithm: {}", name))
    }
}

fn parse_args(args: &Vec<String>, algorithm_constructors: &mut BTreeMap<String, AlgorithmConstructor>) -> Result<(Vec<Color>, Code, Box<dyn Algorithm>), String> {
    if args.len() != 5 {
        return Err(format!("number of arguments should be 5, but was {}", args.len()));
    }

    let available_colors = parse_charset(&args[1])?;
    let solution = parse_solution(&args[2], &args[3], &available_colors)?;
    let algorithm_constructor = parse_algorithm(&args[4], algorithm_constructors)?;
    let algorithm = algorithm_constructor(&available_colors, solution.len());

    Ok((available_colors, solution, algorithm))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut algorithm_constructors = solver::algorithm::algorithm_constructors();
    let algorithm_names: Vec<String> = algorithm_constructors.keys().map(|name| name.clone()).collect();

    match parse_args(&env::args().collect(), &mut algorithm_constructors) {
        Ok((available_colors, solution, solver)) => {
            println!("available colors: {:?}", available_colors);
            println!("solution: {:?}", solution);
            println!("selected algorithm: {}", solver.name());
        },
        Err(s) => {
            println!();
            println!("{}", s);
            println!();
            println!("This is a solver for the game called Mastermind.");
            println!();
            println!("usage 1: {} <charset> generate <length> <algorithm>", args[0]);
            println!("  this generates a solution.");
            println!("usage 2: {} <charset> input <solution> <algorithm>", args[0]);
            println!("  this solves the given solution.");
            println!();
            println!("definitions:");
            println!();
            println!("<charset>: the set of possible values for each position in the code. Examples:");
            println!("  abcdefg");
            println!("  rygbovxcfp");
            println!();
            println!("<length>: positive integer. the number of positions in the solution to be generated.");
            println!();
            println!("<solution>: the code for the program to attempt to solve. Each character must be in the <charset>.");
            println!();
            println!("<algorithm>: the choice of solving algorithm. Valid choices:");
            println!("{:#?}", algorithm_names);
        }
    }

    /*
    solve(4, &Code::from_str("abca"), &("abc".chars().collect()));
    println!();
    let ten_chars = "rygbovxcfp".chars().collect();
    solve(12, &Code::generate(6, &ten_chars), &ten_chars);
    println!();
    solve(12, &Code::from_str("rvcrgfgb"), &ten_chars);
    println!();
    solve(12, &Code::generate(10, &ten_chars), &ten_chars);
    */
}

/*
#[allow(dead_code)]
fn solve_without_tree(max_attempt_count: usize, max_attempt_generations: usize, solution: &Code, available_colors: &Vec<Color>) {
    let mut guesses = Vec::new();

    println!("Solution: {:?}", solution);

    for _attempt_number in 0..max_attempt_count {
        let mut attempted = false;
        for attempt_generation in 0..max_attempt_generations {
            if let Some(attempt) = try_select_code(solution.len(), &guesses, available_colors) {
                let score = compute_score(&attempt, solution);
                let guess = Attempt::new(attempt, score);
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
            let guess = Attempt::new(attempt, score);
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
            let guess = Attempt::new(attempt, score);
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
*/

fn solve(max_attempt_count: usize, solution: &Code, available_colors: &Vec<Color>) {
    //solve_with_tree_per_attempt(max_attempt_count, solution, available_colors);
}

#[allow(dead_code)]
fn try_select_code(length: usize, previous_guesses: &Vec<Attempt>, available_colors: &Vec<Color>) -> Option<Code> {
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

