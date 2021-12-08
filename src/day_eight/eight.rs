use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    return reader.lines().map(|l| l.expect("Could not parse line")).collect();
}

fn get_unique_characters(a: &str, b: &str) -> char {
    let mut chars: Vec<char> = a.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    let this = String::from_iter(chars);

    let mut otherchars: Vec<char> = b.chars().collect();
    otherchars.sort_by(|a, b| b.cmp(a));
    let that = String::from_iter(otherchars);


    let this_set: HashSet<char> = this.chars().collect();
    let that_set: HashSet<char> = that.chars().collect();

    // get which one is shorter
    let (shorter, longer) = if this_set.len() > that_set.len() {
        (that, this)
    }  else {
        (this, that)
    };

    // fill the set with the characters from the shorter string
    let set: HashSet<char> = shorter.chars().collect();
    let character = longer.chars().filter(|c| !set.contains(&c)).collect::<Vec<char>>();
    println!("UNique character between {} and {} was {:?}", a,b , character);
    return character[0];
}

pub fn star_one() -> i32 {
    let input_lines = get_lines("src/day_eight/input.txt");
    let mut sum = 0;
    for input in input_lines {
        let mut line = input.split('|').collect::<Vec<&str>>();
        let mut output = line[1].split_whitespace().collect::<Vec<&str>>();

        for digit in output {
            if digit.len() == 2 || digit.len() == 3 || digit.len() == 4 || digit.len() == 7 {
                sum += 1;
            }
        }
    }

    return sum;
}

pub fn star_two() -> i32 {
    let lines = get_lines("src/day_eight/input.txt");
    let mut number_representation: HashMap<String, i32> = HashMap::new();
    let mut sum = 0;
    number_representation.insert("abcefg".to_string(), 0);
    number_representation.insert("cf".to_string(),1);
    number_representation.insert("acdeg".to_string(),2);
    number_representation.insert("acdfg".to_string(), 3);
    number_representation.insert("bcdf".to_string(),4);
    number_representation.insert("abdfg".to_string(), 5);
    number_representation.insert("abdefg".to_string(), 6);
    number_representation.insert("acf".to_string(),7);
    number_representation.insert("abcdefg".to_string(),8);
    number_representation.insert("abcdfg".to_string(), 9);

    for line in lines {
        let mut actual_map : HashMap<char, char> = HashMap::new();
        let current_io = line.split('|').map(|x| x.to_string()).collect::<Vec<String>>();
        let mut coded_input: Vec<String> = current_io[0].split_whitespace().map(|x| x.trim().to_string()).collect::<Vec<String>>();
        coded_input.sort_by(|x, y| x.len().cmp(&y.len()));

        let mut sorted_coded_input: Vec<String> = vec![];

        for digit in coded_input {
            let mut chars: Vec<char> = digit.chars().collect();
            chars.sort_by(|a, b| a.cmp(b));

            let mut string_for_digit = "".to_string();

            for char in chars {
                string_for_digit.push(char);
            }

            sorted_coded_input.push(string_for_digit);
        }

        let mut output = current_io[1].split_whitespace().map(|x| x.trim()).collect::<Vec<&str>>();

        // decoding number_representation
        let encoded_one = sorted_coded_input.get(0).unwrap();
        let encoded_four =  sorted_coded_input.get(2).unwrap();
        let encoded_seven = sorted_coded_input.get(1).unwrap();
        let encoded_eight = sorted_coded_input.get(sorted_coded_input.len() - 1).unwrap();

        let mut encoded_nine = "";
        for input in &sorted_coded_input {
            if input.len() == 6 {
                let mut contains = 0;
                for char in encoded_seven.chars() {
                    if !input.contains(char) {
                        break;
                    }
                    contains+=1;

                }

                for char in encoded_four.chars() {
                    if !input.contains(char) {
                        break;
                    }
                    contains+=1;
                }

                if contains == 7 {
                    encoded_nine = input;
                    break;
                }
            }
        }

        println!("Resolving a");
        let a = get_unique_characters(encoded_one, encoded_seven);
        println!("Resolving g");
        let g = get_unique_characters(encoded_nine, (&*(encoded_four.to_owned() + encoded_seven)));
        println!("Resolving e");
        let e = get_unique_characters(encoded_nine, encoded_eight);

        let mut encoded_zero = "";
        for input in &sorted_coded_input {
            if input.len() == 6 && input.contains(e) {
                let mut contains_all_from_one = 0;
                for char in encoded_one.chars() {
                    if !input.contains(char) {
                        break;
                    }

                    contains_all_from_one += 1;
                }

                if contains_all_from_one == 2 {
                    encoded_zero = input;
                    break;
                }

            }
        }

        let d = get_unique_characters(encoded_zero, encoded_eight);

        // a g d e
        let mut encoded_two = "";
        for input in &sorted_coded_input {
            if input.len() == 5 && input.contains(e) {
                encoded_two = input;
                break;
            }
        }

        // a c d e g
        println!("Resolving c");
        let c = get_unique_characters(encoded_two, &String::from_iter([a, g, d, e]));
        println!("Resolving f");
        let f = get_unique_characters(encoded_one, &String::from_iter([c]));
        println!("Resolving b");
        let b = get_unique_characters(encoded_four, &String::from_iter([c,d,f]));

        actual_map.insert(a, 'a');
        actual_map.insert(b, 'b');
        actual_map.insert(c, 'c');
        actual_map.insert(d, 'd');
        actual_map.insert(e, 'e');
        actual_map.insert(f, 'f');
        actual_map.insert(g, 'g');


        for (k, v) in actual_map.iter() {
            println!("{} => {}", k, v);
        }

        let mut output_number = "".to_string();
        for digit in output {
            let mut decoded_digit: String = "".to_string();
            println!("Decoding: {}", digit);

            for code in digit.chars().collect::<Vec<char>>() {
                let entry = actual_map.get(&code).unwrap();
                decoded_digit.push(*entry);
            }

            println!("Decoded: {}", decoded_digit);

            let mut decoded_digit_chars: Vec<char> = decoded_digit.chars().collect();
            decoded_digit_chars.sort_by(|a, b| a.cmp(b));
            decoded_digit = String::from_iter(decoded_digit_chars);
            println!("Decoded sorted: {}", decoded_digit);

            let number_of_digit = number_representation.get(&decoded_digit).unwrap().to_string();
            output_number.push(number_of_digit.parse().unwrap());
            println!("Decoded digit: {}", number_of_digit);
        }

        sum += output_number.parse::<i32>().unwrap();
    }


    return sum;
}