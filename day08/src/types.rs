pub(crate) type Coordinate = (usize, usize, usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JunctionBox {
    location: Coordinate,
}

impl JunctionBox {
    pub(crate) fn new(location: Coordinate) -> Self {
        Self { location }
    }

    pub(crate) fn coordinates(&self) -> Coordinate {
        self.location
    }

    pub fn x(&self) -> usize {
        self.location.0
    }
}
