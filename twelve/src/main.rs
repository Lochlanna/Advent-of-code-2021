use std::collections::{HashMap, HashSet};
use std::collections::hash_map::DefaultHasher;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ptr::hash;
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

    fn new(double_small_allowed: bool) -> Path {
        Path{
            path: vec![],
            nodes_in_path: Default::default(),
            double_small: 0,
            double_small_set: false,
            double_small_allowed
        }
    }
    fn append_node(&mut self, node: &Node) -> Result<(), ()> {
        if !string_is_upper(node.name.as_str()) && self.node_in_path(node.hash) {
            if !node.is_start && self.double_small_allowed && !self.double_small_set {
                self.double_small = node.hash;
                self.double_small_set = true;
            } else {
                return Err(());
            }
        }
        self.path.push(node.hash);
        self.nodes_in_path.insert(node.hash);
        Ok(())
    }
    fn node_in_path(&self, node_hash: u64) -> bool {
        self.nodes_in_path.contains(&node_hash)
    }
    fn is_empty(&self) -> bool {
        self.path.is_empty()
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

    fn dehash_path(&self, path: &Path) -> Result<Vec<String>, ()> {
        let mut dehashed = Vec::new();
        for node_hash in &path.path {
            if let Some(node) = self.nodes.get(node_hash) {
                dehashed.push(node.name.clone());
            } else {
                return Err(());
            }
        }
        Ok(dehashed)
    }
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

    fn traverse_from_node(&self, node_hash: u64, mut path: Path, end_hash: u64) -> Option<Vec<Path>> {
        let node = match self.nodes.get(&node_hash) {
            None => panic!("couldn't find node {}", node_hash),
            Some(node) => node
        };
        if path.append_node(node).is_err() {
            //We are not allowed to append this node to the path so this path is a dead end
            return None;
        }
        if node_hash == end_hash {
            //we have found the end!
            return Some(vec![path]);
        }
        let mut paths = Vec::new();
        for connected_node in &node.connections {
            if let Some(mut new_paths) = self.traverse_from_node(*connected_node, path.clone(), end_hash) {
                paths.append(&mut new_paths);
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
    let map = read_input("input");
    let start_hash = default_hash("start");
    let end_hash = default_hash("end");
    let paths_to_end = match map.traverse_from_node(start_hash, Path::new(double_small_allowed), end_hash) {
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
