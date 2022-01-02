use std::fs;

fn run_simulation_step(fish_at_each_day: &mut Vec<usize>)  {
    let num_new_fish = match fish_at_each_day.first() {
        None => 0,
        Some(nnf) => *nnf
    };
    for i in 1..fish_at_each_day.len() {
        fish_at_each_day[i-1] = fish_at_each_day[i];
    }
    fish_at_each_day[6] += num_new_fish;
    fish_at_each_day[8] = num_new_fish;
}

fn read_input(filename: &str) -> Option<Vec<u8>> {
    let file_contents = fs::read_to_string(filename).expect("Couldn't open file");
    if file_contents.is_empty() {
        return None;
    }
    let mut fish_timers = Vec::new();
    for val in file_contents.split(",") {
        match val.parse::<u8>() {
            Ok(v) => fish_timers.push(v),
            Err(_) => return None
        };
    }
    Some(fish_timers)
}

fn count_fish(fish_timers: Vec<u8>) -> Vec<usize> {
    let mut timer_count = Vec::new();
    timer_count.resize(9,0);
    for timer in fish_timers {
        match timer_count.get_mut(timer as usize) {
            Some(entry) => {
                *entry += 1;
            }
            None => panic!("This shouldn't happen...")
        }
    }
    timer_count
}

fn main() {
    let mut fish_timers = match read_input("input") {
        None => {
            panic!("Couldn't read the input");
        },
        Some(ft) => ft
    };
    let mut timer_count = count_fish(fish_timers);
    for i in 0..80 {
        run_simulation_step(&mut timer_count);
    }
    println!("There are {} fish after 80 days", timer_count.iter().sum::<usize>());
    for i in 80..256 {
        run_simulation_step(&mut timer_count);
    }
    println!("There are {} fish after 256 days", timer_count.iter().sum::<usize>());
}
