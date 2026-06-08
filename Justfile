set shell := ["bash", "-cu"]

default:
    @just --list

# Run a specific day, e.g. just run 1 or just run 1 --test
run day args="":
    @d=$(printf "%02d" {{day}}); cargo run -p "day${d}" -- {{args}}

# Test one day crate, e.g. just test 1
test day:
    @d=$(printf "%02d" {{day}}); cargo test -p "day${d}"

# Test entire workspace
check:
    cargo fmt --all -- --check
    cargo clippy --workspace -- -D warnings
    cargo test --workspace

# Format everything
fmt:
    cargo fmt --all

# Lint everything
clippy:
    cargo clippy --workspace -- -D warnings

# Benchmark one day crate when benches exist
bench day:
    @d=$(printf "%02d" {{day}}); cargo bench -p "day${d}"

# Create a new day crate from template, e.g. just new-day 7
new-day day:
    ./scripts/new-day.sh {{day}}
