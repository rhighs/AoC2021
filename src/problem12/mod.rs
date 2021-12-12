use std::collections::HashMap;
use std::collections::HashSet;

struct Node {
    name: String,
    is_upper: bool
}

impl Node {
    fn new(name: String) -> Self {
        let is_upper = !name.chars().any(|c| matches!(c, 'a'..='z'));
        Self { name, is_upper }
    }
}

struct Graph {
    edges: HashMap<String, Vec<Node>>
}

impl Graph {
    fn new() -> Self {
        Self { edges: HashMap::new() }
    }
}

fn make_graph(data: &Vec<String>) -> Graph {
    let mut graph = Graph::new();
    for string in data {
        let edge = string.split("-").collect::<Vec<&str>>();
        graph.edges
            .entry(edge[0].to_string())
            .or_default()
            .push(Node::new(edge[1].to_string()));
        graph.edges
            .entry(edge[1].to_string())
            .or_default()
            .push(Node::new(edge[0].to_string()));
    }
    graph
}

fn paths_1(graph: &Graph, current_path: Vec<String>, found_paths: &mut Vec<Vec<String>>) {
    let last = &current_path[current_path.len() - 1];
    for node in graph.edges.get(last).unwrap() {
        let mut new_paths = current_path.clone();
        new_paths.push(node.name.clone());

        if node.name == "end" {
            found_paths.push(new_paths);
            continue;
        }

        if !node.is_upper && current_path.contains(&node.name) {
            continue;
        }

        paths_1(graph, new_paths, found_paths);
    }
}

fn paths_2(graph: &Graph, current_path: Vec<String>, has_double: bool, found_paths: &mut Vec<Vec<String>>) {
    let last = &current_path[current_path.len() - 1];
    for node in graph.edges.get(last).unwrap() {
        if node.name == "start" {
            continue;
        }

        let mut new_paths = current_path.clone();
        new_paths.push(node.name.clone());

        if node.name == "end" {
            found_paths.push(new_paths);
            continue;
        }

        if !node.is_upper && has_double && current_path.contains(&node.name) {
            continue;
        }

        let only_lower = new_paths
            .iter()
            .filter(|node_name| node_name.chars().any(|c| matches!(c, 'a'..='z')))
            .cloned()
            .collect::<Vec<String>>();

        let this_has_double = only_lower.len() != HashSet::<&String>::from_iter(only_lower.iter()).len();
        paths_2(graph, new_paths, this_has_double, found_paths);
    }
}

pub fn p12_1(data: &Vec<String>) -> u32 {
    let graph = make_graph(data);
    let mut found_paths = Vec::new();
    let current_path = Vec::from([String::from("start")]);
    paths_1(&graph, current_path, &mut found_paths);
    found_paths.len() as u32
}

pub fn p12_2(data: &Vec<String>) -> u32 {
    let graph = make_graph(data);
    let mut found_paths = Vec::new();
    let current_path = Vec::from([String::from("start")]);
    paths_2(&graph, current_path, false, &mut found_paths);
    found_paths.len() as u32
}

