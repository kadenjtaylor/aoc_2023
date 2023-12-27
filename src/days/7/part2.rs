mod common;

use common::{betting_card_from_file, compare_hand, GameMode};

#[test]
fn no_jokers_no_additional_possibilities() {
    use common::{iterate_possible_hands, Card, Hand};
    let no_jokers = iterate_possible_hands(&Hand {
        cards: [Card::A, Card::A, Card::A, Card::A, Card::A],
    });
    assert_eq!(no_jokers.len(), 1)
}

#[test]

fn one_joker_12_additional_possibilities() {
    use common::{iterate_possible_hands, Card, Hand};
    let one_joker = iterate_possible_hands(&Hand {
        cards: [Card::A, Card::J, Card::A, Card::A, Card::A],
    });
    /*There are 13 possible cards A-K. It's not useful to swap for jokers, so
    when we swap, we have 12 possible things to swap to. (1 thru 10 + Q + K)
    That means that we should have 12 possible hands once the swaps have been made.*/
    assert_eq!(one_joker.len(), 12)
}

#[test]
fn example_with_new_rules() {
    use common::{from_text_unsafe, BettingCard};
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
    sheet
        .hands
        .sort_by(|(h1, _), (h2, _)| compare_hand(h1, h2, &GameMode::JsAreJokers));

    // Calculate winnings:
    let mut winnings = 0;
    for (index, (hand, bid)) in sheet.hands.iter().enumerate().rev() {
        let rank = (index + 1) as u32;
        let hand_winnings = bid * rank;
        winnings += hand_winnings;
        println!("{}) [{}], ${} => ${}", rank, hand, bid, hand_winnings);
    }
    assert_eq!(5905, winnings);
}

pub fn run() {
    let mut sheet = betting_card_from_file();

    // Sort the lines of the card by hand-strength in ASCENDING order
    sheet
        .hands
        .sort_by(|(h1, _), (h2, _)| compare_hand(h1, h2, &GameMode::JsAreJokers));

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
