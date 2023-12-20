// #[path = "common.rs"]
mod common;

use common::CellGroup;

struct Gear<'a> {
    first_part: &'a CellGroup,
    second_part: &'a CellGroup,
}

pub fn run() {
    let schematic = common::produce_schematic();

    // Still need part numbers from last time
    let part_numbers = schematic
        .number_groups
        .iter()
        .filter(|&n| common::is_part(n, &schematic.symbols))
        .collect::<Vec<&CellGroup>>();

    // A gear is any '*' symbol that is adjacent to exactly 2 part numbers
    let mut gears: Vec<Gear> = vec![];
    for symbol_cell in schematic.symbols {
        let near_parts = part_numbers
            .iter()
            .filter(|p| common::does_border(p, &symbol_cell))
            .collect::<Vec<_>>();
        if symbol_cell.content == '*' && near_parts.len() == 2 {
            gears.push(Gear {
                first_part: unsafe { near_parts.get_unchecked(0) },
                second_part: unsafe { near_parts.get_unchecked(1) },
            })
        }
    }
    println!("Found {} gears!", gears.len());

    // The gear ratio is the product of the two part numbers
    let ratios = gears.iter().map(|g| {
        let first = g
            .first_part
            .num
            .parse::<i64>()
            .expect("couldn't parse part number");
        let second = g
            .second_part
            .num
            .parse::<i64>()
            .expect("couldn't parse part number");
        first * second
    });

    // The answer is the sum of the gear ratios
    let sum_of_ratios: i64 = ratios.sum();
    println!("Sum of gear ratios: {}", sum_of_ratios);
}
