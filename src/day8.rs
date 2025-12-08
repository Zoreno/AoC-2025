use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Vertex {
    x: u64,
    y: u64,
    z: u64,
}

impl Vertex {
    fn from_string(s: &str) -> Self {
        let (x, y, z) = s
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Vertex::new(x, y, z)
    }

    fn new(x: u64, y: u64, z: u64) -> Self {
        Vertex { x, y, z }
    }

    fn distance(&self, other: &Vertex) -> f64 {
        let sum = self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2);

        (sum as f64).sqrt()
    }
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Vec<Vertex> {
    input.lines().map(Vertex::from_string).collect()
}

#[derive(Clone, Debug)]
struct Edge {
    a: Vertex,
    b: Vertex,
    distance: f64,
}

impl Edge {
    fn from_points(a: Vertex, b: Vertex) -> Self {
        let distance = a.distance(&b);
        Edge { a, b, distance }
    }
}

#[derive(Debug)]
struct Component {
    vertices: HashSet<Vertex>,
    edges: Vec<Edge>,
}

fn compute_weakly_linked_components(vertices: &[Vertex], edges: &[Edge]) -> Vec<Component> {
    let mut adj: HashMap<Vertex, Vec<Vertex>> = HashMap::new();

    for v in vertices {
        adj.insert(v.clone(), Vec::new());
    }

    let mut all_edges: Vec<Edge> = Vec::with_capacity(edges.len());

    for edge in edges {
        adj.entry(edge.a.clone())
            .or_insert(Vec::new())
            .push(edge.b.clone());

        adj.entry(edge.b.clone())
            .or_insert(Vec::new())
            .push(edge.a.clone());

        all_edges.push(edge.clone());
    }

    let mut visited: HashSet<Vertex> = HashSet::new();
    let mut components = Vec::new();

    let all_vertices: Vec<Vertex> = adj.keys().cloned().collect();

    for start in all_vertices {
        if visited.contains(&start) {
            continue;
        }

        let mut queue = VecDeque::new();
        queue.push_back(start);

        let mut comp_vertices: HashSet<Vertex> = HashSet::new();
        let mut comp_edges: Vec<Edge> = Vec::new();

        while let Some(u) = queue.pop_front() {
            if !visited.insert(u.clone()) {
                continue;
            }

            comp_vertices.insert(u.clone());

            if let Some(neighs) = adj.get(&u) {
                for n in neighs {
                    if !visited.contains(n) {
                        queue.push_back(n.clone());
                    }
                }
            }

            for edge in &all_edges {
                if comp_vertices.contains(&edge.a) && comp_vertices.contains(&edge.b) {
                    comp_edges.push(edge.clone());
                }
            }
        }

        components.push(Component {
            vertices: comp_vertices,
            edges: comp_edges,
        });
    }

    components
}

fn get_edge_list(input: &[Vertex]) -> Vec<Edge> {
    input
        .iter()
        .combinations(2)
        .map(|p| Edge::from_points(p[0].clone(), p[1].clone()))
        .sorted_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
        .collect()
}

#[aoc(day8, part1)]
fn part1(input: &[Vertex]) -> usize {
    let mut edges: Vec<Edge> = get_edge_list(input);
    edges.truncate(1000);

    let components = compute_weakly_linked_components(&input, &edges);
    let mut components = components.iter().map(|c| c.vertices.len()).collect_vec();
    components.sort();
    components.iter().rev().take(3).product::<usize>()
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Node {
    vertices: HashSet<Vertex>,
}

impl Node {
    fn new() -> Self {
        Node {
            vertices: HashSet::new(),
        }
    }

    fn new_from(vertex: &Vertex) -> Self {
        Node {
            vertices: HashSet::from([vertex.clone()]),
        }
    }

    fn combine(a: &Node, b: &Node) -> Self {
        let vertices = a.vertices.union(&b.vertices).cloned().collect();

        Node { vertices }
    }

    fn contains(&self, vertex: &Vertex) -> bool {
        self.vertices.contains(vertex)
    }
}

struct NodeList {
    nodes: Vec<Node>,
}

impl NodeList {
    fn new(nodes: &Vec<Vertex>) -> Self {
        NodeList {
            nodes: nodes.iter().map(Node::new_from).collect(),
        }
    }

    fn add_edge(&mut self, edge: Edge) {
        let node_a_idx = self.nodes.iter().position(|n| n.contains(&edge.a)).unwrap();
        let node_b_idx = self.nodes.iter().position(|n| n.contains(&edge.b)).unwrap();

        // Both nodes belong to the same tree.
        if node_a_idx == node_b_idx {
            return;
        }

        let node_a = &self.nodes[node_a_idx];
        let node_b = &self.nodes[node_b_idx];

        let combined = Node::combine(&node_a, &node_b);

        self.nodes.remove(max(node_a_idx, node_b_idx));
        self.nodes.remove(min(node_a_idx, node_b_idx));
        self.nodes.push(combined);
    }

    fn get_len(&self) -> usize {
        self.nodes.len()
    }
}

#[aoc(day8, part2)]
fn part2(input: &Vec<Vertex>) -> u64 {
    let edges = get_edge_list(&input);

    let mut node_list = NodeList::new(input);

    for edge in edges {
        node_list.add_edge(edge.clone());

        if node_list.get_len() == 1 {
            return edge.a.x * edge.b.x;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_weakly_linked_components() {
        let input = vec![Vertex::new(0, 0, 0), Vertex::new(1, 1, 1)];
        let edges = vec![Edge::from_points(input[0].clone(), input[1].clone())];
        let components = compute_weakly_linked_components(&input, &edges);
        assert_eq!(components.len(), 1);
    }

    #[test]
    fn test_compute_weakly_linked_components2() {
        let input = vec![
            Vertex::new(0, 0, 0),
            Vertex::new(1, 1, 1),
            Vertex::new(2, 2, 2),
        ];
        let edges = vec![Edge::from_points(input[0].clone(), input[1].clone())];
        let components = compute_weakly_linked_components(&input, &edges);
        assert_eq!(components.len(), 2);
    }
}
