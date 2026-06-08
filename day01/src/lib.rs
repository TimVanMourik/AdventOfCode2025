use anyhow::{Context, Result};

mod part1;
mod part2;
pub use part1::part1;
pub use part2::part2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    L,
    R,
}

pub struct Move {
    direction: Direction,
    distance: i64,
}

fn parse_line(line: &str) -> Result<Move> {
    let trimmed = line.trim();
    if trimmed.len() < 2 {
        anyhow::bail!("line too short: {}", trimmed);
    }
    let direction = match trimmed.chars().next().unwrap() {
        'L' => Direction::L,
        'R' => Direction::R,
        c => anyhow::bail!("invalid direction: {}", c),
    };
    let distance_str = &trimmed[1..];
    let distance = distance_str
        .parse::<i64>()
        .with_context(|| format!("invalid integer: {}", distance_str))?;
    Ok(Move {
        direction,
        distance,
    })
}

pub fn parse(input: &str) -> impl Iterator<Item = Result<Move>> + '_ {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_line)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "R1\nL2\nR3\nL4\n";

    #[test]
    fn sample_part1() {
        let parsed = parse(SAMPLE)
            .collect::<Result<Vec<_>>>()
            .expect("sample should parse");
        assert_eq!(part1(&parsed), 10);
    }

    #[test]
    fn sample_part2() {
        let parsed = parse(SAMPLE)
            .collect::<Result<Vec<_>>>()
            .expect("sample should parse");
        assert_eq!(part2(&parsed), 24);
    }
}
