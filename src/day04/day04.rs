use std::{fs::read_to_string, collections::HashMap};

fn main() {
    println!("Sample: {}", compute_total_score_pt1("src/inputs/day04/sample.txt"));
    println!("Input: {}", compute_total_score_pt1("src/inputs/day04/input.txt"));
    println!("Sample: {}", compute_total_score_pt2("src/inputs/day04/sample.txt"));
    println!("Input: {}", compute_total_score_pt2("src/inputs/day04/input.txt"));
}

fn compute_total_score_pt1(filename: &str) -> i32 {
    let mut cards = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        cards.push(Card::extract_card_info(String::from(line)));
    }
    let mut total_score = 0; 
    for card in cards {
        total_score += card.compute_score_pt1();
    }
    return total_score;
}

fn compute_total_score_pt2(filename: &str) -> i32 {
    let mut cache: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut cards = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        cards.push(Card::extract_card_info(String::from(line)));
    }
    let mut total_score : i32= 0;
    let mut queue: Vec<i32> = Vec::new();
    // Process the original cards that we have from our main input
    for card in cards {
        // Current card is "processed"
        total_score += 1;
        // Figure out if this card points to more cards
        let mut out = card.compute_score_pt2();
        cache.insert(card.card_id, out.clone());
        queue.append(&mut out);
    }
    // Process the copies of each card in our queue
    loop {
        if queue.is_empty() {
            break;
        }
        // Current card is "processed"
        total_score += 1;
        // Figure out if this card points to more cards
        let curr_item = queue.pop().unwrap();
        let cache_entry = cache.get(&curr_item).unwrap();
        let mut cache_entry_copy = cache_entry.clone(); // Q: Is there a better way to do this???
        queue.append(&mut cache_entry_copy);
    }
    return total_score;
}


#[derive(Debug)]
struct Card {
    card_id: i32,
    winning_numbers: Vec<i32>,
    numbers_have: Vec<i32>,
}

impl Card {
    fn compute_score_pt1(&self) -> i32 {
        let mut score = 0;
        for n in &self.numbers_have {
            let mut found = false;
            for w in &self.winning_numbers {
                if n == w {
                    found = true;
                    break;
                }
            }
            if found {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        
        return score;
    }

    fn compute_score_pt2(&self) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut curr = &self.card_id + 1;
        for n in &self.numbers_have {
            let mut found = false;
            for w in &self.winning_numbers {
                if n == w {
                    found = true;
                    break;
                }
            }
            if found {
                res.push(curr);
                curr += 1;
            }
        }
        
        return res;
    }

    fn extract_card_info(card_info: String) -> Self {
        let colon_loc = card_info.find(":").unwrap();
        let card_id : i32 = card_info[..colon_loc].split(" ").last().unwrap().parse::<i32>().unwrap();
        let v = card_info[colon_loc+1..].split(" ").into_iter();
        let mut is_winning_mode = true;
        let mut winning_numbers = Vec::new();
        let mut numbers_have = Vec::new();
        for s in v {
            if s == "" {
                continue;
            }
            if s == "|" {
                is_winning_mode = false;
                continue;
            }
            if is_winning_mode {
                winning_numbers.push(s.parse::<i32>().unwrap());
            } else {
                numbers_have.push(s.parse::<i32>().unwrap());
            }
        }
        return Card { card_id, winning_numbers, numbers_have};
    }
}
