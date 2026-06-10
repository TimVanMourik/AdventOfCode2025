#!/bin/bash
set -e

rustup default stable
rustup component add rustfmt clippy
cargo install just --locked --force
