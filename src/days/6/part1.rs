#[derive(Debug)]
struct Race {
    time: u32,
    record_distance: u32,
}

#[allow(dead_code)]
fn example_leaderboard() -> Vec<Race> {
    let races = vec![(7, 9), (15, 40), (30, 200)]
        .iter()
        .map(|(time, record_distance)| Race {
            time: *time,
            record_distance: *record_distance,
        })
        .collect::<Vec<Race>>();
    races
}

// Didn't actually read from the file -- too easy to specify in code
fn leaderboard_from_file() -> Vec<Race> {
    let races = vec![(42, 308), (89, 1170), (91, 1291), (89, 1467)]
        .iter()
        .map(|(time, record_distance)| Race {
            time: *time,
            record_distance: *record_distance,
        })
        .collect::<Vec<Race>>();
    races
}

fn boat_distance(released_at: u32, race_time: u32) -> u32 {
    // the boat travels @ v upon release
    let time_at_constant_v = race_time - released_at;
    // the boat gets a velocity = num millis held back
    let velocity = released_at;
    // after that, it's just d = vt
    velocity * time_at_constant_v
}

fn run_calc_for_race(millis: u32) -> impl Iterator<Item = (u32, u32)> {
    (0..millis)
        .map(move |delay| (delay, boat_distance(delay, millis)))
        .into_iter()
}

#[test]
fn example() {
    let races = example_leaderboard();
    let options = races
        .iter()
        .map(|r| run_calc_for_race(r.time).filter(|(_, distance)| distance > &r.record_distance));
    let num_ways: Vec<u32> = options.map(|runs| runs.count() as u32).collect();
    let product = num_ways.iter().fold(1, |acc, arg| acc * arg);
    assert_eq!(product, 288)
}

pub fn run() {
    let races = leaderboard_from_file();

    let options = races
        .iter()
        .map(|r| run_calc_for_race(r.time).filter(|(_, distance)| distance > &r.record_distance));

    let num_ways: Vec<u32> = options.map(|runs| runs.count() as u32).collect();

    let product = num_ways.iter().fold(1, |acc, arg| acc * arg);

    println!("{:?} => {}", num_ways, product);
}
