#[derive(Debug)]
pub struct RGBCounts {
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

#[derive(Debug)]
pub struct Game {
    pub id: i32,
    pub rounds: Vec<RGBCounts>,
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

fn parse_round(round_str: &str) -> Option<RGBCounts> {
    let (mut r_count, mut g_count, mut b_count) = (0, 0, 0);
    let mut chunks = round_str.split(",");
    while let Some(c) = chunks.next().and_then(|c| parse_chunk(c)) {
        match c {
            Chunk::Red(i) => r_count = i,
            Chunk::Green(i) => g_count = i,
            Chunk::Blue(i) => b_count = i,
        }
    }
    Some(RGBCounts {
        r: r_count,
        g: g_count,
        b: b_count,
    })
}

pub fn parse_game(line: &str) -> Option<Game> {
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

pub fn min_required_counts(game: &Game) -> RGBCounts {
    let (mut r_max, mut g_max, mut b_max) = (0, 0, 0);
    let mut rounds = game.rounds.iter();
    while let Some(round) = rounds.next() {
        r_max = r_max.max(round.r);
        g_max = g_max.max(round.g);
        b_max = b_max.max(round.b);
    }
    RGBCounts {
        r: r_max,
        g: g_max,
        b: b_max,
    }
}
