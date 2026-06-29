use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputType {
    Test,
    Real,
}

pub fn input_path(day: u8, input_type: InputType) -> PathBuf {
    let filename = match input_type {
        InputType::Test => "test.txt",
        InputType::Real => "input.txt",
    };
    PathBuf::from(format!("inputs/day{day:02}/{filename}"))
}

pub fn read_input(day: u8, input_type: InputType) -> Result<String> {
    let path = input_path(day, input_type);
    fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))
}
