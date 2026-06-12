use crate::{Computation, Operator};

pub fn part1(items: &[Computation]) -> i64 {
    items
        .iter()
        .map(|comp| {
            let result: i64 = match comp.operator {
                Operator::Add => comp.values.iter().sum(),
                Operator::Multiply => comp.values.iter().product(),
            };
            result
        })
        .sum()
}
