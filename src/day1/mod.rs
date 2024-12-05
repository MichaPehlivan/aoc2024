use std::{fs::File, io::{BufRead, BufReader}};

pub fn solve1() -> usize {
    let file = File::open("src/day1/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut list1 = vec![];
    let mut list2 = vec![];
    let mut dist_sum = 0;

    //put numbers into lists
    for line in reader.lines() {
        let line_clone = line.unwrap().clone();
        let numbers: Vec<&str> = line_clone.split("   ").collect();
        let number1 = usize::from_str_radix(numbers.get(0).unwrap(), 10).unwrap();
        let number2 = usize::from_str_radix(numbers.get(1).unwrap(), 10).unwrap();
        list1.push(number1);
        list2.push(number2);
    }

    //sort
    list1.sort();
    list2.sort();

    for i in 0..list1.len() {
        let num1 = list1.get(i).unwrap();
        let num2 = list2.get(i).unwrap();
        dist_sum += num1.abs_diff(*num2);
    }
    dist_sum
}

pub fn solve2() -> usize {
    let file = File::open("src/day1/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut list1 = vec![];
    let mut list2 = vec![];
    let mut sim_sum = 0;

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();
        let numbers: Vec<&str> = line_clone.split("   ").collect();
        let number1 = usize::from_str_radix(numbers.get(0).unwrap(), 10).unwrap();
        let number2 = usize::from_str_radix(numbers.get(1).unwrap(), 10).unwrap();
        list1.push(number1);
        list2.push(number2);
    }

    for number in list1 {
        let count = list2.iter().filter(|n| *n == &number).count();
        sim_sum += number*count;
    }
    sim_sum
}