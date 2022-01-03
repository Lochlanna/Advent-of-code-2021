use std::collections::HashMap;
use std::error::Error;
use std::num::ParseIntError;

fn median(values: &Vec<f32>) -> f32 {
    let mut sorted_values = values.clone();
    sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let length = sorted_values.len();
    return if length % 2 == 0 {
        //even number
        let upper_middle = (length / 2);
        if upper_middle == 0 {
            return 0f32;
        }
        let lower_middle = upper_middle -1;
        (sorted_values[lower_middle] + sorted_values[upper_middle]) / 2f32
    } else {
        let middle_index = (length as f32 / 2f32).ceil() as usize;
        sorted_values[middle_index]
    }
}


fn read_input(filename: &str) -> Vec<f32> {
    let file_contents = std::fs::read_to_string(filename).expect("Couldn't read file");
    let mut values = Vec::new();
    for val_str in file_contents.split(",") {
        match val_str.parse::<f32>() {
            Ok(v) => values.push(v),
            Err(_) => panic!("Couldn't parse value")
        }
    }
    values
}

fn calculate_fuel(value: f32, test_values: &Vec<f32>) -> f32 {
    let mut fuel = 0f32;
    for v in test_values {
        fuel += (*v - value).abs()
    }
    fuel
}

fn calculate_fuel_exp(value: f32, test_values: &Vec<f32>) -> f64 {
    let mut fuel = 0f64;
    for v in test_values {
        let steps = (*v - value).abs();
        fuel += (steps * (steps + 1f32) / 2f32) as f64;
    }
    fuel
}

fn main() {
    let mut values = read_input("input");
    let median = median(& values);
    let fuel = calculate_fuel(median, & values);
    let average = values.iter().sum::<f32>() / values.len() as f32;
    let fuel_exp_low = calculate_fuel_exp(average.floor(), & values);
    let fuel_exp_high = calculate_fuel_exp(average.ceil(), & values);
    let fuel_exp = if fuel_exp_low < fuel_exp_high {fuel_exp_low} else {fuel_exp_high};
    println!("Fuel required is {} exp is {}", fuel, fuel_exp);
}