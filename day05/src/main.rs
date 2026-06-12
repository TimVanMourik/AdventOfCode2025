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

    let input = aoc_core::read_input(5, input_type)?;
    let parsed = day05::parse(&input);

    println!("Part 1: {}", day05::part1(&parsed));
    println!("Part 2: {}", day05::part2(&parsed));

    Ok(())
}
