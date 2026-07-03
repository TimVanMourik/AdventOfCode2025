use crate::{Computation, Operator};

struct Operation {
    operator: Operator,
    index: usize,
}

fn char_to_operation(char: char, index: usize) -> Option<Operation> {
    match char {
        '+' => Some(Operation {
            operator: Operator::Add,
            index,
        }),
        '*' => Some(Operation {
            operator: Operator::Multiply,
            index,
        }),
        ' ' => None, // Skip empty operator
        _ => panic!("Invalid operator '{}'", char,),
    }
}

fn right_pad(line: &str, longest: usize) -> String {
    let mut padded = line.to_string();
    while padded.len() < longest {
        padded.push(' ');
    }
    padded
}

fn extract_values_for_operation(lines: &[String], operation_index: usize) -> Vec<i64> {
    let mut values = Vec::new();
    let mut index = 0;

    loop {
        let mut value = String::new();
        for line in lines {
            value.push_str(
                &line
                    .chars()
                    .nth(operation_index + index)
                    .unwrap_or(' ')
                    .to_string(),
            );
        }

        if value.trim().is_empty() {
            break;
        }

        values.push(value.trim().parse::<i64>().unwrap_or_else(|_| {
            panic!("Failed to parse '{}' as i64 in line {}", value, index + 1)
        }));
        index += 1;
    }

    values
}

pub fn parse(input: &str) -> Vec<Computation> {
    let mut computation = Vec::new();
    let lines = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>();

    let line_length = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    // Pad shorter lines with spaces
    let lines = lines
        .into_iter()
        .map(|line| right_pad(line, line_length))
        .collect::<Vec<_>>();

    // Find all operatotrs on last line with their indices
    let operations = lines
        .last()
        .unwrap()
        .chars()
        .enumerate()
        .filter_map(|(index, char)| char_to_operation(char, index))
        .collect::<Vec<_>>();

    // loop through enumartion of operations
    for operation in operations {
        let values = extract_values_for_operation(&lines[..lines.len() - 1], operation.index);
        computation.push(Computation {
            values,
            operator: operation.operator,
        });
    }
    computation
}

pub fn part2(input: &String) -> i64 {
    parse(&input).iter().map(|comp| comp.compute()).sum()
}
