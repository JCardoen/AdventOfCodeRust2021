use std::fs::File;
use std::i32;
use std::io::{BufRead, BufReader, Bytes};
use byteorder::{BigEndian, ReadBytesExt}; // 1.2.7


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