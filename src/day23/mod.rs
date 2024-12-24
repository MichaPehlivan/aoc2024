use std::{collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}};

use regex::Regex;

pub fn solve1() -> usize {
    let file = File::open("src/day23/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    let mut tripple_connections: Vec<HashSet<String>> = vec![];

    let re = Regex::new(r"(\S+)-(\S+)").expect("Invalid regex!");

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        let connection = re.captures(&line_clone).unwrap();
        let pc_1 = connection[1].to_string();
        let pc_2 = connection[2].to_string();
        connections.entry(pc_1.clone()).or_insert(HashSet::new()).insert(pc_2.clone());
        connections.entry(pc_2).or_insert(HashSet::new()).insert(pc_1);
    }

    for (pc_1, conn_1) in &connections {
        for pc_2 in conn_1 {
            let conn_2 = connections.get(pc_2).unwrap();
            for pc_3 in conn_1 {
                if conn_2.contains(pc_3) {
                    let mut tripple = HashSet::with_capacity(3);
                    tripple.insert(pc_1.clone());
                    tripple.insert(pc_2.clone());
                    tripple.insert(pc_3.clone());
                    if !tripple_connections.contains(&tripple) {
                        tripple_connections.push(tripple);
                    }
                }
            }
        }
    }

    let mut count = 0;
    for tripple in tripple_connections {
        if contains_t(tripple) {
            count += 1;
        }
    }
    count
}

fn contains_t(lan: HashSet<String>) -> bool {
    let mut contains = false;
    for pc in lan {
        if pc.get(0..1).unwrap() == "t" {
            contains = true;
            break;
        }
    }
    contains
}

pub fn solve2() -> String {
    let file = File::open("src/day23/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    let mut vertex_set = HashSet::new();

    let re = Regex::new(r"(\S+)-(\S+)").expect("Invalid regex!");

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        let connection = re.captures(&line_clone).unwrap();
        let pc_1 = connection[1].to_string();
        let pc_2 = connection[2].to_string();

        if !vertex_set.contains(&pc_1) {
            vertex_set.insert(pc_1.clone());
        }
        if !vertex_set.contains(&pc_2) {
            vertex_set.insert(pc_2.clone());
        }

        connections.entry(pc_1.clone()).or_insert(HashSet::new()).insert(pc_2.clone());
        connections.entry(pc_2).or_insert(HashSet::new()).insert(pc_1);
    }

    let mut cliques = vec![];
    bron_kerbosch(HashSet::new(), vertex_set, HashSet::new(), &connections, &mut cliques);

    let mut max_clique: Vec<String> = cliques.iter().map(|clique| (clique.len(), clique)).max_by(|a, b| a.0.cmp(&b.0)).unwrap().1.iter().cloned().collect();
    max_clique.sort();
    let password = max_clique.join(",");
    password
}

fn bron_kerbosch(r: HashSet<String>, p: HashSet<String>, x: HashSet<String>, graph: &HashMap<String, HashSet<String>>, max_cliques: &mut Vec<HashSet<String>>) {
    if p.is_empty() && x.is_empty() {
        max_cliques.push(r);
        return;
    }

    let mut p_clone = p.clone();
    for v in &p {
        let mut r_union = r.clone();
        r_union.insert(v.clone());

        let p_intersect: HashSet<String> = p_clone.intersection(&graph.get(v).unwrap()).cloned().collect();

        let x_intersect: HashSet<String> = x.intersection(&graph.get(v).unwrap()).cloned().collect();
        bron_kerbosch(r_union, p_intersect, x_intersect, graph, max_cliques);

        p_clone.remove(v);
    }
}

