use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};


pub fn solve1() -> usize {
    let file = File::open("src/day8/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut freq_map: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    let mut antinodes = vec![];

    let mut row = 0;
    for line in reader.lines() {
        let line_clone = line.unwrap().clone();
        let mut col = 0;
        for c in line_clone.chars() {
            if c != '.' {
                if freq_map.contains_key(&c) {
                    let mut current_list = freq_map.get(&c).unwrap().clone();
                    current_list.push((col, row));
                    freq_map.insert(c, current_list);
                }
                else {
                    freq_map.insert(c, vec![(col, row)]);
                }
            }
            col += 1;
        }

        row += 1;
    }

    for key in freq_map.keys() {
        let coords = freq_map[key].clone();
        for coord0 in &coords {
            for coord1 in &coords {
                if coord0 != coord1 {
                    let x_diff = coord1.0 - coord0.0;
                    let y_diff = coord1.1 - coord0.1;
                    let antinode = (coord0.0-x_diff, coord0.1-y_diff);
                    if !antinodes.contains(&antinode) && antinode.0 > -1 && antinode.0 < 50 && antinode.1 > -1 && antinode.1 < 50 {
                        antinodes.push(antinode);
                    }
                }
            }
        }
    }
    antinodes.len()
}

pub fn solve2() -> usize {
    let file = File::open("src/day8/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut freq_map: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    let mut antinodes = vec![];

    let mut row = 0;
    for line in reader.lines() {
        let line_clone = line.unwrap().clone();
        let mut col = 0;
        for c in line_clone.chars() {
            if c != '.' {
                if freq_map.contains_key(&c) {
                    let mut current_list = freq_map.get(&c).unwrap().clone();
                    current_list.push((col, row));
                    freq_map.insert(c, current_list);
                }
                else {
                    freq_map.insert(c, vec![(col, row)]);
                }
            }
            col += 1;
        }

        row += 1;
    }

    for key in freq_map.keys() {
        let coords = freq_map[key].clone();
        for coord0 in &coords {
            for coord1 in &coords {
                if coord0 != coord1 {
                    let x_diff = coord1.0 - coord0.0;
                    let y_diff = coord1.1 - coord0.1;
                    let mut antinode = (coord0.0, coord0.1);
                    while antinode.0 > -1 && antinode.0 < 50 && antinode.1 > -1 && antinode.1 < 50 {
                        if !antinodes.contains(&antinode) {
                            antinodes.push(antinode);
                        }
                        antinode = (antinode.0-x_diff, antinode.1-y_diff);
                    }
                }
            }
        }
    }
    antinodes.len()
}