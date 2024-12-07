use std::{fs::File, io::{BufRead, BufReader}};

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

        if check_possible_1(&operands, 0, result, 0) {
            total += result;
        }
    }

    total
}

pub fn solve2() -> usize {
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

        if check_possible_2(&operands, 0, result, 0) {
            total += result;
        }
    }
    total
}

fn check_possible_1(operands: &Vec<usize>, current_result: usize, target_result: usize, index: usize) -> bool {
    if index == operands.len() {
        return current_result == target_result;
    }
    else if current_result > target_result {
        return false;
    }
    let next = operands.get(index).unwrap().clone();
    if check_possible_1(operands, current_result+next, target_result, index+1) {
        return true;
    }
    if check_possible_1(operands, current_result*next, target_result, index+1) {
        return true;
    }
    false
}

fn check_possible_2(operands: &Vec<usize>, current_result: usize, target_result: usize, index: usize) -> bool {
    if current_result > target_result {
        return false;
    }
    if index == operands.len() {
        return current_result == target_result;
    }
    let next = operands[index];
    if check_possible_2(operands, current_result+next, target_result, index+1) {
        return true;
    }
    if check_possible_2(operands, current_result*next, target_result, index+1) {
        return true
    }
    let next_digits = next.ilog10() + 1;
    if check_possible_2(operands, current_result * 10_usize.pow(next_digits) + next, target_result, index+1) {
        return true;
    }
    false
}