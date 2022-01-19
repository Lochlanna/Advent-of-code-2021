use std::collections::BTreeMap;
use std::env;
use std::time::Instant;

type PairMap = BTreeMap<char, BTreeMap<char,char>>;
type PairCountMap = BTreeMap<char, BTreeMap<char,usize>>;

fn read_input(filename: &str) -> (String, PairMap) {
    let file_contents = std::fs::read_to_string(filename).expect("couldn't read file contents");
    let mut line_iter = file_contents.lines();

    let template = line_iter.next().expect("couldn't read the template string");

    line_iter.next();

    let mut pair_mapping: PairMap = BTreeMap::new();
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
                let mut sub_map = BTreeMap::new();
                sub_map.insert(second_char, map_char);
                pair_mapping.insert(first_char, sub_map);
            }
            Some(sub_map) => {
                sub_map.insert(second_char, map_char);
            }
        }
    }
    (String::from(template), pair_mapping)
}

fn count_pairs(str: &str) -> PairCountMap {
    let mut pair_count:PairCountMap = BTreeMap::new();
    let chars:Vec<char> = str.chars().collect();
    for pair in chars.windows(2) {
        match pair_count.get_mut(&pair[0]) {
            None => {
                let mut sub_tree = BTreeMap::new();
                sub_tree.insert(pair[1], 1);
                pair_count.insert(pair[0], sub_tree);
            }
            Some(sub_tree) => {
                match sub_tree.get_mut(&pair[1]) {
                    None => {
                        sub_tree.insert(pair[1], 1);
                    }
                    Some(count) => *count += 1
                }
            }
        }
    }
    pair_count
}

fn count_letters(str: &str) -> BTreeMap<char, usize> {
    let mut letter_count:BTreeMap<char, usize> = BTreeMap::new();
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

fn get_from_pair_map(char_a:&char, char_b:&char, pair_map: &PairMap) -> Option<char> {
    match pair_map.get(char_a) {
        None => None,
        Some(sub_tree) => {
            sub_tree.get(char_b).copied()
        }
    }
}

fn insert_into_pair_count_map(char_a:char, char_b:char, count: usize, pair_count:&mut PairCountMap) {
    match pair_count.get_mut(&char_a)  {
        None => {
            let mut sub_tree = BTreeMap::new();
            sub_tree.insert(char_b, count);
            pair_count.insert(char_a, sub_tree);
        }
        Some(sub_tree) => {
            match sub_tree.get_mut(&char_b) {
                None => {
                    sub_tree.insert(char_b, count);
                }
                Some(old_count) => *old_count += count
            }
        }
    }
}

fn process_pairs(pair_count: PairCountMap, letter_count: &mut BTreeMap<char, usize>, pair_map: &PairMap) -> PairCountMap {
    let mut new_pair_count:PairCountMap = BTreeMap::new();

    for (char_a, sub_tree) in pair_count {
        for (char_b, old_count) in sub_tree {
            let mapped_char = get_from_pair_map(&char_a, &char_b, pair_map).expect("pair doesn't exist in mapping");
            match letter_count.get_mut(&mapped_char) {
                None => {
                    letter_count.insert(mapped_char, old_count);
                }
                Some(count) => *count += old_count
            }
            insert_into_pair_count_map(char_a, mapped_char, old_count, &mut new_pair_count);
            insert_into_pair_count_map(mapped_char, char_b, old_count, &mut new_pair_count);
        }
    }

    new_pair_count
}

fn get_score(letter_count: &BTreeMap<char, usize>) -> usize {
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
    let (polymer, pair_mapping) = read_input(if args.len() < 2 { "input" } else { args[1].as_str() });
    let mut pair_count = count_pairs(&polymer);
    let mut letter_count = count_letters(polymer.as_str());
    for _ in 0..10 {
        pair_count = process_pairs(pair_count, &mut letter_count, &pair_mapping);
    }
    println!("score after 10 is {}", get_score(&letter_count));

    for _ in 10..40 {
        pair_count = process_pairs(pair_count, &mut letter_count, &pair_mapping);
    }
    println!("score after 40 is {}", get_score(&letter_count));
    let dur = now.elapsed();
    println!("Took a total of {} seconds which is {} millis and is {} nanos", dur.as_secs_f64(), dur.as_millis(), dur.as_nanos());
}
