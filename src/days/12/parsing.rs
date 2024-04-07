#[path = "model.rs"]
mod model;

use model::{DamageRecords, Row};

fn parse_conditions(s: &str) -> (Vec<char>, &str) {
    let (condition_str, next) = s.split_once(" ").unwrap();
    (condition_str.chars().collect(), next)
}

fn parse_runs(s: &str) -> Vec<usize> {
    s.split(",").flat_map(|s| s.parse::<usize>()).collect()
}

fn parse_one_row(line: &str) -> Row {
    let (conditions, next) = parse_conditions(line);
    let runs = parse_runs(next);
    Row {
        parsed_conditions: conditions,
        damaged_runs: runs,
    }
}

pub fn parse(s: &str) -> DamageRecords {
    s.lines()
        .map(|l| l.trim())
        .map(|l| parse_one_row(l))
        .collect()
}
