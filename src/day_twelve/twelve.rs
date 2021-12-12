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

#[derive(Clone)]
struct Node {
    name: String,
    edges: Vec<String>,
}

impl Node {
    pub fn new(name: String) -> Node {
        Self {
            name,
            edges: vec![],
        }
    }
    pub fn add_to_edges(edge: &String, node: &mut Node) {
        if !node.edges.contains(edge) {
            node.edges.push(edge.clone());
        }
    }
}

pub fn star_one() -> i32 {
    let input_lines = get_lines("src/day_twelve/input.txt");
    let mut number_of_distinct_paths = 0;
    let mut nodes: HashMap<String, Vec<String>> = HashMap::new();

    for line in input_lines {
        let paths = line
            .split('-')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        nodes
            .entry(paths[0].clone())
            .or_insert(vec![])
            .push(paths[1].clone());
        nodes
            .entry(paths[1].clone())
            .or_insert(vec![])
            .push(paths[0].clone());
    }

    println!("KEYS: {:?}", nodes.keys());
    println!("VALS: {:?}", nodes.values());

    count_paths(
        "start",
        "end",
        &mut number_of_distinct_paths,
        &nodes,
        vec![],
        false,
    );

    return number_of_distinct_paths;
}

pub fn star_two() -> i32 {
    let input_lines = get_lines("src/day_twelve/input.txt");
    let mut number_of_distinct_paths = 0;
    let mut nodes: HashMap<String, Vec<String>> = HashMap::new();

    for line in input_lines {
        let paths = line
            .split('-')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        nodes
            .entry(paths[0].clone())
            .or_insert(vec![])
            .push(paths[1].clone());
        nodes
            .entry(paths[1].clone())
            .or_insert(vec![])
            .push(paths[0].clone());
    }

    count_paths(
        "start",
        "end",
        &mut number_of_distinct_paths,
        &nodes,
        vec![],
        true,
    );

    return number_of_distinct_paths;
}

fn count_paths(
    start: &str,
    end: &str,
    count: &mut i32,
    nodes: &HashMap<String, Vec<String>>,
    mut visited: Vec<String>,
    mut can_visit_twice: bool,
) {
    visited.push(start.to_string());

    if visited
        .iter()
        .filter(|x| x.to_string() == start.to_string())
        .count()
        >= 2
        && start.chars().next().unwrap().is_lowercase()
    {
        can_visit_twice = false;
    }

    if start.to_string() == end.to_string() {
        *count += 1;
    } else {
        let neighbours = nodes.get(&*start.to_string()).unwrap();
        for neighbour in neighbours {
            if *neighbour != "start".to_string() {
                if visited.contains(neighbour)
                    && neighbour.chars().next().unwrap().is_lowercase()
                    && !can_visit_twice
                {
                    continue;
                }
                count_paths(
                    &neighbour,
                    end,
                    count,
                    nodes,
                    visited.clone(),
                    can_visit_twice,
                );
            } else {
                continue;
            }
        }
    }
}
