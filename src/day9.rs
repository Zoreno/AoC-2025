use itertools::Itertools;

struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn new(x: u64, y: u64) -> Self {
        Point { x, y }
    }

    fn from_string(line: &str) -> Self {
        let (x, y) = line.split_once(',').unwrap();
        Point::new(x.parse().unwrap(), y.parse().unwrap())
    }
}

fn area(point: &Vec<&Point>) -> u64 {
    ((point[1].x).abs_diff(point[0].x) + 1) * ((point[1].y).abs_diff(point[0].y) + 1)
}

fn parse_input(input: &str) -> Vec<Point> {
    input.lines().map(Point::from_string).collect()
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Point> {
    parse_input(input)
}

#[aoc(day9, part1)]
fn part1(input: &[Point]) -> u64 {
    input
        .iter()
        .combinations(2)
        .map(|p| area(&p))
        .max()
        .unwrap()
}
