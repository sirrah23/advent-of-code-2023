use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    compute_lowest_location_number("src/inputs/day05/sample.txt");
}

fn compute_lowest_location_number(filename: &str) -> i32 {
    let mut batch: Vec<String> = Vec::new();
    for input_line in read_to_string(filename).unwrap().lines() {
        if input_line.len() == 0 {
            if let Some(first_line) = batch.first() {
                if first_line.starts_with("seeds:") {
                    let initial_seeds = parse_initial_seeds(first_line);
                    println!("{:?}", initial_seeds);
                } else {
                    let map = Map::new(&batch);
                    println!("Map {:?}", map);
                }
                batch.clear();
            } else {
                batch.clear();
            }
        } else {
            batch.push(input_line.to_string());
        }
    }
    if batch.len() > 0 {
        let map = Map::new(&batch);
        println!("Map {:?}", map);
        batch.clear();
    }
    return 0;
}

fn parse_initial_seeds(seed_line: &String) -> Vec<i32> {
    let mut parsed_seed_values: Vec<i32> = Vec::new();
    let in_seed_values = seed_line.split(' ').into_iter().skip(1);
    for in_seed_value in in_seed_values {
        parsed_seed_values.push(in_seed_value.parse::<i32>().unwrap())
    }
    return parsed_seed_values;
}

#[derive(Debug)]
struct Map {
    name: String,
    hash_map: HashMap<i32, i32>,
}

impl Map {
    fn new(input: &Vec<String>) -> Self {
        let mut hash_map: HashMap<i32, i32> = HashMap::new();
        let first_line: &String = input.first().unwrap();
        let name: String = first_line.split(' ').next().unwrap().to_string();
        for line in input.into_iter().skip(1) {
            let mut numbers_str = line.split_whitespace();
            let destination = numbers_str.next().unwrap().parse::<i32>().unwrap();
            let source = numbers_str.next().unwrap().parse::<i32>().unwrap();
            let range = numbers_str.next().unwrap().parse::<i32>().unwrap();
            for i in 0..range {
                hash_map.insert(source + i, destination + i);
            }
        }
        return Map { name, hash_map };
    }
}
