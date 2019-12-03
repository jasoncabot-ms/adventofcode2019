use std::fs;

fn main() {
    let program = load_program("input.txt");

    let result = execute(program).expect("unsuccessful execution");

    println!("{:?}", result);
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
                println!("exiting");
                running = false;
                exit_code = 0;
                0
            },
            _ => {
                println!("invalid instruction");
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
