use std::{env, io};
use std::fs::File;
use std::io::BufRead;
use std::num::ParseIntError;

fn read_values() -> Vec<i32>{
    let file = File::open("input").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut values = Vec::new();
    for line in lines {
        let line_str = match line {
            Ok(l) => l,
            Err(_) => continue
        };
        if line_str.is_empty() {
            continue;
        }
        let line_val = match str::parse::<i32>(&line_str) {
            Ok(v) => v,
            Err(_) => continue
        };
        values.push(line_val);
    }
    values
}

fn problem_one() {
    let mut values = read_values();
    let mut larger = 0;
    for window in values.windows(2) {
        if window[0] < window[1] {
            larger += 1;
        }
    }

    println!("The number of times it got deeper were {}", larger);
}

fn problem_two() {

    let mut values = read_values();

    let mut larger = 0;
    let mut window_a = values.windows(3);
    let mut window_b = values.windows(3);
    window_b.next();
    loop {
        let win_a = match window_a.next() {
            None => break,
            Some(win) => win
        };
        let win_b = match window_b.next() {
            None => break,
            Some(win) => win
        };
        let sum_a:i32 = win_a.iter().sum();
        let sum_b:i32 = win_b.iter().sum();
        if sum_b > sum_a {
            larger += 1;
        }
    }

    println!("The number of times it got deeper were {}", larger);
}

fn main() {
    problem_one();
    problem_two();
}
