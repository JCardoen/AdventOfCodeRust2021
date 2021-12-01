/**
199  A
200  A B
208  A B C
210    B C D
200  E   C D
207  E F   D
240  E F G
269    F G H
260      G H
263        H
**/



use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_lines() -> Vec<u32> {

    let filename = "src/day_one/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    return reader.lines().map(|x| x.unwrap().trim().parse::<u32>().unwrap()).collect();
}


pub fn star_one() -> u32 {
    let mut counter: u32 = 0;
    let vec = get_lines();

    for (index, _) in vec.iter().enumerate() {
        if index == 0 {
            continue;
        }

        if vec[index - 1] < vec[index] {
            counter += 1;
        }
    }

    return counter;
}

pub fn start_two() -> u32 {
    let vec = get_lines();
    let mut counter: u32 = 0;
    let mut prev = u32::MAX;

    for window in vec.windows(3) {
        let cur: u32 = window.iter().sum();
        if cur > prev {
            counter += 1;
        }
        prev = cur;
    }

    return counter;
}
