use crate::{Direction, Move};

struct Dial {
    position: i64,
    hits: i64,
}

impl Dial {
    fn new() -> Self {
        Dial {
            position: 50,
            hits: 0,
        }
    }

    fn turn(mut self, m: &Move) -> Self {
        self.position = match m.direction {
            Direction::L => (self.position - m.distance).rem_euclid(100),
            Direction::R => (self.position + m.distance).rem_euclid(100),
        };
        if self.position == 0 {
            self.hits += 1;
        }
        self
    }
}

pub fn part1(moves: &[Move]) -> i64 {
    moves.iter().fold(Dial::new(), |dial, m| dial.turn(m)).hits
}
