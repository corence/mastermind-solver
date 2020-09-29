
mod attempt;
mod code;
mod random_index;
mod solver;

use crate::attempt::*;
use crate::code::*;
use crate::random_index::*;
use crate::solver::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::BTreeMap;
use std::env;

fn parse_charset(charset: &str) -> Result<Vec<Color>, String> {
    let mut available_colors: Vec<Color> = charset.as_bytes().to_vec();
    if let Some(_) = available_colors.iter().position(|c| *c == b'.') {
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
            let illegal_colors: Vec<Color> = solution.as_bytes().iter()
                .map(|sc| *sc)
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

fn parse_solver_name(name: &str, solver_constructors: &mut BTreeMap<String, SolverConstructor>) -> Result<SolverConstructor, String> {
    if let Some(constructor) = solver_constructors.remove(name) {
        Ok(constructor)
    } else {
        Err(format!("unrecognized solver name: {}", name))
    }
}

fn parse_args(args: &Vec<String>, solver_constructors: &mut BTreeMap<String, SolverConstructor>) -> Result<(Vec<Color>, Code, Box<dyn Solver>), String> {
    if args.len() != 5 {
        return Err(format!("number of arguments should be 5, but was {}", args.len()));
    }

    let available_colors = parse_charset(&args[1])?;
    let solution = parse_solution(&args[2], &args[3], &available_colors)?;
    let solver_constructor = parse_solver_name(&args[4], solver_constructors)?;
    let solver = solver_constructor(&available_colors, solution.len());

    Ok((available_colors, solution, solver))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut solver_constructors = solver::solver_constructors();
    let solver_names: Vec<String> = solver_constructors.keys().map(|name| name.clone()).collect();

    match parse_args(&env::args().collect(), &mut solver_constructors) {
        Ok((available_colors, solution, mut solver)) => {
            let max_attempt_count = 10;
            println!("available colors: {}", std::str::from_utf8(&available_colors).unwrap());
            println!("solution: {:?}", solution);
            println!("selected solver: {}", solver.name());
            println!("max attempt count: {}", max_attempt_count);

            let submission = solve(&mut (*solver), max_attempt_count, &solution);
            println!("submission: {:?}", submission);
        },
        Err(s) => {
            println!();
            println!("{}", s);
            println!();
            println!("This is a solver for the game called Mastermind.");
            println!();
            println!("usage 1: {} <charset> generate <length> <solver>", args[0]);
            println!("  this generates a solution.");
            println!("usage 2: {} <charset> input <solution> <solver>", args[0]);
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
            println!("<solver>: the choice of solver. Valid choices:");
            println!("{:#?}", solver_names);
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

fn solve(solver: &mut dyn Solver, max_attempt_count: usize, solution: &Code) -> Option<Code> {
    for _attempt_number in 0..max_attempt_count {
        if let Some(candidate) = solver.generate_candidate() {
            let score = compute_score(&candidate, solution);
            println!("recording guess: {:?}", (&candidate, score));
            if score.black == solution.len() {
                return Some(candidate);
            }
            solver.record_attempt(&candidate, score);
        } else {
            break;
        }
    }

    None
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

