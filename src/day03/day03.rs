use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let mut engine = Engine {
        number_positions: Vec::new(),
        symbol_position: HashMap::new(),
    };
    engine.parse_file("src/inputs/day03/input.txt");
    let answer_01 = engine.compute_answer_01();
    println!("answer to part 1 is {answer_01}");
    let answer_02 = engine.compute_answer_02();
    println!("answer to part 2 is {answer_02}");
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct NumberInfo {
    number: i32,
    points: Vec<Point>,
}

impl NumberInfo {
    fn new(number: i32, points: Vec<Point>) -> Self {
        NumberInfo { number, points }
    }
}

#[derive(Debug)]
struct Engine {
    number_positions: Vec<NumberInfo>,
    symbol_position: HashMap<Point, char>,
}

impl Engine {
    fn new_number(&mut self, n: NumberInfo) {
        self.number_positions.push(n);
    }

    fn new_symbol(&mut self, p: Point, c: char) {
        self.symbol_position.insert(p, c);
    }

    fn parse_file(&mut self, filename: &str) {
        for (row_num, line) in read_to_string(filename).unwrap().lines().enumerate() {
            self.parse_line(row_num, line);
        }
    }

    fn parse_line(&mut self, row_num: usize, line: &str) {
        let mut num_buffer: String = String::new();
        let mut points = Vec::new();
        for (col_num, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                num_buffer.push(c);
                points.push(Point {
                    x: row_num as i32,
                    y: col_num as i32,
                });
            } else if c != '.' {
                // TODO: Possible to DRY this up???
                if num_buffer.len() > 0 {
                    self.new_number(NumberInfo {
                        number: num_buffer.parse::<i32>().unwrap(),
                        points: points.clone(),
                    });
                    num_buffer.clear();
                    points.clear();
                }
                self.new_symbol(
                    Point {
                        x: row_num as i32,
                        y: col_num as i32,
                    },
                    c,
                );
            } else {
                if num_buffer.len() > 0 {
                    self.new_number(NumberInfo {
                        number: num_buffer.parse::<i32>().unwrap(),
                        points: points.clone(),
                    });
                    num_buffer.clear();
                    points.clear();
                }
            }
        }
        if num_buffer.len() > 0 {
            self.new_number(NumberInfo {
                number: num_buffer.parse::<i32>().unwrap(),
                points: points.clone(),
            });
            num_buffer.clear();
            points.clear();
        }
    }

    fn compute_answer_01(&mut self) -> i32 {
        let mut answer = 0;
        for number_position in self.number_positions.iter() {
            'point_loop: for point in number_position.points.iter() {
                for i_diff in vec![-1, 0, 1].iter() {
                    for j_diff in vec![-1, 0, 1].iter() {
                        if let Some(_value) = self.symbol_position.get(&Point {
                            x: point.x + i_diff,
                            y: point.y + j_diff,
                        }) {
                            answer += number_position.number;
                            break 'point_loop;
                        }
                    }
                }
            }
        }
        return answer;
    }

    fn compute_answer_02(&mut self) -> i32 {
        let mut temp: HashMap<Point, Vec<i32>> = HashMap::new();
        for number_position in self.number_positions.iter() {
            'point_loop: for point in number_position.points.iter() {
                for i_diff in vec![-1, 0, 1].iter() {
                    for j_diff in vec![-1, 0, 1].iter() {
                        let potential_symbol_pos = Point {
                            x: point.x + i_diff,
                            y: point.y + j_diff,
                        };
                        if let Some(value) = self.symbol_position.get(&potential_symbol_pos) {
                            if *value == '*' {
                                temp.entry(potential_symbol_pos)
                                    .or_insert(Vec::new())
                                    .push(number_position.number);
                                break 'point_loop;
                            }
                        }
                    }
                }
            }
        }
        let mut answer = 0;
        for key in temp.keys() {
            let vec = temp.get(key).unwrap();
            if vec.len() == 2 {
                answer += vec.first().unwrap() * vec.last().unwrap();
            }
        }
        return answer;
    }
}
