use std::fs;

use advent_computer::execute;

fn main() {
    let program = load_program("input.txt");
    let result = execute(program).expect("valid execution");
}

fn load_program(filename: &str) -> Vec<isize> {
    let contents = fs::read_to_string(filename).expect("unable to read file");
    return contents.split(",").map(|i| i.trim().parse().expect("parse error")).collect();
}
