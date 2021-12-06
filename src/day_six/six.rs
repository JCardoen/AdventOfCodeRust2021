use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn get_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    return reader.lines().map(|l| l.expect("Could not parse line")).collect();
}

/*
pub fn exponential_approach_didnt_work() -> i128 {
    let lines = get_lines("src/day_six/sample.txt");
    // initial population is 5

    let number_of_days = 18;
    let mut fish_pop_mtx = 0;
    for number in lines[0].split(',') {
        let fish_state: i32 =  number.parse().unwrap();
        let actual_days = number_of_days - fish_state;
        let fish_producing_fish = (1.0/(7.0 + fish_state as f32) + 1.0).powi(actual_days) as i128;
        println!("Checking fish initial state: {} and he produced {} other fishes", fish_state, fish_producing_fish);
        fish_pop_mtx += fish_producing_fish;

    }
    // For each fish:
    // population = (1+growth_factor)^t
    // since we are considering a fish grows 1/7 per day
    // e.g. 3
    return fish_pop_mtx;
}*/

pub fn star_one_and_two() -> u128 {
    let start = Instant::now();
    let number_of_days: u16 = 256;

    // Inspired by https://doc.rust-lang.org/book/ch16-03-shared-state.html
    let lines = get_lines("src/day_six/input.txt");
    let initial_fishes:Vec<u8> = lines[0].split(',').map(|fish| fish.parse().unwrap()).collect::<Vec<u8>>();
    let fishes_to_compute: Vec<u8> = lines[0].split(',').map(|fish| fish.parse().unwrap()).collect::<HashSet<_>>().into_iter().collect::<Vec<u8>>();

    // Shared across threads
    let fish_pop_mtx = Arc::new(Mutex::new(0 as u128));
    let fish_state_mtx: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(vec![0;8]));

    for fish in initial_fishes {
        let mut fishes_state = fish_state_mtx.lock().unwrap();
        fishes_state[fish as usize] += 1;
    }

    let mut threads = vec![];

    // Only compute necessary fish

    // Fork the threads
    for fish in fishes_to_compute {
        // Clones for threads (addresses)
        let fish_pop_mtx = Arc::clone(&fish_pop_mtx);
        let fish_state_mtx = Arc::clone(&fish_state_mtx);

        let thread = thread::spawn(move || {
            let mut vector_for_fish: Vec<u8> = vec![];
            vector_for_fish.push(fish);
            for _ in 1..number_of_days+1 {
                for idx in 0..vector_for_fish.len() {
                    if vector_for_fish[idx] == 0 {
                        vector_for_fish[idx] = 6;
                        vector_for_fish.push(8);
                    } else {
                        vector_for_fish[idx] -= 1;
                    }
                }
            }
            let mut current_sum = fish_pop_mtx.lock().unwrap();
            let mut fish_count = fish_state_mtx.lock().unwrap();
            *current_sum += (vector_for_fish.len() as u128) * (fish_count[fish as usize] as u128);
            drop(vector_for_fish);
        });
        threads.push(thread);
    }

    // Now join them
    for thread in threads {
        thread.join().unwrap();
    }

    println!("{:.2?} processing time threads", start.elapsed());

    return *fish_pop_mtx.lock().unwrap();
}