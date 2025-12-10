use itertools::Itertools;

//=====================================================================
// Helpers
//=====================================================================

#[derive(Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    fn x(&self) -> i64 {
        self.x
    }

    fn from_string(line: &str) -> Self {
        let (x, y) = line.split_once(',').unwrap();
        Point::new(x.parse().unwrap(), y.parse().unwrap())
    }
}

#[derive(Clone, Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn new(p1: Point, p2: Point) -> Self {
        Line { p1, p2 }
    }

    fn crosses_x_axis_at(&self, x: i64) -> bool {
        (self.p1.x <= x && self.p2.x > x) || (self.p2.x <= x && self.p1.x > x)
    }

    fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }
}

#[derive(Clone, Debug)]
struct Interval {
    start: i64,
    end: i64,
}

impl Interval {
    fn new(start: i64, end: i64) -> Self {
        Interval { start, end }
    }
}

#[derive(Clone, Debug)]
struct Rect {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
}

impl Rect {
    fn new(x1: i64, y1: i64, x2: i64, y2: i64) -> Self {
        Rect { x1, y1, x2, y2 }
    }

    fn from_points(p1: Point, p2: Point) -> Self {
        Rect {
            x1: p1.x.min(p2.x),
            y1: p1.y.min(p2.y),
            x2: p1.x.max(p2.x),
            y2: p1.y.max(p2.y),
        }
    }

    fn area(self) -> i64 {
        (self.x2 - self.x1 + 1) * (self.y2 - self.y1 + 1)
    }

    fn overlaps(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    fn overlaps_any(&self, others: &[Rect]) -> bool {
        others.iter().any(|r| self.overlaps(r))
    }
}

fn area(point: &Vec<&Point>) -> u64 {
    ((point[1].x).abs_diff(point[0].x) + 1) * ((point[1].y).abs_diff(point[0].y) + 1)
}

fn vertical_intervals(poly: &[Point], x: i64) -> Vec<Interval> {
    let mut ys = Vec::new();
    let n = poly.len();

    for i in 0..n {
        let line = Line::new(poly[i].clone(), poly[(i + 1) % n].clone());

        if !line.is_vertical() && line.crosses_x_axis_at(x) {
            ys.push(line.p1.y); // No need to interpolate as we know that the lines are always either fully horizontal or fully vertical
        }
    }

    ys.sort_unstable();

    ys.chunks(2)
        .filter_map(|ab| {
            if ab.len() == 2 {
                Some(Interval::new(ab[0], ab[1]))
            } else {
                None
            }
        })
        .collect()
}

fn merge_rects(rects: Vec<Rect>) -> Vec<Rect> {
    let mut merged: Vec<Rect> = Vec::new();

    for r in rects {
        if let Some(last) = merged.last_mut() {
            if last.y1 == r.y1 && last.y2 == r.y2 && last.x2 == r.x1 {
                last.x2 = r.x2;
                continue;
            }
        }
        merged.push(r);
    }
    merged
}

fn rects_outside(poly: &[Point], width: i64, height: i64) -> Vec<Rect> {
    let mut xs: Vec<i64> = poly.iter().map(Point::x).collect();

    xs.push(0);
    xs.push(width - 1);
    xs.sort_unstable();

    let mut rects = Vec::new();

    for w in xs.windows(2) {
        let x1 = w[0];
        let mut x2 = w[1];

        if x1 == x2 {
            continue;
        }

        x2 -= 1;

        let intervals = vertical_intervals(&poly, x1);

        let mut start = 0i64;

        for interval in intervals {
            if start < interval.start {
                rects.push(Rect::new(x1, start, x2, interval.start - 1));
            }

            start = interval.end;
        }

        if start <= height {
            rects.push(Rect::new(x1, start, x2, height - 1));
        }
    }

    merge_rects(rects)
}

//=====================================================================
// Input
//=====================================================================

fn parse_input(input: &str) -> Vec<Point> {
    input.lines().map(Point::from_string).collect()
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Point> {
    parse_input(input)
}

//=====================================================================
// Solvers
//=====================================================================

#[aoc(day9, part1)]
fn part1(input: &[Point]) -> i64 {
    input
        .iter()
        .combinations(2)
        .map(|p| Rect::from_points(p[0].clone(), p[1].clone()).area())
        .max()
        .unwrap()
}

#[aoc(day9, part2)]
fn part2(input: &[Point]) -> i64 {
    let rects = rects_outside(input, 100000, 100000);

    input
        .iter()
        .combinations(2)
        .map(|p| Rect::from_points(p[0].clone(), p[1].clone()))
        .filter(|r| !r.overlaps_any(&rects))
        .map(Rect::area)
        .max()
        .unwrap()
}

//=====================================================================
// Tests
//=====================================================================
