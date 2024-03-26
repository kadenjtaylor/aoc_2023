mod data;
mod grid;

pub fn run() {
    let data = data::get_file_data();
    println!("Printing grid");
    data.lines()
        .map(|l| l.trim())
        .for_each(|l| println!("{}", l));
    println!("Parsing grid...");
    let grid = grid::parse_grid(data);
    println!("Found a grid of {}x{}", grid.row_max, grid.column_max);
    println!("Tiles: {}", grid.tiles.len());
    println!("Edges: {}", grid.edges.len());
    todo!("Label all the INSIDE, OUTSIDE, and LINE tiles")
}
