use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}};

pub fn solve1() -> usize {
    let file = File::open("src/day19/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut towels: HashSet<String> = HashSet::new();
    let mut patterns: Vec<String> = vec![];
    let mut reading_towels = true;

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        if line_clone == "" {
            reading_towels = false;
            continue;
        }

        if reading_towels {
            towels = line_clone.split(", ").map(|x| x.to_string()).collect();
        }
        else {
            patterns.push(line_clone.clone());
        }
    }

    let mut possible_count = 0;

    for pattern in patterns {
        if check_possible(pattern, &towels) {
            possible_count += 1;
        }
    }
    possible_count
}

pub fn solve2() -> usize {
    let file = File::open("src/day19/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut towels: HashSet<String> = HashSet::new();
    let mut patterns: Vec<String> = vec![];
    let mut reading_towels = true;

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        if line_clone == "" {
            reading_towels = false;
            continue;
        }

        if reading_towels {
            towels = line_clone.split(", ").map(|x| x.to_string()).collect();
        }
        else {
            patterns.push(line_clone.clone());
        }
    }

    let mut possible_count = 0;

    for pattern in patterns {
        possible_count += count_possible(pattern, &towels);
    }
    possible_count
}

fn check_possible(pattern: String, towels: &HashSet<String>) -> bool {
    let n = pattern.len();
    let mut dp = [false; 65];
    dp[0] = true;

    for i in 1..n+1 {
        for j in 0..i {
            if dp[j] && towels.contains(&pattern[j..i]) {
                dp[i] = true;
                break;
            }
        }
    }

    return dp[n];
}

fn count_possible(pattern: String, towels: &HashSet<String>) -> usize {
    let n = pattern.len();
    let mut dp = [0; 65];
    dp[0] = 1;

    for i in 1..n+1 {
        for j in 0..i {
            if towels.contains(&pattern[j..i]) {
                dp[i] += dp[j];
            }
        }
    }

    return dp[n];
}