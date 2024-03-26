mod location;

use location::{determine_hop, down, hop, left, right, up, Direction, Edge, Location};

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Grid {
    pub tiles: HashMap<Location, char>,
    pub edges: HashSet<Edge>,
    pub s_location: Location,
    pub row_max: usize,
    pub column_max: usize,
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
    let mut max_row = 0;
    let mut max_column = 0;
    let mut tiles = HashMap::new();
    let mut edges = HashSet::new();
    let mut s_location = None;
    for (row_num, row) in input.lines().rev().enumerate() {
        let trimmed = row.trim();
        if row_num > max_row {
            max_row = row_num;
        }
        if trimmed.len() > max_column {
            max_column = trimmed.len();
        }
        for (col_num, c) in trimmed.chars().enumerate() {
            let loc = Location {
                x: col_num as i64,
                y: row_num as i64,
            };
            if c == 'S' {
                s_location = Some(loc.clone());
            }
            tiles.insert(loc.clone(), c);
            match find_connections(&loc, c) {
                Some((e1, e2)) => {
                    edges.insert(e1);
                    edges.insert(e2);
                }
                None => (),
            }
        }
    }
    Grid {
        tiles: tiles,
        edges: edges,
        s_location: s_location.unwrap(),
        row_max: max_row,
        column_max: max_column,
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
