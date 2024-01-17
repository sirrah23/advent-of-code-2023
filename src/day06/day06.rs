use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    println!(
        "Sample 1: {}",
        compute_answer_pt1("src/inputs/day06/sample.txt")
    );
    println!(
        "Input 1: {}",
        compute_answer_pt1("src/inputs/day06/input.txt")
    );
    println!(
        "Sample 2: {}",
        compute_answer_pt2("src/inputs/day06/sample.txt")
    );
    println!(
        "Input 2: {}",
        compute_answer_pt2("src/inputs/day06/input.txt")
    );
}

fn parse_file_pt1(filename: &str) -> (Vec<i64>, Vec<i64>) {
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);

    let mut time_line = String::new();
    reader.read_line(&mut time_line).unwrap();
    let mut times = Vec::new();
    for item in time_line.split_whitespace() {
        if item.contains(":") {
            continue;
        }
        times.push(item.parse::<i64>().unwrap());
    }

    let mut distance_line = String::new();
    reader.read_line(&mut distance_line).unwrap();
    let mut distances = Vec::new();
    for item in distance_line.split_whitespace() {
        if item.contains(":") {
            continue;
        }
        distances.push(item.parse::<i64>().unwrap());
    }
    return (times, distances);
}

fn parse_file_pt2(filename: &str) -> (i64, i64) {
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);

    let mut time_line = String::new();
    reader.read_line(&mut time_line).unwrap();
    let mut full_time = String::new();
    for item in time_line.split_whitespace() {
        if item.contains(":") {
            continue;
        }
        full_time.push_str(item);
    }

    let mut distance_line = String::new();
    reader.read_line(&mut distance_line).unwrap();
    let mut full_distance = String::new();
    for item in distance_line.split_whitespace() {
        if item.contains(":") {
            continue;
        }
        full_distance.push_str(item);
    }
    return (
        full_time.parse::<i64>().unwrap(),
        full_distance.parse::<i64>().unwrap(),
    );
}

fn compute_answer_pt1(filename: &str) -> i64 {
    let (times, distances) = parse_file_pt1(filename);
    let mut answer = 1;
    for (time, distance) in times.iter().zip(distances.iter()) {
        let rr = RaceRecord {
            time: *time,
            distance: *distance,
        };
        answer *= rr.compute_ways_to_beat();
    }
    return answer;
}

fn compute_answer_pt2(filename: &str) -> i64 {
    let (time, distance) = parse_file_pt2(filename);
    let rr = RaceRecord {
        time: time,
        distance: distance,
    };
    let answer = rr.compute_ways_to_beat();
    return answer;
}

struct RaceRecord {
    time: i64,
    distance: i64,
}

impl RaceRecord {
    // TODO: Could be optimized by doing half the work
    fn compute_ways_to_beat(&self) -> i64 {
        let mut ways = 0;
        for i in (0..=self.time).into_iter() {
            if (self.time - i) * i > self.distance {
                ways += 1
            }
        }
        return ways;
    }
}
