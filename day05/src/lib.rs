mod part1;
mod part2;
pub use part1::part1;
pub use part2::part2;

type Id = i64;
type FreshRange = (i64, i64);
pub struct Ingredients {
    fresh_ranges: Vec<FreshRange>,
    ids: Vec<Id>,
}

impl Ingredients {
    pub fn is_in_range(&self, value: &i64) -> bool {
        self.fresh_ranges
            .iter()
            .any(|&(start, end)| value >= &start && value <= &end)
    }
}

fn parse_range(line: &str) -> FreshRange {
    let trimmed = line.trim();
    let (start, end) = trimmed.split_once('-').unwrap();
    let start = start.parse::<i64>().unwrap();
    let end = end.parse::<i64>().unwrap();
    (start, end)
}

pub fn parse(input: &str) -> Ingredients {
    let (ranges_block, ids_block) = input
        .split_once("\n\n")
        .or_else(|| input.split_once("\r\n\r\n"))
        .expect("No empty line found in input");

    let fresh_ranges = ranges_block
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_range)
        .collect();

    let ids = ids_block
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().parse::<Id>().unwrap())
        .collect();

    Ingredients { fresh_ranges, ids }
}
