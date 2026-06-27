use std::collections::HashSet;

mod part1;
mod part2;

pub use part1::part1;
pub use part2::part2;

type Location = usize;
type SplitterLocations = HashSet<Location>;
type BeamLocations = HashSet<Location>;

pub struct Manifold {
    start: usize,
    splitters: Vec<SplitterLocations>,
}

pub struct Beam {
    beam_locations: Vec<BeamLocations>,
}

impl std::fmt::Debug for Beam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self
            .beam_locations
            .iter()
            .flat_map(|locs| locs.iter())
            .copied()
            .max()
            .unwrap_or(0)
            + 1;
        for row in &self.beam_locations {
            for col in 0..width {
                write!(f, "{}", if row.contains(&col) { '|' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_line(line: &str) -> SplitterLocations {
    line.char_indices()
        .filter_map(|(index, character)| (character == '^').then_some(index))
        .collect()
}

fn find_start(input: &str) -> Option<usize> {
    input.find("S")
}

pub fn parse(input: &str) -> Manifold {
    let mut lines = input.lines();
    let start = lines.next().map(find_start).flatten().unwrap();

    let splitters = lines
        .filter(|line| !line.trim().is_empty())
        .map(parse_line)
        .collect();

    Manifold { start, splitters }
}
