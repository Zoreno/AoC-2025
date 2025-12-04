use std::collections::HashSet;

//=====================================================================
// Input
//=====================================================================

fn is_roll(c: &char) -> bool {
    *c == '@'
}

fn parse_input(input: &str) -> Grid {
    let iter = input.lines().enumerate().flat_map(|(x, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, c)| is_roll(c))
            .map(move |(y, _)| Point::new(x as i32, y as i32))
    });

    Grid {
        rolls: HashSet::from_iter(iter),
    }
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Grid {
    parse_input(input)
}

//=====================================================================
// Helpers
//=====================================================================

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn offset(&self, x: i32, y: i32) -> Self {
        Point {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

fn kernel_around(center: &Point, radius: i32) -> impl Iterator<Item = Point> {
    (-radius..radius + 1)
        .flat_map(move |x| std::iter::repeat(x).zip(-radius..radius + 1))
        .map(|(x, y)| center.offset(x, y))
}

#[derive(Clone, Debug)]
struct Grid {
    rolls: HashSet<Point>,
}

impl Grid {
    fn has_roll_at(&self, p: &Point) -> bool {
        self.rolls.contains(p)
    }

    fn neighbor_count(&self, p: &Point) -> u32 {
        kernel_around(p, 1).filter(|p| self.has_roll_at(p)).count() as u32
    }
}

//=====================================================================
// Solvers
//=====================================================================

const MAX_NEIGHBOR_COUNT: u32 = 4;

#[aoc(day4, part1)]
fn part1(input: &Grid) -> usize {
    input
        .rolls
        .iter()
        .map(|p| input.neighbor_count(p))
        .filter(|c| *c < MAX_NEIGHBOR_COUNT + 1) // add one to account for the roll itself
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &Grid) -> usize {
    let mut current_grid = input.clone();
    let mut total_removed: usize = 0;

    loop {
        let remove: Vec<Point> = {
            current_grid
                .rolls
                .iter()
                .filter(|p| current_grid.neighbor_count(p) < MAX_NEIGHBOR_COUNT + 1)
                .cloned()
                .collect()
        };

        if remove.is_empty() {
            break;
        }

        total_removed += remove.len();

        for p in remove {
            current_grid.rolls.remove(&p);
        }
    }

    total_removed
}

//=====================================================================
// Tests
//=====================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_around() {
        let p = Point::new(0, 0);
        let radius = 1;

        let kernel: Vec<Point> = kernel_around(&p, radius).collect();

        println!("{:?}", kernel);

        assert_eq!(kernel.len(), 9);
        assert!(kernel.contains(&p.offset(-1, -1)));
        assert!(kernel.contains(&p.offset(-1, 0)));
        assert!(kernel.contains(&p.offset(-1, 1)));
        assert!(kernel.contains(&p.offset(0, -1)));
        assert!(kernel.contains(&p.offset(0, 0)));
        assert!(kernel.contains(&p.offset(0, 1)));
        assert!(kernel.contains(&p.offset(1, -1)));
        assert!(kernel.contains(&p.offset(1, 0)));
        assert!(kernel.contains(&p.offset(1, 1)));
    }
}
