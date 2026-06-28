mod part1;
mod part2;
pub use part1::part1;
pub use part2::part2;

type Coordinate = (usize, usize, usize);

#[derive(Debug)]
pub struct JunctionBox {
    location: Coordinate,
}

fn parse_line(line: &str) -> JunctionBox {
    let trimmed = line.trim();
    let parts: Vec<usize> = trimmed
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    JunctionBox {
        location: (parts[0], parts[1], parts[2]),
    }
}

pub fn parse(input: &str) -> Vec<JunctionBox> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_line)
        .collect()
}
