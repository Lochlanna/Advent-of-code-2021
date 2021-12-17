use std::fs::File;
use std::io;
use std::io::BufRead;
use std::time::Duration;

enum Direction {
    Up,
    Down,
    Forward,
}

impl Direction {
    fn from_string(string_name: &str) -> Result<Direction,()> {
        let lowercase = string_name.to_lowercase();
        return match lowercase.as_str() {
            "up" => Ok(Direction::Up) ,
            "down" => Ok(Direction::Down) ,
            "forward" => Ok(Direction::Forward) ,
            _ => Err(())
        }
    }
}

struct Step {
    direction: Direction,
    units: i32
}


fn read_values() -> Vec<Step>{
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
        let parts:Vec<&str> = line_str.split(" ").collect();
        if parts.len() != 2 {
            continue;
        }
        let direction = match Direction::from_string(parts[0]) {
            Ok(d) => d,
            Err(_) => continue
        };
        let units = match str::parse::<i32>(parts[1]) {
            Ok(v) => v,
            Err(_) => continue
        };
        values.push(Step {direction, units});
    }
    values
}

fn part_two() {
    let steps = read_values();
    let mut y = 0;
    let mut z = 0;
    let mut aim = 0;
    for step in &steps {
        match step.direction {
            Direction::Up => aim -= step.units,
            Direction::Down => aim += step.units,
            Direction::Forward => {
                y += step.units;
                z += aim * step.units;
            }
        }
    }
    println!("Part two multiplied movement is {}", z*y);
}

fn part_one() {
    let steps = read_values();
    let mut y:i32 = 0;
    let mut z:i32 = 0;
    for step in &steps {
        match step.direction {
            Direction::Up => z -= step.units,
            Direction::Down => z += step.units,
            Direction::Forward => y += step.units,
        }
    }
    println!("Multiplied movement is {}", z*y);
}

fn main() {
    part_one();
    part_two();
}
