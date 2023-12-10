use crate::utils::read_file_str;

enum Bag {
    Red = 12,
    Green = 13,
    Blue = 14
}

fn part_one() {
    const FILE_PATH: &str = "data/day_two/data_one.txt";

    let input = read_file_str(FILE_PATH).unwrap();
    let input_lines: Vec<&str> = input.lines().collect();
    let mut valid_games = 0;

    for l in input_lines {
        let mut game = 0;
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        let mut keyword = String::from("");
        let mut number = 0;

        for ch in l.chars() {
            if ch.is_digit(10) {
                number = number * 10 + ch.to_digit(10).unwrap();
            }

            if ch == ';' || ch == ':' || ch == ',' {
                number = 0;
                keyword = String::from("");
                continue;
            }

            if !ch.is_digit(10) {
                keyword.push(ch);
            }

            if keyword.trim() == "Game" && number > 0 {
                game = number;
            }

            match keyword.trim() {
                "red" => if red < number { red = number },
                "green" => if green < number { green = number },
                "blue" => if blue < number { blue = number },
                &_ => continue
            }
        }

        if red <= Bag::Red as u32 && green <= Bag::Green as u32 && blue <= Bag::Blue as u32 {
            valid_games += game;
        }

        // println!("Game '{}' has {} red cubes, {} green cubes and {} blue cubes.", game, red, green, blue);
    }

    println!("Part One - Valid Games are {}", valid_games);
}

fn part_two() {
    const FILE_PATH: &str = "data/day_two/data_one.txt";

    let input = read_file_str(FILE_PATH).unwrap();
    let input_lines: Vec<&str> = input.lines().collect();
    let mut valid_games = 0;

    for l in input_lines {
        let mut _game = 0;
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        let mut keyword = String::from("");
        let mut number = 0;

        for ch in l.chars() {
            if ch.is_digit(10) {
                number = number * 10 + ch.to_digit(10).unwrap();
            }

            if ch == ';' || ch == ':' || ch == ',' {
                number = 0;
                keyword = String::from("");
                continue;
            }

            if !ch.is_digit(10) {
                keyword.push(ch);
            }

            if keyword.trim() == "Game" && number > 0 {
                _game = number;
            }

            match keyword.trim() {
                "red" => if red < number { red = number },
                "green" => if green < number { green = number },
                "blue" => if blue < number { blue = number },
                &_ => continue
            }
        }

        let power_of_cubes = red * green * blue;
        valid_games += power_of_cubes;

        // println!("Game '{}' has {} red cubes, {} green cubes and {} blue cubes with the power of {}.", game, red, green, blue, power_of_cubes);
    }

    println!("Part Two - Valid Games were {}", valid_games);
}

pub fn run() {
    println!("\nDay 2");
    part_one();
    part_two();
}
