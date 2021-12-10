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

pub fn star_one() -> i32 {
    let input_lines = get_lines("src/day_ten/input.txt");

    let mut hash_chars = HashMap::new();
    hash_chars.insert(']', '[');
    hash_chars.insert(')', '(');
    hash_chars.insert('}', '{');
    hash_chars.insert('>', '<');

    let mut syntax_errors: Vec<char> = vec![];

    'outer: for input in input_lines {
        let mut opening_characters: Vec<char> = vec![];
        for char in input.chars() {
            if hash_chars.values().any(|&x| x == char) {
                opening_characters.push(char);
            }

            // let's try to match the closing character
            if hash_chars.contains_key(&char) {
                // check last opening_characters character
                if opening_characters.last().unwrap() != hash_chars.get(&char).unwrap() {
                    syntax_errors.push(char);
                    println!(
                        "Found not matching {} with closing character {}",
                        opening_characters.last().unwrap(),
                        &char
                    );
                    continue 'outer;
                } else {
                    opening_characters.pop();
                }
            }
        }

        println!("Syntax errors: {:?}", syntax_errors);
    }

    let mut score = 0;
    for err in syntax_errors {
        match err {
            ')' => score += 3,
            ']' => score += 57,
            '}' => score += 1197,
            '>' => score += 25137,
            _ => {}
        }
    }

    return score;
}

pub fn star_two() -> u64 {
    let input_lines = get_lines("src/day_ten/input.txt");
    let mut hash_chars = HashMap::new();
    hash_chars.insert(']', '[');
    hash_chars.insert(')', '(');
    hash_chars.insert('}', '{');
    hash_chars.insert('>', '<');
    let mut scores: Vec<u64> = vec![];

    'outer: for input in input_lines {
        let mut score: u64 = 0;
        let mut opening_characters: Vec<char> = vec![];
        for char in input.chars() {
            if hash_chars.values().any(|&x| x == char) {
                opening_characters.push(char);
            }

            // Closing character
            if hash_chars.contains_key(&char) {
                // It is a corrupt line
                if opening_characters.last().unwrap() != hash_chars.get(&char).unwrap() {
                    continue 'outer;
                } else {
                    opening_characters.pop();
                }
            }
        }

        for opening_character in opening_characters.iter().rev() {
            let matching_key = hash_chars
                .iter()
                .find_map(|(key, &val)| {
                    if val == *opening_character {
                        Some(key)
                    } else {
                        None
                    }
                })
                .unwrap();
            score *= 5;
            match matching_key {
                ')' => score += 1,
                ']' => score += 2,
                '}' => score += 3,
                '>' => score += 4,
                _ => {}
            }
        }

        scores.push(score);
        println!();
    }

    scores.sort();
    let mid = scores.len() / 2;

    return scores[mid];
}
