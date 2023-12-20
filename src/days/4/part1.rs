use regex::Regex;

#[derive(Debug)]
struct Card {
    winning: Vec<i32>,
    guesses: Vec<i32>,
}

fn parse_card(line: &str) -> Result<Card, &str> {
    let card_groups = "Card\\s+(?<card_number>.+):\\s+(?<winning>[\\s0-9]+)\\|\\s+(?<guesses>.+)";
    let card_regex = Regex::new(card_groups).unwrap();
    let Some(caps) = card_regex.captures(line) else {
        return Err("non-matching line!");
    };
    // let id = caps["card_number"].parse::<i32>().unwrap();
    let winning = caps["winning"]
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let guesses = caps["guesses"]
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    Ok(Card { winning, guesses })
}

fn winning_numbers(card: &Card) -> Vec<i32> {
    let mut winning_guesses = vec![];
    for guess in card.guesses.to_owned() {
        if card.winning.contains(&guess) {
            winning_guesses.push(guess);
        }
    }
    winning_guesses
}

// TODO: This feels messy
fn score(num_correct: usize) -> i32 {
    if num_correct == 0 {
        0
    } else {
        2_i32.pow((num_correct - 1).try_into().unwrap())
    }
}

fn score_all_cards(cards: impl Iterator<Item = Card>) -> i32 {
    let winning_guesses = cards.map(|c| winning_numbers(&c));
    let points = winning_guesses.map(|guesses| score(guesses.len()));
    points.sum()
}

#[test]
fn example() {
    let lines = [
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    ];
    let cards = lines
        .iter()
        .map(|line| parse_card(line).expect("Failed to parse card"));
    let total_score = score_all_cards(cards);
    assert_eq!(total_score, 13)
}

fn cards_from_file() -> impl Iterator<Item = Card> {
    include_str!("../../../resources/lottery_cards.txt")
        .lines()
        .map(|line| parse_card(line).expect("Failed to parse card"))
}

pub fn run() {
    let cards = cards_from_file();
    let score = score_all_cards(cards);
    println!("Final Score: {}", score)
}
