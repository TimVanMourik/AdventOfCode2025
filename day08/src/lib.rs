mod part1;
mod part2;
pub use part1::part1;
pub use part2::part2;

type Coordinate = (usize, usize, usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JunctionBox {
    location: Coordinate,
}

pub struct Playground {
    junction_boxes: Vec<JunctionBox>,
    distance_matrix: DistanceMatrix,
}

impl Playground {
    pub fn new(junction_boxes: &[JunctionBox]) -> Self {
        let distance_matrix = Self::build_distance_matrix(&junction_boxes);
        Playground {
            junction_boxes: junction_boxes.to_vec(),
            distance_matrix,
        }
    }

    pub fn distance_matrix(&self) -> &DistanceMatrix {
        &self.distance_matrix
    }

    fn build_distance_matrix(junction_boxes: &[JunctionBox]) -> DistanceMatrix {
        let n = junction_boxes.len();
        let mut distances = vec![vec![0 as i64; n]; n];

        for i in 0..n {
            distances[i][i] = 0;
            for j in (i + 1)..n {
                let (x1, y1, z1) = junction_boxes[i].location;
                let (x2, y2, z2) = junction_boxes[j].location;
                let dx = x2 as i64 - x1 as i64;
                let dy = y2 as i64 - y1 as i64;
                let dz = z2 as i64 - z1 as i64;
                let distance = dx * dx + dy * dy + dz * dz;
                distances[i][j] = distance;
                distances[j][i] = distance;
            }
        }

        DistanceMatrix { distances }
    }
}

pub struct DistanceMatrix {
    distances: Vec<Vec<i64>>,
}

impl std::fmt::Debug for DistanceMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.distances {
            for &distance in row {
                write!(f, "{:>5} ", distance)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_line(line: &str) -> JunctionBox {
    let trimmed = line.trim();
    let parts: Vec<usize> = trimmed
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    JunctionBox {
        location: (parts[0], parts[1], parts[2]),
    }
}

pub fn parse(input: &str) -> Vec<JunctionBox> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_line)
        .collect()
}
