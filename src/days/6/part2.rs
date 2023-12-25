mod common;

pub fn run() {
    let races = common::fixed_kerning_leaderboard();
    let num_ways = common::ways_to_beat_records(races);
    let product = num_ways.iter().fold(1, |acc, arg| acc * arg);
    println!("{:?} => {}", num_ways, product);
}
