use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    return reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
}

pub fn star_one() -> i32 {
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;

    for line in get_lines("src/day_two/input.txt") {
        let current_command_value: Vec<&str> = line.split_whitespace().collect();
        let cmd = current_command_value[0];
        let val = current_command_value[1].parse::<i32>().unwrap();
        match cmd {
            "forward" => horizontal += val,
            "down" => depth += val,
            "up" => depth -= val,
            _ => println!(
                "Nothing to match here for command {}",
                current_command_value[0]
            ),
        };
    }

    return horizontal * depth;
}

pub fn star_two() -> i32 {
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for line in get_lines("src/day_two/input.txt") {
        let current_command_value: Vec<&str> = line.split_whitespace().collect();
        let cmd = current_command_value[0];
        let val = current_command_value[1].parse::<i32>().unwrap();
        match cmd {
            "forward" => {
                horizontal += val;
                depth += (aim * val)
            }
            "down" => aim += val,
            "up" => aim -= val,
            _ => println!(
                "Nothing to match here for command {}",
                current_command_value[0]
            ),
        };
    }

    return horizontal * depth;
}
