#[macro_use] extern crate text_io;

pub fn execute(input: Vec<usize>) -> Result<Vec<usize>, usize> {

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
            3 => {
                let a: usize = read!(); // read stdin
                let result_pointer = input[instruction_pointer + 1];
                input.splice(result_pointer..result_pointer + 1, vec![a]);
                instruction_pointer + 2
            },
            4 => {
                let value = input[input[instruction_pointer + 1]];
                println!("{}", value);
                instruction_pointer + 2
            }
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


#[cfg(test)]
mod tests {
    use crate::execute;

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

    // #[test]
    // fn input_prompt() {
    //     let result = execute(vec![3, 0, 99]);
    //     assert_eq!(result.is_ok(), true);
    //     assert_eq!(result.ok().unwrap()[0], 1);
    // }
}
