use std::{collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}};

use regex::Regex;

pub fn solve1() -> usize {
    let file = File::open("src/day24/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut wires: HashMap<String, bool> = HashMap::new();
    let mut rules: HashSet<(String, String, String, usize)> = HashSet::new(); //wire 1 wire 2 wire 3 operator

    let re_rule = Regex::new(r"(\S+) (\S+) (\S+) -> (\S+)").expect("Invalid regex!");

    let mut reading_initial = true;
    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        if line_clone == "" {
            reading_initial = false;
            continue;
        }

        if reading_initial {
            let wire = line_clone.split(": ").nth(0).unwrap().to_string();
            let state: usize = line_clone.split(": ").last().unwrap().parse().unwrap();
            if state == 0 {
                wires.insert(wire, false);
            }
            else {
                wires.insert(wire, true);
            }
        }
        else {
            let capture = re_rule.captures(&line_clone).unwrap();
            let wire1 = capture[1].to_string();
            let operator = match &capture[2] {
                "AND" => 0,
                "OR" => 1,
                "XOR" => 2,
                _=> panic!("Invalid operator")
            };
            let wire2 = capture[3].to_string();
            let wire3 = capture[4].to_string();
            rules.insert((wire1, wire2, wire3, operator));
        }
    }

    while !rules.is_empty() {
        for (wire1, wire2, wire3, operator) in &rules.clone() {
            if wires.contains_key(wire1) && wires.contains_key(wire2) {
                match operator {
                    0 => {
                        wires.insert(wire3.clone(), wires[wire1] && wires[wire2]);
                    },
                    1 => {
                        wires.insert(wire3.clone(), wires[wire1] || wires[wire2]);
                    },
                    2 => {
                        wires.insert(wire3.clone(), wires[wire1] ^ wires[wire2]);
                    },
                    _=> panic!("Invalid operator!")
                }
                rules.remove(&(wire1.clone(), wire2.clone(), wire3.clone(), *operator));
            }
        }
    }
    wires_to_num(wires, 'z')
}

pub fn solve2() -> String {
    let file = File::open("src/day24/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut rules: HashSet<(String, String, String, usize)> = HashSet::new(); //wire 1 wire 2 wire 3 operator

    let re_rule = Regex::new(r"(\S+) (\S+) (\S+) -> (\S+)").expect("Invalid regex!");

    let mut reading_initial = true;
    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        if line_clone == "" {
            reading_initial = false;
            continue;
        }

        if !reading_initial {
            let capture = re_rule.captures(&line_clone).unwrap();
            let wire1 = capture[1].to_string();
            let operator = match &capture[2] {
                "AND" => 0,
                "OR" => 1,
                "XOR" => 2,
                _=> panic!("Invalid operator")
            };
            let wire2 = capture[3].to_string();
            let wire3 = capture[4].to_string();
            rules.insert((wire1, wire2, wire3, operator));
        }
    }

    let mut sus_rules = vec![];
    for rule in &rules {
        if rule.2.starts_with("z") {
            if rule.3 != 2 && rule.2 != "z45" {
                sus_rules.push(rule);
            }
        }
        else if !is_first_layer(rule) && rule.3 == 2 {
            sus_rules.push(rule);
        }
        else if !(rule.0 == "x00" && rule.1 == "y00") {
            if is_first_layer(rule) && rule.3 == 2 {
                if let None = rules.iter().filter(|r| (r.0 == rule.2 || r.1 == rule.2) && r.3 == 2).last() {
                    sus_rules.push(rule);
                }
            }
            else if rule.3 == 0 {
                if let None = rules.iter().filter(|r| (r.0 == rule.2 || r.1 == rule.2) && r.3 == 1).last() {
                    sus_rules.push(rule);
                }
            }
        }
    }

    let mut sus_outputs: Vec<String> = sus_rules.iter().map(|(_, _, output, _)| output).cloned().collect();
    sus_outputs.sort();
    sus_outputs.join(",")
}

fn wires_to_num(wires: HashMap<String, bool>, line: char) -> usize {
    let mut hash_vec: Vec<(&String, &bool)> = wires.iter().filter(|(wire, _)| wire.starts_with(line)).collect();
    hash_vec.sort();
    let mut num = 0;
    let mut pos = 0;
    for bit in hash_vec {
        if *bit.1 {
            num += 1 << pos;
        }
        pos += 1;
    }
    num
}

fn is_first_layer(rule: &(String, String, String, usize)) -> bool {
    (rule.0.starts_with("x") && rule.1.starts_with("y")) || (rule.0.starts_with("y") && rule.1.starts_with("x"))
}