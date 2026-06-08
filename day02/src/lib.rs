use anyhow::{Context, Result};

mod part1;
mod part2;
pub use part1::part1;
pub use part2::part2;

pub struct Item {
    pub value: String,
}

fn parse_line(line: &str) -> Result<Item> {
    let trimmed = line.trim();
    Ok(Item {
        value: trimmed
            .parse::<String>()
            .with_context(|| format!("invalid line: {}", trimmed))?,
    })
}

pub fn parse(input: &str) -> impl Iterator<Item = Result<Item>> + '_ {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_line)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "abc\ndef\n";

    #[test]
    fn sample_part1() {
        let parsed = parse(SAMPLE)
            .collect::<Result<Vec<_>>>()
            .expect("sample should parse");
        assert_eq!(part1(&parsed), 0);
    }

    #[test]
    fn sample_part2() {
        let parsed = parse(SAMPLE)
            .collect::<Result<Vec<_>>>()
            .expect("sample should parse");
        assert_eq!(part2(&parsed), 0);
    }
}
