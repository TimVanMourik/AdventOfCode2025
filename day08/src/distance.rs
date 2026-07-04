use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::JunctionBox;

pub struct Playground {
    pub distance_matrix: DistanceMatrix,
}

impl Playground {
    pub fn new(junction_boxes: &[JunctionBox]) -> Self {
        let distance_matrix = Self::build_distance_matrix(junction_boxes);
        Playground { distance_matrix }
    }

    pub fn distance_matrix(&self) -> &DistanceMatrix {
        &self.distance_matrix
    }

    fn build_distance_matrix(junction_boxes: &[JunctionBox]) -> DistanceMatrix {
        let n = junction_boxes.len();
        let mut distances = vec![vec![0_i64; n]; n];

        for i in 0..n {
            distances[i][i] = 0;
            for j in (i + 1)..n {
                let (x1, y1, z1) = junction_boxes[i].coordinates();
                let (x2, y2, z2) = junction_boxes[j].coordinates();
                let dx = x2 as i64 - x1 as i64;
                let dy = y2 as i64 - y1 as i64;
                let dz = z2 as i64 - z1 as i64;
                let distance = dx * dx + dy * dy + dz * dz;
                distances[i][j] = distance;
                distances[j][i] = distance;
            }
        }

        DistanceMatrix { distances }
    }
}

pub struct DistanceMatrix {
    distances: Vec<Vec<i64>>,
}

impl std::fmt::Debug for DistanceMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.distances {
            for &distance in row {
                write!(f, "{:>5} ", distance)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct ShortestDistanceIter {
    heap: BinaryHeap<Reverse<(i64, usize, usize)>>,
}

impl Iterator for ShortestDistanceIter {
    type Item = (i64, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.heap.pop().map(|Reverse(item)| item)
    }
}

impl DistanceMatrix {
    pub fn shortest_distances(&self) -> ShortestDistanceIter {
        let n = self.distances.len();
        let mut heap = BinaryHeap::with_capacity(n.saturating_mul(n.saturating_sub(1)) / 2);

        for i in 0..n {
            for j in (i + 1)..n {
                heap.push(Reverse((self.distances[i][j], i, j)));
            }
        }

        ShortestDistanceIter { heap }
    }

    pub(crate) fn len(&self) -> usize {
        self.distances.len()
    }
}
