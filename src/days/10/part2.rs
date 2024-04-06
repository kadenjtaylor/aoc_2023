mod data;
mod grid;

use location::Direction;
use std::collections::{HashMap, HashSet};

use crate::current_problem::grid::{
    location::{self, Location},
    print_tiles,
};

use self::grid::{location::hop, Grid};

fn decorate(grid: &grid::Grid) -> HashMap<Location, char> {
    // Setting up the labels
    let mut labels = HashMap::new();
    // Figuring out where to step first
    let options = grid::neighbors(&grid.s_location, &grid.edges);
    let first_pair = (grid.s_location.clone(), options.first().unwrap().clone());
    // The beginning of the path
    let mut path = vec![first_pair];
    let mut done = false;
    while !done {
        let (last, current) = path.last().unwrap();

        if let Some(next) = grid::get_next_location((last.clone(), current.clone()), &grid) {
            labels.insert(current.clone(), LOOP);
            if next == grid.s_location {
                // labels.insert(current.clone(), LOOP);
                labels.insert(next.clone(), LOOP);
                // labels.insert(next.clone(), grid.tiles.get(&next).unwrap().clone());
                done = true;
            }
            // mark which tiles are ON the line
            // labels.insert(current.clone(), grid.tiles.get(current).unwrap().clone());
            let current_tile = grid.tiles.get(current).expect("Missing tile!");
            let (inside, outside) = partition_border_locations(last, current, current_tile);
            for i in inside {
                if !labels.contains_key(&i) {
                    labels.insert(i, INSIDE);
                }
            }
            for o in outside {
                if !labels.contains_key(&o) {
                    labels.insert(o, OUTSIDE);
                }
            }
            path.push((current.clone(), next));
        }
    }
    labels
        .into_iter()
        .filter(|(loc, _)| {
            loc.x >= 0
                && loc.y >= 0
                && loc.x < grid.num_rows.try_into().unwrap()
                && loc.y < grid.num_columns.try_into().unwrap()
        })
        .collect()
}

fn partition_border_locations(
    last: &Location,
    current: &Location,
    tile: &char,
) -> (Vec<Location>, Vec<Location>) {
    let mut starboard = vec![];
    let mut port = vec![];
    let hop_dir = location::determine_hop(last, current).expect("invalid hop");
    match (hop_dir, tile) {
        (Direction::UP, 'F') => {
            for loc in [Direction::UP, Direction::LEFT].map(|d| hop(current, d)) {
                port.push(loc);
            }
        }
        (Direction::UP, '7') => {
            for loc in [Direction::UP, Direction::RIGHT].map(|d| hop(current, d)) {
                starboard.push(loc);
            }
        }
        (Direction::UP, _) => {
            starboard.push(hop(current, Direction::RIGHT));
            port.push(hop(current, Direction::LEFT));
        }
        (Direction::DOWN, _) => {
            starboard.push(hop(current, Direction::LEFT));
            port.push(hop(current, Direction::RIGHT));
        }
        (Direction::LEFT, _) => {
            starboard.push(hop(current, Direction::UP));
            port.push(hop(current, Direction::DOWN));
        }
        (Direction::RIGHT, _) => {
            starboard.push(hop(current, Direction::DOWN));
            port.push(hop(current, Direction::UP));
        }
    };
    (starboard, port)
}

fn ortho(loc: &Location) -> [Location; 4] {
    [
        Direction::UP,
        Direction::DOWN,
        Direction::LEFT,
        Direction::RIGHT,
    ]
    .map(|d| hop(loc, d))
}

fn checkable(
    loc: &Location,
    grid: &Grid,
    visited: &HashSet<Location>,
    labels: &HashMap<Location, char>,
) -> Vec<Location> {
    ortho(loc)
        .into_iter()
        .filter(|loc| !(labels.get(loc).is_some() && *labels.get(loc).unwrap() == LOOP))
        .filter(|loc| grid.tiles.get(loc).is_some())
        .filter(|loc| !visited.contains(loc))
        .collect()
}

fn nearest_label(loc: &Location, grid: &Grid, labels: &HashMap<Location, char>) -> Option<char> {
    let mut visited = HashSet::<Location>::new();
    let mut tiles_to_check: Vec<Location> = checkable(loc, grid, &visited, labels);
    while !tiles_to_check.is_empty() {
        let top = tiles_to_check.pop().unwrap();
        visited.insert(top.clone());
        match labels.get(&top) {
            Some(&INSIDE) => return Some(INSIDE),
            Some(&OUTSIDE) => return Some(OUTSIDE),
            _ => checkable(&top, grid, &visited, labels)
                .iter()
                .for_each(|l| tiles_to_check.push(l.clone())),
        }
    }
    return None;
}

pub fn fill_missing_tiles(labels: HashMap<Location, char>, grid: &Grid) -> HashMap<Location, char> {
    let mut filled: HashMap<Location, char> = HashMap::new();
    let mut num_ignored = 0;
    let mut num_filled = 0;
    for y in (0..grid.num_columns).rev() {
        for x in 0..grid.num_rows {
            let loc = Location {
                x: x as i64,
                y: y as i64,
            };
            match labels.get(&loc) {
                Some(_) => {
                    num_ignored += 1;
                }
                None => {
                    num_filled += 1;
                    let guess = match nearest_label(&loc, grid, &labels) {
                        Some(c) => c,
                        None => UNKNOWN,
                    };
                    filled.insert(loc, guess);
                }
            }
        }
    }
    println!("Labels Known: {} / Filled: {}", num_ignored, num_filled);
    filled
}

const INSIDE: char = 'I';
const OUTSIDE: char = 'O';
const LOOP: char = '.';
const UNKNOWN: char = '?';

fn flip(c: &char) -> char {
    match c {
        &INSIDE => OUTSIDE,
        &OUTSIDE => INSIDE,
        &a => a,
    }
}

fn flip_labels_if_necessary(labels: &HashMap<Location, char>) -> HashMap<Location, char> {
    // find the first label touching the border
    let choices: Vec<char> = labels
        .iter()
        .flat_map(|(loc, &c)| {
            if (loc.x == 0 || loc.y == 0) && c != LOOP && c != UNKNOWN {
                vec![c]
            } else {
                vec![]
            }
        })
        .collect();
    let correct_labels = match choices.first().unwrap() {
        &OUTSIDE => labels.clone(),
        &INSIDE => {
            println!("Flipping INSIDE/OUTSIDE labels");
            labels
                .iter()
                .map(|(loc, c)| (loc.clone(), flip(c)))
                .collect()
        }
        _ => panic!("Not enough known tiles to establish INSIDE/OUTSIDE"),
    };
    correct_labels
}

pub fn run() {
    let data = data::get_file_data();
    println!("Parsing grid...");
    let grid = grid::parse_grid(data);
    println!("Found a grid of {}x{}", grid.num_rows, grid.num_columns);
    println!("Tiles: {}", grid.tiles.len());
    println!("Edges: {}", grid.edges.len());
    println!("------------------------------");
    print_tiles(grid.tiles.clone(), grid.num_rows, grid.num_columns);
    println!("------------------------------");
    let mut labels = decorate(&grid);
    println!("Number of labels: {}", labels.len());
    print_tiles(labels.clone(), grid.num_rows, grid.num_columns);
    println!("------------------------------");
    // TODO: Use decoration to fill the rest of the question marks
    let filler = fill_missing_tiles(labels.clone(), &grid);
    labels.extend(filler);
    let flipped = flip_labels_if_necessary(&labels);
    print_tiles(flipped.clone(), grid.num_rows, grid.num_columns);
    let num_inside = flipped.iter().filter(|(_, &c)| c == INSIDE).count();
    let num_outside = flipped.iter().filter(|(_, &c)| c == OUTSIDE).count();
    let num_loop = flipped.iter().filter(|(_, &c)| c == LOOP).count();
    let num_unknown = flipped.iter().filter(|(_, &c)| c == UNKNOWN).count();
    println!(
        "Inside: {}, Outside: {}, Loop: {}, Unknown: {}, Total: {}",
        num_inside,
        num_outside,
        num_loop,
        num_unknown,
        num_inside + num_outside + num_loop + num_unknown
    );
}

// SO CLOSE
// clockwise - I: 351, O; 5880, L: 13364, ?: 5  - Total: 19600
// counter   - I: 350, O: 5876, L; 13364, ?: 10 - Total: 19600
