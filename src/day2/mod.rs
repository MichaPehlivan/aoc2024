use std::{fs::File, io::{BufRead, BufReader}};

pub fn solve1() -> usize {
    let file = File::open("src/day2/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut safe_count = 0;

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        let mut levels = vec![];
        let levels_str: Vec<&str> = line_clone.split(" ").collect();
        for level in levels_str {
            let num = isize::from_str_radix(level, 10).unwrap();
            levels.push(num);
        }

        let mut safe = true;
        let increase_sign = (levels.get(1).unwrap() - levels.get(0).unwrap()).signum();
        for i in 1..levels.len() {
            let diff = levels.get(i).unwrap() - levels.get(i-1).unwrap();
            if diff == 0 || diff < -3 || diff > 3 {
                safe = false;
                break;
            } 
            if diff.signum() != increase_sign {
                safe = false;
                break;
            }
        }

        if safe {
            safe_count += 1;
        }
    }
    safe_count
}

pub fn solve2() -> usize {
    let file = File::open("src/day2/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut safe_count = 0;

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        let mut levels = vec![];
        let levels_str: Vec<&str> = line_clone.split(" ").collect();
        for level in levels_str {
            let num = isize::from_str_radix(level, 10).unwrap();
            levels.push(num);
        }

        let mut safe = true;
        let mut increase_sign = (levels.get(1).unwrap() - levels.get(0).unwrap()).signum();
        for i in 1..levels.len() {
            let diff = levels.get(i).unwrap() - levels.get(i-1).unwrap();
            if diff == 0 || diff < -3 || diff > 3 {
                safe = false;
            } 
            if diff.signum() != increase_sign {
                safe = false;
            }
        }

        if !safe {
            let mut alternate_levels = vec![];
            for j in 0..levels.len() {
                let mut levels_clone = levels.clone();
                levels_clone.remove(j);
                alternate_levels.push(levels_clone);
            }

            for levels in alternate_levels {
                safe = true;
                increase_sign = (levels.get(1).unwrap() - levels.get(0).unwrap()).signum();
                for i in 1..levels.len() {
                    let diff = levels.get(i).unwrap() - levels.get(i-1).unwrap();
                    if diff == 0 || diff < -3 || diff > 3 {
                        safe = false;
                    } 
                    if diff.signum() != increase_sign {
                        safe = false;
                    }
                }
                if safe {
                    break;
                }
            }
        }

        if safe {
            safe_count += 1;
        }
    }
    safe_count
}