use std::fs::*;

pub fn read_file_str(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(read_to_string(file_path).unwrap())
}

pub mod day_one;
pub mod day_two;
