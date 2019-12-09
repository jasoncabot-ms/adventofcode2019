use std::fs;

use advent_computer::execute;

fn main() {
    let program = load_program("input.txt");

    for noun in 0..99 {
        for verb in 0..99 {
            let mut test_case = program.clone();
            test_case.remove(1);
            test_case.insert(1, noun);
            test_case.remove(2);
            test_case.insert(2, verb);
            let result = execute(test_case).expect("valid execution");
            let result = result[0];
            if result == 19690720 {
                println!("{} and {} = {}", noun, verb, result);
                println!("Answer = {}", 100 * noun + verb);
                return;
            }
        }
    }
}

fn load_program(filename: &str) -> Vec<isize> {
    let contents = fs::read_to_string(filename).expect("unable to read file");
    return contents.split(",").map(|i| i.parse().expect("parse error")).collect();
}
