use anyhow::Result;
use aoc_core::InputType;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input_type = if args.contains(&"--test".to_string()) {
        InputType::Test
    } else {
        InputType::Real
    };

    let input = aoc_core::read_input(8, input_type)?;
    let parsed = day08::parse(&input);

    let number_of_connections = if input_type == InputType::Test {
        10
    } else {
        1000
    };
    println!("Part 1: {}", day08::part1(&parsed, number_of_connections));
    println!("Part 2: {}", day08::part2(&parsed));

    Ok(())
}
