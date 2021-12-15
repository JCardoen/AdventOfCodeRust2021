use std::collections::VecDeque;
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

pub fn star_two() -> usize {
    let input_lines = get_lines("src/day_fifteen/input.txt");
    let mut grid: Vec<Vec<usize>> = vec![];

    for line in input_lines {
        let mut vector_for_line: Vec<usize> = vec![];

        for ch in line.chars() {
            vector_for_line.push(ch.to_string().parse::<usize>().unwrap());
        }

        let initial_vec = vector_for_line.clone();

        for i in 0..4 {
            let mut vector_copy = initial_vec.clone();

            vector_copy = vector_copy
                .iter_mut()
                .map(|x| {
                    if (*x + i + 1) > 9 {
                        *x + 1 + i - 9
                    } else {
                        *x + 1 + i
                    }
                })
                .collect::<Vec<usize>>();
            vector_for_line.append(&mut (vector_copy.clone()));
        }

        grid.push(vector_for_line);
    }

    for i in 0..4 {
        for origin_row in 0..100 {
            let mut original_vec = grid[origin_row].clone();
            let new_vec = original_vec
                .iter_mut()
                .map(|x| {
                    if (*x + i + 1) > 9 {
                        *x + 1 + i - 9
                    } else {
                        *x + 1 + i
                    }
                })
                .collect::<Vec<usize>>();
            grid.push(new_vec.clone());
        }
    }

    let grid_size = grid.len();
    assert_eq!(grid_size, 500);
    let mut costs = vec![vec![usize::MAX; grid_size]; grid_size];
    costs[0][0] = 0;

    let mut queue = VecDeque::new();

    queue.push_front((0, 0));

    while queue.len() > 0 {
        let current = queue.pop_front().unwrap();
        let neighbours = find_neighbours(current.0, current.1, grid_size);

        for neighbour in neighbours {
            let cost_neighbour = costs[neighbour.0][neighbour.1];
            let cost_to_reach_neighbour =
                costs[current.0][current.1] + grid[neighbour.0][neighbour.1];

            if cost_neighbour > cost_to_reach_neighbour {
                costs[neighbour.0][neighbour.1] = cost_to_reach_neighbour;
                queue.push_back(neighbour);
            }
        }
    }

    return costs[grid_size - 1][grid_size - 1];
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

    let mut costs = vec![vec![usize::MAX; grid_size]; grid_size];
    costs[0][0] = 0;

    let mut queue = VecDeque::new();

    queue.push_front((0, 0));

    while queue.len() > 0 {
        let current = queue.pop_front().unwrap();
        let neighbours = find_neighbours(current.0, current.1, grid_size);

        for neighbour in neighbours {
            let cost_neighbour = costs[neighbour.0][neighbour.1];
            let cost_to_reach_neighbour =
                costs[current.0][current.1] + grid[neighbour.0][neighbour.1];

            if cost_neighbour > cost_to_reach_neighbour {
                costs[neighbour.0][neighbour.1] = cost_to_reach_neighbour;
                queue.push_back(neighbour);
            }
        }
    }

    return costs[grid_size - 1][grid_size - 1];
}

fn find_neighbours(current_i: usize, current_j: usize, grid_size: usize) -> Vec<(usize, usize)> {
    let mut positions_to_check: Vec<(usize, usize)> = vec![];
    if current_i > 0 {
        positions_to_check.push((current_i - 1, current_j));
    }
    if current_i < grid_size - 1 {
        positions_to_check.push((current_i + 1, current_j));
    }

    if current_j > 0 {
        positions_to_check.push((current_i, current_j - 1));
    }

    if current_j < grid_size - 1 {
        positions_to_check.push((current_i, current_j + 1));
    }

    return positions_to_check;
}
