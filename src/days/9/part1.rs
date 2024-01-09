#[derive(Debug)]
struct Readings {
    sequences: Vec<Vec<i32>>,
}

#[derive(Debug)]
struct ShownWork {
    seq: Vec<i32>,
    diffs: Vec<Vec<i32>>,
    extrapolated: Vec<i32>,
    prediction: i32,
}

fn parse(file_data: &str) -> Readings {
    let sequences = file_data
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
        })
        .map(|s| s.collect())
        .collect();
    Readings { sequences }
}

fn is_all_zeroes(v: &Vec<i32>) -> bool {
    v.iter().all(|&n| n == 0)
}

fn diff(v: &Vec<i32>) -> Vec<i32> {
    let mut nums = v.iter();
    let mut below = vec![];
    let mut ptr = nums.next().unwrap();
    while let Some(x) = nums.next() {
        let diff = x - ptr;
        below.push(diff);
        ptr = x;
    }
    below
}

fn diff_vec(v: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut all_vecs: Vec<Vec<i32>> = vec![v.clone()];
    let mut current_vec = v;
    while !is_all_zeroes(current_vec) {
        let d = diff(current_vec);
        all_vecs.push(d.clone());
        current_vec = all_vecs.last().unwrap();
    }
    all_vecs
}

fn extrapolate(v: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut guesses = vec![];
    let mut last_items = v.iter().map(|sequence| sequence.last().unwrap()).rev();
    let mut current = last_items.next().unwrap();
    while let Some(item) = last_items.next() {
        guesses.push(current + item);
        current = guesses.last().unwrap();
    }
    guesses
}

fn example() -> i32 {
    let readings = parse(test_data());
    let mut work = vec![];
    for seq in readings.sequences {
        let diffs = diff_vec(&seq);
        let extrapolated = extrapolate(&diffs);
        let prediction = *extrapolated.last().unwrap();
        work.push(ShownWork {
            seq,
            diffs,
            extrapolated,
            prediction,
        });
    }
    work.iter().map(|work| work.prediction).sum()
}

fn part1() -> i32 {
    let readings = parse(real_data());
    let mut work = vec![];
    for seq in readings.sequences {
        let diffs = diff_vec(&seq);
        let extrapolated = extrapolate(&diffs);
        let prediction = *extrapolated.last().unwrap();
        work.push(ShownWork {
            seq,
            diffs,
            extrapolated,
            prediction,
        });
    }
    work.iter().map(|work| work.prediction).sum()
}

// fn part2() {
//     todo!("Implement!")
// }

fn test_data() -> &'static str {
    "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45"
}

fn real_data() -> &'static str {
    include_str!("../../../resources/oasis_readings.txt")
}

pub fn run() {
    let current = part1();
    println!("Sum of predicted values: {}", current);
}
