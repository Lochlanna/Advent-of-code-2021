use std::fs::File;
use std::io;
use std::io::BufRead;

struct Column {
    size: usize,
    sum: usize
}

impl Column {
    fn ones(&self) -> usize {
        self.sum
    }
    fn zeros(&self) -> usize {
        self.size - self.sum
    }
}

struct AdvancedColumn {
    values: Vec<bool>,
    sum: usize
}

impl AdvancedColumn {
    fn insert(&mut self, value: bool) {
        self.values.push(value);
        if value {
            self.sum += 1;
        }
    }
    fn remove(&mut self, index: usize) -> Result<bool, ()> {
        if index > self.values.len() {
            return Err(())
        }
        let value = self.values.remove(index);
        if value {
            self.sum -= 1;
        }
        Ok(value)
    }
    fn ones(&self) -> usize {
        self.sum
    }
    fn zeros(&self) -> usize {
        self.values.len() - self.sum
    }
}

struct ColumnSet {
    columns: Vec<AdvancedColumn>
}

impl ColumnSet {
    fn num_at(&self, index: usize) -> u32 {
        let mut number:u32 = 0;
        for (i, col) in self.columns.iter().rev().enumerate() {
            if col.values[index] {
                number += 2_u32.pow(i as u32)
            }
        }
        number
    }
    fn is_empty(&self) -> bool {
        self.columns.is_empty() || self.columns[0].values.is_empty()
    }
    fn insert_value(&mut self, value: Vec<bool>) {
        for (i, val) in value.iter().enumerate() {
            match self.columns.get_mut(i) {
                None => {
                    self.columns.push(AdvancedColumn{ values: vec![*val], sum: if *val {1} else {0} })
                }
                Some(col) => {
                    col.insert(*val);
                }
            }
        }
    }

    fn remove_value(&mut self, index: usize) {
        for col in &mut self.columns {
            let _ = col.remove(index);
        }
    }
}

fn read_values_raw() -> ColumnSet {
    let file = File::open("input").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut col_set = ColumnSet{ columns: vec![] };
    for line in lines {
        let line_str = match line {
            Ok(l) => l,
            Err(_) => continue
        };
        if line_str.is_empty() {
            continue;
        }
        let mut row = Vec::with_capacity(line_str.len());
        for c in line_str.chars() {
            let value= match c {
                '0' => false,
                '1' => true,
                _ => false
            };
            row.push(value);
        }
        col_set.insert_value(row);
    }
    col_set
}

fn read_values() -> Vec<Column>{
    let file = File::open("input").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut columns = Vec::new();
    for line in lines {
        let line_str = match line {
            Ok(l) => l,
            Err(_) => continue
        };
        if line_str.is_empty() {
            continue;
        }
        for (i, c) in line_str.chars().enumerate() {
            let value:usize = match c {
                '0' => 0,
                '1' => 1,
                _ => 0
            };
            match columns.get_mut(i) {
                None => {
                    columns.push(Column{ size: 1, sum: value })
                }
                Some(col) => {
                    col.sum += value;
                    col.size += 1;
                }
            }
        }
    }
    columns
}

fn part_one() {
    let columns = read_values();
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for (i, col) in columns.iter().rev().enumerate() {
        if col.ones() > col.zeros() {
            gamma += 2_u32.pow(i as u32)
        } else {
            epsilon += 2_u32.pow(i as u32)
        }
    }
    println!("Gamma {}, epsilon {}, multiplied {}", gamma, epsilon, gamma * epsilon);
}

fn part_two(direction: bool) -> u32 {
    let mut values = read_values_raw();
    if values.is_empty() {
        println!("There were no values");
        return 0;
    }

    for col_i in 0..values.columns.len() {
        let mut keep_one = values.columns[col_i].ones() > values.columns[col_i].zeros() || values.columns[col_i].ones() == values.columns[col_i].zeros();
        if !direction {
            keep_one = !keep_one;
        }
        let mut i = 0;
        while i < values.columns[col_i].values.len() {
            if (keep_one && values.columns[col_i].values[i]) || (!keep_one && !values.columns[col_i].values[i]) {
                i += 1;
            } else {
                values.remove_value(i);
            }
        }
        if values.columns[col_i].values.len() == 1 {
            break;
        }
    }
    println!("wow that was a task and a half num is {}", values.num_at(0));
    values.num_at(0)
}

fn main() {
    part_one();
    let a = part_two(true);
    let b = part_two(false);
    println!("life support is {}", a*b);
}
