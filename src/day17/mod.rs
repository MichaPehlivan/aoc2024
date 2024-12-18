use std::{fs::File, io::{BufRead, BufReader}};

pub fn solve1() -> Vec<usize> {
    let file = File::open("src/day17/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut reg_a = 0;
    let mut reg_b = 0;
    let mut reg_c = 0;
    let mut program = vec![];
    let mut output = vec![];
    let mut program_counter = 0;
    
    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        if line_clone.starts_with("Register A") {
            reg_a = usize::from_str_radix(line_clone.split(": ").last().unwrap(), 10).unwrap();
        }
        else if line_clone.starts_with("Register B") {
            reg_b = usize::from_str_radix(line_clone.split(": ").last().unwrap(), 10).unwrap();
        }
        else if line_clone.starts_with("Register C") {
            reg_c = usize::from_str_radix(line_clone.split(": ").last().unwrap(), 10).unwrap();
        }
        else if line_clone.starts_with("Program") {
            let program_oppcodes: Vec<&str> = line_clone.split(": ").last().unwrap().split(",").collect();
            for oppcode in program_oppcodes {
                let code = usize::from_str_radix(oppcode, 10).unwrap();
                program.push(code);
            }
        }
    }

    loop {
        if program_counter+1 >= program.len() {
            break;
        }
        let oppcode = program[program_counter];
        let operand = program[program_counter+1];
        match oppcode {
            0 => {
                let num = reg_a;
                let combo = match operand {
                    0 | 1 | 2 | 3 => operand,
                    4 => reg_a,
                    5 => reg_b,
                    6 => reg_c,
                    7 => panic!("Reserved combo operand!"),
                    _=> panic!("Invalid operand!")
                };
                let den = 2_usize.pow(combo as u32);
                reg_a = num / den;
                program_counter += 2;
            },
            1 => {
                reg_b ^= operand;
                program_counter += 2;
            },
            2 => {
                let combo = match operand {
                    0 | 1 | 2 | 3 => operand,
                    4 => reg_a,
                    5 => reg_b,
                    6 => reg_c,
                    7 => panic!("Reserved combo operand!"),
                    _=> panic!("Invalid operand!")
                };
                reg_b = combo % 8;
                program_counter += 2;
            },
            3 => {
                if reg_a != 0 {
                    program_counter = operand;
                }
                else {
                    program_counter += 2;
                }
            },
            4 => {
                reg_b ^= reg_c;
                program_counter += 2;
            },
            5 => {
                let combo = match operand {
                    0 | 1 | 2 | 3 => operand,
                    4 => reg_a,
                    5 => reg_b,
                    6 => reg_c,
                    7 => panic!("Reserved combo operand!"),
                    _=> panic!("Invalid operand!")
                };
                output.push(combo % 8);
                program_counter += 2;
            },
            6 => {
                let num = reg_a;
                let combo = match operand {
                    0 | 1 | 2 | 3 => operand,
                    4 => reg_a,
                    5 => reg_b,
                    6 => reg_c,
                    7 => panic!("Reserved combo operand!"),
                    _=> panic!("Invalid operand!")
                };
                let den = 2_usize.pow(combo as u32);
                reg_b = num / den;
                program_counter += 2;
            },
            7 => {
                let num = reg_a;
                let combo = match operand {
                    0 | 1 | 2 | 3 => operand,
                    4 => reg_a,
                    5 => reg_b,
                    6 => reg_c,
                    7 => panic!("Reserved combo operand!"),
                    _=> panic!("Invalid operand!")
                };
                let den = 2_usize.pow(combo as u32);
                reg_c = num / den;
                program_counter += 2;
            },
            _=> panic!("Invalid opcode!")
        }
    }
    output
}
