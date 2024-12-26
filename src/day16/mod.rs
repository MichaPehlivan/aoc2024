use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}, fs::File, io::{BufRead, BufReader}, usize};

pub fn solve1() -> usize {
    let file = File::open("src/day16/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut maze: [[bool; 141]; 141] = [[false; 141]; 141];
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut graph: HashMap<(isize, isize, usize), Vec<(isize, isize, usize, usize)>> = HashMap::new();

    let mut row = 0;
    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        let mut col = 0;
        for c in line_clone.chars() {
            if c == '.' {
                maze[col as usize][row as usize] = true;
                graph.insert((col, row, 0), vec![]);
                graph.insert((col, row, 1), vec![]);
                graph.insert((col, row, 2), vec![]);
                graph.insert((col, row, 3), vec![]);
            }
            else if c == 'S' {
                maze[col as usize][row as usize] = true;
                start = (col, row);
                graph.insert((col, row, 0), vec![]);
                graph.insert((col, row, 1), vec![]);
                graph.insert((col, row, 2), vec![]);
                graph.insert((col, row, 3), vec![]);
            }
            else if c == 'E' {
                maze[col as usize][row as usize] = true;
                end = (col, row);
                graph.insert((col, row, 0), vec![]);
                graph.insert((col, row, 1), vec![]);
                graph.insert((col, row, 2), vec![]);
                graph.insert((col, row, 3), vec![]);
            }
            col += 1;
        }
        row += 1;
    }

    //add edges
    for (node, edges) in &mut graph {
        match node.2 {
            0 => {
                edges.push((node.0, node.1, 1, 1000));
                edges.push((node.0, node.1, 3, 1000));
                if maze[node.0 as usize][(node.1-1) as usize] {
                    edges.push((node.0, node.1-1, 0, 1));
                }
            },
            1 => {
                edges.push((node.0, node.1, 2, 1000));
                edges.push((node.0, node.1, 0, 1000));
                if maze[(node.0+1) as usize][node.1 as usize] {
                    edges.push((node.0+1, node.1, 1, 1));
                }
            },
            2 => {
                edges.push((node.0, node.1, 3, 1000));
                edges.push((node.0, node.1, 1, 1000));
                if maze[node.0 as usize][(node.1+1) as usize] {
                    edges.push((node.0, node.1+1, 2, 1));
                }
            },
            3 => {
                edges.push((node.0, node.1, 0, 1000));
                edges.push((node.0, node.1, 2, 1000));
                if maze[(node.0-1) as usize][node.1 as usize] {
                    edges.push((node.0-1, node.1, 3, 1));
                }
            }
            _=> panic!("Invalid direction!")
        }
    }
    
    //Dijkstra
    dijkstra(graph, (start.0, start.1, 1), end)
}

pub fn solve2() -> usize {
    let file = File::open("src/day16/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut maze: [[bool; 141]; 141] = [[false; 141]; 141];
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut graph: HashMap<(isize, isize, usize), Vec<(isize, isize, usize, usize)>> = HashMap::new();

    let mut row = 0;
    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        let mut col = 0;
        for c in line_clone.chars() {
            if c == '.' {
                maze[col as usize][row as usize] = true;
                graph.insert((col, row, 0), vec![]);
                graph.insert((col, row, 1), vec![]);
                graph.insert((col, row, 2), vec![]);
                graph.insert((col, row, 3), vec![]);
            }
            else if c == 'S' {
                maze[col as usize][row as usize] = true;
                start = (col, row);
                graph.insert((col, row, 0), vec![]);
                graph.insert((col, row, 1), vec![]);
                graph.insert((col, row, 2), vec![]);
                graph.insert((col, row, 3), vec![]);
            }
            else if c == 'E' {
                maze[col as usize][row as usize] = true;
                end = (col, row);
                graph.insert((col, row, 0), vec![]);
                graph.insert((col, row, 1), vec![]);
                graph.insert((col, row, 2), vec![]);
                graph.insert((col, row, 3), vec![]);
            }
            col += 1;
        }
        row += 1;
    }

    //add edges
    for (node, edges) in &mut graph {
        match node.2 {
            0 => {
                edges.push((node.0, node.1, 1, 1000));
                edges.push((node.0, node.1, 3, 1000));
                if maze[node.0 as usize][(node.1-1) as usize] {
                    edges.push((node.0, node.1-1, 0, 1));
                }
            },
            1 => {
                edges.push((node.0, node.1, 2, 1000));
                edges.push((node.0, node.1, 0, 1000));
                if maze[(node.0+1) as usize][node.1 as usize] {
                    edges.push((node.0+1, node.1, 1, 1));
                }
            },
            2 => {
                edges.push((node.0, node.1, 3, 1000));
                edges.push((node.0, node.1, 1, 1000));
                if maze[node.0 as usize][(node.1+1) as usize] {
                    edges.push((node.0, node.1+1, 2, 1));
                }
            },
            3 => {
                edges.push((node.0, node.1, 0, 1000));
                edges.push((node.0, node.1, 2, 1000));
                if maze[(node.0-1) as usize][node.1 as usize] {
                    edges.push((node.0-1, node.1, 3, 1));
                }
            }
            _=> panic!("Invalid direction!")
        }
    }
    
    //Dijkstra
    dijkstra2(graph, (start.0, start.1, 1), end)
}


#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct State {
    priority: usize,
    node: (isize, isize, usize)
}

fn dijkstra(graph: HashMap<(isize, isize, usize), Vec<(isize, isize, usize, usize)>>, start: (isize, isize, usize), end: (isize, isize)) -> usize {
    let mut priority_queue: BinaryHeap<Reverse<State>> = BinaryHeap::new();
    let mut distances: HashMap<(isize, isize, usize), usize> = HashMap::new();
    let mut previous: HashMap<(isize, isize, usize), (isize, isize, usize)> = HashMap::new();
    distances.insert(start, 0);
    priority_queue.push(Reverse(State { priority: 0, node: start }));

    for (node, _) in &graph {
        if *node != start {
            distances.insert(node.clone(), usize::MAX);
        }
    }

    while let Some(u) = priority_queue.pop() {

        if u.0.node == (end.0, end.1, 0) {
            return distances[&(end.0, end.1, 0)];
        }

        if u.0.priority > distances[&u.0.node] {
            continue;
        }

        let neighbours = graph.get(&u.0.node).unwrap();

        for v in neighbours {
            let alt = distances.get(&u.0.node).unwrap() + v.3;
            if &alt < distances.get(&(v.0, v.1, v.2)).unwrap() {
                previous.insert((v.0, v.1, v.2), u.0.node);
                *distances.entry((v.0, v.1, v.2)).or_insert(0) = alt;
                priority_queue.push(Reverse(State { priority: alt, node: (v.0, v.1, v.2) }));
            }
        }
    }
    distances[&(end.0, end.1, 0)]
}

fn dijkstra2(graph: HashMap<(isize, isize, usize), Vec<(isize, isize, usize, usize)>>, start: (isize, isize, usize), end: (isize, isize)) -> usize {
    let mut priority_queue: BinaryHeap<Reverse<State>> = BinaryHeap::new();
    let mut distances: HashMap<(isize, isize, usize), usize> = HashMap::new();
    let mut previous: HashMap<(isize, isize, usize), HashSet<(isize, isize, usize)>> = HashMap::new();
    distances.insert(start, 0);
    priority_queue.push(Reverse(State { priority: 0, node: start }));

    for (node, _) in &graph {
        if *node != start {
            distances.insert(node.clone(), usize::MAX);
        }
    }

    while let Some(u) = priority_queue.pop() {

        if u.0.node == (end.0, end.1, 0) {
            let path = build_path(start, (end.0, end.1, 0), &previous);
            let mut num_of_seats = 0;
            let mut already_seen = vec![];
            for node in path {
                if !already_seen.contains(&(node.0, node.1)) {
                    already_seen.push((node.0, node.1));
                    num_of_seats += 1;
                }
            }
            return num_of_seats + 1;
        }

        if u.0.priority > distances[&u.0.node] {
            continue;
        }

        let neighbours = graph.get(&u.0.node).unwrap();

        for v in neighbours {
            let alt = distances.get(&u.0.node).unwrap() + v.3;
            if &alt <= distances.get(&(v.0, v.1, v.2)).unwrap() {
                if &alt < distances.get(&(v.0, v.1, v.2)).unwrap() {
                    previous.entry((v.0, v.1, v.2)).or_insert(HashSet::new()).clear();
                }
                previous.entry((v.0, v.1, v.2)).or_insert(HashSet::new()).insert(u.0.node);
                *distances.entry((v.0, v.1, v.2)).or_insert(0) = alt;
                priority_queue.push(Reverse(State { priority: alt, node: (v.0, v.1, v.2) }));
            }
        }
    }
    0
}

fn build_path(start: (isize, isize, usize), end: (isize, isize, usize), previous: &HashMap<(isize, isize, usize), HashSet<(isize, isize, usize)>>) -> Vec<(isize, isize, usize)> {
    let mut path = vec![];
    if end == start {
        return path;
    }
    for prev in previous.get(&end).unwrap() {
        path.push(*prev);
        path.append(&mut build_path(start, *prev, previous));
    }
    path
}
