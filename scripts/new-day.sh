#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: $0 <day-number>" >&2
  exit 1
fi

if ! [[ $1 =~ ^[0-9]+$ ]]; then
  echo "day must be a number" >&2
  exit 1
fi

day_num=$1
if (( day_num < 1 || day_num > 25 )); then
  echo "day must be between 1 and 25" >&2
  exit 1
fi

day=$(printf "%02d" "$day_num")
crate="day${day}"
crate_dir="$crate"
input_dir="inputs/day${day}"

if [[ -e "$crate_dir" ]]; then
  echo "$crate_dir already exists" >&2
  exit 1
fi

mkdir -p "$crate_dir/src" "$input_dir"

cat > "$crate_dir/Cargo.toml" <<EOF
[package]
name = "$crate"
version = "0.1.0"
edition.workspace = true
license.workspace = true

[dependencies]
aoc-core = { path = "../aoc-core" }
anyhow.workspace = true

[[bin]]
name = "$crate"
path = "src/main.rs"
EOF

cat > "$crate_dir/src/lib.rs" <<'EOF'
use anyhow::{Context, Result};

mod part1;
mod part2;
pub use part1::part1;
pub use part2::part2;

pub struct Item {
    pub value: String,
}

fn parse_line(line: &str) -> Item {
    let trimmed = line.trim();
    Item {
        value: trimmed
            .parse::<String>()
            .with_context(|| format!("invalid line: {}", trimmed))?,
    }
}

pub fn parse(input: &str) -> Vec<Item> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_line)
        .collect()
}
EOF

cat > "$crate_dir/src/part1.rs" <<'EOF'
use crate::Item;

pub fn part1(_items: &[Item]) -> i64 {
    0
}
EOF

cat > "$crate_dir/src/part2.rs" <<'EOF'
use crate::Item;

pub fn part2(_items: &[Item]) -> i64 {
    0
}
EOF

cat > "$crate_dir/src/main.rs" <<EOF
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

    let input = aoc_core::read_input(${day_num}, input_type)?;
    let parsed = ${crate}::parse(&input).collect::<Result<Vec<_>>>()?;

    println!("Part 1: {}", ${crate}::part1(&parsed));
    println!("Part 2: {}", ${crate}::part2(&parsed));

    Ok(())
}
EOF

if [[ ! -e "$input_dir/input.txt" ]]; then
  : > "$input_dir/input.txt"
fi
if [[ ! -e "$input_dir/test.txt" ]]; then
  : > "$input_dir/test.txt"
fi

echo "Created $crate_dir and $input_dir"
