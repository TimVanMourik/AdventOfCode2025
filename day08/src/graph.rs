use std::collections::HashMap;
use std::collections::HashSet;

pub type Circuit = HashSet<usize>;

pub struct ConnectionGraph {
    connections: HashMap<usize, HashSet<usize>>,
}

impl ConnectionGraph {
    pub(crate) fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    pub(crate) fn is_connected(&self, from: usize, to: usize) -> bool {
        self.connections
            .get(&from)
            .is_some_and(|neighbors| neighbors.contains(&to))
    }

    pub(crate) fn connect(&mut self, from: usize, to: usize) {
        self.connections.entry(from).or_default().insert(to);
        self.connections.entry(to).or_default().insert(from);
    }

    pub(crate) fn find_circuit(
        &self,
        junction_box: usize,
        visited: &mut HashSet<usize>,
    ) -> Circuit {
        let mut circuit = Circuit::new();
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

        circuit
    }

    pub(crate) fn find_circuits(&self) -> Vec<Circuit> {
        let mut visited = HashSet::<usize>::new();
        let mut circuits = Vec::<Circuit>::new();

        for &junction_box in self.connections.keys() {
            if !visited.contains(&junction_box) {
                let circuit = self.find_circuit(junction_box, &mut visited);
                if !circuit.is_empty() {
                    circuits.push(circuit);
                }
            }
        }

        circuits
    }
}
