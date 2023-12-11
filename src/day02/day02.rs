use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    assert!(day_02_p01("src/inputs/day02/sample.txt") == 8);
    println!("Real part one: {}", day_02_p01("src/inputs/day02/input.txt"));
    assert!(day_02_p02("src/inputs/day02/sample.txt") == 2286);
    println!("Real part two: {}", day_02_p02("src/inputs/day02/input.txt"));
}

fn day_02_p01(input_file: &str) -> i32 {
    let (red_limit, green_limit, blue_limit) = (12, 13, 14);
    let mut valid_total = 0;
    let lines = read_lines(input_file); 
    for line in lines {
        let (game_id, maxes) = compute_maxes_for_game(line.as_str());
        if maxes["red"] <= red_limit && maxes["green"] <= green_limit && maxes["blue"] <= blue_limit {
            valid_total += game_id;
        }
    }
    return valid_total;
}

fn day_02_p02(input_file: &str) -> i32 {
    let mut power_total = 0;
    let lines = read_lines(input_file); 
    for line in lines {
        let (_game_id, maxes) = compute_maxes_for_game(line.as_str());
        power_total += maxes["red"] * maxes["green"] * maxes["blue"];
    }
    return power_total;
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }

    return result;
}

fn compute_maxes_for_game(game_line: &str) -> (i32, HashMap<&str, i32>) {
    let mut out = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
    
    let game_line_parts: Vec<&str> = game_line.split(": ").collect();
    let game_prefix = game_line_parts[0];
    let rounds_suffix = game_line_parts[1];

    let game_id = get_game_id(game_prefix);

    let rounds_parts: Vec<&str> = rounds_suffix.split("; ").collect();
    for rounds_part in rounds_parts {
        let dice_parts: Vec<&str> = rounds_part.split(", ").collect();
        for dice_part in dice_parts {
            let actual_dice_part: Vec<&str> = dice_part.split_ascii_whitespace().collect();
            let count = actual_dice_part[0].to_string().parse::<i32>().unwrap();
            let color = actual_dice_part[1];
            if let Some(existing_count) = out.get(color) {
                if count > *existing_count {
                    // greatest observed value so far
                    out.insert(color, count);
                }
            } else {
                // brand new
                out.insert(color, count);
            }
        }
    }
    return (game_id, out);
}

fn get_game_id(gp: &str) -> i32 {
    let gp_parts : Vec<&str> = gp.split(' ').collect();
    return gp_parts[1].to_string().parse::<i32>().unwrap();
}