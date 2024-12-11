use std::{collections::HashMap, fs::File, io::{BufReader, Read}};

pub fn solve1() -> usize {
    let file = File::open("src/day11/input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_to_string(&mut line).unwrap();
    
    let mut stones: Vec<usize> = line.split_ascii_whitespace().map(|x| usize::from_str_radix(x, 10).unwrap()).collect();

    for _ in 0..25 {
        let mut new_stones = vec![];
        for i in 0..stones.len() {
            let stone = stones[i];
            
            if stone == 0 {
                new_stones.push(1);
                continue;
            }

            let num_digits = stone.ilog10() + 1;
            
            if num_digits % 2 == 0 {
                let divisor = 10_usize.pow(num_digits / 2);
                let first = stone / divisor;
                let second = stone % divisor;
                new_stones.push(first);
                new_stones.push(second);
            }
            else {
                new_stones.push(stone*2024);
            }
        }
        stones = new_stones;
    }
    stones.len()
}

pub fn solve2() -> usize {
    let file = File::open("src/day11/input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_to_string(&mut line).unwrap();
    
    let stones: Vec<(usize, usize)> = line.split_ascii_whitespace().map(|x| (usize::from_str_radix(x, 10).unwrap(), 1)).collect();
    let mut histogram: HashMap<usize, usize> = stones.into_iter().collect();

    for _ in 0..75 {
        let saved_histogram = histogram.clone();
        for (k, v) in saved_histogram {
            if v != 0 {
                if k == 0 {
                    *histogram.entry(0).or_insert(v) -= v;
                    *histogram.entry(1).or_insert(0) += v;
                    continue;
                }
                
                let num_digits = k.ilog10() + 1;

                if num_digits % 2 == 0 {
                    let divisor = 10_usize.pow(num_digits / 2);
                    let first = k / divisor;
                    let second = k % divisor;
                    *histogram.entry(k).or_insert(v) -= v;
                    *histogram.entry(first).or_insert(0) += v;
                    *histogram.entry(second).or_insert(0) += v;
                }
                else {
                    *histogram.entry(k).or_insert(v) -= v;
                    *histogram.entry(k*2024).or_insert(0) += v;
                }
            }
        }
    }
    
    let mut sum = 0;
    for (_, v) in histogram {
        sum += v;
    }
    sum
}
