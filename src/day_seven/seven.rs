use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_lines(filename: &str) -> Vec<i32> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect::<Vec<String>>();
    return lines[0]
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();
}

pub fn star_one() -> i32 {
    // Here we can simply use the median value
    let mut crabs = get_lines("src/day_seven/input.txt");
    crabs.sort();
    let median = crabs[(crabs.len() - 1) / 2];
    let mut fuel_cost = 0;
    for crab in crabs {
        let cost_for_crab = crab - median;
        fuel_cost += cost_for_crab.abs();
    }

    return fuel_cost;
}

pub fn star_two() -> f64 {
    // Based on the released paper: https://www.reddit.com/r/adventofcode/comments/rawxad/2021_day_7_part_2_i_wrote_a_paper_on_todays/
    let mut crabs = get_lines("src/day_seven/input.txt");
    let crabs = crabs.iter().map(|s| *s as f64).collect::<Vec<f64>>();

    let mut fuel_cost_low = 0.0;
    let mut fuel_cost_high = 0.0;

    // now we know that actually the fuel consumption is normally distributed
    let sum_of_crabs: f64 = crabs.iter().sum();
    println!("Sum crabs: {}", sum_of_crabs);

    let mean_crab_position_low = (sum_of_crabs / crabs.len() as f64).floor();
    let mean_crab_position_high = (sum_of_crabs / crabs.len() as f64).ceil();

    for crab in crabs {
        let mut x = (crab - mean_crab_position_low).abs();
        let mut y = (crab - mean_crab_position_high).abs();

        x = x * (x + 1.0) / 2.0;
        y = y * (y + 1.0) / 2.0;
        fuel_cost_low += x;
        fuel_cost_high += y;
    }

    println!("{:.8} {:.8}", fuel_cost_low, fuel_cost_high);

    return fuel_cost_low.min(fuel_cost_high).ceil();
}
