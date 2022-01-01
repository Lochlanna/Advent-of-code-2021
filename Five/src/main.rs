use std::fs;
use std::panic;

struct Line {
    start_x: u32,
    start_y: u32,
    end_x: u32,
    end_y: u32,
}

impl Line {
    fn new(start_x:u32, start_y: u32, end_x: u32, end_y: u32) -> Line {
        Line {
            start_x,
            start_y,
            end_x,
            end_y
        }
    }
    fn iter(&self) -> LineIterator {
        let vertical = self.start_x == self.end_x;
        let horizontal = self.start_y == self.end_y;
        let num_steps;
        if vertical {
            num_steps = (self.end_y as i32 - self.start_y as i32).unsigned_abs();
        } else {
            num_steps = (self.end_x as i32 - self.start_x as i32).unsigned_abs();
        }
        let diagonal_direction = DiagonalDirection::from_points(self.start_x, self.start_y, self.end_x, self.end_y);
        LineIterator {
            line: self,
            step: 0,
            num_steps,
            vertical,
            horizontal,
            diagonal_direction
        }
    }
}
enum DiagonalDirection {
    NE,
    SE,
    SW,
    NW
}

impl DiagonalDirection {
    fn from_points(x1: u32, y1: u32, x2: u32, y2:u32) -> Option<DiagonalDirection> {
        if x1 == x2 || y1 == y2 {
            return None
        }
        return if x1 < x2 {
            //east
            if y1 < y2 {
                //north
                Some(DiagonalDirection::NE)
            } else {
                //south
                Some(DiagonalDirection::SE)
            }
        } else {
            //west
            if y1 < y2 {
                //north
                Some(DiagonalDirection::NW)
            } else {
                //south
                Some(DiagonalDirection::SW)
            }
        }
    }
}

struct LineIterator<'a> {
    line : &'a Line,
    step: u32,
    num_steps: u32,
    vertical: bool,
    horizontal: bool,
    diagonal_direction: Option<DiagonalDirection>
}

impl Iterator for LineIterator<'_> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.step > self.num_steps {
            return None
        }
        let current_step = self.step;
        self.step += 1;
        return if self.vertical {
            return if self.line.start_y < self.line.end_y {
                Some((self.line.start_x, self.line.start_y + current_step))
            } else {
                Some((self.line.start_x, self.line.start_y - current_step))
            }
        } else if self.horizontal {
            return if self.line.start_x < self.line.end_x {
                Some((self.line.start_x + current_step, self.line.start_y))
            } else {
                Some((self.line.start_x - current_step, self.line.start_y))
            }
        } else {
            // diagonal
            match &self.diagonal_direction {
                None => panic! ("Line is not horizontal, vertical or diagonal..."),
                Some(dd) => {
                    return match dd {
                        DiagonalDirection::NE => {
                            Some((self.line.start_x + current_step, self.line.start_y + current_step))
                        }
                        DiagonalDirection::SE => {
                            Some((self.line.start_x + current_step, self.line.start_y - current_step))
                        }
                        DiagonalDirection::SW => {
                            Some((self.line.start_x - current_step, self.line.start_y - current_step))
                        }
                        DiagonalDirection::NW => {
                            Some((self.line.start_x - current_step, self.line.start_y + current_step))
                        }
                    }
                }
            }
        }
    }
}

fn read_input(filename: &str, diagonal: bool) -> (Vec<Line>, u32, u32) {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut lines = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        if parts.len() != 2 {
            panic!("A line has more than two elements");
        }
        let start_point_parts: Vec<&str> = parts[0].split(",").collect();
        if start_point_parts.len() != 2 {
            panic!("A coordinate has too many elements");
        }
        let start_x = start_point_parts[0].parse::<u32>().expect("Couldn't parse input");
        if start_x > max_x {
            max_x = start_x;
        }
        let start_y = start_point_parts[1].parse::<u32>().expect("Couldn't parse input");
        if start_y > max_y {
            max_y = start_y;
        }

        let end_point_parts: Vec<&str> = parts[1].split(",").collect();
        if end_point_parts.len() != 2 {
            panic!("A coordinate has too many elements");
        }
        let end_x = end_point_parts[0].parse::<u32>().expect("Couldn't parse input");
        if end_x > max_x {
            max_x = end_x;
        }
        let end_y = end_point_parts[1].parse::<u32>().expect("Couldn't parse input");
        if end_y > max_y {
            max_y = end_y;
        }

        if diagonal || end_y == start_y || end_x == start_x {
            lines.push(Line{
                start_x,
                start_y,
                end_x,
                end_y
            });
        }
    }
    (lines, max_x, max_y)
}

fn do_test(lines: Vec<Line>, mut max_x: u32, mut max_y: u32) {
    max_x += 1;
    max_y += 1;
    let mut board = Vec::with_capacity(max_x as usize);
    for i in 0..max_x {
        let mut column: Vec<u8> = Vec::new();
        column.resize(max_y as usize, 0);
        board.push(column);
    }
    let mut overlap_counter = 0;
    for (i, line) in lines.iter().enumerate() {
        // println!("start line {}", i);
        for point in line.iter() {
            // println!("point is {},{}", point.0, point.1);
            board[point.0 as usize][point.1 as usize] += 1;
            if board[point.0 as usize][point.1 as usize] == 2 {
                overlap_counter += 1;
            }
        }
    }
    let mut debug_str = String::new();
    for x in 0..max_x as usize {
        for y in 0..max_y as usize {
            debug_str += board[y][x].to_string().as_str();
            debug_str += " ";
        }
        debug_str += "\n"
    }
    // println!("{}\nThere were {} overlaps", debug_str, overlap_counter)
    println!("There were {} overlaps", overlap_counter);
}

fn main() {
    let (lines , mut max_x, mut max_y) = read_input("input", false);
    do_test(lines, max_x, max_y);
    let (lines , mut max_x, mut max_y) = read_input("input", true);
    do_test(lines, max_x, max_y);
}
