use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ptr::{hash, null};

fn get_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    return reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
}

fn print_map(map: &Vec<Vec<char>>) {
    for i in 0..map.capacity() {
        for j in 0..map[0].capacity() {
            print!("{} ", map[i][j])
        }
        println!()
    }
}

pub fn solve() -> i32 {
    let input_lines = get_lines("src/day_thirteen/input.txt");
    let mut dots = 0;
    const KNOWN: char = '#';
    const UNKNOWN: char = '.';
    let nb_of_lines = input_lines.len() * 2;
    let mut map: Vec<Vec<char>> = vec![vec![UNKNOWN; nb_of_lines]; nb_of_lines];
    let mut fold_instructions: Vec<(String, i32)> = vec![];
    let is_star_one = false;

    for line in input_lines {
        if line.starts_with("fold") {
            let parse_line = line.replace("fold along ", "");
            let instr = parse_line.split('=').map(|x| x).collect::<Vec<&str>>();
            fold_instructions.push((instr[0].parse().unwrap(), instr[1].parse().unwrap()));
        } else {
            let paths = line
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>();
            map[paths[1] as usize][paths[0] as usize] = KNOWN;
        }
    }

    for i in 0..map.capacity() {
        for j in 0..map[0].capacity() {
            if map[i][j].len_utf8() == 0 {
                map[i][j] = UNKNOWN;
            }
        }
    }

    for instruction in fold_instructions {
        if instruction.0 == "y" {
            let mut folded_row = 1;
            let start_index = instruction.1 + 1 as i32;
            let map_length = map.len() as i32;
            for i in start_index..map_length {
                let real_row_index = instruction.1 - folded_row;
                for j in 0..map[i as usize].len() {
                    let element = map[i as usize][j as usize];
                    if element == KNOWN {
                        map[real_row_index as usize][j as usize] = map[i as usize][j as usize];
                        map[i as usize][j as usize] = UNKNOWN;
                    }
                }
                folded_row += 1;
            }
        }

        if instruction.0 == "x" {
            let mut folded_column = 1;
            let map_length = map.len() as i32;
            let start_index = instruction.1 + 1 as i32;
            let row_length = map[0].len() as i32;

            for i in 0..map_length {
                folded_column = 1;
                for j in start_index..row_length {
                    let real_column_index = instruction.1 - folded_column;

                    if map[i as usize][j as usize] == KNOWN {
                        map[i as usize][real_column_index as usize] = map[i as usize][j as usize];
                        map[i as usize][j as usize] = UNKNOWN;
                    }
                    folded_column += 1;
                }
            }
        }

        if is_star_one {
            for i in &map {
                for j in i {
                    if *j == KNOWN {
                        dots += 1;
                    }
                }
            }
            break;
        }
    }

    println!("After folding");
    print_map(&map.to_owned());

    return dots;
}
