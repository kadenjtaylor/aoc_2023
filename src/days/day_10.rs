use std::collections::{HashMap, HashSet};

fn get_easy_data() -> &'static str {
    ".....
  .S-7.
  .|.|.
  .L-J.
  ....."
}

fn get_complex_data() -> &'static str {
    "..F7.
    .FJ|.
    SJ.L7
    |F--J
    LJ..."
}

fn get_file_data() -> &'static str {
    include_str!("../../resources/pipe_grid.txt")
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Location {
    x: i64,
    y: i64,
}

type Edge = (Location, Location);

#[derive(Debug)]
struct Grid {
    tiles: HashMap<Location, char>,
    edges: HashSet<Edge>,
    s_location: Location,
    row_max: usize,
    column_max: usize,
}

fn up(loc: &Location) -> Edge {
    (
        loc.clone(),
        Location {
            x: loc.x,
            y: loc.y + 1,
        },
    )
}

fn down(loc: &Location) -> Edge {
    (
        loc.clone(),
        Location {
            x: loc.x,
            y: loc.y - 1,
        },
    )
}

fn left(loc: &Location) -> Edge {
    (
        loc.clone(),
        Location {
            x: loc.x - 1,
            y: loc.y,
        },
    )
}

fn right(loc: &Location) -> Edge {
    (
        loc.clone(),
        Location {
            x: loc.x + 1,
            y: loc.y,
        },
    )
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

// Determines if a "hop" of one orthogonal space occurred and what it was
fn determine_hop(origin: &Location, destination: &Location) -> Option<Direction> {
    let y_diff = destination.y - origin.y;
    let x_diff = destination.x - origin.x;
    match (x_diff, y_diff) {
        (1, 0) => Some(Direction::RIGHT),
        (0, 1) => Some(Direction::UP),
        (-1, 0) => Some(Direction::LEFT),
        (0, -1) => Some(Direction::DOWN),
        _ => None,
    }
}

fn hop(loc: &Location, d: Direction) -> Location {
    match d {
        Direction::RIGHT => right(&loc).1,
        Direction::UP => up(&loc).1,
        Direction::DOWN => down(&loc).1,
        Direction::LEFT => left(&loc).1,
    }
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

fn parse_grid(input: &'static str) -> Grid {
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
fn neighbors(loc: &Location, edges: &HashSet<Edge>) -> Vec<Location> {
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

fn get_next_location((last, current): (Location, Location), grid: &Grid) -> Option<Location> {
    let current_char = grid.tiles.get(&current).unwrap().clone();
    let outgoing_direction =
        determine_hop(&last, &current).and_then(|dir| next_direction(current_char, dir));
    outgoing_direction.map(|dir| hop(&current, dir))
}

pub fn run() {
    let data = get_file_data();
    println!("Printing grid");
    data.lines()
        .map(|l| l.trim())
        .for_each(|l| println!("{}", l));
    println!("Parsing grid...");
    let grid = parse_grid(data);
    println!("Found a grid of {}x{}", grid.row_max, grid.column_max);
    println!("Tiles: {}", grid.tiles.len());
    println!("Edges: {}", grid.edges.len());
    println!("S Location: {:?}", grid.s_location);

    // Start at S location
    let options = neighbors(&grid.s_location, &grid.edges);

    let first_pair = (grid.s_location.clone(), options.first().unwrap().clone());
    let mut path = vec![first_pair];
    let mut done = false;
    while !done {
        let (last, current) = path.last().unwrap();

        if let Some(next) = get_next_location((last.clone(), current.clone()), &grid) {
            if next == grid.s_location {
                done = true;
            }
            path.push((current.clone(), next));
        }
    }
    println!("Path of Length: {:?} found", path.len());
    println!("Midpoint Distance: {}", path.len() / 2);
}
