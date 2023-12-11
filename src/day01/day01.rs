use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let lines = read_lines("./src/inputs/day01/day01_p1.txt");
    let mut calibration_total = 0;
    let mut val = 1;
    for line in lines {
        let cv = get_calibration_value_02(line.as_str());
        println!("cv {} is {}", val, cv);
        calibration_total += cv;
        val += 1;
    }
    println!("Total: {}", calibration_total);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }

    return result;
}

#[allow(dead_code)]
fn get_calibration_value(line: &str) -> u32 {
    let mut first_digit: u32 = 0;
    let mut last_digit: u32 = 0;
    for c in line.chars() {
        if c.is_digit(10) {
            first_digit = c.to_digit(10).unwrap();
            break
        }
    }
    for c in line.chars().rev() {
        if c.is_digit(10) {
            last_digit = c.to_digit(10).unwrap();
            break
        }
    }
    return 10 * first_digit + last_digit;
}

fn get_calibration_value_02(line: &str) -> u32 {
    let mut hm: HashMap<&str, u32> = HashMap::new();
    hm.insert("one", 1);
    hm.insert("two", 2);
    hm.insert("three", 3);
    hm.insert("four", 4);
    hm.insert("five", 5);
    hm.insert("six", 6);
    hm.insert("seven", 7);
    hm.insert("eight", 8);
    hm.insert("nine", 9);

    let mut digits: Vec<u32> = Vec::new();
    // let mut buffer = String::new(); 
    let mut idx = 0;
    'outer: loop {
        if idx >= line.len() {
            break;
        }
        if line.chars().nth(idx).unwrap().is_digit(10) {
            digits.push(line.chars().nth(idx).unwrap().to_digit(10).unwrap());
            idx += 1;
            continue
        } else {
            let mut inner_idx = idx;
            // let mut num_iters = 0;
            let mut buffer  = String::new();
            loop {
                // num_iters += 1;
                if inner_idx >= line.len() {
                    if idx == line.len() - 1 {
                        break 'outer;
                    }
                    break;
                } 
                buffer.push(line.chars().nth(inner_idx).unwrap());
                // println!("inner buffer: {}", buffer.as_str());
                let res = num_check(buffer.as_str(), &hm);
                if !res.1 {
                    idx += 1;
                    break;
                } else {
                    if res.0 != 0 {
                        digits.push(res.0);
                        idx += buffer.len();
                        break;
                    } else {
                        inner_idx += 1;
                    }
                }
            }
        }
    }
    for d in &digits {
        print!("{} ", d);
    }
    println!("");
    return digits.first().unwrap() * 10 +  digits.last().unwrap();
}

fn num_check(buffer: &str, num_map: &HashMap<&str, u32>) -> (u32, bool) {
    for num in num_map.keys() {
        let res = potential_or_is_the_num(num, buffer, &num_map);
        if res.1 {
            return res
        }
    }
    return (0, false);
}

fn potential_or_is_the_num(the_num: &str, buffer: &str, num_map: &HashMap<&str, u32>) -> (u32, bool) {
    if the_num.eq(buffer) {
        return (*num_map.get(buffer).unwrap(), true)
    } else if the_num.starts_with(buffer){
        return (0, true)
    } else {
        return (0, false)
    }
}
