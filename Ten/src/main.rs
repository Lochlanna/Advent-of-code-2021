use std::collections::VecDeque;

fn read_input(filename:&str) -> Vec<String> {
    match std::fs::read_to_string(filename) {
        Ok(fc) => fc.lines().map(|l| l.to_string()).collect(),
        Err(_) => panic!("Couldn't read the input file")
    }
}

fn get_opposite(c: char) -> char {
    return match c {
        '{' => '}',
        '}' => '{',
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '<' => '>',
        '>' => '<',
        _ => ' '
    }
}

fn get_line_score(line: &String) -> Result<u64,u64> {
    let mut openers = VecDeque::new();
    for c in line.chars() {
        match c {
            '{' | '(' | '[' | '<' => openers.push_front(c),
            '}' | ')' | ']' | '>' => {
                match openers.pop_front() {
                    None => return Err(char_to_corrupt_score(c)),
                    Some(opener) => {
                        if opener != get_opposite(c) {
                            return Err(char_to_corrupt_score(c));
                        }
                    }
                }
            }
            _ => {
                panic!("O ohh");
            }
        }
    }
    let mut repair_score:u64 = 0;
    while let Some(opener) = openers.pop_front() {
        repair_score = repair_score * 5 + match get_opposite(opener) {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0
        }
    }
    Ok(repair_score)
}

fn char_to_corrupt_score(c: char) -> u64 {
    return match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0
    }
}

fn main() {
    let input = read_input("input");
    let mut corrupt_score = 0;
    let mut repair_scores = Vec::new();
    for line in &input {
        match get_line_score(line) {
            Ok(repair_score) => {
                repair_scores.push(repair_score);
            }
            Err(line_corrupt_score) => {
                corrupt_score += line_corrupt_score;
            }
        }
    }
    repair_scores.sort();
    let repair_score = repair_scores[repair_scores.len()/2];
    println!("The corrupt score is {}, repair score is {}", corrupt_score, repair_score);
}
