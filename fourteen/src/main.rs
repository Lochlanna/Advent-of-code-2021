use std::collections::BTreeMap;
use std::env;
use std::time::Instant;

struct PairCount{
    data: Vec<usize>,
    cursor:usize
}

impl PairCount {
    fn new() -> Self {
        PairCount{ data: vec![] }
    }
    fn add(&mut self, part_a:char, part_b:char, value:usize) {
        let index = (part_a as usize + part_b as usize) - 'A' as usize;
        if self.data.len() < index {
            self.data.resize(index+1, 0)
        }
        self.data[index] += value;
    }

    fn get(&self, part_a:char, part_b:char) -> Option<&usize> {
        let index = (part_a as usize + part_b as usize) - 'A' as usize;
        self.data.get(index)
    }
}

type PairMap = BTreeMap<char, BTreeMap<char, char>>;
type PairCountMap = BTreeMap<char, BTreeMap<char, usize>>;

fn read_input(filename: &str) -> (String, PairMap) {
    let file_contents = std::fs::read_to_string(filename).expect("couldn't read file contents");
    let mut template = String::new();
    let mut pair_mapping: PairMap = BTreeMap::new();
    for (i, line) in file_contents.lines().enumerate() {
        match i {
            0 => template.push_str(line),
            1 => {},
            _ => {
                if line.len() != 7 {
                    panic!("Couldn't parse mapping line {}", line);
                }
                let mut first_char = '.';
                let mut second_char = '.';
                let mut map_char = '.';
                for (i, char) in line.chars().enumerate() {
                    match i {
                        0 => first_char = char,
                        1 => second_char = char,
                        6 => map_char = char,
                        _ => {}
                    }
                }
                pair_mapping.entry(first_char).or_insert_with(BTreeMap::new).insert(second_char,map_char);
            }
        }
    }
    (template, pair_mapping)
}

fn count_pairs(str: &str) -> PairCountMap {
    let mut pair_count: PairCountMap = BTreeMap::new();
    let chars: Vec<char> = str.chars().collect();
    for pair in chars.windows(2) {
        let sub_tree = pair_count.entry(pair[0]).or_insert_with(BTreeMap::new);
        sub_tree.entry(pair[1]).and_modify(|c| *c += 1).or_insert(1);
    }
    pair_count
}

fn count_letters(str: &str) -> BTreeMap<char, usize> {
    let mut letter_count: BTreeMap<char, usize> = BTreeMap::new();
    for letter in str.chars() {
        letter_count.entry(letter).and_modify(|c| *c += 1).or_insert(1);
    }
    letter_count
}

fn insert_into_pair_count_map(char_a: char, char_b: char, count: usize, pair_count: &mut PairCountMap) {
    let subtree = pair_count.entry(char_a).or_insert_with(BTreeMap::new);
    subtree.entry(char_b).and_modify(|c| *c += count).or_insert(count);
}

fn process_pairs(pair_count: PairCount, letter_count: &mut BTreeMap<char, usize>, pair_map: &PairMap) -> PairCount {
    let mut new_pair_count: PairCount =  PairCount::new();
    for (char_a, sub_tree) in pair_count {
        for (char_b, count) in sub_tree {
            let mapped_char = *pair_map.get(&char_a).and_then(|st| st.get(&char_b)).expect("pair doesn't exist in mapping");
            letter_count.entry(mapped_char).and_modify(|c| *c += count).or_insert(count);
            insert_into_pair_count_map(char_a, mapped_char, count, &mut new_pair_count);
            insert_into_pair_count_map(mapped_char, char_b, count, &mut new_pair_count);
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
    println!("Took a total of {} seconds which is {} millis and is {} nanos and is {} microseconds", dur.as_secs_f64(), dur.as_millis(), dur.as_nanos(), dur.as_micros());
}
