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

fn find_galaxies_expanded(grid: &Vec<Vec<char>>, expansion_factor: usize) -> Vec<(usize, usize)> {
    let mut galaxies = vec![];
    let empty_columns = empty_columns(&grid);
    let empty_rows = emtpy_rows(&grid);
    println!("Expanding empty rows: {:?}", empty_rows);
    println!("Expanding empty columns: {:?}", empty_columns);
    for row in 0..grid.len() {
        for column in 0..grid.get(row).unwrap().len() {
            if *grid.get(row).unwrap().get(column).unwrap() == GALAXY {
                let num_empty_rows = empty_rows.iter().filter(|r_index| r_index < &&row).count()
                    * (expansion_factor - 1);
                let num_empty_columns = empty_columns
                    .iter()
                    .filter(|r_index| r_index < &&column)
                    .count()
                    * (expansion_factor - 1);
                galaxies.push((row + num_empty_rows, column + num_empty_columns));
            }
        }
    }
    galaxies
}

fn parse(data: &str, expansion_factor: usize) -> GalaxyMap {
    let grid: Vec<Vec<char>> = data.lines().map(|row| row.chars().collect()).collect();
    let galaxies = find_galaxies_expanded(&grid, expansion_factor);
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
    let galaxy_map = parse(data, 1000000);
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
