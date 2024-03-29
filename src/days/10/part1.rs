mod data;
mod grid;

use data::get_file_data;
use grid::{get_next_location, neighbors, parse_grid};

pub fn run() {
    let data = get_file_data();
    println!("Printing grid");
    data.lines()
        .map(|l| l.trim())
        .for_each(|l| println!("{}", l));
    println!("Parsing grid...");
    let grid = parse_grid(data);
    println!("Found a grid of {}x{}", grid.num_rows, grid.num_columns);
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
