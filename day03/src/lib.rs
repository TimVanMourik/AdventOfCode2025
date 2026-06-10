mod part1;
mod part2;
pub use part1::part1;
pub use part2::part2;

pub type Battery = u8;

#[derive(Debug)]
pub struct Bank {
    batteries: Vec<Battery>,
}

fn parse_line(line: &str) -> Bank {
    Bank {
        batteries: line
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as Battery)
            .collect(),
    }
}

pub fn parse(input: &str) -> Vec<Bank> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_line)
        .collect::<Vec<_>>()
}
