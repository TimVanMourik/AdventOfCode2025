mod part1;
mod part2;
pub use part1::part1;
pub use part2::part2;

pub struct Shelf {
    has_paper: bool,
    x: usize,
    y: usize,
}

pub struct Warehouse {
    shelves: Vec<Vec<Shelf>>,
}

impl Warehouse {
    fn neighbours(&self, row: usize, col: usize) -> impl Iterator<Item = &Shelf> {
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        directions.into_iter().filter_map(move |(dr, dc)| {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;

            if new_row >= 0
                && new_row < self.shelves.len() as isize
                && new_col >= 0
                && new_col < self.shelves[new_row as usize].len() as isize
            {
                Some(&self.shelves[new_row as usize][new_col as usize])
            } else {
                None
            }
        })
    }
}

fn parse_line(line: &str, row: usize) -> Vec<Shelf> {
    let trimmed = line.trim();
    trimmed
        .chars()
        .enumerate()
        .map(|(i, c)| Shelf {
            has_paper: c == '@',
            x: i,
            y: row,
        })
        .collect()
}

pub fn parse(input: &str) -> Warehouse {
    let shelves = input
        .lines()
        .enumerate()
        .map(|(row, line)| parse_line(line, row))
        .collect();

    Warehouse { shelves }
}
