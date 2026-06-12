mod part1;
mod part2;
pub use part1::part1;
pub use part2::part2;

#[derive(Debug)]
enum Direction {
    L,
    R,
}

pub struct Move {
    direction: Direction,
    distance: i64,
}

fn parse_line(line: &str) -> Move {
    let trimmed = line.trim();
    if trimmed.len() < 2 {
        panic!("line too short: {}", trimmed);
    }
    let direction = match trimmed.chars().next().unwrap() {
        'L' => Direction::L,
        'R' => Direction::R,
        c => panic!("invalid direction: {}", c),
    };
    let distance_str = &trimmed[1..];
    let distance = distance_str
        .parse::<i64>()
        .unwrap_or_else(|_| panic!("invalid integer: {}", distance_str));
    Move {
        direction,
        distance,
    }
}

pub fn parse(input: &str) -> Vec<Move> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_line)
        .collect()
}
