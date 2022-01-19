use std::collections::{BTreeMap, HashMap};
use std::env;
use std::time::Instant;


fn char_vec_to_string(chars: &[char]) -> String {
    chars.iter().collect()
}


fn read_input(filename: &str) -> (Vec<char>, BTreeMap<char, BTreeMap<char, char>>) {
    let file_contents = std::fs::read_to_string(filename).expect("coudln't read file contents");
    let mut line_iter = file_contents.lines();

    let template_str = line_iter.next().expect("couldn't read the template string");
    let template: Vec<char> = template_str.chars().collect();

    line_iter.next();

    let mut pair_mapping: BTreeMap<char, BTreeMap<char, char>> = BTreeMap::new();
    for line in line_iter.by_ref() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        if parts.len() != 2 {
            panic!("couldn't parse pair mapping line {}", line);
        }
        let mut key_chars = parts[0].chars();
        let first_char = key_chars.next().expect("couldn't get first key char");
        let second_char = key_chars.next().expect("couldn't get second key char");
        let map_char = parts[1].chars().next().expect("couldn't get the mapped part");
        match pair_mapping.get_mut(&first_char) {
            None => {
                //doesn't exist yet so create a new one
                let mut new_tree = BTreeMap::new();
                new_tree.insert(second_char, map_char);
                pair_mapping.insert(first_char, new_tree);
            }
            Some(sub_tree) => {
                sub_tree.insert(second_char, map_char);
            }
        }
    }

    (template, pair_mapping)
}

fn get_mapped_value_from_slice(key: &[char], pair_mapping: &BTreeMap<char, BTreeMap<char, char>>) -> Option<char> {
    if key.len() != 2 {
        return None
    }
    match pair_mapping.get(&key[0]) {
        None => None,
        Some(sub_tree) => {
            sub_tree.get(&key[1]).copied()
        }
    }
}

fn process_pairs(polymer: &[char], pair_mapping: &BTreeMap<char, BTreeMap<char, char>>) -> Vec<char> {
    let mut new_polymer = Vec::with_capacity(polymer.len() * 2 - 1);
    let mut first = true;
    for pair in polymer.windows(2) {
        let new_char = get_mapped_value_from_slice(pair, pair_mapping).expect("couldn't get mapping from pair to char");
        if first {
            new_polymer.push(pair[0]);
            first = false;
        }
        new_polymer.push(new_char);
        new_polymer.push(pair[1]);
    }
    new_polymer
}

fn min_max_occurrences(polymer: &[char]) -> Option<((char, usize), (char, usize))> {
    if polymer.is_empty() {
        return None;
    }
    let mut table:HashMap<char, usize> = HashMap::with_capacity(26);
    for c in polymer {
        match table.get_mut(c) {
            None => {
                table.insert(*c, 1);
            }
            Some(entry) => *entry += 1
        }
    }
    let mut max = ('.', 0usize);
    let mut min = ('.', usize::MAX);
    for (c, count) in table {
        if count > max.1 {
            max.0 = c;
            max.1 = count;
        }
        if count < min.1 {
            min.0 = c;
            min.1 = count;
        }
    }
    Some((min, max))
}


fn main() {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    let (mut polymer, pair_mapping) = read_input(if args.len() < 2 { "input_small" } else { args[1].as_str() });
    println!("template is {} mapping is {:?}", char_vec_to_string(&polymer), pair_mapping);
    for step in 0..10 {
        polymer = process_pairs(&polymer, &pair_mapping);
        println!("on step {} length is {}", step + 1, polymer.len());
    }
    // let ((min_char, min_count), (max_char, max_count)) = min_max_occurrences(&polymer).expect("couldn't count values in polymer");
    // println!("The min char was {}:{} the max char was {}:{}. Score is {}", min_char, min_count, max_char, max_count, max_count - min_count);
    for step in 0..30 {
        polymer = process_pairs(&polymer, &pair_mapping);
        println!("on step {} length is {}", 10 + step + 1, polymer.len());
    }
    // let ((min_char, min_count), (max_char, max_count)) = min_max_occurrences(&polymer).expect("couldn't count values in polymer");
    // println!("The min char was {}:{} the max char was {}:{}. Score is {}", min_char, min_count, max_char, max_count, max_count - min_count);
    println!("Took a total of {} seconds", now.elapsed().as_secs_f64());
}
