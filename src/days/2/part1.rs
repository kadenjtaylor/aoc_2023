mod common;

use common::{min_required_counts, parse_game, Game};

fn is_valid_game(game: &Game, r: i32, g: i32, b: i32) -> bool {
    let counts = min_required_counts(game);
    let is_valid = counts.r <= r && counts.g <= g && counts.b <= b;
    println!(
        "[{}] #{} - {:?}",
        if is_valid { "X" } else { "_" },
        game.id,
        (counts.r, counts.g, counts.b)
    );
    is_valid
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
    let games = lines
        .iter()
        .map(|line| {
            let error_msg = format!("Failed to parse into game: \"{}\"", line);
            parse_game(line).expect(&error_msg)
        })
        .filter(|game| is_valid_game(game, 12, 13, 14));
    let id_total: i32 = games.map(|g| g.id).sum();
    assert_eq!(id_total, 8)
}

pub fn run() {
    let lines = include_str!("../../../resources/problem_2.txt").lines();

    let valid_games = lines
        .map(|line| {
            let error_msg = format!("Failed to parse into game: \"{}\"", line);
            parse_game(line).expect(&error_msg)
        })
        .filter(|game| is_valid_game(game, 12, 13, 14));

    let id_total: i32 = valid_games.clone().map(|g| g.id).sum();

    println!("Sum of valid game ids: {id_total}");
}
