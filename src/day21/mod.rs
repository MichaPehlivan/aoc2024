use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

pub fn solve1() -> usize {
    let file = File::open("src/day21/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut codes = vec![];

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();
        codes.push(line_clone);
    }

    let mut total_complexity = 0;

    for code in codes {
        let mut current_numpad = 'A';
        let mut numpad_moves = String::new();
        for c in code.chars() {
            let moves = shortest_on_numpad(current_numpad, c);
            current_numpad = c;
            numpad_moves.push_str(moves.as_str());
            numpad_moves.push('A');
        }

        let mut current_directional = 'A';
        let mut dirpad1_moves = String::new();
        for c in numpad_moves.chars() {
            let moves = shortest_directional(current_directional, c);
            current_directional = c;
            dirpad1_moves.push_str(moves.as_str());
            dirpad1_moves.push('A');
        }

        current_directional = 'A';
        let mut dirpad2_moves = String::new();
        for c in dirpad1_moves.chars() {
            let moves = shortest_directional(current_directional, c);
            current_directional = c;
            dirpad2_moves.push_str(moves.as_str());
            dirpad2_moves.push('A');
        }

        let sequence_lenght = dirpad2_moves.len();
        let code_num = usize::from_str_radix(&code[0..3], 10).unwrap();
        total_complexity += sequence_lenght*code_num;
    }
    total_complexity
}

pub fn solve2() -> usize {
    let file = File::open("src/day21/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut codes = vec![];

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();
        codes.push(line_clone);
    }

    let mut total_complexity = 0;

    for code in codes {
        let mut current_numpad = 'A';
        let mut numpad_moves = String::new();
        for c in code.chars() {
            let moves = shortest_on_numpad(current_numpad, c);
            current_numpad = c;
            numpad_moves.push_str(moves.as_str());
            numpad_moves.push('A');
        }

        let mut sequences: Vec<String> = numpad_moves.split_inclusive("A").map(|s| s.to_string()).collect();
        let mut sequence_map: HashMap<String, usize> = HashMap::new();
        for sequence in &sequences {
            *sequence_map.entry(sequence.clone()).or_insert(0) += 1;
        }

        for _ in 0..25 {
            for (sequence, frequency) in sequence_map.clone() {
                sequences.clear();
                let mut dirpad_buffer = String::new();
                let mut current_directional = 'A';
                for c in sequence.chars() {
                    let moves = shortest_directional(current_directional, c);
                    current_directional = c;
                    dirpad_buffer.push_str(moves.as_str());
                    dirpad_buffer.push('A');
                }
                sequences = dirpad_buffer.split_inclusive("A").map(|s| s.to_string()).collect();
                
                *sequence_map.entry(sequence).or_insert(1) -= frequency;
                for s in &sequences {
                    *sequence_map.entry(s.clone()).or_insert(0) += frequency;
                }
            } 
        }

        let mut sequence_lenght = 0;
        for (sequence, freq) in sequence_map {
            sequence_lenght += freq*sequence.len();
        }
        let code_num = usize::from_str_radix(&code[0..3], 10).unwrap();
        total_complexity += sequence_lenght*code_num;
    }
    total_complexity
}

fn shortest_on_numpad(start: char, end: char) -> String {
    let (start_x, start_y) = char_to_coord_numpad(start);
    let (end_x, end_y) = char_to_coord_numpad(end);

    if start == '0' && end == '1' {
        return "^<".to_string();
    }
    else if start == '0' && end == '4' {
        return "^<^".to_string();
    }
    else if start == '0' && end == '7' {
        return "^<^^".to_string();
    }
    if start == 'A' && end == '1' {
        return "^<<".to_string();
    }
    else if start == 'A' && end == '4' {
        return "^^<<".to_string();
    }
    else if start == 'A' && end == '7' {
        return "^^^<<".to_string();
    }
    else if start == '7' && end == '0' {
        return ">vvv".to_string();
    }
    else if start == '7' && end == 'A' {
        return ">>vvv".to_string();
    }
    else if start == '4' && end == '0' {
        return ">vv".to_string();
    }
    else if start == '4' && end == 'A' {
        return ">>vv".to_string();   
    }
    else if start == '1' && end == '0' {
        return ">v".to_string();
    }
    else if start == '1' && end == 'A' {
        return ">>v".to_string();
    }

    let mut moves = String::new();
    //order: left down right up
    if start_x > end_x {
        for _ in 0..(start_x-end_x) {
            moves.push('<');
        }
    }
    if start_y > end_y {
        for _ in 0..(start_y-end_y) {
            moves.push('^');
        }
    }
    if start_y < end_y {
        for _ in 0..(end_y-start_y) {
            moves.push('v');
        }
    }
    if start_x < end_x {
        for _ in 0..(end_x-start_x) {
            moves.push('>');
        }
    }
    moves
}

fn shortest_directional(start: char, end: char) -> String {
    let (start_x, start_y) = char_to_coord_directional(start);
    let (end_x, end_y) = char_to_coord_directional(end);

    if start == '<' && end == '^' {
        return ">^".to_string();
    }
    else if start == '<' && end == 'A' {
        return ">>^".to_string();
    }
    else if start == '^' && end == '<' {
        return "v<".to_string();
    }
    else if start == 'A' && end == '<' {
        return "v<<".to_string();
    }

    let mut moves = String::new();
    //order: left down right up
    if start_x > end_x {
        for _ in 0..(start_x-end_x) {
            moves.push('<');
        }
    }
    if start_y > end_y {
        moves.push('^');
    }
    if start_y < end_y {
        moves.push('v');
    }
    if start_x < end_x {
        for _ in 0..(end_x-start_x) {
            moves.push('>');
        }
    }
    moves
}

fn char_to_coord_numpad(c: char) -> (usize, usize) {
    match c {
        '0' => (1, 3),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        'A' => (2, 3),
        _=> panic!("Invalid character!")
    }   
}

fn char_to_coord_directional(c: char) -> (usize, usize) {
    match c {
        '^' => (1, 0),
        'v' => (1, 1),
        '<' => (0, 1),
        '>' => (2, 1),
        'A' => (2, 0),
        _=> panic!("Invalid character!")
    }
}