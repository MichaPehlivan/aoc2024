use std::{fs::File, io::{BufRead, BufReader}};


pub fn solve1() -> usize {
    let file = File::open("src/day15/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut grid: [[char; 50]; 50] = [['.'; 50]; 50];
    let mut commands = String::new();
    let mut robot_pos = (0, 0);

    let mut row = 0;
    let mut reading_grid = true;
    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        if line_clone == "" {
            reading_grid = false;
        }

        if reading_grid {
            let mut col = 0;
            for c in line_clone.chars() {
                if c == '@' {
                    robot_pos = (col, row);
                }
                else {
                    grid[col][row] = c;
                }

                col += 1;
            }

            row += 1;
        }
        else {
            commands.push_str(&line_clone);
        }
    }

    //perform movement
    for command in commands.chars() {
        match command {
            '^' => {
                let target = grid[robot_pos.0][robot_pos.1-1];
                if target == '#' {
                    continue;
                }
                else if target == '.' {
                    robot_pos.1 -= 1;
                }
                else {
                    for y in (0..(robot_pos.1-1)).rev() {
                        if grid[robot_pos.0][y] == '.' {
                            grid[robot_pos.0][y] = 'O';
                            robot_pos.1 -= 1;
                            grid[robot_pos.0][robot_pos.1] = '.';
                            break;
                        }
                        else if grid[robot_pos.0][y] == '#' {
                            break;
                        }
                    }
                }
            },
            '>' => {
                let target = grid[robot_pos.0+1][robot_pos.1];
                if target == '#' {
                    continue;
                }
                else if target == '.' {
                    robot_pos.0 += 1;
                }
                else {
                    for x in robot_pos.0+2..10 {
                        if grid[x][robot_pos.1] == '.' {
                            grid[x][robot_pos.1] = 'O';
                            robot_pos.0 += 1;
                            grid[robot_pos.0][robot_pos.1] = '.';
                            break;
                        }
                        else if grid[x][robot_pos.1] == '#' {
                            break;
                        }
                    }
                }
            },
            'v' => {
                let target = grid[robot_pos.0][robot_pos.1+1];
                if target == '#' {
                    continue;
                }
                else if target == '.' {
                    robot_pos.1 += 1;
                }
                else {
                    for y in robot_pos.1+2..10 {
                        if grid[robot_pos.0][y] == '.' {
                            grid[robot_pos.0][y] = 'O';
                            robot_pos.1 += 1;
                            grid[robot_pos.0][robot_pos.1] = '.';
                            break;
                        }
                        else if grid[robot_pos.0][y] == '#' {
                            break;
                        }
                    }
                }
            },
            '<' => {
                let target = grid[robot_pos.0-1][robot_pos.1];
                if target == '#' {
                    continue;
                }
                else if target == '.' {
                    robot_pos.0 -= 1;
                }
                else {
                    for x in (0..(robot_pos.0-1)).rev() {
                        if grid[x][robot_pos.1] == '.' {
                            grid[x][robot_pos.1] = 'O';
                            robot_pos.0 -= 1;
                            grid[robot_pos.0][robot_pos.1] = '.';
                            break;
                        }
                        else if grid[x][robot_pos.1] == '#' {
                            break;
                        }
                    }
                }
            },
            _=> panic!("invalid command!")
        }
    }

    let mut total_coords = 0;

    //calculate sum of gps coords
    for x in 0..50 {
        for y in 0..50 {
            if grid[x][y] == 'O' {
                total_coords += 100*y + x;
            }
        }
    }
    total_coords
}

pub fn solve2() -> usize {
    let file = File::open("src/day15/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut grid: [[char; 50]; 100] = [['.'; 50]; 100];
    let mut commands = String::new();
    let mut robot_pos = (0, 0);

    let mut row = 0;
    let mut reading_grid = true;
    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        if line_clone == "" {
            reading_grid = false;
        }

        if reading_grid {
            let mut col = 0;
            for c in line_clone.chars() {
                if c == '@' {
                    robot_pos = (col, row);
                }
                else if c == '#' || c == '.' {
                    grid[col][row] = c;
                    grid[col+1][row] = c;
                }
                else if c == 'O' {
                    grid[col][row] = '[';
                    grid[col+1][row] = ']';
                }

                col += 2;
            }

            row += 1;
        }
        else {
            commands.push_str(&line_clone);
        }
    }

    //perform movement
    for command in commands.chars() {
        match command {
            '^' => {
                let target = grid[robot_pos.0][robot_pos.1-1];
                let mut to_push = vec![];
                if target == '#' {
                    continue;
                }
                else if target == '.' {
                    robot_pos.1 -= 1;
                }
                else if can_push(&grid, robot_pos, command, &mut to_push) {
                    let mut new_grid = grid.clone();
                    for block in &to_push {
                        new_grid[block.0][block.1] = '.';
                    }
                    for block in &to_push {
                        new_grid[block.0][block.1-1] = grid[block.0][block.1];
                    }
                    grid = new_grid;
                    robot_pos.1 -= 1;
                }
            },
            '>' => {
                let target = grid[robot_pos.0+1][robot_pos.1];
                let mut to_push = vec![];
                if target == '#' {
                    continue;
                }
                else if target == '.' {
                    robot_pos.0 += 1;
                }
                else if can_push(&grid, robot_pos, command, &mut to_push) {
                    let mut new_grid = grid.clone();
                    for block in &to_push {
                        new_grid[block.0][block.1] = '.';
                    }
                    for block in &to_push {
                        new_grid[block.0+1][block.1] = grid[block.0][block.1];
                    }
                    grid = new_grid;
                    robot_pos.0 += 1;
                }
            },
            'v' => {
                let target = grid[robot_pos.0][robot_pos.1+1];
                let mut to_push = vec![];
                if target == '#' {
                    continue;
                }
                else if target == '.' {
                    robot_pos.1 += 1;
                }
                else if can_push(&grid, robot_pos, command, &mut to_push) {
                    let mut new_grid = grid.clone();
                    for block in &to_push {
                        new_grid[block.0][block.1] = '.';
                    }
                    for block in &to_push {
                        new_grid[block.0][block.1+1] = grid[block.0][block.1];
                    }
                    grid = new_grid;
                    robot_pos.1 += 1;
                }
            },
            '<' => {
                let target = grid[robot_pos.0-1][robot_pos.1];
                let mut to_push = vec![];
                if target == '#' {
                    continue;
                }
                else if target == '.' {
                    robot_pos.0 -= 1;
                }
                else if can_push(&grid, robot_pos, command, &mut to_push) {
                    let mut new_grid = grid.clone();
                    for block in &to_push {
                        new_grid[block.0][block.1] = '.';
                    }
                    for block in &to_push {
                        new_grid[block.0-1][block.1] = grid[block.0][block.1];
                    }
                    grid = new_grid;
                    robot_pos.0 -= 1;
                }
            },
            _=> panic!("invalid command!")
        }
    }

    let mut total_coords = 0;

    //calculate sum of gps coords
    let mut already_counted = vec![];
    for x in 0..100 {
        for y in 0..50 {
            if grid[x][y] == '[' && !already_counted.contains(&(x+1, y)) {
                let y_from_top = y;
                let x_from_left = x;
                total_coords += 100*y_from_top + x_from_left;
                already_counted.push((x, y));
            }
            else if grid[x][y] == ']' && !already_counted.contains(&(x-1, y)) {
                let y_from_top = y;
                let x_from_left = x-1;
                total_coords += 100*y_from_top + x_from_left;
                already_counted.push((x, y));
            }
        }
    }
    total_coords
}

fn can_push(grid: &[[char; 50]; 100], robot_pos: (usize, usize), command: char, to_push: &mut Vec<(usize, usize)>) -> bool {
    match command {
        '^' => {
            if grid[robot_pos.0][robot_pos.1-1] == '.' {
                return true;
            }
            else if grid[robot_pos.0][robot_pos.1-1] == '#' {
                return false;
            }
            else if grid[robot_pos.0][robot_pos.1-1] == '[' {
                if !to_push.contains(&(robot_pos.0, robot_pos.1-1)) {
                    to_push.push((robot_pos.0, robot_pos.1-1));
                }
                if !to_push.contains(&(robot_pos.0+1, robot_pos.1-1)) {
                    to_push.push((robot_pos.0+1, robot_pos.1-1));
                }
                return can_push(grid, (robot_pos.0, robot_pos.1-1), command, to_push) && can_push(grid, (robot_pos.0+1, robot_pos.1-1),  command, to_push);
            }
            else {
                if !to_push.contains(&(robot_pos.0, robot_pos.1-1)) {
                    to_push.push((robot_pos.0, robot_pos.1-1));
                }
                if !to_push.contains(&(robot_pos.0-1, robot_pos.1-1)) {
                    to_push.push((robot_pos.0-1, robot_pos.1-1));
                }
                return can_push(grid, (robot_pos.0, robot_pos.1-1), command, to_push) && can_push(grid, (robot_pos.0-1, robot_pos.1-1), command, to_push);
            }
        },
        '>' => {
            if grid[robot_pos.0+1][robot_pos.1] == '.' {
                return true;
            }
            else if grid[robot_pos.0+1][robot_pos.1] == '#' {
                return false;
            }
            else {
                if !to_push.contains(&(robot_pos.0+1, robot_pos.1)) {
                    to_push.push((robot_pos.0+1, robot_pos.1));
                }
                return can_push(grid, (robot_pos.0+1, robot_pos.1), command, to_push)
            }
        },
        'v' => {
            if grid[robot_pos.0][robot_pos.1+1] == '.' {
                return true;
            }
            else if grid[robot_pos.0][robot_pos.1+1] == '#' {
                return false;
            }
            else if grid[robot_pos.0][robot_pos.1+1] == '[' {
                if !to_push.contains(&(robot_pos.0, robot_pos.1+1)) {
                    to_push.push((robot_pos.0, robot_pos.1+1));
                }
                if !to_push.contains(&(robot_pos.0+1, robot_pos.1+1)) {
                    to_push.push((robot_pos.0+1, robot_pos.1+1));
                }
                return can_push(grid, (robot_pos.0, robot_pos.1+1), command, to_push) && can_push(grid, (robot_pos.0+1, robot_pos.1+1),  command, to_push);
            }
            else {
                if !to_push.contains(&(robot_pos.0, robot_pos.1+1)) {
                    to_push.push((robot_pos.0, robot_pos.1+1));
                }
                if !to_push.contains(&(robot_pos.0-1, robot_pos.1+1)) {
                    to_push.push((robot_pos.0-1, robot_pos.1+1));
                }
                return can_push(grid, (robot_pos.0, robot_pos.1+1), command, to_push) && can_push(grid, (robot_pos.0-1, robot_pos.1+1), command, to_push);
            }
        },
        '<' => {
            if grid[robot_pos.0-1][robot_pos.1] == '.' {
                return true;
            }
            else if grid[robot_pos.0-1][robot_pos.1] == '#' {
                return false;
            }
            else {
                if !to_push.contains(&(robot_pos.0-1, robot_pos.1)) {
                    to_push.push((robot_pos.0-1, robot_pos.1));
                }
                return can_push(grid, (robot_pos.0-1, robot_pos.1), command, to_push)
            }
        },
        _=> panic!("invalid command!")
    }
}