use std::cmp::min;
use std::collections::{HashMap, HashSet, VecDeque};
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
    let input_lines = get_lines("src/day_fifteen/input.txt");
    let mut grid: Vec<Vec<usize>> = vec![];

    for line in input_lines {
        let mut vector_for_line: Vec<usize> = vec![];

        for ch in line.chars() {
        
            vector_for_line.push(ch.to_string().parse::<usize>().unwrap());
        }

        grid.push(vector_for_line.clone());
    }

    let grid_size = grid.len();

    let mut costs = vec![vec![usize::MAX;grid_size];grid_size];
    let mut neighbours: Vec<(usize, usize)> = vec![];
    costs[0][0] = 0;
    
    let mut queue = VecDeque::new();

    queue.push_front((0, 0));

    while queue.len() > 0 {
        let current = queue.pop_front().unwrap();
        println!("Determining cost of {} {}", current.0, current.1);
        let neighbours = find_neighbours(current.0, current.1, grid_size);

        for neighbour in neighbours {
            let cost_neighbour = costs[neighbour.0][neighbour.1];
            let cost_to_reach_neighbour = costs[current.0][current.1] + grid[neighbour.0][neighbour.1];

            if cost_neighbour > cost_to_reach_neighbour {
                costs[neighbour.0][neighbour.1] = cost_to_reach_neighbour;
                queue.push_back(neighbour);
            }
        }
    }

    return costs[grid_size-1][grid_size-1];
}


fn find_neighbours(current_i: usize, current_j: usize, grid_size: usize) -> Vec<(usize, usize)> {
    let mut positions_to_check: Vec<(usize, usize)> = vec![];
    if current_i > 0 {
        positions_to_check.push((current_i-1, current_j));
    }
    if current_i < grid_size-1 {
        positions_to_check.push((current_i+1, current_j));
    }

    if current_j > 0 {
        positions_to_check.push((current_i, current_j-1));
    }

    if current_j < grid_size-1 {
        positions_to_check.push((current_i, current_j+1));
    }

    return positions_to_check;
}
