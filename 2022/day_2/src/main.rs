use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

// const FILE_PATH: &str = "./sample.txt";
const FILE_PATH: &str = "./input.txt";

enum Options {
    Rock = 1, // X // A
    Paper = 2, // Y // B
    Scissors = 3, // Z // C
}

enum Scores {
    Loss = 0,
    Draw = 3,
    Won = 6,
}

fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;

    let score_map: HashMap<String, u32> = HashMap::from([
        ("A X".to_string(), Options::Scissors as u32 + Scores::Loss as u32),
        ("A Y".to_string(), Options::Rock as u32 + Scores::Draw as u32),
        ("A Z".to_string(), Options::Paper as u32 + Scores::Won as u32),
        ("B X".to_string(), Options::Rock as u32 + Scores::Loss as u32),
        ("B Y".to_string(), Options::Paper as u32 + Scores::Draw as u32),
        ("B Z".to_string(), Options::Scissors as u32 + Scores::Won as u32),
        ("C X".to_string(), Options::Paper as u32 + Scores::Loss as u32),
        ("C Y".to_string(), Options::Scissors as u32 + Scores::Draw as u32),
        ("C Z".to_string(), Options::Rock as u32 + Scores::Won as u32),
    ]);


    for line in reader.lines() {
        sum = sum + score_map[&line.unwrap()] as u32;
    }

    println!("{}", sum);

    Ok(())
}

fn main() {
    read_file_line_by_line(&FILE_PATH).unwrap();
}
