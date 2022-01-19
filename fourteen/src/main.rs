use std::collections::HashMap;
use std::env;
use std::time::Instant;


fn read_input(filename: &str) -> (String, HashMap<[char;2], char>) {
    let file_contents = std::fs::read_to_string(filename).expect("coudln't read file contents");
    let mut line_iter = file_contents.lines();

    let template = line_iter.next().expect("couldn't read the template string");

    line_iter.next();

    let mut pair_mapping: HashMap<[char;2], char> = HashMap::new();
    for line in line_iter.by_ref() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        if parts.len() != 2 {
            panic!("couldn't parse pair mapping line {}", line);
        }
        let mut key_chars = parts[0].chars();
        let first_char = key_chars.next().expect("couldn't get first key char");
        let second_char = key_chars.next().expect("couldn't get second key char");
        let map_char = parts[1].chars().next().expect("couldn't get the mapped part");
        pair_mapping.insert([first_char, second_char], map_char);
    }
    (String::from(template), pair_mapping)
}

fn count_pairs(str: &str, pair_map: &HashMap<[char;2], char>) -> HashMap<[char; 2], usize> {
    let mut pair_count:HashMap<[char;2], usize> = HashMap::with_capacity(pair_map.len());
    let chars:Vec<char> = str.chars().collect();
    for pair in chars.windows(2) {
        match pair_count.get_mut(pair) {
            None => {
                pair_count.insert(pair.try_into().expect("Slice with an incorrect length"), 1);
            },
            Some(count) => *count += 1
        }
    }
    pair_count
}

fn count_letters(str: &str) -> HashMap<char, usize> {
    let mut letter_count:HashMap<char, usize> = HashMap::new();
    for letter in str.chars() {
        match letter_count.get_mut(&letter) {
            None => {
                letter_count.insert(letter, 1);
            },
            Some(count) => *count += 1
        };
    }
    letter_count
}

fn process_pairs(pair_count: HashMap<[char; 2], usize>, letter_count: &mut HashMap<char, usize>, pair_map: &HashMap<[char;2], char>) -> HashMap<[char; 2], usize> {
    let mut new_pair_count = HashMap::with_capacity(pair_count.len());

    for (pair, old_count) in pair_count {
        let mapped_char = pair_map.get(&pair).expect("pair doesn't exist in mapping");
        match letter_count.get_mut(mapped_char) {
            None => {
                letter_count.insert(*mapped_char, old_count);
            }
            Some(count) => *count += old_count
        }

        let pair_a = [pair[0], *mapped_char];
        match new_pair_count.get_mut(&pair_a)  {
            None => {
                new_pair_count.insert(pair_a, old_count);
            }
            Some(count) => *count += old_count
        }
        let pair_b = [*mapped_char, pair[1]];
        match new_pair_count.get_mut(&pair_b)  {
            None => {
                new_pair_count.insert(pair_b, old_count);
            }
            Some(count) => *count += old_count
        }
    }
    new_pair_count
}

fn get_score(letter_count: &HashMap<char, usize>) -> usize {
    if letter_count.is_empty() {
        return 0;
    }
    let mut min = usize::MAX;
    let mut max = usize::MIN;
    for count in letter_count.values() {
        min = std::cmp::min(min, *count);
        max = std::cmp::max(max, *count);
    }
    max - min
}

fn main() {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    let (polymer, pair_mapping) = read_input(if args.len() < 2 { "input_small" } else { args[1].as_str() });
    let mut pair_count = count_pairs(&polymer, &pair_mapping);
    let polymer_str: String = polymer.chars().collect();
    let mut letter_count = count_letters(polymer_str.as_str());
    for _ in 0..10 {
        pair_count = process_pairs(pair_count, &mut letter_count, &pair_mapping);
    }
    println!("score after 10 is {}", get_score(&letter_count));

    for _ in 10..40 {
        pair_count = process_pairs(pair_count, &mut letter_count, &pair_mapping);
    }
    println!("score after 40 is {}", get_score(&letter_count));

    println!("Took a total of {} seconds", now.elapsed().as_secs_f64());
}
