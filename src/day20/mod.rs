use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

pub fn solve1() -> usize {
    let file = File::open("src/day20/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut maze: [[bool; 141]; 141] = [[false; 141]; 141];
    let mut start= (0, 0);
    let mut end = (0, 0);

    let mut row = 0;
    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        let mut col = 0;
        for c in line_clone.chars() {
            if c == '.' {
                maze[col][row] = true;
            }
            else if c == 'S' {
                maze[col][row] = true;
                start = (col as isize, row as isize);
            }
            else if c == 'E' {
                maze[col][row] = true;
                end = (col as isize, row as isize);
            }
            col += 1;
        }

        row += 1;
    }

    let mut pathtime_map: HashMap<(isize, isize), isize> = HashMap::new();
    let base_sol = find_base_sol(&maze, start, end);
    let mut time_counter = 0;
    for tile in base_sol.iter().rev() {
        pathtime_map.insert(*tile, time_counter);
        time_counter += 1;
    }

    //check cheats
    let mut count = 0;
    for (tile_x, tile_y) in base_sol {
        //for every tile on path within manhattan distance 2
        for x in tile_x-2..=tile_x+2 {
            for y in tile_y-(2-isize::abs(x-tile_x))..=tile_y+(2-isize::abs(x-tile_x)) {
                if x > -1 && x < 141 && y > -1 && y < 141 && maze[x as usize][y as usize] { //point on grid and on path
                    let manhattan_distance = isize::abs(x-tile_x) + isize::abs(y-tile_y);
                    let time_saved = pathtime_map.get(&(tile_x, tile_y)).unwrap() - pathtime_map.get(&(x, y)).unwrap() - manhattan_distance;
                    if time_saved >= 100 {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn find_base_sol(maze: &[[bool; 141]; 141], start: (isize, isize), end: (isize, isize)) -> Vec<(isize, isize)> {
    let (mut dx, mut dy) = (0, -1);
    let (mut x, mut y) = start;
    let mut path = vec![];
    while (x, y) != end {
        path.push((x, y));
        if maze[(x+dx) as usize][(y+dy) as usize] {
            x += dx;
            y += dy;
        }
        else {
            for (dx_new, dy_new) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                if (dx_new, dy_new) != (-dx, -dy) && maze[(x+dx_new) as usize][(y+dy_new) as usize] {
                    dx = dx_new;
                    dy = dy_new;
                    x += dx;
                    y += dy;
                    break;
                }
            } 
        }
    }
    path.push(end);
    path
}
