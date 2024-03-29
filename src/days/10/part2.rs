mod data;
mod grid;

use std::collections::HashMap;

use grid::location::location;

use crate::current_problem::grid::{location::Location, print_tiles};

fn print_markers(v: &Vec<Vec<char>>) {
    for row in v {
        println!(
            "{}",
            row.iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
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
    let mut markers: HashMap<Location, char> = HashMap::new();
    for c in 0..grid.num_columns {
        let mut flip = false;
        for r in 0..grid.num_rows {
            let loc = location(r, c);
            let current_char = grid.tiles.get(&loc);
            let output = match (current_char, flip) {
                (Some('L' | 'F' | '7' | 'J' | '-' | 'S'), _) => '.',
                (Some('|'), _) => {
                    flip = !flip;
                    '.'
                }
                (Some('.'), true) => 'I',
                (Some('.'), false) => 'O',
                _ => '?',
            };
            markers.insert(
                Location {
                    x: r as i64,
                    y: c as i64,
                },
                output,
            );
        }
    }
    print_tiles(markers, grid.num_rows, grid.num_columns);
}
