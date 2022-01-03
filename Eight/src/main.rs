use std::collections::{HashMap, HashSet};

struct Problem {
    input_values: Vec<String>,
    output_values: Vec<String>
}

fn read_input(filename: &str) -> Vec<Problem>{
    let file_contents = std::fs::read_to_string(filename).expect("Couldn't open the file");
    let mut problems = Vec::new();
    for line in file_contents.lines() {
        let sections: Vec<&str> = line.split("|").collect();
        let mut problem = Problem {input_values: Vec::new(), output_values: Vec::new()};
        match sections.first() {
            None => panic!("Didn't have the input section for the problem"),
            Some(input_section) => {
                for segment in input_section.split(" ") {
                    if segment.len() == 0 {
                        continue;
                    }
                    problem.input_values.push(segment.to_string());
                }
            }
        }
        match sections.get(1) {
            None => panic!("Didn't have the output section for the problem"),
            Some(input_section) => {
                for segment in input_section.split(" ") {
                    if segment.len() == 0 {
                        continue;
                    }
                    problem.output_values.push(segment.to_string());
                }
            }
        }
        problems.push(problem);
    }
    problems
}

fn num_match(a: &String, b: &String) -> usize {
    let mut matching_chars = 0;
    for a_char in a.chars() {
        if b.contains(a_char) {
            matching_chars += 1;
        }
    }
    matching_chars
}

impl Problem {
    fn count_unique_nums_output(&self) -> usize {
        let mut count:usize = 0;
        for output_value in &self.output_values {
            match output_value.len() {
                2 | 3 | 4 | 7  => count += 1,
                _ => {
                    //do nothing here
                }
            }
        }
        count
    }

    fn decode(&self) -> u32 {
        let mut known_examples = HashMap::new();
        for value in &self.input_values {
            match value.len() {
                // We only care about these two because with these we can figure out the others
                2 => {
                    known_examples.insert(1, value.clone());
                },
                4 => {
                    known_examples.insert(4, value.clone());
                },
                _ => {}
            }
            if known_examples.len() == 2 {
                break;
            }
        }
        if known_examples.len() != 2 {
            panic!("We can't solve this :(");
        }
        let mut decoded = Vec::new();
        for out_val in self.output_values.iter().rev() {
            match out_val.len() {
                2 => decoded.push(1),
                3 => decoded.push(7),
                4 => decoded.push(4),
                5 => {
                    if num_match(&out_val, &known_examples[&4]) == 2 {
                        decoded.push(2);
                    } else if num_match(&out_val, &known_examples[&1]) == 1 {
                        decoded.push(5);
                    } else {
                        decoded.push(3);
                    }
                }
                6 => {
                    if num_match(&out_val, &known_examples[&1]) == 1 {
                        decoded.push(6);
                    } else if num_match(&out_val, &known_examples[&4]) == 3 {
                        decoded.push(0);
                    } else {
                        decoded.push(9);
                    }
                }
                7 => decoded.push(8),
                _ => {}
            }
        }
        let mut decoded_output:u32 = 0;
        for (i, decoded_value) in decoded.iter().enumerate() {
            decoded_output += *decoded_value as u32 * (10u32.pow(i as u32));
        }
        decoded_output
    }

}


fn main() {
    let problems = read_input("input");
    let mut count = 0;
    let mut output_sum = 0;
    for problem in &problems {
        count += problem.count_unique_nums_output();
        output_sum += problem.decode();
    }
    println!("There are {} instances of 1, 4, 7 or 8. Output sum is {}", count, output_sum);
}
