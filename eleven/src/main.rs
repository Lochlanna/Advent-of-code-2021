use std::fmt::{Debug, Formatter};

struct Jellyfish {
    energy_level:u8,
    last_flashed:usize
}

impl Debug for Jellyfish {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.energy_level.to_string().as_str())
    }
}

impl Jellyfish {
    fn new(energy_level: u8) -> Jellyfish {
        Jellyfish {
            energy_level,
            last_flashed: 0
        }
    }
}

struct Grid {
    data: Vec<Vec<Jellyfish>>,
    width: usize
}

impl Grid {
    fn new() -> Grid {
        Grid {
            data: vec![],
            width: 0
        }
    }

    fn insert_row(&mut self, new_row: Vec<Jellyfish>) -> Result<(),()> {
        if new_row.is_empty() {
            return Err(());
        }
        if self.width == 0 {
            self.width = new_row.len();
        }
        if self.width == new_row.len() {
            self.data.push(new_row);
            return Ok(());
        }
        Err(())
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Jellyfish> {
        if let Some(row) = self.data.get_mut(y) {
            return row.get_mut(x);
        }
        None
    }
}

fn increase_power(grid: &mut Grid, x: usize, y: usize, step:usize) -> u32 {
    let jellyfish = match grid.get_mut(x,y) {
        None => return 0,
        Some(jf) => jf
    };
    if jellyfish.last_flashed == step {
        return 0;
    }
    if jellyfish.energy_level < 9 {
        jellyfish.energy_level += 1;
        return 0;
    }
    let mut flash_count:u32 = 1;
    jellyfish.energy_level = 0;
    jellyfish.last_flashed = step;
    flash_count += increase_power(grid, x + 1,y, step); // east
    flash_count += increase_power(grid, x + 1,y + 1, step); // north east
    flash_count += increase_power(grid, x,y + 1, step); // north
    if x > 0 {
        flash_count += increase_power(grid, x - 1,y + 1, step); // north west
        flash_count += increase_power(grid, x - 1, y, step); // west
    }
    if x > 0 && y > 0 {
        flash_count += increase_power(grid, x - 1,y - 1, step); // south west
    }
    if y > 0 {
        flash_count += increase_power(grid, x,y - 1, step); // south
        flash_count += increase_power(grid, x + 1,y - 1, step); // south east
    }
    flash_count
}

fn step(grid: &mut Grid, step_number: usize) -> u32 {
    let mut flash_count = 0;
    for y in 0..grid.data.len() {
        for x in 0..grid.width {
            flash_count += increase_power(grid, x, y, step_number);
        }
    }
    flash_count
}

fn read_input(filename: &str) -> Grid {
    let file_contents = match std::fs::read_to_string(filename) {
        Ok(fc) => fc,
        Err(_) => panic!("Couldn't read in the input file")
    };
    let mut grid = Grid::new();
    for line in file_contents.lines() {
        let mut row = Vec::with_capacity(line.len());
        for c in line.chars() {
            let energy_level = c.to_digit(10).unwrap_or_else(|| panic!("Couldn't parse the character"));
            row.push(Jellyfish::new(energy_level as u8));
        }
        let _ = grid.insert_row(row);
    }
    grid
}

fn main() {
    let mut input = read_input("input");
    let n_jellyfish = (input.width * input.data.len()) as u32;
    let mut flash_count = 0;
    let mut index = 1;
    loop {
        let fc = step(&mut input, index);
        if fc == n_jellyfish {
            println!("After {} iterations all the jellyfish ({}) flashed", index, n_jellyfish);
            break;
        }
        flash_count += fc;
        if index == 100 {
            println!("There were {} flashes after 100 iterations", flash_count);
        }
        index += 1;
    }
}
