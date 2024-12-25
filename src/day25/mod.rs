use std::{fs::File, io::{BufRead, BufReader}};

pub fn solve1() -> usize {
    let file = File::open("src/day25/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut locks = vec![];
    let mut keys = vec![];

    let mut buffer: [isize; 5] = [0; 5];

    let mut reading_type = true;
    let mut reading_lock = true;
    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        if line_clone == "" {
            reading_type = true;

            if reading_lock {
                locks.push(buffer);
                buffer = [0; 5];
            }
            else {
                keys.push(buffer);
                buffer = [0; 5];
            }

            continue;
        }

        let mut col = 0;
        for c in line_clone.chars() {
            if reading_type {

                if c == '#' {
                    reading_lock = true;
                }
                else {
                    reading_lock = false;
                    buffer = [-1; 5];
                }
                reading_type = false;
                break;
            }
            else if reading_lock && c == '#' {
                buffer[col] += 1;
            }
            else if !reading_lock && c == '#' {
                buffer[col] += 1;
            }
            col += 1;
        }
    }
    if reading_lock {
        locks.push(buffer);
    }
    else {
        keys.push(buffer);
    }
    
    let mut fitting_pairs = 0;

    for key in keys {
        for lock in &locks {
            let mut fitting = true;
            for i in 0..5 {
                if key[i] + lock[i] > 5 {
                    fitting = false;
                    break;
                }
            }

            if fitting {
                fitting_pairs += 1;
            }
        }
    }

    fitting_pairs
}