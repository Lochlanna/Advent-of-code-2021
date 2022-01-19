use std::collections::{HashMap, HashSet};
use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::str::Split;
use std::time::Instant;


fn default_hash<T>(obj: T) -> u64
    where
        T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

fn string_is_upper(test_string: &str) -> bool {
    if test_string.is_empty() {
        return false;
    }
    for c in test_string.chars() {
        if c.is_lowercase() {
            return false
        }
    }
    true
}



#[derive(Clone, Debug)]
struct Path {
    path: Vec<u64>,
    nodes_in_path: HashSet<u64>,
    double_small:u64,
    double_small_set:bool,
    double_small_allowed:bool
}

impl Path {

    fn new(first_node: &Node, double_small_allowed: bool) -> Path {
        let mut path = Path{
            path: vec![first_node.hash],
            nodes_in_path: Default::default(),
            double_small: 0,
            double_small_set: false,
            double_small_allowed
        };
        path.nodes_in_path.insert(first_node.hash);
        path
    }
    fn append_node_to_clone(&self, node: &Node) -> Option<Self> {
        let mut cloned;
        if !string_is_upper(node.name.as_str()) && self.node_in_path(node.hash) {
            if !node.is_start && self.double_small_allowed && !self.double_small_set {
                cloned = self.clone();
                cloned.double_small = node.hash;
                cloned.double_small_set = true;
            } else {
                return None;
            }
        } else {
            cloned = self.clone();
        }
        cloned.path.push(node.hash);
        cloned.nodes_in_path.insert(node.hash);
        Some(cloned)
    }
    fn node_in_path(&self, node_hash: u64) -> bool {
        self.nodes_in_path.contains(&node_hash)
    }
}

#[derive(Debug)]
struct Node {
    is_start: bool,
    name: String,
    hash: u64,
    connections: Vec<u64>
}

#[derive(Debug)]
struct Map {
    nodes: HashMap<u64, Node>
}

fn hash_from_path_part(part_iter: &mut Split<&str>) -> Option<(u64, String)> {
    match part_iter.next() {
        None => None,
        Some(part) => {
            if part.is_empty() {
                return None;
            }
            Some((default_hash(part), part.to_string()))
        }
    }
}

impl Map {
    fn read_path(&mut self, path: &str) {
        let mut parts = path.split("-");
        let node_a = match hash_from_path_part(&mut parts) {
            None => return,
            Some(part) => part
        };
        let node_b = match hash_from_path_part(&mut parts) {
            None => return,
            Some(part) => part
        };
        
        let a_is_start = node_a.1 == "start";
        let b_is_start = node_b.1 == "start";
        

        match self.nodes.get_mut(&node_a.0) {
            None => {
                self.nodes.insert(node_a.0, Node { is_start: a_is_start, name: node_a.1, hash: node_a.0, connections: vec![node_b.0] });
            },
            Some(start_node) => {
                start_node.connections.push(node_b.0);
            }
        };
        match self.nodes.get_mut(&node_b.0) {
            None => {
                self.nodes.insert(node_b.0, Node { is_start: b_is_start, name: node_b.1, hash: node_b.0, connections: vec![node_a.0] });
            },
            Some(end_node) => {
                end_node.connections.push(node_a.0);
            }
        };
    }

    fn traverse_from_node(&self, node: &Node, path: Path, end_hash: u64) -> Option<Vec<Path>> {
        if node.hash == end_hash {
            return Some(vec![path]);
        }
        let mut paths = Vec::new();
        for connected_node_hash in &node.connections {
            let connected_node = match self.nodes.get(connected_node_hash) {
                None => panic!("Connected node doesn't exist"),
                Some(connected_node) => connected_node
            };
            if let Some(new_path) = path.append_node_to_clone(connected_node) {
                if let Some(mut new_paths) = self.traverse_from_node(connected_node, new_path, end_hash) {
                    paths.append(&mut new_paths);
                }
            }
        }
        if paths.is_empty() {
            return None;
        }
        Some(paths)
    }
}

fn read_input(filename: &str) -> Map {
    let mut map = Map{ nodes: Default::default() };
    let file_contents = match std::fs::read_to_string(filename) {
        Ok(fc) => fc,
        Err(_) => panic!("couldn't read the input file")
    };
    for line in file_contents.lines() {
        map.read_path(line);
    }
    map
}

fn do_problem(double_small_allowed: bool) -> usize{
    let args: Vec<String> = env::args().collect();
    let map = read_input(if args.len() < 2 {"input"} else {args[1].as_str()});
    let start_hash = default_hash("start");
    let end_hash = default_hash("end");
    let start_node = map.nodes.get(&start_hash).expect("couldn't find the start node");
    let paths_to_end = match map.traverse_from_node(start_node, Path::new(start_node, double_small_allowed), end_hash) {
        None => return 0,
        Some(paths) => paths
    };

    paths_to_end.len()
}

fn main() {
    //problem one
    println!("problem one: {}", do_problem(false));

    //problem two
    let now = Instant::now();
    println!("problem two: {} time taken was {} seconds", do_problem(true), now.elapsed().as_secs_f64());
}
