use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

#[derive(PartialEq)]
enum DiskContent1 {
    EMPTY,
    FILE(usize)
}

#[derive(PartialEq)]
enum DiskContent2 {
    EMPTY(usize),
    FILE(usize, usize)
}

pub fn solve1() -> usize {
    let file = File::open("src/day9/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut checksum = 0;
    let mut filecount = 0;
    let mut disk: Vec<DiskContent1> = vec![];

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();
        let mut reading_empty = false;
        let mut id = 0;
        for c in line_clone.chars() {
            let num = usize::from_str_radix(&c.to_string(), 10).unwrap();
            for _ in 0..num {
                if reading_empty {
                    disk.push(DiskContent1::EMPTY);
                }
                else {
                    disk.push(DiskContent1::FILE(id));
                    filecount += 1;
                }
            }
            if !reading_empty {
                id += 1;
            }
            reading_empty ^= true;
        }
    }

    let mut reverse_counter = disk.len()-1;
    for i in 0..filecount {
        if disk[i] == DiskContent1::EMPTY {
            disk.swap(i, reverse_counter);

            while disk[reverse_counter] == DiskContent1::EMPTY {
                reverse_counter -= 1;
            }
        }
    }

    for i in 0..disk.len() {
        if disk[i] == DiskContent1::EMPTY {
            break;
        }
        if let DiskContent1::FILE(id) = disk[i] {
            checksum += i*id;
        }
    }

    checksum
}

pub fn solve2() -> usize {
    let file = File::open("src/day9/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut checksum = 0;
    let mut disk: Vec<DiskContent2> = vec![];
    let mut search_map: HashMap<usize, usize> = HashMap::with_capacity(9);

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();
        let mut reading_empty = false;
        let mut id = 0;
        for c in line_clone.chars() {
            let num = usize::from_str_radix(&c.to_string(), 10).unwrap();
            if reading_empty {
                disk.push(DiskContent2::EMPTY(num));
            }
            else {
                disk.push(DiskContent2::FILE(id, num));
            }
            if !reading_empty {
                id += 1;
            }
            reading_empty ^= true;
        }
    }

    let disk_size = disk.len();
    for i in (0..disk_size).rev() {
        match disk[i] {
            DiskContent2::EMPTY(_) => continue,
            DiskContent2::FILE(id, num) => {
                let search_start = match search_map.get(&num) {
                    Some(start) => *start,
                    None => 0,
                };
                for j in search_start..i {
                    match disk[j] {
                        DiskContent2::EMPTY(lenght) => {
                            if lenght == num {
                                disk.swap(i, j);
                                search_map.insert(num, j+1);
                                break;
                            }
                            else if lenght > num {
                                let diff = lenght - num;
                                disk.splice(j..=j, [DiskContent2::FILE(id, num), DiskContent2::EMPTY(diff)]);
                                disk[i+1] = DiskContent2::EMPTY(num);
                                search_map.insert(num, j+1);
                                break;
                            }
                        },
                        DiskContent2::FILE(_, _) => continue,
                    }
                }
            },
        }
    }

    let mut i = 0;
    for block in &disk {
        match block {
            DiskContent2::EMPTY(num) => i += num,
            DiskContent2::FILE(id, num) => {
                for _ in 0..*num {
                    checksum += i*id;
                    i += 1;
                }
            },
        }
    }

    checksum
}