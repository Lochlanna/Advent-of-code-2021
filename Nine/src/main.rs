use std::cmp::Ordering;
use std::num::ParseIntError;

struct Grid {
    values: Vec<Vec<(u32, bool)>>,
    width: usize
}

impl Grid {
    fn new() -> Grid {
        return Grid {
            values: vec![],
            width: 0
        }
    }
    fn add_row(&mut self, new_row: Vec<u32>) -> bool {
        if self.values.is_empty() && !new_row.is_empty(){
            self.width = new_row.len();
        } else if new_row.len() != self.width || new_row.is_empty() {
            return false;
        }
        let mut row = Vec::with_capacity(new_row.len());
        for col in new_row {
            row.push((col, false));
        }
        self.values.push(row);
        true
    }


    fn get_value(&self, x: usize, y: usize) -> Option<u32> {
        if let Some(row) = self.values.get(y) {
            if let Some(value) = row.get(x) {
                return Some((*value).0);
            }
        }
        None
    }

    fn position_searched(&self, x: usize, y: usize) -> bool {
        if let Some(row) = self.values.get(y) {
            if let Some(value) = row.get(x) {
                return (*value).1;
            }
        }
        false
    }

    fn cmp_locations(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> Option<std::cmp::Ordering> {
        let location_one = match self.get_value(x1, y1) {
            None => return None,
            Some(location) => location
        };

        let location_two = match self.get_value(x2, y2) {
            None => return None,
            Some(location) => location
        };
        return Some(location_one.cmp(&location_two));
    }

    fn diff_locations(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> Option<u32> {
        let location_one = match self.get_value(x1, y1) {
            None => return None,
            Some(location) => location
        };

        let location_two = match self.get_value(x2, y2) {
            None => return None,
            Some(location) => location
        };
        return Some(location_one - location_two);
    }

    fn position_is_low_point(&self, x: usize, y:usize) -> Option<bool> {
        if self.get_value(x, y).is_none() {
            return None;
        }
        if x > 0 {
            if let Some(ord) = self.cmp_locations(x,y, x-1, y) {
                if !ord.is_lt() {
                    return Some(false);
                }
            }
        }
        if let Some(ord) = self.cmp_locations(x,y, x+1, y) {
            if !ord.is_lt() {
                return Some(false);
            }
        }
        if y > 0 {
            if let Some(ord) = self.cmp_locations(x,y, x, y-1) {
                if !ord.is_lt() {
                    return Some(false);
                }
            }
        }
        if let Some(ord) = self.cmp_locations(x,y, x, y+1) {
            if !ord.is_lt() {
                return Some(false);
            }
        }
        Some(true)
    }

    fn set_position_searched(&mut self, x: usize, y:usize) -> Result<(),()> {
        if let Some(row) = self.values.get_mut(y) {
            if let Some(value) = row.get_mut(x) {
                (*value).1 = true;
                return Ok(());
            }
        }
        return Err(());
    }

    fn calculate_basin_size(&mut self, x: usize, y:usize) -> usize {
        if self.position_searched(x,y) {
            return 0;
        }
        if let Some(value) = self.get_value(x,y) {
            if value == 9 {
                return 0;
            }
        } else {
            panic!("This shouldn't happen");
        }
        // this position hasn't been searched so we can continue
        let _ = self.set_position_searched(x,y);
        let mut count = 1;
        if x > 0 {
            if let Some(ord) = self.cmp_locations(x,y, x-1, y) {
                if ord.is_lt() {
                    count += self.calculate_basin_size(x-1, y);
                }
            }
        }
        if let Some(ord) = self.cmp_locations(x,y, x+1, y) {
            if ord.is_lt() {
                count += self.calculate_basin_size(x+1, y);
            }
        }
        if y > 0 {
            if let Some(ord) = self.cmp_locations(x,y, x, y-1) {
                if ord.is_lt() {
                    count += self.calculate_basin_size(x, y-1);
                }
            }
        }
        if let Some(ord) = self.cmp_locations(x,y, x, y+1) {
            if ord.is_lt() {
                count += self.calculate_basin_size(x, y+1);
            }
        }
        count
    }

    fn find_low_point_values(&self) -> Vec<u32> {
        let mut low_points = Vec::new();
        for x in 0..self.width {
            for y in 0..self.values.len() {
                if let Some(result) = self.position_is_low_point(x, y) {
                    if result {
                        match self.get_value(x, y) {
                            None => {},
                            Some(value) => low_points.push(value)
                        };
                    }
                }
            }
        }
        low_points
    }

    fn find_basin_sizes(&mut self) -> Vec<usize> {
        let mut basin_sizes = Vec::new();
        for x in 0..self.width {
            for y in 0..self.values.len() {
                if let Some(result) = self.position_is_low_point(x, y) {
                    if result {
                        let size = self.calculate_basin_size(x,y);
                        basin_sizes.push(size);
                    }
                }
            }
        }
        basin_sizes
    }
}

fn read_input(filename: &str) -> Option<Grid> {
    let file_contents = match std::fs::read_to_string(filename) {
        Ok(fc) => fc,
        Err(_) => {
            return None;
        }
    };
    let mut grid = Grid::new();
    for (y, line) in file_contents.lines().enumerate() {
        let mut row = Vec::new();
        for (x, value) in line.chars().enumerate() {
            let parsed = match value.to_digit(10) {
                Some(p) => p,
                None => {
                    return None;
                }
            };
            row.push(parsed);
        }
        grid.add_row(row);
    }
    Some(grid)
}




fn main() {
    let mut grid = match read_input("input") {
        None => panic!("Couldn't read input"),
        Some(grid) => grid
    };
    let low_points = grid.find_low_point_values();
    let mut risk = low_points.len() as u32;
    for point in low_points {
        risk += point;
    }
    let mut basin_sizes = grid.find_basin_sizes();
    basin_sizes.sort();
    basin_sizes.reverse();
    let n = if basin_sizes.len() >= 3 {3} else {basin_sizes.len()};
    let mut basin_size_score = 1;
    for i in 0..n {
        basin_size_score = basin_size_score * basin_sizes[i];
    }
    println!("The risk is {} basins are {:?}, basin size score is {}", risk, basin_sizes, basin_size_score);
}
