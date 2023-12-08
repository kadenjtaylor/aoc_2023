fn parse_digit(d: &str) -> Option<i32> {
    match d {
        "1" | "one" => Some(1),
        "2" | "two" => Some(2),
        "3" | "three" => Some(3),
        "4" | "four" => Some(4),
        "5" | "five" => Some(5),
        "6" | "six" => Some(6),
        "7" | "seven" => Some(7),
        "8" | "eight" => Some(8),
        "9" | "nine" => Some(9),
        "0" | "zero" => Some(0),
        _ => None,
    }
}

const DIGITS: &'static [&'static str] = &[
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine", "zero",
];

fn extract(s: &str) -> i32 {
    let first_digit = first(s).expect("First missing");
    let last_digit = last(s).expect("Last missing");
    first_digit * 10 + last_digit
}

fn first(s: &str) -> Option<i32> {
    let results = DIGITS
        .iter()
        .flat_map(|&n| s.find(n).and_then(|i| Some((i, n))));
    results.min().and_then(|f| parse_digit(f.1))
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn last(s: &str) -> Option<i32> {
    let digits = DIGITS.iter().map(|&d| reverse(d));
    let results = digits.flat_map(|n| reverse(s).find(&n).and_then(|i| Some((i, n))));
    let min = results
        .min()
        .map(|(i, d)| (s.len() - (i + d.len()), reverse(&d)));
    min.and_then(|f| parse_digit(&f.1))
}

#[test]
fn test() {
    fn run_test_case(s: &str, num: i32) {
        let result = extract(s);
        println!("{s} -> {result}");
        assert_eq!(result, num);
    }

    use std::collections::HashMap;
    let cases = HashMap::from([
        ("v4", 44),
        ("two1nine", 29),
        ("eightwothree", 83),
        ("abcone2threexyz", 13),
        ("xtwone3four", 24),
        ("4nineeightseven2", 42),
        ("zoneight234", 14),
        ("7pqrstsixteen", 76),
        ("eightwo7threeight", 88),
    ]);
    cases
        .iter()
        .for_each(|(line, num)| run_test_case(line, *num))
}

pub fn run() {
    let lines = include_str!("../resources/problem_1.txt").lines();

    let updated: i32 = lines.map(|line| extract(line)).sum();

    println!("Updated sum: {}", updated);
}
