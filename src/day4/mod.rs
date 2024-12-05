use std::{fs::File, io::{BufRead, BufReader}};

pub fn solve1() -> usize {
    let file = File::open("src/day4/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut char_grid: [[char; 140]; 140] = [[' '; 140]; 140];
    let mut count = 0;

    let mut row = 0;
    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        let mut col = 0;
        for c in line_clone.chars() {
            char_grid[row][col] = c;
            col += 1;
        }
        row += 1;
    }

    for row in 0..140 {
        for col in 0..140 {
            //check up
            if row > 2 {
                if char_grid[row][col] == 'X' && char_grid[row-1][col] == 'M' && char_grid[row-2][col] == 'A' && char_grid[row-3][col] == 'S' {
                    count += 1;
                }
            }
            //check down
            if row < 137 {
                if char_grid[row][col] == 'X' && char_grid[row+1][col] == 'M' && char_grid[row+2][col] == 'A' && char_grid[row+3][col] == 'S' {
                    count += 1;
                }
            }
            //check left
            if col > 2 {
                if char_grid[row][col] == 'X' && char_grid[row][col-1] == 'M' && char_grid[row][col-2] == 'A' && char_grid[row][col-3] == 'S' {
                    count += 1;
                }
            }
            //check right
            if col < 137 {
                if char_grid[row][col] == 'X' && char_grid[row][col+1] == 'M' && char_grid[row][col+2] == 'A' && char_grid[row][col+3] == 'S' {
                    count += 1;
                }
            }
            //check up-left
            if row > 2 && col > 2 {
                if char_grid[row][col] == 'X' && char_grid[row-1][col-1] == 'M' && char_grid[row-2][col-2] == 'A' && char_grid[row-3][col-3] == 'S' {
                    count += 1;
                }
            }
            //check up-right
            if row > 2 && col < 137 {
                if char_grid[row][col] == 'X' && char_grid[row-1][col+1] == 'M' && char_grid[row-2][col+2] == 'A' && char_grid[row-3][col+3] == 'S' {
                    count += 1;
                }
            }
            //check down-right
            if row < 137 && col < 137 {
                if char_grid[row][col] == 'X' && char_grid[row+1][col+1] == 'M' && char_grid[row+2][col+2] == 'A' && char_grid[row+3][col+3] == 'S' {
                    count += 1;
                }
            }
            //check down-left
            if row < 137 && col > 2 {
                if char_grid[row][col] == 'X' && char_grid[row+1][col-1] == 'M' && char_grid[row+2][col-2] == 'A' && char_grid[row+3][col-3] == 'S' {
                    count += 1;
                }
            }
        }
    }
    count
}

pub fn solve2() -> usize {
    let file = File::open("src/day4/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut char_grid: [[char; 140]; 140] = [[' '; 140]; 140];
    let mut count = 0;

    let mut row = 0;
    for line in reader.lines() {
        let line_clone = line.unwrap().clone();
        let mut col = 0;
        for c in line_clone.chars() {
            char_grid[row][col] = c;
            col += 1;
        }
        row += 1;
    }

    for row in 0..140 {
        for col in 0..140 {
            if row > 0 && row < 139 && col > 0 && col < 139 {
                //case 1: M's on top
                if char_grid[row][col] == 'A' && char_grid[row-1][col-1] == 'M' && char_grid[row-1][col+1] == 'M' && char_grid[row+1][col-1] == 'S' && char_grid[row+1][col+1] == 'S' {
                    count += 1;
                }
                //case 2: M's on bottom
                if char_grid[row][col] == 'A' && char_grid[row-1][col-1] == 'S' && char_grid[row-1][col+1] == 'S' && char_grid[row+1][col-1] == 'M' && char_grid[row+1][col+1] == 'M' {
                    count += 1;
                }
                //case 3: M's left
                if char_grid[row][col] == 'A' && char_grid[row-1][col-1] == 'M' && char_grid[row-1][col+1] == 'S' && char_grid[row+1][col-1] == 'M' && char_grid[row+1][col+1] == 'S' {
                    count += 1;
                }
                //case 4: M's right
                if char_grid[row][col] == 'A' && char_grid[row-1][col-1] == 'S' && char_grid[row-1][col+1] == 'M' && char_grid[row+1][col-1] == 'S' && char_grid[row+1][col+1] == 'M' {
                    count += 1;
                }
            }
        }
    }
    count
}