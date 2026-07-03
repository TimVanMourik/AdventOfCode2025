use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::DistanceMatrix;
use crate::{JunctionBox, Playground};

type Circuit = HashSet<usize>;

pub struct ConnectionGraph {
    connections: HashMap<usize, HashSet<usize>>,
}

impl ConnectionGraph {
    fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    fn is_connected(&self, from: usize, to: usize) -> bool {
        self.connections
            .get(&from)
            .is_some_and(|neighbors| neighbors.contains(&to))
    }

    fn connect(&mut self, from: usize, to: usize) {
        self.connections.entry(from).or_default().insert(to);
        self.connections.entry(to).or_default().insert(from);
    }

    fn find_circuits(&self) -> Vec<Circuit> {
        let mut visited = HashSet::<usize>::new();
        let mut circuits = Vec::<Circuit>::new();

        for &junction_box in self.connections.keys() {
            if !visited.contains(&junction_box) {
                let mut circuit = HashSet::new();
                self.explore_circuit(junction_box, &mut visited, &mut circuit);
                if !circuit.is_empty() {
                    circuits.push(circuit);
                }
            }
        }

        circuits
    }

    fn explore_circuit(
        &self,
        junction_box: usize,
        visited: &mut HashSet<usize>,
        circuit: &mut Circuit,
    ) {
        let mut stack = vec![junction_box];

        while let Some(node) = stack.pop() {
            if !visited.insert(node) {
                continue;
            }

            circuit.insert(node);

            if let Some(neighbors) = self.connections.get(&node) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        stack.push(neighbor);
                    }
                }
            }
        }
    }
}

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
    pub fn wire_up(&self, n: usize) -> ConnectionGraph {
        let mut connections = ConnectionGraph::new();
        let mut n_connections = 0;
        let mut it = self.distance_matrix().shortest_distances();
        while n_connections < n {
            let Some((_d, i, j)) = it.next() else {
                break;
            };

            if !connections.is_connected(i, j) {
                connections.connect(i, j);
                n_connections += 1;
            }
        }
        connections
    }
}

pub fn part1(items: &[JunctionBox], number_of_connections: usize) -> usize {
    let playground = Playground::new(items);
    let graph = playground.wire_up(number_of_connections);
    let circuits = graph.find_circuits();

    let mut circuit_sizes: Vec<usize> = circuits.iter().map(HashSet::len).collect();
    circuit_sizes.sort_unstable_by(|a, b| b.cmp(a));
    circuit_sizes.into_iter().take(3).product()
}
