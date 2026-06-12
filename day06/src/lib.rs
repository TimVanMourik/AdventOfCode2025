mod part1;
mod part2;
pub use part1::part1;
pub use part2::part2;

#[derive(Debug, Copy, Clone)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug)]
pub struct Computation {
    values: Vec<i64>,
    operator: Operator,
}

impl Computation {
    fn compute(&self) -> i64 {
        match self.operator {
            Operator::Add => self.values.iter().sum(),
            Operator::Multiply => self.values.iter().product(),
        }
    }
}
