use std::collections::HashSet;

use crate::{Beam, BeamLocations, Location, Manifold, SplitterLocations};

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

pub fn part1(manifold: &Manifold) -> usize {
    let (_beam, n_splits) = propagate_beam(manifold.start, &manifold.splitters);
    n_splits
}
