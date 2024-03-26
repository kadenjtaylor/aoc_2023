#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Location {
    pub x: i64,
    pub y: i64,
}

pub type Edge = (Location, Location);

pub fn up(loc: &Location) -> Edge {
    (
        loc.clone(),
        Location {
            x: loc.x,
            y: loc.y + 1,
        },
    )
}

pub fn down(loc: &Location) -> Edge {
    (
        loc.clone(),
        Location {
            x: loc.x,
            y: loc.y - 1,
        },
    )
}

pub fn left(loc: &Location) -> Edge {
    (
        loc.clone(),
        Location {
            x: loc.x - 1,
            y: loc.y,
        },
    )
}

pub fn right(loc: &Location) -> Edge {
    (
        loc.clone(),
        Location {
            x: loc.x + 1,
            y: loc.y,
        },
    )
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

// Determines if a "hop" of one orthogonal space occurred and what it was
pub fn determine_hop(origin: &Location, destination: &Location) -> Option<Direction> {
    let y_diff = destination.y - origin.y;
    let x_diff = destination.x - origin.x;
    match (x_diff, y_diff) {
        (1, 0) => Some(Direction::RIGHT),
        (0, 1) => Some(Direction::UP),
        (-1, 0) => Some(Direction::LEFT),
        (0, -1) => Some(Direction::DOWN),
        _ => None,
    }
}

pub fn hop(loc: &Location, d: Direction) -> Location {
    match d {
        Direction::RIGHT => right(&loc).1,
        Direction::UP => up(&loc).1,
        Direction::DOWN => down(&loc).1,
        Direction::LEFT => left(&loc).1,
    }
}
