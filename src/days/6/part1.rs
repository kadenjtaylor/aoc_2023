mod common;

use common::{leaderboard_from_file, ways_to_beat_records};

#[test]
fn example() {
    assert_eq!(
        288,
        ways_to_beat_records(common::example_leaderboard())
            .iter()
            .fold(1, |acc, arg| acc * arg)
    )
}

pub fn run() {
    let races = leaderboard_from_file();
    let num_ways = ways_to_beat_records(races);
    let product = num_ways.iter().fold(1, |acc, arg| acc * arg);
    println!("{:?} => {}", num_ways, product);
}
