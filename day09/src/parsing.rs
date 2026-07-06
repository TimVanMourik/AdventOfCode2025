use crate::types::{Theater, Tile};

fn parse_line(line: &str) -> Tile {
    let mut trimmed = line.trim().split(",");
    let x = trimmed.next().unwrap().parse::<usize>().unwrap();
    let y = trimmed.next().unwrap().parse::<usize>().unwrap();
    Tile::new(x, y)
}

pub fn parse(input: &str) -> Theater {
    Theater::new(
        input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(parse_line)
            .collect(),
    )
}
