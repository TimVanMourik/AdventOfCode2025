mod part1;
mod part2;
pub use part1::part1;
pub use part2::part2;

#[derive(Clone)]
pub struct Shelf {
    has_paper: bool,
    x: usize,
    y: usize,
}

#[derive(Clone)]
pub struct Warehouse {
    shelves: Vec<Vec<Shelf>>,
}

const MAX_NEIGHBOURS: i64 = 3;

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

    fn is_roll_removable(&self, shelf: &Shelf) -> bool {
        if !shelf.has_paper {
            return false;
        }
        let neighbour_count = self
            .neighbours(shelf.y, shelf.x)
            .filter(|n| n.has_paper)
            .count() as i64;

        neighbour_count <= MAX_NEIGHBOURS
    }

    fn removable_rolls(&self) -> Vec<(usize, usize)> {
        self.shelves
            .iter()
            .flat_map(|row| row.iter())
            .filter(|shelf| self.is_roll_removable(shelf))
            .map(|shelf| (shelf.y, shelf.x))
            .collect()
    }

    fn remove_rolls(&mut self, shelves_to_remove: &[(usize, usize)]) {
        for &(y, x) in shelves_to_remove {
            self.shelves[y][x].has_paper = false;
        }
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
