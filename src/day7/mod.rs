use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

use itertools::{repeat_n, Itertools};

pub fn solve1() -> usize {
    let file = File::open("src/day7/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();
        let subdiv: Vec<&str> = line_clone.split(": ").collect();
        let result_str = *subdiv.first().unwrap();
        let operands_str: Vec<&str> =  (*subdiv.last().unwrap()).split(" ").collect();

        let result = usize::from_str_radix(result_str, 10).unwrap();
        let mut operands = vec![];
        for op in operands_str {
            let operand = usize::from_str_radix(op, 10).unwrap();
            operands.push(operand);
        }

        let oplist_len = operands.len()-1;
        let possible_combinations = 1 << oplist_len;
        let mut comb_list: Vec<Vec<bool>> = vec![];

        //generate possible operators
        for i in 0..possible_combinations {
            let mut combination = vec![];
            for j in 0..oplist_len {
                //check if jth bit of i is set
                combination.push((i & (1 << j)) != 0); //false = +, true = *
            }
            comb_list.push(combination);
        }

        //try all possible operators
        for combination in comb_list {
            let mut operation_result = *operands.first().unwrap();
            let mut operator_index = 0;
            for operand in &operands[1..] {
                if *combination.get(operator_index).unwrap() {
                    operation_result *= operand;
                }
                else {
                    operation_result += operand;
                }
                operator_index += 1;
            }

            if operation_result == result {
                total += result;
                break;
            }
        }
    }

    total
}

pub fn solve2() -> usize {
    let file = File::open("src/day7/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;

    let mut permutation_map: HashMap<usize, Vec<Vec<&i32>>> = HashMap::new(); 
    let options = [0, 1, 2]; //0 = *, 1 = +, 2 = ||

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();
        let subdiv: Vec<&str> = line_clone.split(": ").collect();
        let result_str = *subdiv.first().unwrap();
        let operands_str: Vec<&str> =  (*subdiv.last().unwrap()).split(" ").collect();

        let result = usize::from_str_radix(result_str, 10).unwrap();
        let mut operands = vec![];
        for op in operands_str {
            let operand = usize::from_str_radix(op, 10).unwrap();
            operands.push(operand);
        }

        //generate possible combinations
        let oplist_len: usize = operands.len()-1;
        let mut comb_list= vec![];
        if permutation_map.contains_key(&oplist_len) {
            comb_list = permutation_map.get(&oplist_len).unwrap().clone();
        }
        else {
            for perm in repeat_n(options.iter(), oplist_len).multi_cartesian_product() {
                comb_list.push(perm);
            }
            permutation_map.insert(oplist_len, comb_list.clone());
        }

        //try all combinations
        for combination in comb_list {
            let mut operation_result = *operands.first().unwrap();
            let mut operator_index = 0;
            for operand in &operands[1..] {
                let operator = **combination.get(operator_index).unwrap();
                if operator == 0 {
                    operation_result *= operand;
                    if operation_result > result {
                        break;
                    }
                }
                else if operator == 1 {
                    operation_result += operand;
                    if operation_result > result {
                        break;
                    }
                }
                else {
                    let mut result_string = operation_result.to_string();
                    let operand_string = operand.to_string();
                    result_string.push_str(operand_string.as_str());
                    operation_result = usize::from_str_radix(&result_string, 10).unwrap();
                    if operation_result > result {
                        break;
                    }
                }
                operator_index += 1;
            }

            if operation_result == result {
                total += result;
                break;
            }
        }
    }


    total
}