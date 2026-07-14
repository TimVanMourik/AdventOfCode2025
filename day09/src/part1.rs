use std::collections::HashSet;

use crate::{Theater, Tile};

pub fn part1(theater: &Theater) -> u128 {
    let mut hull: Vec<_> = compute_convex_hull(theater.red_tiles())
        .into_iter()
        .collect();
    hull.sort_unstable_by_key(|tile| (tile.x, tile.y));
    let mut max_area: u128 = 0;

    for i in 0..hull.len() {
        for j in (i + 1)..hull.len() {
            let dx = (hull[i].x as i128 - hull[j].x as i128).abs() + 1;
            let dy = (hull[i].y as i128 - hull[j].y as i128).abs() + 1;
            let distance = (dx * dy) as u128;

            max_area = max_area.max(distance);
        }
    }

    max_area
}

fn compute_convex_hull(tiles: &HashSet<Tile>) -> HashSet<Tile> {
    let mut sorted_tiles: Vec<Tile> = tiles.iter().copied().collect();
    sorted_tiles.sort_unstable_by_key(|tile| (tile.x, tile.y));

    if sorted_tiles.len() <= 1 {
        return sorted_tiles.into_iter().collect();
    }

    let mut lower_hull: Vec<Tile> = Vec::new();
    for &tile in &sorted_tiles {
        while lower_hull.len() >= 2
            && cross_product(
                lower_hull[lower_hull.len() - 2],
                lower_hull[lower_hull.len() - 1],
                tile,
            ) <= 0
        {
            lower_hull.pop();
        }
        lower_hull.push(tile);
    }

    let mut upper_hull: Vec<Tile> = Vec::new();
    for &tile in sorted_tiles.iter().rev() {
        while upper_hull.len() >= 2
            && cross_product(
                upper_hull[upper_hull.len() - 2],
                upper_hull[upper_hull.len() - 1],
                tile,
            ) <= 0
        {
            upper_hull.pop();
        }
        upper_hull.push(tile);
    }

    lower_hull.pop();
    upper_hull.pop();

    lower_hull.into_iter().chain(upper_hull).collect()
}

fn cross_product(origin: Tile, a: Tile, b: Tile) -> i128 {
    let ax = a.x as i128 - origin.x as i128;
    let ay = a.y as i128 - origin.y as i128;
    let bx = b.x as i128 - origin.x as i128;
    let by = b.y as i128 - origin.y as i128;

    ax * by - ay * bx
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{compute_convex_hull, part1};
    use crate::{Theater, Tile};

    fn theater_from_tiles(tiles: &[(usize, usize)]) -> Theater {
        Theater::new(tiles.iter().map(|&(x, y)| Tile::new(x, y)).collect())
    }

    #[test]
    fn convex_hull_excludes_interior_points() {
        let theater = theater_from_tiles(&[(0, 0), (2, 0), (2, 2), (0, 2), (1, 1)]);

        let expected: HashSet<Tile> = [
            Tile::new(0, 0),
            Tile::new(2, 0),
            Tile::new(2, 2),
            Tile::new(0, 2),
        ]
        .into_iter()
        .collect();

        assert_eq!(compute_convex_hull(theater.red_tiles()), expected);
    }

    #[test]
    fn convex_hull_of_collinear_points_keeps_endpoints() {
        let theater = theater_from_tiles(&[(0, 0), (1, 0), (2, 0), (3, 0)]);

        let expected: HashSet<Tile> = [Tile::new(0, 0), Tile::new(3, 0)].into_iter().collect();

        assert_eq!(compute_convex_hull(theater.red_tiles()), expected);
    }

    #[test]
    fn part1_uses_hull_only() {
        let theater = theater_from_tiles(&[(0, 0), (2, 0), (2, 2), (0, 2), (1, 1)]);

        assert_eq!(part1(&theater), 9);
    }
}
