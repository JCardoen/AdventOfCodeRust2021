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

pub fn star_one() -> usize {
    let input_lines = get_lines("src/day_fifteen/sample.txt");
    let mut grid: Vec<Vec<usize>> = vec![];

    for line in input_lines {
        let mut vector_for_line: Vec<usize> = vec![];

        for ch in line.chars() {
        
            vector_for_line.push(ch.to_string().parse::<usize>().unwrap());
        }

        grid.push(vector_for_line.clone());
    }

    let mut path_cost: usize = 0;
    let mut reached_end = false;
    let mut current_i: usize = 0;
    let mut current_j: usize = 0;
    let mut visited: Vec<(usize, usize)> = vec![];
    visited.push((0 as usize, 0 as usize));
    while !reached_end {
        path_cost += grid[current_i][current_j];
        println!("Current position: {} {}", current_i, current_j);
        let new_pos = set_next_position(&grid, current_i, current_j, &visited);
        current_i = new_pos.0;
        current_j = new_pos.1;        
        visited.push((current_i, current_j));

        if current_i == grid.len()-1 && current_j == grid.len()-1 {
            reached_end = true;
        }
    }

    return path_cost;
}

fn set_next_position(grid: &Vec<Vec<usize>>, mut current_i: usize, mut current_j: usize, visited: &Vec<(usize,usize)>) -> (usize, usize) {
    let mut positions_to_check: Vec<(usize, usize)> = vec![];

    if current_i == 0 {
        if current_j == 0 {
            positions_to_check.push((current_i+1, current_j));
            positions_to_check.push((current_i, current_j+1));
        }

        else if current_j == grid.len()-1 {
            positions_to_check.push((current_i+1, current_j));
            positions_to_check.push((current_i, current_j-1));
        }

        else {
            positions_to_check.push((current_i+1, current_j));
            positions_to_check.push((current_i, current_j-1));
            positions_to_check.push((current_i, current_j+1));
        }
    }
    else if current_i == grid.len()-1 {
    
        if current_j == 0 {
            positions_to_check.push((current_i-1, current_j));
            positions_to_check.push((current_i, current_j+1));
        }

        else if current_j == grid.len()-1 {
            positions_to_check.push((current_i, current_j-1));
            positions_to_check.push((current_i-1, current_j));
        } 
        else {
            positions_to_check.push((current_i-1, current_j));
            positions_to_check.push((current_i, current_j-1));
            positions_to_check.push((current_i, current_j+1));
        }
    }
    else {

        if current_j == 0 {
             
            positions_to_check.push((current_i-1, current_j));
            positions_to_check.push((current_i+1, current_j));
            positions_to_check.push((current_i, current_j+1));
        }

        else if current_j == grid.len()-1 {
            positions_to_check.push((current_i-1, current_j));
            positions_to_check.push((current_i+1, current_j));
            positions_to_check.push((current_i, current_j-1));
        } 

        else {
        
            positions_to_check.push((current_i-1, current_j));
            positions_to_check.push((current_i+1, current_j));
            positions_to_check.push((current_i, current_j-1));
            positions_to_check.push((current_i, current_j+1));
        }
    }
    let mut min: usize = usize::MAX;
    for (i, j) in positions_to_check.iter() {
        if !visited.contains(&(*i,*j)) {

            if grid[*i][*j] <= min {
                min = grid[*i][*j];
                current_i = *i;
                current_j = *j;
            }
        }
    }
    println!("Next position should be {} {}", current_i, current_j);
    return (current_i, current_j);
}
