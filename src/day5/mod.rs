use std::{fs::File, io::{BufRead, BufReader}};

pub fn solve1() -> usize {
    let file = File::open("src/day5/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut pairs = vec![];
    let mut page_updates = vec![];

    let mut correct_updates = vec![];

    let mut total_of_middles = 0;

    let mut reading_pairs = true;
    for line in reader.lines() {
        let line_clone = line.unwrap().clone();
        if line_clone == "" {
            reading_pairs = false;
            continue;
        }
        if reading_pairs {
            let pair: Vec<&str> = line_clone.split("|").collect();
            let num1 = usize::from_str_radix(pair.get(0).unwrap(), 10).unwrap();
            let num2 = usize::from_str_radix(pair.get(1).unwrap(), 10).unwrap();
            pairs.push((num1, num2));
        }
        else {
            let mut page_nums = vec![];
            let pages: Vec<&str> = line_clone.split(",").collect();
            for page in pages {
                let page_num = usize::from_str_radix(page, 10).unwrap();
                page_nums.push(page_num);
            }
            page_updates.push(page_nums);
        }
    }

    for update in page_updates {
        let mut correct = true;
        let mut already_seen = vec![];        
        for page_num in &update {
            for pair in &pairs {
                if pair.0 == *page_num && already_seen.contains(&pair.1) {
                    correct = false;
                    break;
                }
            }
            if !correct {
                break;
            }
            already_seen.push(*page_num);
        }
        if correct {
            correct_updates.push(update);
        }
    }

    for update in correct_updates {
        let middle = update.len() / 2;
        total_of_middles += update.get(middle).unwrap();
    }
    total_of_middles
}

pub fn solve2() -> usize {
    let file = File::open("src/day5/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut pairs = vec![];
    let mut page_updates = vec![];

    let mut incorrect_updates = vec![];

    let mut total_of_middles = 0;

    let mut reading_pairs = true;
    for line in reader.lines() {
        let line_clone = line.unwrap().clone();
        if line_clone == "" {
            reading_pairs = false;
            continue;
        }
        if reading_pairs {
            let pair: Vec<&str> = line_clone.split("|").collect();
            let num1 = usize::from_str_radix(pair.get(0).unwrap(), 10).unwrap();
            let num2 = usize::from_str_radix(pair.get(1).unwrap(), 10).unwrap();
            pairs.push((num1, num2));
        }
        else {
            let mut page_nums = vec![];
            let pages: Vec<&str> = line_clone.split(",").collect();
            for page in pages {
                let page_num = usize::from_str_radix(page, 10).unwrap();
                page_nums.push(page_num);
            }
            page_updates.push(page_nums);
        }
    }

    for update in page_updates {
        let mut correct = true;
        let mut already_seen = vec![];        
        for page_num in &update {
            for pair in &pairs {
                if pair.0 == *page_num && already_seen.contains(&pair.1) {
                    correct = false;
                    break;
                }
            }
            if !correct {
                break;
            }
            already_seen.push(*page_num);
        }
        if !correct {
            incorrect_updates.push(update);
        }
    }

    for update in incorrect_updates {
        let mut sorted_update = update.clone();
        for _ in 0..4 { //yeah, idk either
            for pair in &pairs {
                if let (Some(index0), Some(index1)) = (sorted_update.iter().position(|x| x == &pair.0), sorted_update.iter().position(|x| x == &pair.1)) {
                    if index0 > index1 {
                        sorted_update.swap(index0, index1);
                    }
                }
            }
        }
        let middle = sorted_update.len() / 2;
        total_of_middles += *sorted_update.get(middle).unwrap();
    }
    total_of_middles
}
