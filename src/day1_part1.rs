fn first_digit(s: &str) -> Option<char> {
    s.chars().find(|&c| c.is_digit(10))
}

// Note: I don't like how similar this is to the above fn
fn last_digit(s: &str) -> Option<char> {
    s.chars().rev().find(|&c| c.is_digit(10))
}

// Note: This is crying out for a for-comprehension
fn extract_number(s: &str) -> Option<i8> {
    first_digit(s)
        .and_then(|a| last_digit(s).and_then(|b| Some((a, b))))
        .and_then(|(a, b)| (a.to_string() + &b.to_string()).parse().ok())
}

pub fn run() {
    let lines = include_str!("../resources/problem_1.txt").lines();
    let sum: i32 = lines
        .map(|s| extract_number(s))
        .flatten() // this is only okay because we know all the lines have a first/last digit
        .map(|i| i as i32) // Here we see the cost of the i8 choice
        .sum();
    println!("Sum: {}", sum)
}
