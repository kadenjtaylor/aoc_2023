use std::{collections::HashMap, fmt::Display};

use regex::Regex;

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

impl Display for Node<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} = ({}, {})",
            self.name, self.left, self.right
        ))
    }
}

#[derive(Debug)]
struct Network<'a> {
    nodes: HashMap<&'a str, Node<'a>>,
}

fn parse_network<'a>(lines: impl Iterator<Item = &'a str>) -> Network<'a> {
    let mut net = Network {
        nodes: HashMap::new(),
    };
    lines.flat_map(|l| parse_node(&l)).for_each(|n| {
        net.nodes.insert(n.name, n);
    });
    net
}

fn parse_node(line: &str) -> Option<Node<'_>> {
    let regex_def = r"(?<name>[A-Z0-9]{3}) \= \((?<left>[A-Z0-9]{3}), (?<right>[A-Z0-9]{3})\)";
    let r = Regex::new(regex_def);
    let a = r.unwrap();
    let caps = a.captures(line);
    caps.map(|c| Node {
        name: c.name("name").expect("no match").as_str(),
        left: c.name("left").expect("no match").as_str(),
        right: c.name("right").expect("no match").as_str(),
    })
    // .expect(&format!("Failed to parse node from line {}", line))
}

#[derive(Debug)]
enum Move {
    L,
    R,
}

#[derive(Debug)]
struct Pattern {
    current: usize,
    moves: Vec<Move>,
}

fn next(p: &mut Pattern) -> &Move {
    match p.moves.get(p.current + 1) {
        Some(m) => {
            p.current = p.current + 1;
            m
        }
        None => {
            p.current = 0;
            p.moves.get(0).unwrap()
        }
    }
}

fn run_pattern<'a, F>(map: &'a Map<'a>, start: &'a str, is_end: F) -> Vec<&'a str>
where
    F: Fn(&str) -> bool,
{
    // println!("Running from {} to {}...", start, end);
    let mut pattern = parse_pattern(map.sequence);
    let mut path = vec![];
    let mut current_name = start;
    while !is_end(current_name) {
        let node_ptr = map.network.nodes.get(current_name).unwrap();
        path.push(current_name);
        // println!("Pushing {}, ({})", current_name, current_name.len());
        match next(&mut pattern) {
            Move::L => current_name = node_ptr.left,
            Move::R => current_name = node_ptr.right,
        }
    }
    path.push(current_name);
    // println!("Pushing {}", current_name);
    path
}

fn parse_pattern(pattern: &str) -> Pattern {
    let mut result = vec![];
    for c in pattern.chars() {
        result.push(match c {
            'L' => Move::L,
            'R' => Move::R,
            _ => todo!(),
        });
    }
    Pattern {
        current: result.len() - 1,
        moves: result,
    }
}
#[derive(Debug)]
struct Map<'a> {
    sequence: &'a str,
    network: Network<'a>,
}

fn parse_map(map_data: &str) -> Map {
    // for (i, line) in map_data.lines().enumerate() {}
    let mut pattern = "";
    let mut chunks = map_data.lines();
    if let Some(line) = chunks.next() {
        pattern = line;
    }
    chunks.next();
    Map {
        sequence: pattern,
        network: parse_network(chunks),
    }
}

fn num_steps_to_zzz(map_data: &str) -> usize {
    run_pattern(&parse_map(map_data), "AAA", |s| s == "ZZZ").len() - 1
}

#[test]
fn example_1() {
    let map_data = "RL
    
    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)";
    let map = parse_map(map_data);
    let p = run_pattern(&map, "AAA", |s| s == "ZZZ");
    assert_eq!(vec!["AAA", "CCC", "ZZZ"], p)
}

#[test]
fn example_2() {
    let map_data = "LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)";
    assert_eq!(num_steps_to_zzz(map_data), 6)
}

pub fn run() {
    let map_data = include_str!("../../../resources/network_map.txt");
    let n = num_steps_to_zzz(map_data);
    println!("Found a solution in {:?} steps!", n);
}
