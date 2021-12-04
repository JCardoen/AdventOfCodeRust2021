use std::fs::File;
use std::i32;
use std::io::{BufRead, BufReader, Bytes};
use byteorder::{BigEndian, ReadBytesExt};


fn get_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    return reader.lines().map(|l| l.expect("Could not parse line")).collect();
}

pub fn star_one() -> u32 {
    let vec = get_lines("src/day_three/input.txt");
    let size: usize = vec[0].len();
    let mut counter_zeros_for_each_position: Vec<u32> = vec![0; size];
    let mut counter_ones_for_each_position: Vec<u32> = vec![0; size];
    let ptr_one = &'1';
    let ptr_zero = &'0';

    for line in vec {
        for (index, char) in line.chars().enumerate() {
            match char {
                '1' => counter_ones_for_each_position[index] += 1,
                '0' => counter_zeros_for_each_position[index] += 1,
                _ => println!("Nothing to do here")
            };
        }
    }

    let mut epsilon_rate : Vec<&char> = vec![&'-'; size];
    let mut gamma_rate : Vec<&char> = vec![&'x'; size];

    for (idx, value) in counter_zeros_for_each_position.iter().enumerate() {
        if counter_ones_for_each_position[idx] > *value {
            gamma_rate[idx] = ptr_one;
            epsilon_rate[idx] = ptr_zero;
            continue;
        }

        gamma_rate[idx] = ptr_zero;
        epsilon_rate[idx] = ptr_one;

    }

    let gamma_string: String = gamma_rate.into_iter().collect();
    let epsilon_string: String = epsilon_rate.into_iter().collect();
    // byte string to int let prev_int =
    let actual_gamma_rate = u32::from_str_radix(&*gamma_string, 2).unwrap();
    let actual_epsilon_rate = u32::from_str_radix(&*epsilon_string, 2).unwrap();
    return (actual_gamma_rate * actual_epsilon_rate);

}

pub fn star_two() -> u32 {
    let mut vec = get_lines("src/day_three/input.txt");
    let oxy = filter_values_oxy(vec.clone());
    let scrub = filter_values_scrub(vec.clone());

    let oxy_int = u32::from_str_radix(&*oxy[0], 2).unwrap();
    let scrub_int = u32::from_str_radix(&*scrub[0], 2).unwrap();

    println!("OXY {}", oxy_int);
    println!("scrib {}", scrub_int);

    return oxy_int * scrub_int;
}

fn filter_values_scrub(mut vec: Vec<String>) -> Vec<String> {
    let mut valid_lines: Vec<String> = Vec::new();
    let size = vec[0].len();
    let mut i = 0;

    while i < size {
        if vec.len() == 1 {
            return vec;
        }

        valid_lines = Vec::new();
        let char: char = get_most_common_at_position(&vec, i);

        for line in vec {
            if line.chars().nth(i).unwrap() != char {
                valid_lines.push(line);
            }
        }

        vec = valid_lines;
        valid_lines = Vec::new();

        i += 1;
    }

    return vec;
}

fn filter_values_oxy(mut vec: Vec<String>) -> Vec<String> {
    let mut valid_lines: Vec<String> = Vec::new();
    let size = vec[0].len();
    let mut i = 0;

    while i < size {
        if vec.len() == 1 {
            return vec;
        }

        valid_lines = Vec::new();
        let char: char = get_most_common_at_position(&vec, i);

        for line in vec {
            if line.chars().nth(i).unwrap() == char {
                valid_lines.push(line);
            }
        }

        vec = valid_lines;
        valid_lines = Vec::new();

        i += 1;
    }

    return vec;
}

fn get_most_common_at_position(vec:&Vec<String>, position: usize) -> char {
    let mut ones: u32 = 0;
    let mut zeros: u32 = 0;

    for line in vec.iter() {
        let char = line.chars().nth(position).unwrap();
        match char {
            '1' => ones += 1,
            '0' => zeros += 1,
            _ => println!("Nothing to do here")
        };
    }

    if ones >= zeros {
        return '1';
    }
    return '0';
}