use crate::ConnectionGraph;
use crate::{JunctionBox, Playground};

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

    let mut circuit_sizes: Vec<usize> = circuits.iter().map(|circuit| circuit.len()).collect();
    circuit_sizes.sort_unstable_by(|a, b| b.cmp(a));
    circuit_sizes.into_iter().take(3).product()
}
