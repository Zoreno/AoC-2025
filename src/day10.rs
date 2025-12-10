use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Machine {
    target: u64,
    size: usize,
    buttons: Vec<u64>,
    requirements: Vec<u64>,
}

fn get_matches<'a>(line: &'a str, re: &str) -> Vec<&'a str> {
    Regex::new(re)
        .unwrap()
        .captures_iter(line)
        .map(|c| {
            let (_, [m]) = c.extract();
            m
        })
        .collect()
}

fn target_from_string(line: &str) -> u64 {
    line.chars()
        .enumerate()
        .map(|(i, c)| if c == '#' { 1 << i } else { 0 })
        .sum()
}

fn button_from_string(line: &str) -> u64 {
    line.split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .map(|b| 1 << b)
        .sum()
}

fn buttons_from_strings(lines: &[&str]) -> Vec<u64> {
    lines.iter().map(|s| button_from_string(s)).collect()
}

fn requirements_from_string(line: &str) -> Vec<u64> {
    line.split(',').map(|s| s.parse::<u64>().unwrap()).collect()
}

impl Machine {
    fn from_string(line: &str) -> Self {
        // format: [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}

        let target = get_matches(line, r"\[([.#]+)\]")[0];
        let buttons = get_matches(line, r"\((\d[,\d]*)\)");
        let requirements = get_matches(line, r"\{(\d[,\d]*)\}")[0];

        Machine {
            target: target_from_string(target),
            size: target.len(),
            buttons: buttons_from_strings(&buttons),
            requirements: requirements_from_string(requirements),
        }
    }
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<Machine> {
    input.lines().map(Machine::from_string).collect()
}

#[derive(Clone, Debug)]
struct Node {
    id: u64,
    edges: Vec<u64>,
}

impl Node {
    fn new(id: u64) -> Self {
        Node {
            id,
            edges: Vec::new(),
        }
    }

    fn add_edge(&mut self, edge: u64) {
        self.edges.push(edge);
    }
}

#[derive(Debug)]
struct Graph {
    vertices: Vec<Node>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            vertices: Vec::new(),
        }
    }

    fn add_node(&mut self, vertex: Node) {
        self.vertices.push(vertex);
    }
}

fn build_graph(input: &Machine) -> Graph {
    let mut graph = Graph {
        vertices: Vec::new(),
    };

    for i in 0..(1 << input.size) {
        let mut node = Node::new(i);

        for button in &input.buttons {
            node.add_edge(button ^ i);
        }

        graph.add_node(node);
    }

    graph
}

// Compute cost from the start node to each node using Dijkstra
fn solve_graph(input: &Graph) -> HashMap<u64, u64> {
    let mut costs = HashMap::new();

    // Initialize costs
    for node in &input.vertices {
        if node.id == 0 {
            costs.insert(node.id, 0);
        } else {
            costs.insert(node.id, u64::MAX);
        }
    }

    let mut queue = Vec::new();
    queue.push(0);

    while !queue.is_empty() {
        let current = queue.remove(0);
        let current_cost = *costs.get(&current).unwrap();

        for edge in &input
            .vertices
            .iter()
            .find(|n| n.id == current)
            .unwrap()
            .edges
        {
            let new_cost = current_cost + 1;

            if new_cost < *costs.get(edge).unwrap() {
                costs.insert(*edge, new_cost);
                queue.push(*edge);
            }
        }
    }

    costs
}

fn lowest_cost_to_target(input: &Machine) -> u64 {
    let graph = build_graph(input);
    let costs = solve_graph(&graph);
    *costs.get(&input.target).unwrap()
}

#[aoc(day10, part1)]
fn part1(input: &[Machine]) -> u64 {
    input.iter().map(lowest_cost_to_target).sum()
}
