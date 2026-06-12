mod part1;
mod part2;
pub use part1::part1;
pub use part2::part2;

enum Operator {
    Add,
    Multiply,
}

pub struct Computation {
    values: Vec<i64>,
    operator: Operator,
}

pub fn parse(input: &str) -> Vec<Computation> {
    let mut computation = Vec::new();
    let lines = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Check that all lines are of equal length
    let line_length = lines.first().map_or(0, |line| line.len());
    if !lines.iter().all(|line| line.len() == line_length) {
        panic!("All lines must have the same number of elements");
    }
    for i in 0..line_length {
        let mut values = Vec::new();
        for j in 0..lines.len() - 1 {
            let value = lines[j][i].parse::<i64>().unwrap_or_else(|_| {
                panic!("Failed to parse '{}' as i64 in line {}", lines[j][i], j + 1)
            });
            values.push(value);
        }
        let operator_str = lines.last().unwrap()[i];
        let operator = match operator_str {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            _ => panic!(
                "Invalid operator '{}' in line {}",
                operator_str,
                lines.len()
            ),
        };
        computation.push(Computation { values, operator });
    }

    computation
}
