use crate::{
    types::{Buttons, Joltage, Light, Lights},
    Machine,
};

fn parse_lights(input: &str) -> Lights {
    input
        .chars()
        .map(|ch| match ch {
            '#' => Light::On,
            _ => Light::Off,
        })
        .collect()
}

fn parse_wirings(input: &str) -> Buttons {
    input
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.trim_matches(|c| c == '(' || c == ')')
                .split(',')
                .filter_map(|n| n.parse::<usize>().ok())
                .collect()
        })
        .collect()
}

fn parse_joltage(input: &str) -> Joltage {
    input
        .trim_matches(|c| c == '{' || c == '}')
        .split(',')
        .filter_map(|n| n.trim().parse::<usize>().ok())
        .collect()
}

fn parse_line(line: &str) -> Machine {
    let trimmed = line.trim();
    let tokens: Vec<_> = trimmed.split_whitespace().collect();

    let lights = tokens
        .first()
        .copied()
        .map(|token| parse_lights(token.trim_matches(|c| c == '[' || c == ']')))
        .unwrap_or_default();

    let wirings = tokens
        .iter()
        .filter(|token| token.starts_with('(') && token.ends_with(')'))
        .map(|token| parse_wirings(token))
        .flatten()
        .collect();

    let joltage = tokens
        .iter()
        .filter(|token| token.starts_with('{') && token.ends_with('}'))
        .map(|token| parse_joltage(token))
        .flatten()
        .collect();

    Machine::new(lights, wirings, joltage)
}

pub fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_line)
        .collect()
}
