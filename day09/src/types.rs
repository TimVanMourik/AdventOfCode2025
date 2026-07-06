use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
}

impl Tile {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub struct Theater {
    red_tiles: HashSet<Tile>,
}

impl Theater {
    pub fn new(red_tiles: HashSet<Tile>) -> Self {
        Self { red_tiles }
    }

    pub fn red_tiles(&self) -> &HashSet<Tile> {
        &self.red_tiles
    }
}

impl std::fmt::Debug for Theater {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid: HashSet<(i32, i32)> = HashSet::new();

        for tile in &self.red_tiles {
            grid.insert((tile.x as i32, tile.y as i32));
        }

        // Get max x and y to determine the size of the grid
        let max_x = grid.iter().map(|(x, _)| *x).max().unwrap_or(0);
        let max_y = grid.iter().map(|(_, y)| *y).max().unwrap_or(0);
        for y in 0..=max_y + 1 {
            for x in 0..=max_x + 1 {
                if grid.contains(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
