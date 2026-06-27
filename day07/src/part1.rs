use crate::Manifold;

pub fn part1(manifold: &Manifold) -> usize {
    let (_beam, n_splits) = crate::propagate_beam(manifold.start, &manifold.splitters);
    n_splits
}
