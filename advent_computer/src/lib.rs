#[macro_use] extern crate text_io;

fn read_instruction(input: &Vec<isize>, pointer: usize) -> (usize, Vec<usize>) {
    let opcode = *input.get(pointer).expect("instruction") as usize;
    let parameter_modes = vec!(
        (opcode / 100) % 10,
        (opcode / 1000) % 10,
        (opcode / 10000) % 10,
    );
    return (opcode as usize % 100, parameter_modes);
}

pub fn execute(input: Vec<isize>) -> Result<Vec<isize>, usize> {
    return execute_from(input, 0);
}

const MODE_IMMEDIATE: usize = 1;

fn execute_from(input: Vec<isize>, start_of_program: usize) -> Result<Vec<isize>, usize> {

    let mut input = input.clone();
    let mut running = true;

    let mut instruction_pointer = start_of_program;
    while running {
        let (opcode, param_modes) = read_instruction(&input, instruction_pointer);

        let read_mode = |location, mode| {
            if mode == MODE_IMMEDIATE {
                // direct
                input[instruction_pointer + location]
            } else {
                // pointer
                input[input[instruction_pointer + location] as usize]
            }
        };

        let read = |location| {
            let mode = param_modes[location - 1];
            return read_mode(location, mode)
        };

        let (updated_pointer, return_code) = match opcode {
            1 => {
                let a = read(1);
                let b = read(2);
                let result_pointer = read_mode(3, MODE_IMMEDIATE) as usize;
                input[result_pointer] = a + b;
                (instruction_pointer + 4, 0)
            },
            2 => {
                let a = read(1);
                let b = read(2);
                let result_pointer = read_mode(3, MODE_IMMEDIATE) as usize;
                input[result_pointer] = a * b;
                (instruction_pointer + 4, 0)
            },
            3 => {
                let a: isize = read!(); // read stdin
                let result_pointer = read_mode(1, MODE_IMMEDIATE) as usize;
                input[result_pointer] = a;
                (instruction_pointer + 2, 0)
            },
            4 => {
                let value = read(1);
                println!("{}", value);
                (instruction_pointer + 2, 0)
            }
            99 => {
                running = false;
                (0, 0)
            },
            _ => {
                running = false;
                (0, 1)
            }
        };
    
        instruction_pointer = updated_pointer;

        // if this instruction didn't execute successfully
        if return_code != 0 {
            return Err(return_code);
        }
    }

    return Ok(input);
}


#[cfg(test)]
mod tests {
    use crate::execute;
    use crate::read_instruction;

    #[test]
    fn can_execute_1() {
        let result = execute(vec![1, 0, 0, 0, 99]); // 1 + 1 = 2
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.ok().unwrap()[0], 2);
    }

    #[test]
    fn can_execute_2() {
        let result = execute(vec![2, 3, 0, 3, 99]); // 3 * 2 = 6
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.ok().unwrap()[3], 6);
    }

    #[test]
    fn can_execute_3() {
        let result = execute(vec![2, 4, 4, 5, 99, 0]); // 99 * 99 = 9801
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.ok().unwrap()[5], 9801);
    }

    #[test]
    fn can_execute_4() {
        let result = execute(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.ok().unwrap()[0], 30);
    }

    #[test]
    fn read_min_instruction() {
        let (opcode, parameter_modes) = read_instruction(&vec!(0), 0);
        assert_eq!(opcode, 0);
        assert_eq!(parameter_modes, vec!(0, 0, 0));
    }

    #[test]
    fn read_max_instruction() {
        let (opcode, parameter_modes) = read_instruction(&vec!(11199), 0);
        assert_eq!(opcode, 99);
        assert_eq!(parameter_modes, vec!(1, 1, 1));
    }

    #[test]
    fn can_process_negative_integers() {
        let (_, parameter_modes) = read_instruction(&vec!(1101), 0);
        assert_eq!(parameter_modes, vec![1, 1, 0]);
        let result = execute(vec![1101, 100, -1, 4, 0]);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.ok().unwrap()[4], 99);
    }

    #[test]
    fn read_mixed_param_modes() {
        let (opcode, parameter_modes) = read_instruction(&vec!(1002), 0);
        assert_eq!(opcode, 2);
        assert_eq!(parameter_modes, vec!(0, 1, 0));
    }

    #[test]
    fn can_use_correct_modes() {
        let result = execute(vec![1002,4,3,4,33]);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.ok().unwrap()[4], 99);
    }
}
