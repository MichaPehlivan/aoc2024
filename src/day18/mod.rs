use std::{collections::{HashMap, HashSet, VecDeque}, fs::File, io::{BufRead, BufReader}};

pub fn solve1() -> usize {
    let file = File::open("src/day18/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut coords = vec![];
    let mut grid: [[bool; 71]; 71] = [[true; 71]; 71];
    let start = (0, 0);
    let end = (70, 70);

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();
        let coord_vec: Vec<&str>= line_clone.split(",").collect();
        let x = usize::from_str_radix(coord_vec.first().unwrap(), 10).unwrap();
        let y = usize::from_str_radix(coord_vec.last().unwrap(), 10).unwrap();
        coords.push((x, y));
    }

    for i in 0..1024 {
        let (x, y) = coords[i];
        grid[x][y] = false;
    }
    
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut parent: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut path = vec![];

    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        if current == end {
            path.push(current);
            let mut trace = current;
            while let Some(&prev) = parent.get(&trace) {
                path.push(prev);
                trace = prev;
            }
            break;
        }

        let (x, y) = current;

        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (nx, ny) = (x as isize + dx, y as isize + dy);

            if nx > -1 && nx < 71 && ny > -1 && ny < 71 {
                let neigbour = (nx as usize, ny as usize);

                if !visited.contains(&neigbour) && grid[neigbour.0][neigbour.1] {
                    queue.push_back(neigbour);
                    visited.insert(neigbour);
                    parent.insert(neigbour, current);
                }
            }
        }
    }
    path.len() - 1
}

pub fn solve2() -> (usize, usize) {
    let file = File::open("src/day18/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut coords = vec![];
    let mut grid: [[bool; 71]; 71] = [[true; 71]; 71];
    let start = (0, 0);
    let end = (70, 70);

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();
        let coord_vec: Vec<&str>= line_clone.split(",").collect();
        let x = usize::from_str_radix(coord_vec.first().unwrap(), 10).unwrap();
        let y = usize::from_str_radix(coord_vec.last().unwrap(), 10).unwrap();
        coords.push((x, y));
    }

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut parent: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut path = vec![];
    let mut final_block = (0, 0);

    for i in 0..coords.len() {
        let (x, y) = coords[i];
        grid[x][y] = false;

        queue.clear();
        visited.clear();
        parent.clear();
        path.clear();

        queue.push_back(start);
        visited.insert(start);

        while let Some(current) = queue.pop_front() {
            if current == end {
                path.push(current);
                let mut trace = current;
                while let Some(&prev) = parent.get(&trace) {
                    path.push(prev);
                    trace = prev;
                }
                break;
            }

            let (x, y) = current;

            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (nx, ny) = (x as isize + dx, y as isize + dy);

                if nx > -1 && nx < 71 && ny > -1 && ny < 71 {
                    let neigbour = (nx as usize, ny as usize);

                    if !visited.contains(&neigbour) && grid[neigbour.0][neigbour.1] {
                        queue.push_back(neigbour);
                        visited.insert(neigbour);
                        parent.insert(neigbour, current);
                    }
                }
            }
        }

        if path.len() == 0 {
            final_block = (x, y);
            break;
        }
    }
    
    final_block
}