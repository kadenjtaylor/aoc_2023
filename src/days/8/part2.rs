mod part1;

use part1::{parse_map, Network};

fn steps_till_z_node<'a>(
    network: &Network<'a>,
    start_node: &'a str,
    pattern_iterator: &mut impl Iterator<Item = char>,
) -> (i32, String) {
    // println!("Starting a path with: \"{}\"", start_node);
    let mut ptr = start_node;
    let mut step_number = 0;
    let mut done = ptr.contains("Z");
    while !done {
        let next_move = pattern_iterator.next().unwrap();
        ptr = network
            .nodes
            .get(ptr)
            .and_then(|n| match next_move {
                'L' => Some(n.left),
                'R' => Some(n.right),
                _ => None,
            })
            .unwrap();
        // Loop maintenance
        step_number += 1;
        done = ptr.contains("Z");
    }
    (step_number, ptr.to_owned())
}

// Greatest Common Factor
fn gcf(a: u64, b: u64) -> u64 {
    if a == b {
        return a;
    }

    let (mut greater, mut lesser) = if a > b { (a, b) } else { (b, a) };

    while lesser > 0 {
        let temp = greater;
        greater = lesser;
        lesser = temp % lesser;
    }

    return greater;
}

// Least Common Multiple
//  LCM (a, b) = (a × b) / HCF(a, b)
fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcf(a, b)
}

pub fn run() {
    let map_data = include_str!("../../../resources/network_map.txt");
    let map = parse_map(map_data);

    let current_nodes: Vec<&str> = map
        .network
        .nodes
        .values()
        .map(|n| n.name)
        .filter(|node_name| node_name.contains("A"))
        .collect();

    let mut pattern_iterator: std::iter::Cycle<std::str::Chars<'_>> = map.sequence.chars().cycle();

    let nums = current_nodes
        .iter()
        .map(|n| steps_till_z_node(&map.network, n, &mut pattern_iterator))
        .map(|(num_steps, _)| num_steps as u64);

    let answer = nums.reduce(lcm);

    println!("Answer: {:?}", answer);
}
