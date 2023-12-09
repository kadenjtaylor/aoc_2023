#[path = "day2/model.rs"]
mod day2;

use day2::{min_required_counts, parse_game, RGBCounts};

fn power(counts: RGBCounts) -> i32 {
    counts.r * counts.g * counts.b
}

#[test]
fn run_test_case() {
    let lines = [
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    ];
    let power_sum: i32 = lines
        .iter()
        .map(|line| {
            let error_msg = format!("Failed to parse into game: \"{}\"", line);
            parse_game(line).expect(&error_msg)
        })
        .map(|g| min_required_counts(&g))
        .map(|c| power(c))
        .sum();
    assert_eq!(power_sum, 2286)
}

pub fn run() {
    let lines = include_str!("../resources/problem_2.txt").lines();
    let power_sum: i32 = lines
        .map(|line| {
            let error_msg = format!("Failed to parse into game: \"{}\"", line);
            parse_game(line).expect(&error_msg)
        })
        .map(|g| min_required_counts(&g))
        .map(|c| power(c))
        .sum();
    println!("Power Sum: {power_sum}");
}
