use std::{fs::File, io::{BufRead, BufReader}};

use regex::Regex;

pub fn solve1() -> f64 {
    let file = File::open("src/day13/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut inputs: Vec<((isize, isize), (isize, isize), (isize, isize))> = vec![];

    let mut total: f64 = 0_f64;

    let mut button_a = (0, 0);
    let mut button_b = (0, 0);
    let mut prize;

    let re_ab = Regex::new(r"X\+(\d+), Y\+(\d+)").expect("Invalid regex!");
    let re_p = Regex::new(r"X=(\d+), Y=(\d+)").expect("Invalid regex!");

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        if line_clone.starts_with("Button A") {
            let capture = re_ab.captures(&line_clone).unwrap();
            let x = isize::from_str_radix(&capture[1], 10).unwrap();
            let y = isize::from_str_radix(&capture[2], 10).unwrap();
            button_a = (x, y);
        }
        else if line_clone.starts_with("Button B") {
            let capture = re_ab.captures(&line_clone).unwrap();
            let x = isize::from_str_radix(&capture[1], 10).unwrap();
            let y = isize::from_str_radix(&capture[2], 10).unwrap();
            button_b = (x, y);
        }
        else if line_clone.starts_with("Prize") {
            let capture = re_p.captures(&line_clone).unwrap();
            let x = isize::from_str_radix(&capture[1], 10).unwrap();
            let y = isize::from_str_radix(&capture[2], 10).unwrap();
            prize = (x, y);
            inputs.push((button_a, button_b, prize));
        }
    }
    
    for ((a, c), (b, d), (p, q)) in inputs {
        let den = a*d-b*c;

        if den != 0 {
            let numx: f64 = ((d*p-b*q) as f64)/(den as f64);
            let numy: f64 = ((a*q-c*p) as f64)/(den as f64);

            if numx > 0_f64 && numy > 0_f64 && numx.fract() == 0_f64 && numy.fract() == 0_f64 {
                let price = 3_f64*numx + numy;
                println!("price: {price}, A {numx} times + B {numy} times");
                total += price;
            }
        }
    }
    total
}

pub fn solve2() -> f64 {
    let file = File::open("src/day13/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut inputs: Vec<((isize, isize), (isize, isize), (isize, isize))> = vec![];

    let mut total: f64 = 0_f64;

    let mut button_a = (0, 0);
    let mut button_b = (0, 0);
    let mut prize;

    let re_ab = Regex::new(r"X\+(\d+), Y\+(\d+)").expect("Invalid regex!");
    let re_p = Regex::new(r"X=(\d+), Y=(\d+)").expect("Invalid regex!");

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        if line_clone.starts_with("Button A") {
            let capture = re_ab.captures(&line_clone).unwrap();
            let x = isize::from_str_radix(&capture[1], 10).unwrap();
            let y = isize::from_str_radix(&capture[2], 10).unwrap();
            button_a = (x, y);
        }
        else if line_clone.starts_with("Button B") {
            let capture = re_ab.captures(&line_clone).unwrap();
            let x = isize::from_str_radix(&capture[1], 10).unwrap();
            let y = isize::from_str_radix(&capture[2], 10).unwrap();
            button_b = (x, y);
        }
        else if line_clone.starts_with("Prize") {
            let capture = re_p.captures(&line_clone).unwrap();
            let x = isize::from_str_radix(&capture[1], 10).unwrap();
            let y = isize::from_str_radix(&capture[2], 10).unwrap();
            prize = (x+10000000000000, y+10000000000000);
            inputs.push((button_a, button_b, prize));
        }
    }
    
    for ((a, c), (b, d), (p, q)) in inputs {
        let den = a*d-b*c;

        if den != 0 {
            let numx: f64 = ((d*p-b*q) as f64)/(den as f64);
            let numy: f64 = ((a*q-c*p) as f64)/(den as f64);

            if numx > 0_f64 && numy > 0_f64 && numx.fract() == 0_f64 && numy.fract() == 0_f64 {
                let price = 3_f64*numx + numy;
                total += price;
            }
        }
    }
    total
}