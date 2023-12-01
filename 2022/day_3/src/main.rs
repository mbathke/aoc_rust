use std::{fs::*, collections::HashMap, rt::panic_count::count_is_zero};

// const FILE_PATH: &str = "./input.txt";
const FILE_PATH: &str = "./sample.txt";

fn read_file_str(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(read_to_string(file_path).unwrap())
}

fn main() {
    println!("Initializing character map...");

    let char_map: HashMap<char, u32> = HashMap::from([
        ('a', 1), ('b', 2), ('c', 3), ('d', 4), ('e', 5), ('f', 6),
        ('g', 7), ('h', 8), ('i', 9), ('j', 10), ('k', 11), ('l', 12),
        ('m', 13), ('n', 14), ('o', 15), ('p', 16), ('q', 17), ('r', 18),
        ('s', 19), ('t', 20), ('u', 21), ('v', 22), ('w', 23), ('x', 24),
        ('y', 25), ('z', 26),
        ('A', 27), ('B', 28), ('C', 29), ('D', 30), ('E', 31), ('F', 32),
        ('G', 33), ('H', 34), ('I', 35), ('J', 36), ('K', 37), ('L', 38),
        ('M', 39), ('N', 40), ('O', 41), ('P', 42), ('Q', 43), ('R', 44),
        ('S', 45), ('T', 46), ('U', 47), ('V', 48), ('W', 49), ('X', 50),
        ('Y', 51), ('Z', 52),
    ]);

    println!("Reading file...");

    let mut sum: u32 = 0;
    let mut count_badges: u32 = 0;
    let mut group: Vec<&str> = vec![];

    for rucksack in read_file_str(FILE_PATH)
            .unwrap()
            .split('\n')
            .filter(|item| !item.is_empty())
            .into_iter() {

        if count_badges < 3 {
            group.push(rucksack)
        } else if count_badges == 3 {
            count_badges = 0;
            // for group_rucksack in group.into_iter() {
            //
            // }
        }

        count_badges += 1;

        // TODO: for the second part it is not necessary to half the rucksack
        // instead compare 3 rucksacks
        // let rucksack_size = line.len() / 2;
        // let compartment_1 = line.get(0..rucksack_size).unwrap();
        // let compartment_2 = line.get(rucksack_size..).unwrap();
        // let mut found_value: bool = false;

        // for item in compartment_1.chars() {
        //     for another_item in compartment_2.chars() {
        //         if item == another_item && !found_value {
        //             let item_value = char_map.get(&item).unwrap();
        //             println!("Item {} == Another item {} - Code {}", item, another_item, item_value);
        //             sum += item_value;
        //             found_value = true;
        //         }
        //     }
        // }
    }

    println!("Total: {}", sum);
}
