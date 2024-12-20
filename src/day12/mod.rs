use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}};

pub fn solve1() -> isize {
    let file = File::open("src/day12/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut grid: [[char; 140]; 140] = [['.'; 140]; 140];
    let mut visited: HashSet<(isize, isize)> = HashSet::new();


    let mut row = 0;
    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        let mut col = 0;
        for c in line_clone.chars() {
            grid[col][row] = c;
            col += 1;
        }

        row += 1;
    }

    let mut total = 0;
    for x in 0..140 {
        for y in 0..140 {
            let (a, p) = flood_fill((x, y), grid[x as usize][y as usize], &grid, &mut visited);
            total += a*p;
        }
    }
    total
}

fn flood_fill(node: (isize, isize), c: char, grid: &[[char; 140]; 140], visited: &mut HashSet<(isize, isize)>) -> (isize, isize) {
    if node.0 < 0 || node.0 > 139 || node.1 < 0 || node.1 > 139 || grid[node.0 as usize][node.1 as usize] != c {
        return (0, 1);
    }
    else if visited.contains(&node) {
        return (0, 0);
    }

    let mut area = 1;
    let mut perim = 0;

    visited.insert(node);

    for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let (a, p) = flood_fill((node.0 + dx, node.1 + dy), c, grid, visited);
        area += a;
        perim += p;
    }
    (area, perim)
}

pub fn solve2() -> usize {
    let file = File::open("src/day12/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut grid: [[char; 140]; 140] = [['.'; 140]; 140];
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut filled: HashSet<(isize, isize)> = HashSet::new();


    let mut row = 0;
    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        let mut col = 0;
        for c in line_clone.chars() {
            grid[col][row] = c;
            col += 1;
        }

        row += 1;
    }

    let mut total = 0;
    for x in 0..140 {
        for y in 0..140 {
            if !visited.contains(&(x, y)) {
                flood_fill_2((x, y), grid[x as usize][y as usize], &grid, &mut visited, &mut filled);
                let mut corners = 0;
                for (xi, yi) in &filled {
                    let eu = *yi == 0 || grid[*xi as usize][(*yi-1) as usize] != grid[x as usize][y as usize]; //edge up
                    let ed = *yi == 139 || grid[*xi as usize][(*yi+1) as usize] != grid[x as usize][y as usize]; //edge down
                    let el = *xi == 0 || grid[(*xi-1) as usize][*yi as usize] != grid[x as usize][y as usize]; //edge left
                    let er = *xi == 139 || grid[(*xi+1) as usize][*yi as usize] != grid[x as usize][y as usize]; //edge right

                    if eu && ed && el && er {
                        corners += 4;
                    }
                    else if (el && eu && ed && !er) || (er && eu && ed && !el) || (el && eu && er && !ed) || (el && ed && er && !eu) {
                        corners += 2;
                    }
                    else if el && eu && !er && !ed {
                        corners += 1;
                        if grid[(*xi+1) as usize][(*yi+1) as usize] != grid[x as usize][y as usize] {
                            corners += 1;
                        }
                    }
                    else if er && eu && !el && !ed {
                        corners += 1;
                        if grid[(*xi-1) as usize][(*yi+1) as usize] != grid[x as usize][y as usize] {
                            corners += 1;
                        }
                    }
                    else if el && ed && !er && !eu {
                        corners += 1;
                        if grid[(*xi+1) as usize][(*yi-1) as usize] != grid[x as usize][y as usize] {
                            corners += 1;
                        }
                    }
                    else if ed && er && !eu && !el {
                        corners += 1;
                        if grid[(*xi-1) as usize][(*yi-1) as usize] != grid[x as usize][y as usize] {
                            corners += 1;
                        }
                    }
                    else if eu && !ed && !el && !er {
                        if grid[(*xi+1) as usize][(*yi+1) as usize] != grid[x as usize][y as usize] {
                            corners += 1;
                        }
                        if grid[(*xi-1) as usize][(*yi+1) as usize] != grid[x as usize][y as usize] {
                            corners += 1;
                        }
                    }
                    else if ed && !eu && !el && !er {
                        if grid[(*xi+1) as usize][(*yi-1) as usize] != grid[x as usize][y as usize] {
                            corners += 1;
                        }
                        if grid[(*xi-1) as usize][(*yi-1) as usize] != grid[x as usize][y as usize] {
                            corners += 1;
                        }
                    }
                    else if el && !eu && !ed && !er {
                        if grid[(*xi+1) as usize][(*yi+1) as usize] != grid[x as usize][y as usize] {
                            corners += 1;
                        }
                        if grid[(*xi+1) as usize][(*yi-1) as usize] != grid[x as usize][y as usize] {
                            corners += 1;
                        }
                    }
                    else if er && !eu && !ed && !el {
                        if grid[(*xi-1) as usize][(*yi+1) as usize] != grid[x as usize][y as usize] {
                            corners += 1;
                        }
                        if grid[(*xi-1) as usize][(*yi-1) as usize] != grid[x as usize][y as usize] {
                            corners += 1;
                        }
                    }
                    else if !eu && !ed && !el && !er {
                        if grid[(*xi-1) as usize][(*yi-1) as usize] != grid[x as usize][y as usize] {
                            corners += 1;
                        }
                        if grid[(*xi-1) as usize][(*yi+1) as usize] != grid[x as usize][y as usize] {
                            corners += 1;
                        }
                        if grid[(*xi+1) as usize][(*yi-1) as usize] != grid[x as usize][y as usize] {
                            corners += 1;
                        }
                        if grid[(*xi+1) as usize][(*yi+1) as usize] != grid[x as usize][y as usize] {
                            corners += 1;
                        }
                    }
                }
                total += filled.len()*corners;
                filled.clear();
            }
        }
    }
    total
}

fn flood_fill_2(node: (isize, isize), c: char, grid: &[[char; 140]; 140], visited: &mut HashSet<(isize, isize)>, filled: &mut HashSet<(isize, isize)>) {
    if node.0 < 0 || node.0 > 139 || node.1 < 0 || node.1 > 139 || grid[node.0 as usize][node.1 as usize] != c || filled.contains(&node) {
        return;
    }

    visited.insert(node);
    filled.insert(node);
    
    for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        flood_fill_2((node.0 + dx, node.1 + dy), c, grid, visited, filled);
    }
}
