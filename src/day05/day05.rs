// use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::read_to_string;

fn main() {
    println!(
        "Sample Part 1: {}",
        compute_lowest_location_number("src/inputs/day05/sample.txt", false)
    );
    println!(
        "Sample Part 2: {}",
        compute_lowest_location_number("src/inputs/day05/sample.txt", true)
    );
    println!(
        "Input Part 1: {}",
        compute_lowest_location_number("src/inputs/day05/input.txt", false)
    );
    println!(
        "Input Part 2: {}",
        compute_lowest_location_number("src/inputs/day05/input.txt", true)
    );
}

fn compute_lowest_location_number(filename: &str, seed_range_flag: bool) -> i64 {
    let mut batch: Vec<String> = Vec::new();
    let mut mc: MapChain = MapChain::new();
    let mut initial_seeds: Option<Vec<i64>> = None;
    for input_line in read_to_string(filename).unwrap().lines() {
        if input_line.len() == 0 {
            if let Some(first_line) = batch.first() {
                if first_line.starts_with("seeds:") {
                    initial_seeds = Some(parse_initial_seeds(first_line));
                } else {
                    let map = Map::new(&batch);
                    mc.append(map);
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
        mc.append(map);
        batch.clear();
    }
    let mut answer = i64::MAX;
    match initial_seeds {
        Some(s) => {
            if seed_range_flag {
                let mut sb = SeedBag::new(s);
                loop {
                    let next_seed_value = sb.next();
                    if next_seed_value == -1 {
                        break;
                    }
                    let result = mc.follow_chain(next_seed_value);
                    if result < answer {
                        answer = result;
                    }
                }
            } else {
                for seed in s {
                    let result = mc.follow_chain(seed);
                    if result < answer {
                        answer = result
                    }
                }
            }
        }
        None => panic!("something has gone wrong..."),
    }
    return answer;
}

fn parse_initial_seeds(seed_line: &String) -> Vec<i64> {
    let mut parsed_seed_values: Vec<i64> = Vec::new();
    let in_seed_values = seed_line.split(' ').into_iter().skip(1);
    for in_seed_value in in_seed_values {
        parsed_seed_values.push(in_seed_value.parse::<i64>().unwrap())
    }
    return parsed_seed_values;
}

struct SeedBag {
    seed_values: VecDeque<i64>,
    start: i64,
    range: i64,
    ptr: i64,
}

impl SeedBag {
    fn new(seed_values: Vec<i64>) -> Self {
        SeedBag {
            seed_values: VecDeque::from(seed_values),
            start: -1,
            range: -1,
            ptr: 0,
        }
    }

    // TODO: Instead of iterating through each number one-by-one probably makes more sense to work
    // with the ranges directly somehow...
    // OR, reverse map from location to seed
    fn next(&mut self) -> i64 {
        if (self.ptr > self.range) && (self.seed_values.len() == 0) {
            return -1;
        }
        if (self.start == -1 && self.range == -1) || (self.ptr > self.range) {
            self.start = self.seed_values.pop_front().unwrap();
            self.range = self.seed_values.pop_front().unwrap();
            self.ptr = 0;
        }
        let result = self.start + self.ptr;
        self.ptr += 1;
        return result;
    }
}

#[derive(Debug)]
struct Map {
    name: String,
    entries: Vec<MapEntry>,
}

#[derive(Debug)]
struct MapEntry {
    source: i64,
    destination: i64,
    range: i64,
}

impl Map {
    fn new(input: &Vec<String>) -> Self {
        let mut entries = Vec::new();
        let first_line: &String = input.first().unwrap();
        let name: String = first_line.split(' ').next().unwrap().to_string();
        for line in input.into_iter().skip(1) {
            let mut numbers_str = line.split_whitespace();
            let destination = numbers_str.next().unwrap().parse::<i64>().unwrap();
            let source = numbers_str.next().unwrap().parse::<i64>().unwrap();
            let range = numbers_str.next().unwrap().parse::<i64>().unwrap();
            entries.push(MapEntry {
                source,
                destination,
                range,
            })
        }
        return Map { name, entries };
    }

    fn get(&self, key: i64) -> i64 {
        for entry in &self.entries {
            if key < entry.source || key > entry.source + entry.range {
                continue;
            }
            return entry.destination + key - entry.source;
        }
        return key;
    }
}

struct MapChain {
    chain: Vec<Map>,
}

impl MapChain {
    fn new() -> Self {
        return MapChain { chain: Vec::new() };
    }

    fn append(&mut self, m: Map) {
        self.chain.push(m);
    }

    fn follow_chain(&self, seed: i64) -> i64 {
        let mut value = seed;
        for map in &self.chain {
            value = map.get(value);
        }
        return value;
    }
}
