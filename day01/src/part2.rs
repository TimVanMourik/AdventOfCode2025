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
        // Count the number of times we hit 0 during the turn. Take into account
        // turns of more than 100, and turns that wrap around from 99 to 0.

        match m.direction {
            Direction::R => {
                self.hits += (m.distance + self.position) / 100;
            }
            Direction::L => {
                if self.position == 0 {
                    self.hits += (m.distance + self.position) / 100;
                } else {
                    self.hits += (m.distance + 100 - self.position) / 100;
                }
            }
        }
        self.position = match m.direction {
            Direction::L => (self.position - m.distance).rem_euclid(100),
            Direction::R => (self.position + m.distance).rem_euclid(100),
        };

        self
    }
}

pub fn part2(moves: &[Move]) -> i64 {
    moves.iter().fold(Dial::new(), |dial, m| dial.turn(m)).hits
}
