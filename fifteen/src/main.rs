use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeSet};
use std::env;
use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};
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
    previous: (usize, usize),
    distance: usize,
    risk: u8
}

impl Default for Node {
    fn default() -> Self {
        Node {
            previous: (0,0),
            distance: usize::MAX,
            risk: 0
        }
    }
}

impl Node {
    fn new(risk:u8) -> Self {
        Node {
            previous: (0,0),
            distance: usize::MAX,
            risk
        }
    }
    fn clone_with_risk(&self, risk:u8) -> Self {
        Node {
            previous: self.previous,
            distance: self.distance,
            risk
        }
    }
}

#[derive(Debug, PartialEq)]
struct SearchToken {
    x: usize,
    y: usize,
    distance: usize
}

impl SearchToken {
    pub fn new(x: usize, y: usize, distance: usize) -> Self {
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
    data: Vec<Vec<Node>>
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.data.is_empty() {
            return f.write_str("");
        }
        let mut map_str = String::with_capacity(self.data.len() * self.data.first().unwrap().len());
        for row in &self.data {
            for node in row {
                map_str.push_str(node.risk.to_string().as_str());
            }
            map_str.push('\n');
        }
        f.write_str(map_str.as_str())
    }
}

impl Map {
    fn new() -> Self {
        Map {
            data: vec![]
        }
    }
    fn insert_row(&mut self, row: Vec<Node>) {
        self.data.push(row);
    }
    fn search(&mut self, start_x: usize, start_y: usize, end_x: usize, end_y: usize) -> bool {
        let start_node = self.get_mut(start_x, start_y).expect("couldn't get the start node");
        start_node.distance = 0;
        let mut path_q:BinaryHeap<SearchToken> = BinaryHeap::new();
        path_q.push(SearchToken::new(start_x, start_y, 0));
        while let Some(closest_token) = path_q.pop() {
            if closest_token.x == end_x && closest_token.y == end_y {
                println!("found it");
                return true; // we have found the shortest path
            }
            for direction in Direction::iter() {
                match self.get_by_direction(closest_token.x, closest_token.y, direction) {
                    None => {} //nothing in this direction
                    Some((node, x, y)) => {
                        if node.distance >= closest_token.distance + node.risk as usize {
                            node.distance = closest_token.distance + node.risk as usize;
                            node.previous = (closest_token.x, closest_token.y);
                            let new_token = SearchToken::new(x,y,node.distance);
                            path_q.push(new_token);
                        }
                        // else there is already a better way of getting here!
                    }
                };
            }
        }
        false
    }

    fn get(&self, x:usize, y: usize) -> Option<&Node> {
        self.data.get(y).and_then(|row| row.get(x))
    }

    fn get_mut(&mut self, x:usize, y: usize) -> Option<&mut Node> {
        self.data.get_mut(y).and_then(|row| row.get_mut(x))
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
        (self.data.first().unwrap().len(), self.data.len())
    }

    fn tile(&mut self, x:usize, y:usize) {
        let mut tiled_horizontal = Vec::with_capacity(self.data.len());
        for row in &self.data {
            let mut new_row = Vec::with_capacity(row.len() * x);
            for i in 0..x {
                for value in row {
                    let mut new_risk = value.risk as usize + i;
                    if new_risk >= 10 {
                        new_risk = (new_risk%10) + 1;
                    }
                    new_row.push(value.clone_with_risk(new_risk as u8));
                }
            }
            tiled_horizontal.push(new_row);
        }
        let mut new_data = Vec::new();
        for i in 0..y {
            for row in &tiled_horizontal {
                let mut new_row = Vec::new();
                for node in row {
                    let mut new_risk = node.risk as usize + i;
                    if new_risk >= 10 {
                        new_risk = (new_risk%10) + 1;
                    }
                    new_row.push(node.clone_with_risk(new_risk as u8));
                }
                new_data.push(new_row);
            }
        }
        self.data = new_data;
    }
}

fn read_input(filename:&str) -> Map {
    let file_contents = std::fs::read_to_string(filename).expect("couldn't read file");
    let lines: Vec<&str> = file_contents.lines().collect();
    if lines.is_empty() {
        panic!("couldn't read input");
    }
    let mut map = Map::new();
    for line in file_contents.lines(){
        let mut row = Vec::with_capacity(line.len());
        for c in line.chars() {
            let parsed = c.to_digit(10).expect("couldn't parse character to digit") as u8;
            row.push(Node::new(parsed));
        }
        map.insert_row(row);
    }
    map
}

fn get_path(map: Map, end_x: usize, end_y:usize) -> (usize, Vec<(usize, usize)>) {
    let mut x = end_x-1;
    let mut y = end_y-1;
    let mut current_node = map.get(x, y).expect("couldn't get end node");
    let distance = current_node.distance;
    let mut path = vec![(x, y)];
    while let (x, y) = current_node.previous {
        current_node = map.get(x, y).expect("couldn't get the next node along path");
        path.push((x,y));
        if x == 0 && y == 0 {
            break;
        }

    }
    (distance, path)
}

fn problem_one() {
    let now = Instant::now();
    let args:Vec<String> = env::args().collect();
    let mut map = read_input(if args.len() != 2 {"input_small"} else {args[1].as_str()});
    let (end_x, end_y) = map.dimensions();
    if !map.search(0,0, end_x, end_y) {
        println!("couldn't find a path");
    }
    let (distance, path) = get_path(map, end_x, end_y);
    let dur = now.elapsed();
    println!("distance was {} in {} steps time taken was {} seconds or {} microseconds", distance, path.len(), dur.as_secs_f64(), dur.as_micros());
}
fn problem_two() {
    let now = Instant::now();
    let args:Vec<String> = env::args().collect();
    let mut map= read_input(if args.len() != 2 {"input_small"} else {args[1].as_str()});
    map.tile(5,5);
    let (end_x, end_y) = map.dimensions();
    if !map.search(0,0, end_x, end_y) {
        println!("couldn't find a path");
    }
    let (distance, path) = get_path(map, end_x, end_y);
    let dur = now.elapsed();
    println!("distance was {} in {} steps time taken was {} seconds or {} microseconds", distance, path.len(), dur.as_secs_f64(), dur.as_micros());
}

fn main() {
    problem_one();
    problem_two();
}
