use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    println!(
        "Sample 1: {}",
        compute_answer("src/inputs/day06/sample.txt")
    );
    println!("Input 1: {}", compute_answer("src/inputs/day06/input.txt"));
}

fn parse_file(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);

    let mut time_line = String::new();
    reader.read_line(&mut time_line).unwrap();
    let mut times = Vec::new();
    for item in time_line.split_whitespace() {
        if item.contains(":") {
            continue;
        }
        times.push(item.parse::<i32>().unwrap());
    }

    let mut distance_line = String::new();
    reader.read_line(&mut distance_line).unwrap();
    let mut distances = Vec::new();
    for item in distance_line.split_whitespace() {
        if item.contains(":") {
            continue;
        }
        distances.push(item.parse::<i32>().unwrap());
    }
    return (times, distances);
}

fn compute_answer(filename: &str) -> i32 {
    let (times, distances) = parse_file(filename);
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

struct RaceRecord {
    time: i32,
    distance: i32,
}

impl RaceRecord {
    // TODO: Could be optimized by doing half the work
    fn compute_ways_to_beat(&self) -> i32 {
        let mut ways = 0;
        for i in (0..=self.time).into_iter() {
            if (self.time - i) * i > self.distance {
                ways += 1
            }
        }
        return ways;
    }
}
