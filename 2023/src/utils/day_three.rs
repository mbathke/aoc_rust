use std::str::Lines;

use crate::utils::read_file_str;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Symbol {
    None,
    Some,
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct MachinePart {
    num: u32,
    symbol: Symbol
}

fn has_symbol_near_by(grid: &Lines, x: usize, y: usize) -> Symbol {
    let new_grid: Vec<_> = grid.clone().map(|l| l.chars().collect::<Vec<_>>()).collect();

    if y > 0 {
        // check one above
        if !new_grid[y - 1][x].is_digit(10) && new_grid[y - 1][x] != '.' {
            return Symbol::Some;
        }
    }

    if y > 0 && x < new_grid[y].len() - 1 {
        // check one above right
        if !new_grid[y - 1][x + 1].is_digit(10) && new_grid[y - 1][x + 1] != '.' {
            return Symbol::Some;
        }
    }

    if y > 0 && x > 0 {
        // check one above left
        if !new_grid[y - 1][x - 1].is_digit(10) && new_grid[y - 1][x - 1] != '.' {
            return Symbol::Some;
        }
    }

    if x > 0 {
        // check one before
        if !new_grid[y][x - 1].is_digit(10) && new_grid[y][x - 1] != '.' {
            return Symbol::Some;
        }
    }

    if x < new_grid[y].len() - 1 {
        // check one after
        if !new_grid[y][x + 1].is_digit(10) && new_grid[y][x + 1] != '.' {
            return Symbol::Some;
        }
    }

    if y < new_grid.len() - 1 {
        // check one beneath
        if !new_grid[y + 1][x].is_digit(10) && new_grid[y + 1][x] != '.' {
            return Symbol::Some;
        }
    }

    if y < new_grid.len() - 1 && x < new_grid[y].len() - 1 {
        // check one beneath right
        if !new_grid[y + 1][x + 1].is_digit(10) && new_grid[y + 1][x + 1] != '.' {
            return Symbol::Some;
        }
    }

    if y < new_grid.len() - 1 && x > 0 {
        // check one beneath left
        if !new_grid[y + 1][x - 1].is_digit(10) && new_grid[y + 1][x - 1] != '.' {
            return Symbol::Some;
        }
    }

    // println!("y: {}, x: {}", y, x);
    // println!("New Grid: {:?}", new_grid);

    Symbol::None
}

fn part_one() {
    const FILE_PATH: &str = "data/day_three/data.txt";

    let input = read_file_str(FILE_PATH).unwrap();
    let orig_lines = input.lines();
    let lines = orig_lines.clone();

    let mut machine_parts: Vec<MachinePart> = vec![];

    for (ly, line) in orig_lines.enumerate() {
        let mut num = 0;
        let mut symbol = Symbol::None;
        let mut found_symbol = false;

        // println!("Process line: {}", line);

        for (lx, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                num = num * 10 + char.to_digit(10).unwrap();

                if !found_symbol {
                    symbol = has_symbol_near_by(&lines, lx, ly);
                    if symbol == Symbol::Some {
                        found_symbol = true;
                    }
                }
            } 

            if num > 0 && !char.is_digit(10) {
                // println!("Saving Machine Part: {}", num);
                let machine_part = MachinePart {
                    num,
                    symbol,
                };

                machine_parts.push(machine_part);
                symbol = Symbol::None;
                found_symbol = false;
                num = 0;
            }
        }
    }

    let sum: u32 = machine_parts.into_iter()
        .filter(|part| part.symbol == Symbol::Some)
        .inspect(|machine| println!("Number: {:?}", machine))
        .map(|part| part.num)
        .sum();

    println!("Part One: {}", sum);
}

pub fn run() {
    println!("\nDay 3");
    part_one();
}
