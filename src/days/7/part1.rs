use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{Display, Write},
};

#[derive(PartialOrd, PartialEq, Eq, Ord, Hash)]
enum Card {
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
    let mut stuff = m.values().map(|&n| n).collect::<Vec<u32>>();
    stuff.sort();
    stuff.reverse();
    stuff
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

fn from_text_unsafe(txt: &str) -> Hand {
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

#[derive(PartialOrd, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn cmp_card_vec(a: Vec<&Card>, b: Vec<&Card>) -> Ordering {
            let mut result = std::cmp::Ordering::Equal;
            for (c1, c2) in a.iter().zip(b) {
                match c1.cmp(&c2) {
                    std::cmp::Ordering::Greater => {
                        result = std::cmp::Ordering::Greater;
                        break;
                    }
                    std::cmp::Ordering::Less => {
                        result = std::cmp::Ordering::Less;
                        break;
                    }
                    _ => (),
                }
            }
            result
        }

        // println!("Comparing {} to {}", self, other);
        let self_type = compute_type(self);
        let other_type = compute_type(other);
        // println!("{:?} vs {:?}", self_type, other_type);
        match self_type.cmp(&other_type) {
            std::cmp::Ordering::Equal => {
                let self_vec = self.cards.iter().collect::<Vec<&Card>>();
                let other_vec = other.cards.iter().collect::<Vec<&Card>>();
                cmp_card_vec(self_vec, other_vec)
            }
            result => result,
        }
    }
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

type BidAmount = u32;
struct BettingCard {
    hands: Vec<(Hand, BidAmount)>,
}

#[test]
fn example() {
    // Create the betting card
    let mut sheet = BettingCard {
        hands: vec![
            (from_text_unsafe("32T3K"), 765),
            (from_text_unsafe("T55J5"), 684),
            (from_text_unsafe("KK677"), 28),
            (from_text_unsafe("KTJJT"), 220),
            (from_text_unsafe("QQQJA"), 483),
        ],
    };
    // Sort the lines of the card by hand-strength in ASCENDING order
    sheet.hands.sort_by(|(h1, _), (h2, _)| h1.cmp(h2));
    // Calculate winnings:
    let mut winnings = 0;
    for (index, (_, bid)) in sheet.hands.iter().enumerate() {
        let rank = (index + 1) as u32;
        let hand_winnings = bid * rank;
        winnings += hand_winnings;
    }
    assert_eq!(6440, winnings);
}

fn betting_card_from_file() -> BettingCard {
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

pub fn run() {
    let mut sheet = betting_card_from_file();

    // Sort the lines of the card by hand-strength in ASCENDING order
    sheet.hands.sort_by(|(h1, _), (h2, _)| h1.cmp(h2));

    // Calculate winnings:
    let mut winnings = 0;
    for (index, (hand, bid)) in sheet.hands.iter().enumerate().rev() {
        let rank = (index + 1) as u32;
        let hand_winnings = bid * rank;
        winnings += hand_winnings;
        println!("{}) [{}], ${} => ${}", rank, hand, bid, hand_winnings);
    }
    println!("Total Winnings: ${}", winnings);
}
