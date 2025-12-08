use itertools::Itertools;
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
        adj.entry(v.clone()).or_insert(Vec::new());
    }

    let mut all_edges: Vec<Edge> = Vec::with_capacity(edges.len());

    for edge in edges {
        adj.entry(edge.a.clone())
            .or_insert_with(Vec::new)
            .push(edge.b.clone());

        adj.entry(edge.b.clone())
            .or_insert_with(Vec::new)
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

            let mut comp_edges: Vec<Edge> = Vec::new();

            for edge in &all_edges {
                if comp_vertices.contains(&edge.a) && comp_vertices.contains(&edge.b) {
                    comp_edges.push(edge.clone());
                }
            }

            components.push(Component {
                vertices: comp_vertices.clone(),
                edges: comp_edges,
            });

            comp_vertices.clear();
        }
    }

    components
}

#[aoc(day8, part1)]
fn part1(input: &[Vertex]) -> u64 {
    let mut edges: Vec<Edge> = input
        .iter()
        .combinations(2)
        .map(|p| Edge::from_points(p[0].clone(), p[1].clone()))
        .collect();

    edges.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    edges.truncate(1000);

    edges.iter().for_each(|e| println!("{:?}", e));

    let mut components = compute_weakly_linked_components(&input, &edges);

    for component in &components {
        println!("{:?}", component);
    }

    let mut components: Vec<usize> = components.iter().map(|c| c.vertices.len()).collect();

    components.sort();

    for component in components.iter().rev().take(3) {
        println!("{:?}", component);
    }

    0
}
