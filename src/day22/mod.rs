use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

pub fn solve1() -> usize {
    let file = File::open("src/day22/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut initial = vec![];

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        initial.push(usize::from_str_radix(&line_clone, 10).unwrap());
    }

    let mut total = 0;

    for buyer in initial {
        let mut num = buyer;
        for _ in 0..2000 {
            num ^= num*64;
            num %= 16777216;
            num ^= num/32;
            num %= 16777216;
            num ^= num*2048;
            num %= 16777216;
        }
        total += num;
    }
    total
}

pub fn solve2() -> isize {
    let file = File::open("src/day22/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut initial = vec![];

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        initial.push(usize::from_str_radix(&line_clone, 10).unwrap());
    }

    let mut price_delta: Vec<(isize, isize)> = vec![];
    let mut sequence_prizes: HashMap<[isize; 4], isize> = HashMap::default();   

    for buyer in initial {
        let mut num = buyer;
        let mut prev_price = (num % 10) as isize;
        for _ in 0..2000 {
            num ^= num*64;
            num %= 16777216;
            num ^= num/32;
            num %= 16777216;
            num ^= num*2048;
            num %= 16777216;
            let price = (num % 10) as isize;
            price_delta.push((price, price - prev_price));
            prev_price = price;
        }

        let delta_vec = price_delta.iter().map(|(_, delta)| *delta).collect::<Vec<isize>>();
        let mut already_seen = vec![];
        for (position, window) in delta_vec.windows(4).enumerate() {
            if !already_seen.contains(&window) {
                *sequence_prizes.entry(window.try_into().unwrap()).or_insert(0) += price_delta[position+3].0;
                already_seen.push(window);
            }
        }

        price_delta = vec![];
    }

    *sequence_prizes.values().max().unwrap()
}