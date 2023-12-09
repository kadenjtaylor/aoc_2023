#[derive(Debug)]
struct Round {
    r: i32,
    g: i32,
    b: i32,
}

#[derive(Debug)]
struct Game {
    id: i32,
    rounds: Vec<Round>,
}

enum Chunk {
    Red(i32),
    Green(i32),
    Blue(i32),
}

fn parse_chunk(chunk_str: &str) -> Option<Chunk> {
    let mut info = chunk_str.split_whitespace();
    let count = info.next().and_then(|c| c.parse::<i32>().ok());
    count.and_then(|count| {
        info.next().and_then(|color| match color {
            "red" => Some(Chunk::Red(count)),
            "green" => Some(Chunk::Green(count)),
            "blue" => Some(Chunk::Blue(count)),
            _ => None,
        })
    })
}

fn parse_round(round_str: &str) -> Option<Round> {
    let (mut r_count, mut g_count, mut b_count) = (0, 0, 0);
    let mut chunks = round_str.split(",");
    while let Some(c) = chunks.next().and_then(|c| parse_chunk(c)) {
        match c {
            Chunk::Red(i) => r_count = i,
            Chunk::Green(i) => g_count = i,
            Chunk::Blue(i) => b_count = i,
        }
    }
    Some(Round {
        r: r_count,
        g: g_count,
        b: b_count,
    })
}

fn parse_game(line: &str) -> Option<Game> {
    let mut stuff = line.split(":");
    let game_id = stuff.next().and_then(|g_str| {
        g_str
            .split_whitespace()
            .last()
            .and_then(|id_num| id_num.parse::<i32>().ok())
    });
    let mut rounds_acc = vec![];
    if let Some(r) = stuff.next() {
        let mut round_str = r.split(";");
        while let Some(round) = round_str.next().and_then(|r_str| parse_round(r_str)) {
            rounds_acc.push(round);
        }
    }
    game_id.map(|g_id| Game {
        id: g_id,
        rounds: rounds_acc,
    })
}

fn is_valid_game(game: &Game, r: i32, g: i32, b: i32) -> bool {
    let (mut r_max, mut g_max, mut b_max) = (0, 0, 0);
    let mut rounds = game.rounds.iter();
    while let Some(round) = rounds.next() {
        r_max = r_max.max(round.r);
        g_max = g_max.max(round.g);
        b_max = b_max.max(round.b);
    }

    let is_valid = r_max <= r && g_max <= g && b_max <= b;
    let check = if is_valid { "X" } else { "_" };
    println!("[{}] #{} - {:?}", check, game.id, (r_max, g_max, b_max));
    is_valid
}

#[test]
fn run_test_case() {
    let lines = [
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    ];
    let games = lines
        .iter()
        .map(|line| {
            let error_msg = format!("Failed to parse into game: \"{}\"", line);
            parse_game(line).expect(&error_msg)
        })
        .filter(|game| is_valid_game(game, 12, 13, 14));
    let id_total: i32 = games.map(|g| g.id).sum();
    assert_eq!(id_total, 8)
}

pub fn run() {
    let lines = include_str!("../resources/problem_2.txt").lines();

    let valid_games = lines
        .map(|line| {
            let error_msg = format!("Failed to parse into game: \"{}\"", line);
            parse_game(line).expect(&error_msg)
        })
        .filter(|game| is_valid_game(game, 12, 13, 14));

    let id_total: i32 = valid_games.clone().map(|g| g.id).sum();

    println!("Sum of valid game ids: {id_total}");
}
