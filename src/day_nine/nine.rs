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