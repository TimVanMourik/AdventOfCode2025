mod part1;
mod part2;
pub use part1::part1;
pub use part2::part2;

#[derive(Debug)]
pub struct IdRange {
    pub first: i64,
    pub last: i64,
}

fn parse_range(line: &str) -> IdRange {
    let trimmed = line.trim();
    let parts: Vec<&str> = trimmed.split('-').collect();
    if parts.len() != 2 {
        panic!("invalid line: {}", trimmed);
    }
    let first = parts[0]
        .parse::<i64>()
        .unwrap_or_else(|_| panic!("invalid line: {}", trimmed));
    let last = parts[1]
        .parse::<i64>()
        .unwrap_or_else(|_| panic!("invalid line: {}", trimmed));
    IdRange { first, last }
}

pub fn parse(input: &str) -> Vec<IdRange> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .flat_map(|line| line.split(','))
        .map(parse_range)
        .collect()
}
