use crate::Circuit;
use crate::ConnectionGraph;
use crate::JunctionBox;
use crate::Playground;

impl Playground {
    pub fn wire_all(&self) -> (usize, usize) {
        let mut connections = ConnectionGraph::new();
        let mut main_circuit = Circuit::new();
        let mut it = self.distance_matrix().shortest_distances();
        let mut last_connected = (0, 0);
        while main_circuit.len() < self.distance_matrix().len() {
            let Some((_d, i, j)) = it.next() else {
                return last_connected;
            };

            if !connections.is_connected(i, j) {
                connections.connect(i, j);
            }

            // Update main circuit with the newly connected junction boxes
            if main_circuit.is_empty() {
                main_circuit.insert(i);
                main_circuit.insert(j);
            } else {
                if main_circuit.contains(&i) && main_circuit.contains(&j) {
                    continue;
                }
                if main_circuit.contains(&i) || main_circuit.contains(&j) {
                    let circuit = connections.find_circuit(i, &mut Circuit::new());
                    main_circuit.extend(circuit);
                    last_connected = (i, j);
                }
            }
        }
        last_connected
    }
}

pub fn part2(items: &[JunctionBox]) -> usize {
    let playground = Playground::new(items);
    let last_connected = playground.wire_all();

    let x1 = items[last_connected.0].x();
    let x2 = items[last_connected.1].x();

    x1 * x2
}
