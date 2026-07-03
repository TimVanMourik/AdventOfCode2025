use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::DistanceMatrix;
use crate::{JunctionBox, Playground};

pub struct ShortestDistanceIter {
    heap: BinaryHeap<Reverse<(i64, usize, usize)>>, // (distance, i, j)
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
}

impl Playground {
    pub fn wire_up(&self, n: usize) -> HashMap<&JunctionBox, Vec<&JunctionBox>> {
        let mut connections: HashMap<&JunctionBox, Vec<&JunctionBox>> = HashMap::new();
        let mut n_connections = 0;
        let mut it = self.distance_matrix().shortest_distances();
        loop {
            if let Some((_d, i, j)) = it.next() {
                let source = &self.junction_boxes[i];
                let target = &self.junction_boxes[j];
                let is_connected = connections
                    .get(source)
                    .map_or(false, |v| v.contains(&target));
                if !is_connected {
                    Self::connect(source, target, &mut connections);
                    n_connections += 1;
                }
            } else {
                break;
            }
            if n_connections >= n {
                break;
            }
        }
        connections
    }

    fn connect<'a>(
        from: &'a JunctionBox,
        to: &'a JunctionBox,
        memo: &mut HashMap<&'a JunctionBox, Vec<&'a JunctionBox>>,
    ) {
        memo.entry(from).or_default().push(to);
        memo.entry(to).or_default().push(from);
    }
}

fn find_circuits<'a>(
    connections: &HashMap<&'a JunctionBox, Vec<&'a JunctionBox>>,
) -> Vec<Circuit<'a>> {
    let mut visited = HashSet::<&'a JunctionBox>::new();
    let mut circuits = Vec::<Circuit<'a>>::new();

    for &junction_box in connections.keys() {
        if !visited.contains(junction_box) {
            let mut circuit = HashSet::new();
            explore_circuit(junction_box, connections, &mut visited, &mut circuit);
            if !circuit.is_empty() {
                circuits.push(circuit);
            }
        }
    }

    circuits
}

fn explore_circuit<'a>(
    junction_box: &'a JunctionBox,
    connections: &HashMap<&'a JunctionBox, Vec<&'a JunctionBox>>,
    visited: &mut HashSet<&'a JunctionBox>,
    circuit: &mut Circuit<'a>,
) {
    if !visited.insert(junction_box) {
        return;
    }

    circuit.insert(junction_box);

    if let Some(neighbors) = connections.get(junction_box) {
        for &neighbor in neighbors {
            if !visited.contains(neighbor) {
                explore_circuit(neighbor, connections, visited, circuit);
            }
        }
    }
}

type Circuit<'a> = HashSet<&'a JunctionBox>;

pub fn part1(items: &[JunctionBox], number_of_connections: usize) -> usize {
    let playground = Playground::new(items);
    let connections = playground.wire_up(number_of_connections);
    let circuits = find_circuits(&connections);

    let mut circuit_sizes: Vec<usize> = circuits.iter().map(HashSet::len).collect();
    circuit_sizes.sort_unstable_by(|a, b| b.cmp(a));
    circuit_sizes.into_iter().take(3).product()
}
