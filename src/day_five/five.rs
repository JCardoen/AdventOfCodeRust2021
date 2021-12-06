use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Point {
    x: i32,
    y: i32
}

fn get_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    return reader.lines().map(|l| l.expect("Could not parse line")).collect();
}

pub fn star_one() -> i32 {
    let lines = get_lines("src/day_five/input.txt");
    let size = lines.len() * 2;
    let mut hashmap_for_points: HashMap<i32, Vec<i32>> = HashMap::new();
    // x1,y1 -> x2,y2
    for line in lines {
        let line_parts = line.split("->").collect::<Vec<&str>>();;

        let coords_first = line_parts[0].split(',').map(|el| el.trim().parse().unwrap()).collect::<Vec<i32>>();;
        let coords_second = line_parts[1].split(',').map(|el| el.trim().parse().unwrap()).collect::<Vec<i32>>();;

        let p = Point { x: coords_first[0], y: coords_first[1] };
        let p2 = Point { x: coords_second[0], y: coords_second[1] };

        println!("{}", line);


        // vertical
        if p.x == p2.x {
            println!("Vertical");

            for idx in min(p.y, p2.y)..max(p.y, p2.y)+1 {
                let mut new_vec = vec![0;size];
                let mut entry = hashmap_for_points.entry(idx).or_insert(new_vec);
                entry[p.x as usize] += 1;
                println!("Incrementing {} {} with 1", p.x, idx);
            }

            continue;
        }

        // horizontal
        if p.y == p2.y {
            println!("Horizontal");
            let mut new_vec = vec![0;size];
            let mut entry = hashmap_for_points.entry(p.y).or_insert(new_vec);

            for idx in min(p.x, p2.x)..max(p.x, p2.x)+1 {
                println!("Incrementing {} {} with 1", idx, p.y);

                entry[idx as usize] += 1;
            }

            continue;
        }

        println!("Diagonal doing nothing");

    }

    let mut count = 0;
    for (idx, value) in hashmap_for_points {
        let filtered = value.iter().filter(|x| x > &&1);
        count += filtered.count();
    }

    return count as i32;
}

pub fn star_two() -> i32 {
    let lines = get_lines("src/day_five/sample.txt");
    let size = lines.len() * 2;
    let mut hashmap_for_points: HashMap<i32, Vec<i32>> = HashMap::new();
    // x1,y1 -> x2,y2
    for line in lines {
        let line_parts = line.split("->").collect::<Vec<&str>>();;

        let coords_first = line_parts[0].split(',').map(|el| el.trim().parse().unwrap()).collect::<Vec<i32>>();;
        let coords_second = line_parts[1].split(',').map(|el| el.trim().parse().unwrap()).collect::<Vec<i32>>();;

        let p = Point { x: coords_first[0], y: coords_first[1] };
        let p2 = Point { x: coords_second[0], y: coords_second[1] };

        println!("{}", line);


        // vertical
        if p.x == p2.x {
            println!("Vertical");

            for idx in min(p.y, p2.y)..max(p.y, p2.y)+1 {
                let mut new_vec = vec![0;size];
                let mut entry = hashmap_for_points.entry(idx).or_insert(new_vec);
                entry[p.x as usize] += 1;
                println!("Incrementing {} {} with 1", p.x, idx);
            }

            continue;
        }

        // horizontal
        if p.y == p2.y {
            println!("Horizontal");
            let mut new_vec = vec![0;size];
            let mut entry = hashmap_for_points.entry(p.y).or_insert(new_vec);

            for idx in min(p.x, p2.x)..max(p.x, p2.x)+1 {
                println!("Incrementing {} {} with 1", idx, p.y);

                entry[idx as usize] += 1;
            }
            continue;
        }

        // Diagonal

        let slope = (p2.y - p.y)/(p2.x - p.x);

        if slope == 1 {
            println!("Diagonal");
            let mut y_trav = min(p.y, p2.y);
            let mut x_trav = min(p.x, p2.x);
            while y_trav <= max(p.y, p2.y){
                let mut new_vec = vec![0;size];
                let mut entry = hashmap_for_points.entry(y_trav).or_insert(new_vec);
                entry[x_trav as usize] += 1;
                println!("Incrementing {} {} with 1", x_trav, y_trav);

                x_trav += 1;
                y_trav += 1;
            }
        }

        if slope == -1 {
            println!("Diagonal traverse backwards");
            let mut y_trav = max(p.y, p2.y);
            let mut x_trav = min(p.x, p2.x);
            while y_trav >= min(p.y, p2.y){
                let mut new_vec = vec![0;size];
                let mut entry = hashmap_for_points.entry(y_trav).or_insert(new_vec);
                entry[x_trav as usize] += 1;
                println!("Incrementing {} {} with 1", x_trav, y_trav);

                x_trav += 1;
                y_trav -= 1;
            }
        }
    }

    let mut count = 0;
    for (idx, value) in hashmap_for_points {
        let filtered = value.iter().filter(|x| x > &&1);
        count += filtered.count();
    }

    return count as i32;
}