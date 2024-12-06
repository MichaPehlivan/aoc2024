use std::{fs::File, io::{BufRead, BufReader}};

pub fn solve1() -> usize {
    let file = File::open("src/day6/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut unique_positions: Vec<(usize, usize)> = vec![];
    let mut guard_dir = 0; //north: 0, east: 1, south: 2, west: 3
    let mut guard_pos: (usize, usize) = (0, 0);
    let mut grid: [[char; 130]; 130] = [['.'; 130]; 130];

    let mut row_index = 0;
    for line in reader.lines() {
        let line_clone = line.unwrap().clone();
        
        let mut col_index = 0;
        for c in line_clone.chars() {
            if c == '#' {
                grid[col_index][row_index] = '#';
            }
            if c == '^' {
                guard_pos = (col_index, row_index);
            }
            col_index += 1;
        }
        row_index += 1;
    }
    unique_positions.push(guard_pos);

    loop {
        match guard_dir {
            0 => {
                if guard_pos.1 == 0 {
                    break;
                }
                if grid[guard_pos.0][guard_pos.1-1] == '#' {
                    guard_dir = 1;
                }
                else {
                    guard_pos = (guard_pos.0, guard_pos.1-1);
                    if !unique_positions.contains(&guard_pos) {
                        unique_positions.push(guard_pos);
                    }
                }
            },
            1 => {
                if guard_pos.0 == 129 {
                    break;
                }
                if grid[guard_pos.0+1][guard_pos.1] == '#' {
                    guard_dir = 2;
                }
                else {
                    guard_pos = (guard_pos.0+1, guard_pos.1);
                    if !unique_positions.contains(&guard_pos) {
                        unique_positions.push(guard_pos);
                    }
                }
            },
            2 => {
                if guard_pos.1 == 129 {
                    break;
                }
                if grid[guard_pos.0][guard_pos.1+1] == '#' {
                    guard_dir = 3;
                }
                else {
                    guard_pos = (guard_pos.0, guard_pos.1+1);
                    if !unique_positions.contains(&guard_pos) {
                        unique_positions.push(guard_pos);
                    }
                }
            },
            3 => {
                if guard_pos.0 == 0 {
                    break;
                }
                if grid[guard_pos.0-1][guard_pos.1] == '#' {
                    guard_dir = 0;
                }
                else {
                    guard_pos = (guard_pos.0-1, guard_pos.1);
                    if !unique_positions.contains(&guard_pos) {
                        unique_positions.push(guard_pos);
                    }
                }
            },
            _ => panic!("invalid guard direction!"),
        }
    }

    unique_positions.len()
}

pub fn solve2() -> usize {
    let file = File::open("src/day6/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut unique_posdirs: Vec<(usize, usize, usize)> = vec![]; //x, y, dir
    let mut unique_positions: Vec<(usize, usize)> = vec![];
    let mut guard_dir = 0; //north: 0, east: 1, south: 2, west: 3
    let mut guard_pos: (usize, usize);
    let mut guard_pos_start: (usize, usize) = (0, 0);
    let mut grid: [[char; 130]; 130] = [['.'; 130]; 130];

    let mut loop_count = 0;

    let mut row_index = 0;
    for line in reader.lines() {
        let line_clone = line.unwrap().clone();
        
        let mut col_index = 0;
        for c in line_clone.chars() {
            if c == '#' {
                grid[col_index][row_index] = '#';
            }
            if c == '^' {
                guard_pos_start = (col_index, row_index);
            }
            col_index += 1;
        }
        row_index += 1;
    }
    unique_posdirs.push((guard_pos_start.0, guard_pos_start.1, guard_dir));
    guard_pos = guard_pos_start;

    loop {
        match guard_dir {
            0 => {
                if guard_pos.1 == 0 {
                    break;
                }
                if grid[guard_pos.0][guard_pos.1-1] == '#' {
                    guard_dir = 1;
                }
                else {
                    guard_pos = (guard_pos.0, guard_pos.1-1);
                    if !unique_positions.contains(&guard_pos) {
                        unique_positions.push(guard_pos);
                    }
                }
            },
            1 => {
                if guard_pos.0 == 129 {
                    break;
                }
                if grid[guard_pos.0+1][guard_pos.1] == '#' {
                    guard_dir = 2;
                }
                else {
                    guard_pos = (guard_pos.0+1, guard_pos.1);
                    if !unique_positions.contains(&guard_pos) {
                        unique_positions.push(guard_pos);
                    }
                }
            },
            2 => {
                if guard_pos.1 == 129 {
                    break;
                }
                if grid[guard_pos.0][guard_pos.1+1] == '#' {
                    guard_dir = 3;
                }
                else {
                    guard_pos = (guard_pos.0, guard_pos.1+1);
                    if !unique_positions.contains(&guard_pos) {
                        unique_positions.push(guard_pos);
                    }
                }
            },
            3 => {
                if guard_pos.0 == 0 {
                    break;
                }
                if grid[guard_pos.0-1][guard_pos.1] == '#' {
                    guard_dir = 0;
                }
                else {
                    guard_pos = (guard_pos.0-1, guard_pos.1);
                    if !unique_positions.contains(&guard_pos) {
                        unique_positions.push(guard_pos);
                    }
                }
            },
            _ => panic!("invalid guard direction!"),
        }
    }

    //loop over possible obstacles locations
    for (x, y) in unique_positions {
        if grid[x][y] == '.' {
            grid[x][y] = '#';
            unique_posdirs = vec![];
            guard_pos = guard_pos_start;
            guard_dir = 0;
            unique_posdirs.push((guard_pos_start.0, guard_pos_start.1, guard_dir));
            loop {
                match guard_dir {
                    0 => {
                        if guard_pos.1 == 0 {
                            break;
                        }
                        if grid[guard_pos.0][guard_pos.1-1] == '#' {
                            guard_dir = 1;
                        }
                        else {
                            guard_pos = (guard_pos.0, guard_pos.1-1);
                        }
                        if unique_posdirs.contains(&(guard_pos.0, guard_pos.1, guard_dir)) {
                            loop_count += 1;
                            break;
                        }
                        else {
                            unique_posdirs.push((guard_pos.0, guard_pos.1, guard_dir));
                        }
                    },
                    1 => {
                        if guard_pos.0 == 129 {
                            break;
                        }
                        if grid[guard_pos.0+1][guard_pos.1] == '#' {
                            guard_dir = 2;
                        }
                        else {
                            guard_pos = (guard_pos.0+1, guard_pos.1);
                        }
                        if unique_posdirs.contains(&(guard_pos.0, guard_pos.1, guard_dir)) {
                            loop_count += 1;
                            break;
                        }
                        else {
                            unique_posdirs.push((guard_pos.0, guard_pos.1, guard_dir));
                        }
                    },
                    2 => {
                        if guard_pos.1 == 129 {
                            break;
                        }
                        if grid[guard_pos.0][guard_pos.1+1] == '#' {
                            guard_dir = 3;
                        }
                        else {
                            guard_pos = (guard_pos.0, guard_pos.1+1);
                        }
                        if unique_posdirs.contains(&(guard_pos.0, guard_pos.1, guard_dir)) {
                            loop_count += 1;
                            break;
                        }
                        else {
                            unique_posdirs.push((guard_pos.0, guard_pos.1, guard_dir));
                        }
                    },
                    3 => {
                        if guard_pos.0 == 0 {
                            break;
                        }
                        if grid[guard_pos.0-1][guard_pos.1] == '#' {
                            guard_dir = 0;
                        }
                        else {
                            guard_pos = (guard_pos.0-1, guard_pos.1);
                        }
                        if unique_posdirs.contains(&(guard_pos.0, guard_pos.1, guard_dir)) {
                            loop_count += 1;
                            break;
                        }
                        else {
                            unique_posdirs.push((guard_pos.0, guard_pos.1, guard_dir));
                        }

                    },
                    _ => panic!("invalid guard direction!"),
                }
            }
            grid[x][y] = '.';
        }
    }
    loop_count
}