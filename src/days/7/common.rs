use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{Display, Write},
};

#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J,
    Q,
    K,
    A,
}
impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out_c = match self {
            Card::A => 'A',
            Card::K => 'K',
            Card::Q => 'Q',
            Card::J => 'J',
            Card::Ten => 'T',
            Card::Nine => '9',
            Card::Eight => '8',
            Card::Seven => '7',
            Card::Six => '6',
            Card::Five => '5',
            Card::Four => '4',
            Card::Three => '3',
            Card::Two => '2',
        };
        f.write_char(out_c)
    }
}

fn from_char(c: &char) -> Option<Card> {
    match c {
        'A' => Some(Card::A),
        'K' => Some(Card::K),
        'Q' => Some(Card::Q),
        'J' => Some(Card::J),
        'T' => Some(Card::Ten),
        '9' => Some(Card::Nine),
        '8' => Some(Card::Eight),
        '7' => Some(Card::Seven),
        '6' => Some(Card::Six),
        '5' => Some(Card::Five),
        '4' => Some(Card::Four),
        '3' => Some(Card::Three),
        '2' => Some(Card::Two),
        _ => None,
    }
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn hand_shape(hand: &Hand) -> Vec<u32> {
    let mut m: HashMap<String, u32> = HashMap::new();
    for card in hand.cards.iter() {
        let card_name = card.to_string();
        let score = match m.get(&card_name) {
            None => 1,
            Some(&score) => score + 1,
        };
        // println!("{} - score: {}", card, score);
        m.insert(card_name, score);
    }
    let mut counts = m.values().map(|&n| n).collect::<Vec<u32>>();
    counts.sort();
    counts.reverse();
    counts
}

fn compute_type(h: &Hand) -> HandType {
    // n-of-a-kind check
    let shape: Vec<u32> = hand_shape(h);
    // println!("shape: {:?}", shape);
    match shape[..] {
        [5] => HandType::FiveOfAKind,
        [4, 1] => HandType::FourOfAKind,
        [3, 2] => HandType::FullHouse,
        [3, 1, 1] => HandType::ThreeOfAKind,
        [2, 2, 1] => HandType::TwoPair,
        [2, 1, 1, 1] => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

pub fn from_text_unsafe(txt: &str) -> Hand {
    if txt.len() != 5 {
        None
    } else {
        let a = txt.chars().nth(0).expect("Missing a card");
        let b = txt.chars().nth(1).expect("Missing a card");
        let c = txt.chars().nth(2).expect("Missing a card");
        let d = txt.chars().nth(3).expect("Missing a card");
        let e = txt.chars().nth(4).expect("Missing a card");
        Some(Hand {
            cards: [a, b, c, d, e].map(|ch| from_char(&ch).expect("Failed to parse")),
        })
    }
    .expect("Unsafely grabbing a hand")
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Copy, Clone)]
pub struct Hand {
    pub cards: [Card; 5],
}

#[allow(dead_code)]
pub enum GameMode {
    JsAreJacks,
    JsAreJokers,
}

pub fn iterate_possible_hands(h: &Hand) -> Vec<Hand> {
    let mut possibilities: Vec<String> = vec!["".to_string()];
    for c in h.cards.as_slice() {
        if *c == Card::J {
            possibilities = possibilities
                .iter()
                .flat_map(|s| {
                    JOKERS_RANKINGS
                        .iter()
                        .filter(|&c| *c != Card::J)
                        .map(|c| s.to_owned() + &c.to_string())
                })
                .collect()
        } else {
            possibilities = possibilities
                .iter()
                .map(|s| s.to_owned() + &c.to_string())
                .collect()
        }
    }
    possibilities.iter().map(|s| from_text_unsafe(s)).collect()
}

const JACKS_RANKINGS: [Card; 13] = [
    Card::Two,
    Card::Three,
    Card::Four,
    Card::Five,
    Card::Six,
    Card::Seven,
    Card::Eight,
    Card::Nine,
    Card::Ten,
    Card::J,
    Card::Q,
    Card::K,
    Card::A,
];

const JOKERS_RANKINGS: [Card; 13] = [
    Card::J,
    Card::Two,
    Card::Three,
    Card::Four,
    Card::Five,
    Card::Six,
    Card::Seven,
    Card::Eight,
    Card::Nine,
    Card::Ten,
    Card::Q,
    Card::K,
    Card::A,
];

fn rank(c: &Card, mode: &GameMode) -> usize {
    match mode {
        GameMode::JsAreJokers => JOKERS_RANKINGS,
        GameMode::JsAreJacks => JACKS_RANKINGS,
    }
    .iter()
    .position(|card| card == c)
    .unwrap_or(14)
}

fn compare_card(a: &Card, b: &Card, mode: &GameMode) -> Ordering {
    rank(a, &mode).cmp(&rank(b, &mode))
}

fn simple_compare_type(a: &Hand, b: &Hand) -> Ordering {
    compute_type(a).cmp(&compute_type(b))
}

fn simple_compare_lex(a: &Hand, b: &Hand, mode: &GameMode) -> Ordering {
    a.cards
        .iter()
        .zip(b.cards.iter())
        .map(|(a, b)| compare_card(a, b, mode))
        .find(|&o| o != Ordering::Equal)
        .unwrap_or(Ordering::Equal)
}

pub fn compare_hand(a: &Hand, b: &Hand, mode: &GameMode) -> Ordering {
    match mode {
        GameMode::JsAreJacks => simple_compare_type(a, b).then(simple_compare_lex(a, b, mode)),
        GameMode::JsAreJokers => {
            let a_result = best_possible_hand(a, false);
            let b_result = best_possible_hand(b, false);
            let result =
                simple_compare_type(&a_result, &b_result).then(simple_compare_lex(a, b, mode));
            // println!(
            //     "{} (using {}) {} {} (using {})",
            //     a,
            //     a_result,
            //     match result {
            //         Ordering::Equal => "=".to_string(),
            //         Ordering::Less => "<".to_string(),
            //         Ordering::Greater => ">".to_string(),
            //     },
            //     b,
            //     b_result
            // );
            result
        }
    }
}

fn best_possible_hand(hand: &Hand, iterate_all: bool) -> Hand {
    let mut hands: Vec<Hand> = if iterate_all {
        iterate_possible_hands(hand)
    } else {
        JOKERS_RANKINGS
            .map(|card| hand.to_string().replace("J", &card.to_string()))
            .map(|s| from_text_unsafe(&s))
            .iter()
            .map(|h| *h)
            .collect()
    };
    hands.sort_by(|a, b| {
        simple_compare_type(a, b).then(simple_compare_lex(a, b, &GameMode::JsAreJokers))
    });
    let choice = hands.last().map(|&h| h);
    choice.expect("Look ma, no hands!")
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stuff = self
            .cards
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("");
        f.write_fmt(format_args!("{}", stuff))
    }
}

pub type BidAmount = u32;
pub struct BettingCard {
    pub hands: Vec<(Hand, BidAmount)>,
}

pub fn betting_card_from_file() -> BettingCard {
    let lines = include_str!("../../../resources/camel_cards.txt").lines();
    let bits = lines
        .map(|line| {
            let chunks = line.split_ascii_whitespace().collect::<Vec<&str>>();
            (
                from_text_unsafe(chunks[0]),
                chunks[1].parse::<u32>().expect("Failed to parse bid"),
            )
        })
        .collect::<Vec<(Hand, BidAmount)>>();
    BettingCard { hands: bits }
}
