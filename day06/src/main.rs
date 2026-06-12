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

    let input = aoc_core::read_input(6, input_type)?;

    println!("Part 1: {}", day06::part1(&input));
    println!("Part 2: {}", day06::part2(&input));

    Ok(())
}
