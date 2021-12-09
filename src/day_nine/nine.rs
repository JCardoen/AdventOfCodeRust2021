use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    return reader.lines().map(|l| l.expect("Could not parse line")).collect();
}

pub fn star_one() -> i32 {
    let input_lines = get_lines("src/day_nine/input.txt");
    let mut grid: Vec<Vec<i32>> = vec![];
    let mut lps: Vec<i32> = vec![];

    for input in input_lines {
        let mut row_vec = vec![];
        for char in input.chars() {
            let nbr = char.to_string().parse().unwrap();
            row_vec.push(nbr);
        }
        grid.push(row_vec)
    }


    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            // Top
            if i == 0 {
                // Left
                if j == 0 {
                    if col < &grid[i + 1][j] && col < &grid[i][j + 1] {
                        lps.push(*col);
                    }
                    continue;
                }
                // Right
                if j == row.len() - 1 {

                    if col < &grid[i + 1][j] && col < &grid[i][j - 1] {
                        lps.push(*col);
                    }
                    continue;
                }

                // Normal
                if col < &grid[i + 1][j] && col < &grid[i][j - 1] && col < &grid[i][j + 1] {
                    lps.push(*col);
                }
                continue;
            }

            // Bottom left
            if i == grid.len() - 1 {
                // Left
                if j == 0 {
                    if col < &grid[i - 1][j] && col < &grid[i][j + 1] {
                        lps.push(*col);
                    }
                    continue;
                }
                // Right
                if j == row.len() - 1 {

                    if col < &grid[i - 1][j] && col < &grid[i][j - 1] {
                        lps.push(*col);
                    }
                    continue;
                }

                // Normal
                if col < &grid[i - 1][j] && col < &grid[i][j - 1] && col < &grid[i][j + 1] {
                    lps.push(*col);
                    continue;
                }
                continue;
            }

            // Left
            if j == 0 {
                if col < &grid[i - 1][j] && col < &grid[i + 1][j] && col < &grid[i][j + 1] {
                    lps.push(*col);
                    continue;
                }
                continue;
            }

            // Right
            if j == row.len() - 1 {

                if col < &grid[i - 1][j] && col < &grid[i + 1][j] && col < &grid[i][j - 1] {
                    lps.push(*col);
                    continue;
                }
                continue;
            }

            // Normal
            if col < &grid[i - 1][j] && col < &grid[i + 1][j] && col < &grid[i][j - 1] && col < &grid[i][j + 1] {
                lps.push(*col);
                continue;
            }
        }
    }

    for point in &lps {
        println!("found point {}", point);
    }

    return lps.iter().map(|x| x + 1).collect::<Vec<i32>>().iter().sum();
}

pub fn star_two() -> i32 {
    fn dfs(i: isize, j: isize, grid: &Vec<Vec<i32>>, visited: &mut HashSet<(isize, isize)>) {
        if i<0 || j<0 || i>= grid.len() as isize || j>= grid[i as usize].len() as isize || visited.contains(&(i, j)) || grid[i as usize][j as usize]==9 {
            return
        }
        visited.insert((i, j));
        dfs(i-1, j, grid, visited);
        dfs(i+1, j, grid, visited);
        dfs(i, j-1, grid, visited);
        dfs(i, j+1, grid, visited);
    }

    let input_lines = get_lines("src/day_nine/input.txt");
    let mut grid: Vec<Vec<i32>> = vec![];
    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    for input in input_lines {
        let mut row_vec = vec![];
        for char in input.chars() {
            let nbr = char.to_string().parse().unwrap();
            row_vec.push(nbr);
        }
        grid.push(row_vec)
    }

    let mut memo = vec![];
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if (!visited.contains(&(i as isize, j as isize))) && grid[i][j]!=9 {
                let prev = visited.len();
                dfs(i as isize, j as isize, &grid, &mut visited);
                memo.push(visited.len() - prev);
            }
        }

    }

    memo.sort();
    let product = memo[memo.len() - 3] * memo[memo.len() - 2] * memo[memo.len() - 1];
    println!("{:?}", memo);
    return product as i32;
}