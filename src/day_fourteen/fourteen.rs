use std::collections::{HashMap, VecDeque};
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

pub fn star_one() -> usize {
    let input_lines = get_lines("src/day_fourteen/input.txt");
    let mut polymer_template = input_lines[0].clone();
    let mut insertion_rules: HashMap<String, String> = HashMap::new();
    for line in input_lines.clone() {
        if line.contains(" -> ") {
            let parts = line
                .split(" -> ")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            insertion_rules.insert(parts[0].clone(), parts[1].clone());
        }
    }

    let mut step_result = "".to_string();
    for pass in 0..10 {
        for (i, _) in polymer_template.clone().chars().enumerate() {
            let pair = format!(
                "{}{}",
                polymer_template.chars().nth(i).unwrap(),
                polymer_template.chars().nth(i + 1).unwrap_or('_')
            );
            step_result.push(polymer_template.chars().nth(i).unwrap());

            for (rule, res) in &insertion_rules {
                if *rule == pair {
                    step_result += &*res;
                }
            }
        }

        polymer_template = step_result.clone();
        step_result = "".to_string();
        println!("{}", step_result);
    }


    let mut char_counter: HashMap<char, i32> = HashMap::new();

    for character in polymer_template.to_owned().chars() {
        *char_counter.entry(character).or_insert(0) += 1;
    }

    for (key, val) in char_counter {
        println!{"{} {}", key, val};
    }

    return 0;
}

pub fn star_two() -> usize {
    let input_lines = get_lines("src/day_fourteen/input.txt");
    let mut polymer_template = input_lines[0].clone();
    let mut insertion_rules: HashMap<String, String> = HashMap::new();
    let mut pair_counter: HashMap<(char, char), usize> = HashMap::new();

    for line in input_lines.clone() {
        if line.contains(" -> ") {
            let parts = line
                .split(" -> ")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            insertion_rules.insert(parts[0].clone(), parts[1].clone());
        }
    }

    for (_, pair) in polymer_template.clone().chars().collect::<Vec<char>>().windows(2).enumerate() {
        *pair_counter.entry((pair[0], pair[1])).or_insert(0) += 1;
    }

    for (key, val) in pair_counter.clone() {
        println!{"{} {} {}", key.0, key.1, val};
    }

    //SHHBNFBCKNHCNOSHHVFF

    for pass in 0..40 {
        let mut intermediary_counter: HashMap<(char, char), usize> = HashMap::new();

        for (p1, count) in pair_counter  {
            let mut did_rule = false;
            for (rule, res) in &insertion_rules {
                if *rule == format!(
                    "{}{}",
                    p1.0,
                    p1.1
                ) {
                    *intermediary_counter.entry((p1.0, res.chars().nth(0).unwrap())).or_insert(0) += count;
                    *intermediary_counter.entry((res.chars().nth(0).unwrap(), p1.1)).or_insert(0) += count;
                    did_rule = true;
                }
            }


        }
        pair_counter = intermediary_counter;
    }


    let mut char_counter: HashMap<char, usize> = HashMap::new();

    for (p1, count) in pair_counter {
        *char_counter.entry(p1.0).or_insert(0) += count;
    }

    let mut count_vec: Vec<(&char, &usize)> = char_counter.iter().collect();

    for i in 0..count_vec.len() {
        println!{"{} {}", count_vec[i].0, count_vec[i].1};
    }


    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    println!{"{} {}", count_vec[0].1, count_vec[count_vec.len()-1].1};

    return count_vec[0].1 - count_vec[count_vec.len()-1].1 - 1; // minus 1 because last character is counted twice
}
