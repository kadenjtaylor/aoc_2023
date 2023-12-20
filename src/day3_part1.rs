#[path = "day3/common.rs"]
mod common;

use common::CellGroup;

pub fn run() {
    let schematic = common::produce_schematic();

    println!("Numbers found: {}", schematic.number_groups.len());

    let part_numbers = schematic
        .number_groups
        .iter()
        .filter(|&n| common::is_part(n, &schematic.symbols))
        .collect::<Vec<&CellGroup>>();

    println!("Part Numbers found: {}", part_numbers.len());
    let sum = part_numbers
        .iter()
        .map(|cg| cg.num.parse::<i32>().expect("FAILED TO PARSE NUMBER"))
        .sum::<i32>();
    println!("Sum of part numbers: {}", sum);
}
