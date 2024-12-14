use std::{fs::File, io::{BufRead, BufReader}};

use regex::Regex;

pub fn solve1() -> usize {
    let file = File::open("src/day14/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut robots: Vec<((isize, isize), (isize, isize))> = vec![];
    
    let re = Regex::new(r"p=(\d+),(\d+)\s+v=(-?\d+),(-?\d+)").expect("invalid regex!");

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();
        let capture = re.captures(&line_clone).unwrap();

        let pos_x = capture[1].parse().unwrap();
        let pos_y = capture[2].parse().unwrap();
        let v_x = capture[3].parse().unwrap();
        let v_y = capture[4].parse().unwrap();

        robots.push(((pos_x, pos_y), (v_x, v_y)));
    }
    
    for _ in 0..100 {
        for robot in &mut robots {
            let new_x = (((robot.0.0 + robot.1.0) % 101) + 101) % 101;
            let new_y = (((robot.0.1 + robot.1.1) % 103) + 103) % 103;
            robot.0 = (new_x, new_y);
        }
    }

    let mut first_quad = 0;
    let mut second_quad = 0;
    let mut third_quad = 0;
    let mut fourth_quad = 0;

    for ((x, y), (_, _)) in robots {
        if x > 50 && y < 51 {
            first_quad += 1;
        }
        else if x < 50 && y < 51 {
            second_quad += 1;
        }
        else if x < 50 && y > 51 {
            third_quad += 1;
        }
        else if x > 50 && y > 51 {
            fourth_quad += 1;
        }
    }

    first_quad * second_quad * third_quad * fourth_quad
}

pub fn solve2() {
    let file = File::open("src/day14/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut robots: Vec<((isize, isize), (isize, isize))> = vec![];
    
    let re = Regex::new(r"p=(\d+),(\d+)\s+v=(-?\d+),(-?\d+)").expect("invalid regex!");

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();
        let capture = re.captures(&line_clone).unwrap();

        let pos_x = capture[1].parse().unwrap();
        let pos_y = capture[2].parse().unwrap();
        let v_x = capture[3].parse().unwrap();
        let v_y = capture[4].parse().unwrap();

        robots.push(((pos_x, pos_y), (v_x, v_y)));
    }
    
    let mut grid: [[char; 103]; 101] = [['.'; 103]; 101];
    
    for _ in 0..8168 {
        for robot in &mut robots {
            let new_x = (((robot.0.0 + robot.1.0) % 101) + 101) % 101;
            let new_y = (((robot.0.1 + robot.1.1) % 103) + 103) % 103;
            robot.0 = (new_x, new_y);
        }
    }

    for robot in robots {
        grid[robot.0.0 as usize][robot.0.1 as usize] = 'X';
    }

    for y in 47..80 {
        for x in 20..51 {
            print!("{}", grid[x][y]);
        }
        println!("");
    }
}