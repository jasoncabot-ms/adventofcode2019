use std::fs;

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

fn execute(input: Vec<usize>) -> Result<Vec<usize>, usize> {

    let mut input = input.clone();
    let mut running = true;
    let mut exit_code = 0;
    let start_of_program = 0;

    let mut instruction_pointer = start_of_program;
    while running {
        let instruction = input.get(instruction_pointer).expect("instruction");

        let updated_pointer = match instruction {
            1 => {
                let a = input[input[instruction_pointer + 1]];
                let b = input[input[instruction_pointer + 2]];
                let result_pointer = input[instruction_pointer + 3];
                input.splice(result_pointer..result_pointer + 1, vec![a + b]);
                instruction_pointer + 4
            },
            2 => {
                let a = input[input[instruction_pointer + 1]];
                let b = input[input[instruction_pointer + 2]];
                let result_pointer = input[instruction_pointer + 3];
                input.splice(result_pointer..result_pointer + 1, vec![a * b]);
                instruction_pointer + 4
            },
            99 => {
                running = false;
                exit_code = 0;
                0
            },
            _ => {
                running = false;
                exit_code = 1;
                0
            }
        };
    
        instruction_pointer = updated_pointer;
    }

    if exit_code != 0 {
        return Err(exit_code);
    }

    return Ok(input);
}

fn load_program(filename: &str) -> Vec<usize> {
    let contents = fs::read_to_string(filename).expect("unable to read file");
    return contents.split(",").map(|i| i.parse().expect("parse error")).collect();
}
