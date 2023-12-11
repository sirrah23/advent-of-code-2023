use std::fs::read_to_string;

fn main() {
    println!("Sample: {}", compute_total_score("src/inputs/day04/sample.txt"));
    println!("Input: {}", compute_total_score("src/inputs/day04/input.txt"));
}

fn compute_total_score(filename: &str) -> i32 {
    let mut cards = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        cards.push(Card::extract_card_info(String::from(line)));
    }
    let mut total_score = 0; 
    for card in cards {
        total_score += card.compute_score();
    }
    return total_score;
}


#[derive(Debug)]
struct Card {
    winning_numbers: Vec<i32>,
    numbers_have: Vec<i32>,
}

impl Card {
    fn compute_score(&self) -> i32 {
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

    fn extract_card_info(card_info: String) -> Self {
        let colon_loc = card_info.find(":").unwrap();
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
        return Card { winning_numbers, numbers_have};
    }
}
