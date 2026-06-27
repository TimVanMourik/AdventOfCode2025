use std::collections::HashMap;

use crate::{Manifold, SplitterLocations};

fn compute_timelines(
    row_index: usize,
    start: usize,
    locations: &[SplitterLocations],
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(&cached) = memo.get(&(row_index, start)) {
        return cached;
    }

    let result = match locations.split_first() {
        None => 1,
        Some((row, rest)) => {
            if row.contains(&start) {
                let left = start.checked_sub(1).map_or(0, |new_start| {
                    compute_timelines(row_index + 1, new_start, rest, memo)
                });
                let right = start.checked_add(1).map_or(0, |new_start| {
                    compute_timelines(row_index + 1, new_start, rest, memo)
                });
                left + right
            } else {
                compute_timelines(row_index + 1, start, rest, memo)
            }
        }
    };

    memo.insert((row_index, start), result);
    result
}

pub fn part2(manifold: &Manifold) -> usize {
    let mut memo = HashMap::new();
    compute_timelines(0, manifold.start, &manifold.splitters, &mut memo)
}
