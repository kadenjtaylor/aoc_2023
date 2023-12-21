mod common;

use common::{cards_from_file, winning_numbers, Card};

// TODO: This feels messy
fn score_card(num_correct: usize) -> i32 {
    if num_correct == 0 {
        0
    } else {
        2_i32.pow((num_correct - 1).try_into().unwrap())
    }
}

fn score_all_cards(cards: impl Iterator<Item = Card>) -> i32 {
    let winning_guesses = cards.map(|c| winning_numbers(&c));
    let points = winning_guesses.map(|guesses| score_card(guesses.len()));
    points.sum()
}

#[test]
fn example() {
    let cards = common::cards_from_example();
    let total_score = score_all_cards(cards);
    assert_eq!(total_score, 13)
}

pub fn run() {
    let cards = cards_from_file();
    let score = score_all_cards(cards);
    println!("Final Score: {}", score)
}
