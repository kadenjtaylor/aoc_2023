#[path = "location.rs"]
pub mod location;

use location::{determine_hop, down, hop, left, right, up, Direction, Edge, Location};

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Grid {
    pub tiles: HashMap<Location, char>,
    pub edges: HashSet<Edge>,
    pub s_location: Location,
    pub num_rows: usize,
    pub num_columns: usize,
}

fn find_connections(loc: &Location, c: char) -> Option<(Edge, Edge)> {
    match c {
        '|' => Some((up(loc), down(loc))),
        '-' => Some((left(loc), right(loc))),
        'L' => Some((up(loc), right(loc))),
        'F' => Some((down(loc), right(loc))),
        'J' => Some((up(loc), left(loc))),
        _ => None,
    }
}

pub fn parse_grid(input: &'static str) -> Grid {
    let mut column_max = 0;
    let mut row_max = 0;
    let mut tiles = HashMap::new();
    let mut edges = HashSet::new();
    let mut s_location = None;
    for (column_index, column_content) in input.lines().rev().enumerate() {
        let trimmed = column_content.trim();
        if column_index > column_max {
            column_max = column_index;
        }
        if trimmed.len() > row_max {
            row_max = trimmed.len();
        }
        for (row_index, c) in trimmed.chars().enumerate() {
            let loc = Location {
                x: row_index as i64,
                y: column_index as i64,
            };
            if c == 'S' {
                s_location = Some(loc.clone());
            }
            tiles.insert(loc.clone(), c);
            match find_connections(&loc, c) {
                Some((e1, e2)) => {
                    edges.insert(e1.clone());
                    edges.insert(e2.clone());
                }
                None => (),
            }
        }
    }
    Grid {
        tiles: tiles,
        edges: edges,
        s_location: s_location.unwrap(),
        num_rows: row_max,
        num_columns: column_max + 1,
    }
}

pub fn print_tiles(tiles: HashMap<Location, char>, num_rows: usize, num_columns: usize) {
    for y in (0..num_columns).rev() {
        for x in 0..num_rows {
             match tiles.get(&Location {
                x: x as i64,
                y: y as i64
            }) {
                Some(c) => print!("{}",c),
                None => print!("?")
            }
        }
        println!();
    }
}

// Finds cells that are next to each other in the grid/graph
pub fn neighbors(loc: &Location, edges: &HashSet<Edge>) -> Vec<Location> {
    edges
        .iter()
        .flat_map(|(origin, dest)| {
            if origin == loc {
                vec![dest]
            } else if dest == loc {
                vec![origin]
            } else {
                vec![]
            }
        })
        .map(|l| l.clone())
        .collect()
}

// Takes the tile and the direction of entry into the tile, produces out direction
fn next_direction(c: char, incoming: Direction) -> Option<Direction> {
    match (c, incoming) {
        ('J', Direction::RIGHT) => Some(Direction::UP),
        ('J', Direction::DOWN) => Some(Direction::LEFT),
        ('7', Direction::RIGHT) => Some(Direction::DOWN),
        ('7', Direction::UP) => Some(Direction::LEFT),
        ('F', Direction::UP) => Some(Direction::RIGHT),
        ('F', Direction::LEFT) => Some(Direction::DOWN),
        ('L', Direction::LEFT) => Some(Direction::UP),
        ('L', Direction::DOWN) => Some(Direction::RIGHT),
        ('|', Direction::DOWN) => Some(Direction::DOWN),
        ('|', Direction::UP) => Some(Direction::UP),
        ('-', Direction::LEFT) => Some(Direction::LEFT),
        ('-', Direction::RIGHT) => Some(Direction::RIGHT),
        ('.', _) => panic!("We're off course!"),
        _ => None,
    }
}

pub fn get_next_location((last, current): (Location, Location), grid: &Grid) -> Option<Location> {
    let current_char = grid.tiles.get(&current).unwrap().clone();
    let outgoing_direction =
        determine_hop(&last, &current).and_then(|dir| next_direction(current_char, dir));
    outgoing_direction.map(|dir| hop(&current, dir))
}
