use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

// const FILE_PATH: &str = "./sample_input.txt";
const FILE_PATH: &str = "./input.txt";

fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut temp_vec: Vec<u32> = vec![];
    let mut temp_value: u32 = 0;

    for line in reader.lines() {
        match line.as_ref() {
            Ok(ref x) if x.is_empty() => {
                temp_vec.push(temp_value);
                temp_value = 0;
            },
            _ => temp_value = temp_value + line?.parse::<u32>().unwrap(),
        }
        // temp_vec.push(line.unwrap());
    }

    temp_vec.sort();

    println!("{:?}", temp_vec);
    println!("{:?}", temp_vec.last().unwrap());

    println!("{:?}", temp_vec[temp_vec.len() - 1] + temp_vec[temp_vec.len() - 2] + temp_vec[temp_vec.len() - 3]);

    Ok(())
}

fn main() {
    println!("In file {}", FILE_PATH);

    read_file_line_by_line(&FILE_PATH).unwrap();
}
