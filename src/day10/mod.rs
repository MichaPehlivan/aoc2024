use std::{fs::File, io::{BufRead, BufReader}};

pub fn solve1() -> usize {
    let file = File::open("src/day10/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut grid: [[usize; 54]; 54] = [[0; 54]; 54];
    let mut trailheads: Vec<(usize, usize)> = vec![];
    let mut total_score = 0;

    let mut row = 0;
    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        let mut col = 0;
        for c in line_clone.chars() {
            let num = c as usize - '0' as usize;

            if num == 0 {
                trailheads.push((col, row));
            }

            grid[col][row] = num;
            col += 1;
        }
        row += 1;
    }

    for trailhead in trailheads {
        let mut ends_found = vec![];
        total_score += find_trail_score1((trailhead.0, trailhead.1), &grid, 0, &mut ends_found);
    }

    total_score
}

pub fn solve2() -> usize {
    let file = File::open("src/day10/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut grid: [[usize; 54]; 54] = [[0; 54]; 54];
    let mut trailheads: Vec<(usize, usize)> = vec![];
    let mut total_score = 0;

    let mut row = 0;
    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        let mut col = 0;
        for c in line_clone.chars() {
            let num = c as usize - '0' as usize;

            if num == 0 {
                trailheads.push((col, row));
            }

            grid[col][row] = num;
            col += 1;
        }
        row += 1;
    }

    for trailhead in trailheads {
        total_score += find_trail_score2((trailhead.0, trailhead.1), &grid, 0);
    }

    total_score
}

fn find_trail_score1(current_loc: (usize, usize), grid: &[[usize; 54]; 54], target_value: usize, ends_found: &mut Vec<(usize, usize)>) -> usize{
    if grid[current_loc.0][current_loc.1] != target_value {
        return 0;
    }
    if target_value == 9 {
        if !ends_found.contains(&current_loc) {
            ends_found.push(current_loc);
            return 1;
        }
        return 0;
    }

    let mut score = 0;

    
    if current_loc.0 > 0 {
        score += find_trail_score1((current_loc.0-1, current_loc.1), grid, target_value+1, ends_found);
    }
    if current_loc.0 < 53 {
        score += find_trail_score1((current_loc.0+1, current_loc.1), grid, target_value+1, ends_found);
    }
    if current_loc.1 > 0 {
        score += find_trail_score1((current_loc.0, current_loc.1-1), grid, target_value+1, ends_found);
    }
    if current_loc.1 < 53 {
        score += find_trail_score1((current_loc.0, current_loc.1+1), grid, target_value+1, ends_found);
    }
    score
}

fn find_trail_score2(current_loc: (usize, usize), grid: &[[usize; 54]; 54], target_value: usize) -> usize{
    if grid[current_loc.0][current_loc.1] != target_value {
        return 0;
    }
    if target_value == 9 {
        return 1;
    }

    let mut score = 0;

    
    if current_loc.0 > 0 {
        score += find_trail_score2((current_loc.0-1, current_loc.1), grid, target_value+1);
    }
    if current_loc.0 < 53 {
        score += find_trail_score2((current_loc.0+1, current_loc.1), grid, target_value+1);
    }
    if current_loc.1 > 0 {
        score += find_trail_score2((current_loc.0, current_loc.1-1), grid, target_value+1);
    }
    if current_loc.1 < 53 {
        score += find_trail_score2((current_loc.0, current_loc.1+1), grid, target_value+1);
    }
    score
}