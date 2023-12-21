mod common;
mod simple_queue;

use std::{collections::HashMap, time::Instant};

use common::Card;
use simple_queue::SimpleQueue;

// Works, very fun - takes FOREVER, so I added an optional cache to help
fn queue_method(cards: impl Iterator<Item = Card>, with_cache: bool) -> i32 {
    println!("Solving using queue solution...");
    println!("Using cache: {}", with_cache);
    // We begin with all of the cards we need, indexed by their ids
    let cards_by_id: HashMap<i32, Card> = cards.map(|c| (c.id, c)).collect();

    // Create a queue and fill it with the initial cards
    let mut queue = simple_queue::simple_queue::<&Card>();
    cards_by_id.values().for_each(|c| queue.add(c));

    // Set up a cache so we don't have to recompute stuff
    let mut winning_number_cache: HashMap<i32, i32> = HashMap::new();
    fn cached_call(
        cards: &HashMap<i32, Card>,
        cache: &mut HashMap<i32, i32>,
        question: i32,
    ) -> i32 {
        match cache.get(&question) {
            Some(&answer) => answer,
            None => {
                let a = common::winning_numbers(
                    cards.get(&question).expect("Failed to retrieve card by id"),
                )
                .len() as i32;
                cache.insert(question, a);
                a
            }
        }
    }

    // Need a counter for the cards
    let mut count: i32 = 0;

    // Now we just rip through the queue, adding new cards when we need to
    while let Some(card) = queue.next() {
        count = count + 1;
        let card_num = card.id;
        let n = if with_cache {
            cached_call(&cards_by_id, &mut winning_number_cache, card.id)
        } else {
            common::winning_numbers(card).len() as i32
        };
        let range = (card_num + 1)..(card_num + n + 1);
        // println!("Card: {:?} had {:?} winning numbers.", card_num, n);
        for index in range {
            // println!("Adding card #{}", index);
            queue.add(cards_by_id.get(&index).expect("Whoops"));
        }
    }
    // finally, just return the count
    count
}

// Idea that involves cascading a computation through an array -also wicked fun
fn dictionary_method(cards: impl Iterator<Item = Card>) -> i32 {
    println!("Solving using dictionary solution...");
    let cards_by_id: HashMap<i32, Card> = cards.map(|c| (c.id, c)).collect();

    let mut counts_by_id: HashMap<i32, i32> = cards_by_id.keys().map(|&i| (i, 1)).collect();

    let mut pointer = 1;

    while let Some(card) = cards_by_id.get(&pointer) {
        // Figure out what the number of this card is
        let n = common::winning_numbers(card).len() as i32;

        // Get the current count
        let current = counts_by_id
            .get(&card.id)
            .expect("Failed to get current count value")
            .to_owned();

        // Use the answer to modify the counts
        let range = (card.id + 1)..(card.id + n + 1);
        // println!(
        //     "Card {} (count = {}) got cards: {:?}",
        //     card.id,
        //     current,
        //     range.clone().collect::<Vec<i32>>()
        // );

        for index in range {
            // println!("Incrementing #{} by {}", index, current);
            let target = counts_by_id.get(&index).expect("Failed to get value");
            counts_by_id.insert(index, target + current);
        }

        // Move to the next slot
        pointer += 1;
    }

    counts_by_id.values().sum()
}

#[allow(dead_code)]
#[derive(Debug)]
enum RunMode {
    QueueMethod(bool),
    DictionaryMethod,
}

pub fn run() {
    let cards = common::cards_from_file();
    let mode = RunMode::DictionaryMethod;

    let start = Instant::now();
    println!("Started at: {:?}", start);
    let result = match mode {
        RunMode::QueueMethod(with_cache) => queue_method(cards, with_cache),
        RunMode::DictionaryMethod => dictionary_method(cards),
    };
    let end = Instant::now();

    println!("Ended at: {:?}", end);
    println!(" took: {:?}", end - start);
    println!("Got result: {}", result);
}
