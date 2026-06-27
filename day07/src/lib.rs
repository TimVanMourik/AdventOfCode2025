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

fn apply_splitter(
    (mut locs, splits): (BeamLocations, usize),
    loc: Location,
    splitter_locations: &SplitterLocations,
) -> (BeamLocations, usize) {
    if splitter_locations.contains(&loc) {
        locs.extend([loc - 1, loc + 1]);
        (locs, splits + 1)
    } else {
        locs.insert(loc);
        (locs, splits)
    }
}

fn propagate_beam(start: usize, splitters: &[SplitterLocations]) -> (Beam, usize) {
    splitters.iter().fold(
        (
            Beam {
                beam_locations: vec![HashSet::from([start])],
            },
            0,
        ),
        |(mut beam, n_splits), splitter_locations| {
            let (new_locations, splits) = beam
                .beam_locations
                .last()
                .unwrap()
                .iter()
                .fold((HashSet::new(), 0), |acc, &loc| {
                    apply_splitter(acc, loc, splitter_locations)
                });
            beam.beam_locations.push(new_locations);
            (beam, n_splits + splits)
        },
    )
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
