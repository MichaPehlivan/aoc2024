use std::{fs::File, io::{BufRead, BufReader}};
use regex::Regex;

pub fn solve1() -> usize {
    let file = File::open("src/day3/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;

    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex!");

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        for capture in re.captures_iter(&line_clone) {
            let a = &capture[1];
            let b = &capture[2];
            let a_num = usize::from_str_radix(a, 10).unwrap();
            let b_num = usize::from_str_radix(b, 10).unwrap();
            total += a_num*b_num;
        }

    }
    total
}

pub fn solve2() -> usize {
    let file = File::open("src/day3/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;
    let mut command_vec = vec![];

    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").expect("Invalid regex!");

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        for capture in re.captures_iter(&line_clone) {
            let matched = capture[0].to_string();
            command_vec.push(matched);
        }
    }

    let remul = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex!");

    let mut enabled = true;
    for command in command_vec {
        match command.as_str() {
            "don't()" => enabled = false,
            "do()" => enabled = true,
            _ => {
                if enabled {
                    let capture = remul.captures(&command).unwrap();
                    let a = &capture[1];
                    let b = &capture[2];
                    let a_num = usize::from_str_radix(a, 10).unwrap();
                    let b_num = usize::from_str_radix(b, 10).unwrap();
                    total += a_num*b_num;
                }
            },
        }
    }
    total
}