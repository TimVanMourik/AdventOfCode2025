# Advent of Code 2025 (Rust)

## Quick start

```bash
just check
just run 1
just test 1
```

## Add a new day

```bash
just new-day 2
just run 2
```

## Layout

- `aoc-core/`: shared input and helper utilities
- `day01/`: day-specific crate (library + binary)
- `inputs/2025/`: puzzle inputs per day
- `scripts/new-day.sh`: scaffolds `dayXX` crates
