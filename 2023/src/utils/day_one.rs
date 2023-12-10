use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::utils::read_file_str;
use regex::Regex;

lazy_static! {
    static ref DIGITS: HashMap<&'static str, u32> = {
        HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9)
        ])
    };
}

fn part_one() {
    const FILE_PATH: &str = "data/day_one/data_one.txt";

    let re = Regex::new(r"[^\d]").unwrap();
    let input = read_file_str(FILE_PATH).unwrap();
    let sum = input.split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| re.replace_all(l, ""))
        .map(|n| {
            let mut first_and_last: String = Default::default();
            first_and_last.push(n.chars().next().unwrap());
            first_and_last.push(n.chars().last().unwrap());
            return first_and_last;
        })
        .reduce(|a, b| (a.to_string().parse::<i32>().unwrap() + &b.to_string().parse::<i32>().unwrap()).to_string())
        .unwrap()
        .parse::<u32>()
        .unwrap();

    println!("Part One: {}", sum);
}

fn part_two() {
    const FILE_PATH: &str = "data/day_one/data_two.txt";
    let numbers: Vec<String> = vec![
        "\\d".to_string(),
        "one".to_string(), 
        "two".to_string(),
        "three".to_string(), 
        "four".to_string(),
        "five".to_string(), 
        "six".to_string(),
        "seven".to_string(), 
        "eight".to_string(),
        "nine".to_string(), 
    ];

    let first_numbers = format!(r"({})", numbers.join("|"));
    let last_numbers = format!(r".*({})", numbers.join("|"));
    let re_first_number = Regex::new(&first_numbers).unwrap();
    let re_last_number = Regex::new(&last_numbers).unwrap();

    let input = read_file_str(FILE_PATH).unwrap();
    let output: u32 = input.lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            // used this solution from Gobbel2000
            // https://github.com/Gobbel2000/advent-of-code/blob/a86bed09f1721c410d9c0f8dffca21be987c9ee3/src/bin/2023day1-1.rs
            let d = re_first_number.find(l).unwrap().as_str();
            let first = u32::from_str_radix(d, 10).unwrap_or_else(|_| DIGITS[d]);

            let last_match = re_last_number.captures(l).unwrap();
            let d = last_match.get(1).unwrap().as_str();
            let last = u32::from_str_radix(d, 10).unwrap_or_else(|_| DIGITS[d]);

            first * 10 + last
        })
        .sum();

    println!("Part Two: {}", output);
}

pub fn run() {
    println!("\nDay 1");
    part_one();
    part_two();
}
