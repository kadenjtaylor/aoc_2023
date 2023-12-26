mod common;

use common::{betting_card_from_file, GameMode, compare_hand};

#[test]
fn example() {
    use common::{from_text_unsafe, BettingCard};

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
    sheet
        .hands
        .sort_by(|(h1, _), (h2, _)| compare_hand(h1, h2, &GameMode::JsAreJacks));
    // Calculate winnings:
    let mut winnings = 0;
    for (index, (_, bid)) in sheet.hands.iter().enumerate() {
        let rank = (index + 1) as u32;
        let hand_winnings = bid * rank;
        winnings += hand_winnings;
    }
    assert_eq!(6440, winnings);
}

pub fn run() {
    let mut sheet = betting_card_from_file();

    // Sort the lines of the card by hand-strength in ASCENDING order
    sheet
        .hands
        .sort_by(|(h1, _), (h2, _)| compare_hand(h1, h2, &GameMode::JsAreJacks));

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
