use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::env;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::time::Instant;
use ndarray::{Array2, ArrayView};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

#[derive(Debug, Clone, Copy)]
struct Node {
    distance: usize,
    risk: u8,
    searched: bool
}

impl Default for Node {
    fn default() -> Self {
        Node::new(0)
    }
}

impl Node {
    fn new(risk:u8) -> Self {
        Node {
            distance: usize::MAX,
            risk,
            searched: false
        }
    }
    fn clone_with_risk(&self, risk:u8) -> Self {
        Node {
            distance: self.distance,
            risk,
            searched: false
        }
    }
}

#[derive(Debug, PartialEq)]
struct SearchToken {
    x: usize,
    y: usize,
    distance: u32
}

impl SearchToken {
    pub fn new(x: usize, y: usize, distance: u32) -> Self {
        SearchToken { x, y, distance }
    }
}
impl Eq for SearchToken {}

impl PartialOrd<Self> for SearchToken {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchToken {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

struct Map {
    data: ndarray::Array2<Node>
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.data.is_empty() {
            return f.write_str("");
        }
        let mut map_str = String::with_capacity(self.data.len() * self.data.ncols());
        for row in self.data.rows() {
            for node in row {
                map_str.push_str(node.risk.to_string().as_str());
            }
            map_str.push('\n');
        }
        f.write_str(map_str.as_str())
    }
}


impl Map {
    fn new(data: Vec<Node>, n_rows:usize, n_cols:usize) -> Self {
        Map {
            data: Array2::from_shape_vec((n_rows,n_cols), data).expect("Couldn't create array")
        }
    }
    fn search(&mut self, start_x: usize, start_y: usize, end_x: usize, end_y: usize) -> Option<usize> {
        let start_node = self.get_mut(start_x, start_y).expect("couldn't get the start node");
        start_node.distance = 0;
        let mut path_q:BinaryHeap<SearchToken> = BinaryHeap::with_capacity(10000);
        path_q.push(SearchToken::new(start_x, start_y, 0));
        while let Some(closest_token) = path_q.pop() {
            let current_node = self.get_mut(closest_token.x, closest_token.y).expect("Couldn't find node for token");
            if closest_token.x == end_x-1 && closest_token.y == end_y-1 {
                return Some(current_node.distance); // we have found the shortest path
            }
            current_node.searched = true;
            let current_dist = current_node.distance;
            for direction in Direction::iter() {
                match self.get_by_direction(closest_token.x, closest_token.y, direction) {
                    None => {} //nothing in this direction
                    Some((node, x, y)) => {
                        if !node.searched && node.distance >= current_dist + node.risk as usize {
                            node.distance = current_dist + node.risk as usize;
                            let distance_to_end = (((x as i32 - end_x as i32).pow(2) + (y as i32 - end_y as i32).pow(2)) as f32).sqrt();
                            let new_token = SearchToken::new(x,y,distance_to_end as u32 + node.distance as u32);
                            path_q.push(new_token);
                        }
                    }
                };
            }
        }
        None
    }

    fn get_mut(&mut self, x:usize, y: usize) -> Option<&mut Node> {
        self.data.get_mut((y,x))
    }

    fn get_by_direction(&mut self, current_x: usize, current_y: usize, direction:Direction) -> Option<(&mut Node, usize, usize)> {
        let mut x = current_x;
        let mut y = current_y;
        match direction {
            Direction::Left => {
                if current_x <= 0 {
                    return None;
                }
                x-=1;
            }
            Direction::Right => x+=1,
            Direction::Up => {
                if current_y <= 0 {
                    return None;
                }
                y-=1;
            }
            Direction::Down => y+=1
        }
        self.get_mut(x, y).and_then(|n| Some((n, x, y)))
    }

    fn dimensions(&self)->(usize,usize) {
        if self.data.is_empty() {
            return (0,0);
        }
        (self.data.ncols(), self.data.nrows())
    }

    fn tile(&mut self, x:usize, y:usize) {
        let original_num_rows = self.data.nrows();
        let original_num_columns = self.data.ncols();
        let mut new_data = Vec::with_capacity(original_num_rows*y *original_num_columns*x);
        for v in 0..(original_num_rows*y) {
            for h in 0..original_num_columns*x {
                let vertical_addition = v / original_num_rows; // int division rounds down
                let horizontal_addition = h / original_num_columns;
                let x = h % original_num_columns;
                let y = v % original_num_rows;
                let node = self.data.get((y,x)).expect("Couldn't get node from original data");
                let mut new_risk = node.risk as usize + vertical_addition + horizontal_addition;
                if new_risk >= 10 {
                    new_risk = (new_risk%10) + 1;
                }
                new_data.push(node.clone_with_risk(new_risk as u8));
            }
        }
        self.data = Array2::from_shape_vec((original_num_rows*y, original_num_columns*x), new_data).expect("Couldn't create arr2 from flattened")
    }
}

fn read_input(filename:&str) -> Map {
    let file_contents = std::fs::read_to_string(filename).expect("couldn't read file");
    let mut data = Vec::new();
    let mut width = 0;
    let mut row_count = 0;
    for line in file_contents.lines(){
        for c in line.chars() {
            let parsed = c.to_digit(10).expect("couldn't parse character to digit") as u8;
            data.push(Node::new(parsed));
        }
        width = line.len();
        row_count+= 1;
    }
    Map::new(data, row_count, width)
}
fn problem_one() {
    let now = Instant::now();
    let args:Vec<String> = env::args().collect();
    let mut map = read_input(if args.len() != 2 {"input"} else {args[1].as_str()});
    let (end_x, end_y) = map.dimensions();

    if let Some(distance) = map.search(0,0, end_x, end_y) {
        println!("distance was {}", distance);
    } else {
        println!("couldn't find a path");
    }
    let dur = now.elapsed();
    println!("time taken was {} seconds or {} microseconds", dur.as_secs_f64(), dur.as_micros());

}
fn problem_two() {
    let now = Instant::now();
    let args:Vec<String> = env::args().collect();
    let mut map= read_input(if args.len() != 2 {"input"} else {args[1].as_str()});
    map.tile(5,5);
    let (end_x, end_y) = map.dimensions();
    if let Some(distance) = map.search(0,0, end_x, end_y) {
        println!("distance was {}", distance);
    } else {
        println!("couldn't find a path");
    }
    let dur = now.elapsed();
    println!("time taken was {} seconds or {} microseconds", dur.as_secs_f64(), dur.as_micros());
}

fn main() {
    problem_one();
    problem_two();
}
