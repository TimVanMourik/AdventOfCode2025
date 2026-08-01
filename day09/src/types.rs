#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
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
    pub red_tiles: Vec<Tile>,
}

impl Theater {
    pub fn new(red_tiles: Vec<Tile>) -> Self {
        Self { red_tiles }
    }

    pub fn red_tiles(&self) -> &Vec<Tile> {
        &self.red_tiles
    }
}

impl std::fmt::Debug for Theater {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self.red_tiles.iter().map(|tile| tile.x).max().unwrap_or(0);
        let max_y = self.red_tiles.iter().map(|tile| tile.y).max().unwrap_or(0);
        for y in 0..=max_y + 1 {
            for x in 0..=max_x + 1 {
                let tile = Tile::new(x, y);
                if self.red_tiles.contains(&tile) {
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

#[cfg(test)]
mod tests {
    use super::{Theater, Tile};

    #[test]
    fn stores_red_tiles() {
        let theater = Theater::new(vec![Tile::new(1, 2)]);

        assert!(theater.red_tiles().contains(&Tile::new(1, 2)));
    }
}
