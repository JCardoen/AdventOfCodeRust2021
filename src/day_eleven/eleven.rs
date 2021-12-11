use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ptr::hash;

fn get_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    return reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
}

fn reset_octopus_flashers(grid: &mut Vec<Vec<i32>>) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] >= 10 {
                grid[i][j] = 0;
            }
        }
    }
}

fn increase_neighbors(grid: &mut Vec<Vec<i32>>, i: i32, j: i32, flashed_for_current_step: &mut Vec<(usize, usize)>) -> i32 {
    let mut rows_to_check:Vec<i32> = vec![];
    let mut cols_to_check:Vec<i32> = vec![];
    let idx_last_row = (grid.len() - 1) as i32;
    let idx_last_col = (grid[0].len() - 1) as i32;
    rows_to_check.push(i);
    cols_to_check.push(j);
    let mut causal_flashes = 0;

    match i {
        0 => rows_to_check.push(1), // First row
        i if i == idx_last_row => rows_to_check.push(i-1), // Last row
        _ => {rows_to_check.push(i+1); rows_to_check.push(i-1);} // default behaviour: check top and bottom
    }

    match j {
        0 => {
            cols_to_check.push(1);
        }, // First col
        j if j == idx_last_col => {
            cols_to_check.push(j-1);
        }, // last col
        _ => {
            cols_to_check.push(j+1);
            cols_to_check.push(j-1);
        }
    }

    for row in rows_to_check.iter().map(|x| *x as usize).collect::<Vec<usize>>() {
        for col in cols_to_check.iter().map(|x| *x as usize).collect::<Vec<usize>>() {
            grid[row][col] += 1;

            // If it is flashing but didnt flash yet
            if grid[row][col] > 9 && !flashed_for_current_step.contains(&(row, col)) {
                flashed_for_current_step.push((row, col));
                causal_flashes += 1;
                causal_flashes += increase_neighbors(grid, row as i32, col as i32, flashed_for_current_step);
            }
        }
    }

    return causal_flashes;
}

fn traverse_octopus_grid(grid: &mut Vec<Vec<i32>>, mut flashed_for_current_step: Vec<(usize, usize)>) -> i32{
    let mut flashes_in_this_step = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            // Increase current octopus by 1
            grid[i][j] += 1;
        }
    }

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            // this one is flashing but we cant flash twice
            if grid[i][j] > 9 {
                if !flashed_for_current_step.contains(&(i, j)) {
                    flashes_in_this_step += 1;
                    flashed_for_current_step.push((i, j));
                    flashes_in_this_step += increase_neighbors(grid, i as i32, j as i32, &mut flashed_for_current_step);
                }
            }
        }
    }

    return flashes_in_this_step;
}

fn print_grid(octopus_grid: &Vec<Vec<i32>>) {
    for i in 0..octopus_grid.len() {
        for j in 0..octopus_grid.len() {
            print!("{}\t", octopus_grid[i][j]);
        }
        println!();
    }
    println!();
    println!();
    println!();
}

pub fn star_one() -> i32 {
    let input_lines = get_lines("src/day_eleven/input.txt");
    let mut octopus_grid: Vec<Vec<i32>> = vec![];
    let mut number_of_flashes = 0;
    for line in input_lines {
        let mut octopus_row = vec![];
        for char in line.chars() {
            octopus_row.push(char.to_string().parse().unwrap());
        }

        octopus_grid.push(octopus_row);
    }
    print_grid(&octopus_grid);
    for i in 0..100 {
        println!("STEP {}", i);
        println!("-----------------------------------------------------------------");
        let mut flashed_for_current_step: Vec<(usize, usize)> = vec![];
        number_of_flashes += traverse_octopus_grid(&mut octopus_grid, flashed_for_current_step);
        reset_octopus_flashers(&mut octopus_grid);
        print_grid(&octopus_grid);
    }

    return number_of_flashes;
}

fn simultaneous_flash(octopus_grid: &Vec<Vec<i32>>) {

}

pub fn star_two() -> i32 {
    let input_lines = get_lines("src/day_eleven/sample.txt");
    let mut octopus_grid: Vec<Vec<i32>> = vec![];
    let mut number_of_flashes = 0;
    for line in input_lines {
        let mut octopus_row = vec![];
        for char in line.chars() {
            octopus_row.push(char.to_string().parse().unwrap());
        }

        octopus_grid.push(octopus_row);
    }

    let mut step_all_zero = 0;

    // BRUTE FORCE IT - CBA TO THINK
    for i in 0..500 {
        let mut flashed_for_current_step: Vec<(usize, usize)> = vec![];
        number_of_flashes += traverse_octopus_grid(&mut octopus_grid, flashed_for_current_step);
        reset_octopus_flashers(&mut octopus_grid);

        if octopus_grid.iter().flatten().sum::<i32>() == 0 {
            println!("All 0 at step: {}", i+1);
            step_all_zero = i + 1;
            break;
        }
    }

    return step_all_zero;
}