use std::{collections::{HashMap, HashSet, VecDeque}, fs::File, io::{BufRead, BufReader}};

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

    let base_sol = find_shortest_path(&maze, start, end);

    //identify posssible skips
    let mut possible_skips = vec![];
    for x in 1..140 {
        for y in 1..140 {
            if !maze[x][y] && (maze[x-1][y] || maze[x+1][y] || maze[x][y-1] || maze[x][y+1]) {
                possible_skips.push((x, y));
            }
        }
    }

    let mut count = 0;
    for skip in &possible_skips {
        maze[skip.0][skip.1] = true;
        let path_len = find_shortest_path(&maze, start, end);
        if (base_sol - path_len) >= 100 {
            count += 1;
        }
        maze[skip.0][skip.1] = false;
    }

    count
}

fn find_shortest_path(maze: &[[bool; 141]; 141], start: (isize, isize), end: (isize, isize)) -> usize {
    let mut queue: VecDeque<(isize, isize)> = VecDeque::new();
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut parent: HashMap<(isize, isize), (isize, isize)> = HashMap::new();
    let mut path = vec![];

    visited.insert(start);
    queue.push_back(start);
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

        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let neighbour = (current.0 + dx, current.1 + dy);
            if maze[neighbour.0 as usize][neighbour.1 as usize] && !visited.contains(&(neighbour.0, neighbour.1)) {
                visited.insert((neighbour.0, neighbour.1));
                parent.insert(neighbour, current);
                queue.push_back(neighbour);
            }
        }
    }
    path.len()-1
}