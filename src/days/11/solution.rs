use std::collections::HashMap;

mod data;

const GALAXY: char = '#';
const EMPTY: char = '.';

struct GalaxyMap {
    galaxy_locations: Vec<(usize, usize)>,
}

fn emtpy_rows(grid: &Vec<Vec<char>>) -> Vec<usize> {
    grid.into_iter()
        .enumerate()
        .flat_map(|(index, row)| {
            if row.into_iter().all(|&c| c == EMPTY) {
                vec![index]
            } else {
                vec![]
            }
        })
        .collect()
}

fn empty_columns(grid: &Vec<Vec<char>>) -> Vec<usize> {
    let column_numbers = 0..grid.get(0).unwrap().len();
    column_numbers
        .filter(|c| {
            (0..grid.len())
                .map(|r| grid.get(r).unwrap().get(*c).unwrap())
                .all(|&c| c == EMPTY)
        })
        .collect()
}

fn add_empty_row_at(grid: &mut Vec<Vec<char>>, index: usize) {
    let row_length = grid.get(0).unwrap().len();
    let row = vec![EMPTY; row_length];
    grid.insert(index, row);
}

fn add_empty_column_at(grid: &mut Vec<Vec<char>>, index: usize) {
    for row in grid {
        row.insert(index, EMPTY);
    }
}

fn expand(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let columns = empty_columns(&grid);
    let rows = emtpy_rows(&grid);
    println!("Empty Columns: {:?}", columns);
    println!("Empty Rows: {:?}", rows);
    let mut temp = grid.clone();
    let mut column_counter = 0;
    for column_num in columns {
        add_empty_column_at(&mut temp, column_num + column_counter);
        column_counter += 1;
    }
    let mut row_counter = 0;
    for row_num in rows {
        add_empty_row_at(&mut temp, row_num + row_counter);
        row_counter += 1;
    }
    temp
}

fn find_galaxies(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut galaxies = vec![];
    for row in 0..grid.len() {
        for column in 0..grid.get(row).unwrap().len() {
            if *grid.get(row).unwrap().get(column).unwrap() == GALAXY {
                galaxies.push((row, column));
            }
        }
    }
    galaxies
}

fn parse(data: &str) -> GalaxyMap {
    let grid: Vec<Vec<char>> = data.lines().map(|row| row.chars().collect()).collect();
    // print_grid(&grid);

    let gravity_expanded = expand(grid);
    // print_grid(&gravity_expanded);

    let galaxies = find_galaxies(&gravity_expanded);
    println!("Found {} galaxies.", galaxies.len());

    GalaxyMap {
        galaxy_locations: galaxies,
    }
}

fn taxicab_distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

pub fn run() {
    let data = data::from_file();
    let galaxy_map = parse(data);
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    for a in 0..galaxy_map.galaxy_locations.len() {
        for b in 0..galaxy_map.galaxy_locations.len() {
            if a < b {
                let a_loc = galaxy_map.galaxy_locations.get(a).unwrap();
                let b_loc = galaxy_map.galaxy_locations.get(b).unwrap();
                distances.insert((a, b), taxicab_distance(a_loc, b_loc));
            }
        }
    }
    let total = distances.into_iter().fold(0, |acc, ((_, _), d)| acc + d);
    println!("Total of shortest paths: {}", total);
}
