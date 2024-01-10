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
                    batch.clear();
                }
            } else {
                // TODO: Parse out a map data structure here and append to map chain
                batch.clear();
            }
        } else {
            batch.push(input_line.to_string());
        }
    }
    batch.clear();
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
