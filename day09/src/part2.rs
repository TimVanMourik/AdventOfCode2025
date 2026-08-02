use std::collections::{HashMap, HashSet};

use crate::{Theater, Tile};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CoordinateAxis {
    values: Vec<usize>,
}

impl CoordinateAxis {
    pub fn new(values: impl IntoIterator<Item = usize>) -> Self {
        let mut values: Vec<_> = values.into_iter().collect();
        values.sort_unstable();
        values.dedup();
        Self { values }
    }

    pub fn compress(&self, value: usize) -> usize {
        self.values
            .binary_search(&value)
            .unwrap_or_else(|_| panic!("value {value} not present in compressed axis"))
    }

    pub fn decompress(&self, index: usize) -> usize {
        self.values
            .get(index)
            .copied()
            .unwrap_or_else(|| panic!("index {index} out of range for compressed axis"))
    }

    pub fn prev_index(&self, index: usize) -> Option<usize> {
        index.checked_sub(1)
    }

    pub fn next_index(&self, index: usize) -> Option<usize> {
        index
            .checked_add(1)
            .filter(|&next| next < self.values.len())
    }
}

struct Floor {
    x_axis: CoordinateAxis,
    y_axis: CoordinateAxis,
    red_tiles: Vec<Tile>,
}

impl Floor {
    fn new(theater: &Theater) -> Self {
        let tiles = theater.red_tiles();
        let x_max = tiles.iter().map(|tile| tile.x).max().unwrap_or(0);
        let y_max = tiles.iter().map(|tile| tile.y).max().unwrap_or(0);
        let x_axis = CoordinateAxis::new(
            std::iter::once(0)
                .chain(tiles.iter().map(|tile| tile.x))
                .chain(std::iter::once(x_max + 1)),
        );
        let y_axis = CoordinateAxis::new(
            std::iter::once(0)
                .chain(tiles.iter().map(|tile| tile.y))
                .chain(std::iter::once(y_max + 1)),
        );

        let red_tiles = tiles
            .iter()
            .map(|tile| Tile::new(x_axis.compress(tile.x), y_axis.compress(tile.y)))
            .collect();

        Self {
            x_axis,
            y_axis,
            red_tiles,
        }
    }

    fn tile_floor(&self) -> TiledFloor {
        let mut tiles = HashMap::new();

        self.mark_red_tiles(&mut tiles);
        self.mark_green_edges(&mut tiles);
        self.fill_space(&mut tiles);

        TiledFloor {
            x_axis: self.x_axis.clone(),
            y_axis: self.y_axis.clone(),
            tiles,
            corners: self.red_tiles.clone(),
        }
    }

    fn mark_red_tiles(&self, tiles: &mut HashMap<Tile, TileColor>) {
        for tile in &self.red_tiles {
            tiles.insert(*tile, TileColor::Red);
        }
    }

    fn mark_green_edges(&self, tiles: &mut HashMap<Tile, TileColor>) {
        let red_tiles = &self.red_tiles;
        for (i, current_tile) in red_tiles.iter().copied().enumerate() {
            let next_tile = red_tiles[(i + 1) % red_tiles.len()];
            let dx = next_tile.x as i128 - current_tile.x as i128;
            let dy = next_tile.y as i128 - current_tile.y as i128;
            let delta = (dx.signum(), dy.signum());
            let mut edge_tile = current_tile;
            while edge_tile != next_tile {
                edge_tile = Tile::new(
                    (edge_tile.x as i128 + delta.0) as usize,
                    (edge_tile.y as i128 + delta.1) as usize,
                );
                tiles.insert(edge_tile, TileColor::Green);
            }

            if dx == 0 {
                // Insert green tiles along the vertical edge along the compressed axis
                let x = current_tile.x;
                let y = current_tile.y.min(next_tile.y) + 1;
                let y_end = current_tile.y.max(next_tile.y);
                self.y_axis
                    .values
                    .iter()
                    .filter(|&&y_val| y_val > y && y_val < y_end)
                    .for_each(|&y_val| {
                        let compressed_y = self.y_axis.compress(y_val);
                        tiles.insert(Tile::new(x, compressed_y), TileColor::Green);
                    });
            } else if dy == 0 {
                // Insert green tiles along the horizontal edge along the compressed axis
                let y = current_tile.y;
                let x = current_tile.x.min(next_tile.x) + 1;
                let x_end = current_tile.x.max(next_tile.x);
                self.x_axis
                    .values
                    .iter()
                    .filter(|&&x_val| x_val > x && x_val < x_end)
                    .for_each(|&x_val| {
                        let compressed_x = self.x_axis.compress(x_val);
                        tiles.insert(Tile::new(compressed_x, y), TileColor::Green);
                    });
            }
        }
    }

    fn fill_space(&self, tiles: &mut HashMap<Tile, TileColor>) {
        let mut visited = HashSet::new();
        // Start in the top-left corner of the grid, which is assumed to be outside the enclosed area.
        let mut stack = vec![Tile::new(0, 0)];

        while let Some(current_tile) = stack.pop() {
            if visited.contains(&current_tile) {
                continue;
            }
            visited.insert(current_tile);

            if matches!(
                tiles.get(&current_tile),
                Some(TileColor::Red | TileColor::Green)
            ) {
                continue;
            }

            tiles.insert(current_tile, TileColor::None);

            let neighbors = [
                self.x_axis
                    .prev_index(current_tile.x)
                    .map(|x| Tile::new(x, current_tile.y)),
                self.y_axis
                    .prev_index(current_tile.y)
                    .map(|y| Tile::new(current_tile.x, y)),
                self.x_axis
                    .next_index(current_tile.x)
                    .map(|x| Tile::new(x, current_tile.y)),
                self.y_axis
                    .next_index(current_tile.y)
                    .map(|y| Tile::new(current_tile.x, y)),
            ]
            .into_iter()
            .flatten();

            for neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    stack.push(neighbor);
                }
            }
        }

        for x in 0..self.x_axis.values.len() {
            for y in 0..self.y_axis.values.len() {
                let tile = Tile::new(x, y);
                if !tiles.contains_key(&tile) {
                    tiles.insert(tile, TileColor::Green);
                }
            }
        }
    }
}

impl std::fmt::Debug for TiledFloor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.x_axis.decompress(self.x_axis.values.len() - 1);
        let height = self.y_axis.decompress(self.y_axis.values.len() - 1);
        for y in 0..=height + 1 {
            for x in 0..=width + 2 {
                let compressed_tile = match (
                    self.x_axis.values.binary_search(&x),
                    self.y_axis.values.binary_search(&y),
                ) {
                    (Ok(cx), Ok(cy)) => Some(Tile::new(cx, cy)),
                    _ => None,
                };
                match compressed_tile.and_then(|tile| self.tiles.get(&tile)) {
                    Some(TileColor::Red) => write!(f, "#")?,
                    Some(TileColor::Green) => write!(f, "X")?,
                    Some(TileColor::None) => write!(f, ".")?,
                    _ => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
enum TileColor {
    Red,
    Green,
    None,
}

struct TiledFloor {
    x_axis: CoordinateAxis,
    y_axis: CoordinateAxis,
    tiles: HashMap<Tile, TileColor>,
    corners: Vec<Tile>,
}

impl TiledFloor {
    fn area_contains_tile_color(&self, corner_i: &Tile, corner_j: &Tile, color: TileColor) -> bool {
        let min_x = corner_i.x.min(corner_j.x);
        let max_x = corner_i.x.max(corner_j.x);
        let min_y = corner_i.y.min(corner_j.y);
        let max_y = corner_i.y.max(corner_j.y);

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let tile = Tile::new(x, y);
                if self
                    .tiles
                    .get(&tile)
                    .is_some_and(|tile_color| *tile_color == color)
                {
                    return true;
                }
            }
        }
        false
    }

    fn compute_area_between_corners(corner_i: &Tile, corner_j: &Tile) -> i64 {
        let min_x = corner_i.x.min(corner_j.x);
        let max_x = corner_i.x.max(corner_j.x);
        let min_y = corner_i.y.min(corner_j.y);
        let max_y = corner_i.y.max(corner_j.y);

        let width = (max_x - min_x + 1) as i64;
        let height = (max_y - min_y + 1) as i64;

        width * height
    }

    fn find_largest_enclosed_area(&self) -> i64 {
        let mut largest_area = 0;
        for corner_i in &self.corners {
            for corner_j in &self.corners {
                if self.area_contains_tile_color(corner_i, corner_j, TileColor::None) {
                    continue;
                }
                let decompressed_i = Tile::new(
                    self.x_axis.decompress(corner_i.x),
                    self.y_axis.decompress(corner_i.y),
                );
                let decompressed_j = Tile::new(
                    self.x_axis.decompress(corner_j.x),
                    self.y_axis.decompress(corner_j.y),
                );
                let area = Self::compute_area_between_corners(&decompressed_i, &decompressed_j);
                largest_area = largest_area.max(area);
            }
        }
        largest_area
    }
}

pub fn part2(theater: &Theater) -> i64 {
    let floor = Floor::new(theater);
    let tiled_floor = floor.tile_floor();
    tiled_floor.find_largest_enclosed_area()
}
