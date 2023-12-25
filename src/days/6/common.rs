#[derive(Debug)]
pub struct Race {
    pub time: u64,
    pub record_distance: u64,
}

fn boat_distance(released_at: u64, race_time: u64) -> u64 {
    // the boat travels @ v upon release
    let time_at_constant_v = race_time - released_at;
    // the boat gets a velocity = num millis held back
    let velocity = released_at;
    // after that, it's just d = vt
    velocity * time_at_constant_v
}

fn run_calc_for_race(millis: u64) -> impl Iterator<Item = (u64, u64)> {
    (0..millis)
        .map(move |delay| (delay, boat_distance(delay, millis)))
        .into_iter()
}

pub fn ways_to_beat_records(leader_board: Vec<Race>) -> Vec<u64> {
    leader_board
        .iter()
        .map(|r| run_calc_for_race(r.time).filter(|(_, distance)| distance > &r.record_distance))
        .map(|runs| runs.count() as u64)
        .collect()
}

// -------------------- Data -------------------- //

#[allow(dead_code)]
pub fn example_leaderboard() -> Vec<Race> {
    let races = vec![(7, 9), (15, 40), (30, 200)]
        .iter()
        .map(|(time, record_distance)| Race {
            time: *time,
            record_distance: *record_distance,
        })
        .collect::<Vec<Race>>();
    races
}

#[allow(dead_code)]
// Didn't actually read from the file -- too easy to specify in code
pub fn leaderboard_from_file() -> Vec<Race> {
    let races = vec![(42, 308), (89, 1170), (91, 1291), (89, 1467)]
        .iter()
        .map(|(time, record_distance)| Race {
            time: *time,
            record_distance: *record_distance,
        })
        .collect::<Vec<Race>>();
    races
}

pub fn fixed_kerning_leaderboard() -> Vec<Race> {
    let races = vec![(42899189, 308117012911467)]
        .iter()
        .map(|(time, record_distance)| Race {
            time: *time,
            record_distance: *record_distance,
        })
        .collect::<Vec<Race>>();
    races
}
