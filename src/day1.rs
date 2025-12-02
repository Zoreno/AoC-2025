const START_POS: i32 = 50;
const DIAL_COUNT: i32 = 100;

//=====================================================================
// Input
//=====================================================================

#[derive(Debug, Eq, PartialEq)]
pub struct Input {
    direction: char,
    distance: i32,
}

fn line_to_input(line: &str) -> Input {
    Input {
        direction: line.chars().next().unwrap(),
        distance: line[1..].parse().unwrap(),
    }
}

fn parse_input(input: &str) -> Vec<Input> {
    input.lines().map(line_to_input).collect()
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Input> {
    parse_input(input)
}

//=====================================================================
// Helpers
//=====================================================================

fn input_to_distance(input: &Input) -> i32 {
    match input {
        Input {
            direction: 'R',
            distance,
        } => *distance,
        Input {
            direction: 'L',
            distance,
        } => -distance,
        Input { .. } => {
            panic!("Invalid direction")
        }
    }
}

struct DialState {
    current: i32,
    zero_hit_count: i32,
}

impl DialState {
    fn new() -> Self {
        DialState {
            current: START_POS,
            zero_hit_count: 0,
        }
    }

    fn advance_part1(&mut self, d: i32) -> i32 {
        self.current = (self.current + d) % DIAL_COUNT;
        if self.current == 0 {
            self.zero_hit_count += 1;
        }
        self.zero_hit_count
    }

    fn advance_part2(&mut self, d: i32) -> i32 {
        // This is not efficient, but it works
        let (delta, steps) = if d > 0 { (1, d) } else { (-1, -d) };
        for _ in 0..steps {
            self.advance_part1(delta);
        }
        self.zero_hit_count
    }
}

//=====================================================================
// Solvers
//=====================================================================

fn solve_part1(input: &[Input]) -> i32 {
    input
        .iter()
        .map(input_to_distance)
        .scan(DialState::new(), |state, d| Some(state.advance_part1(d)))
        .last()
        .unwrap()
}

#[aoc(day1, part1)]
pub fn part1(input: &[Input]) -> i32 {
    solve_part1(input)
}

fn solve_part2(input: &[Input]) -> i32 {
    input
        .iter()
        .map(input_to_distance)
        .scan(DialState::new(), |state, d| Some(state.advance_part2(d)))
        .last()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &[Input]) -> i32 {
    solve_part2(input)
}

//=====================================================================
// Tests
//=====================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_to_input() {
        assert_eq!(
            line_to_input("R50"),
            Input {
                direction: 'R',
                distance: 50,
            }
        );

        assert_eq!(
            line_to_input("L50"),
            Input {
                direction: 'L',
                distance: 50,
            }
        );
    }

    #[test]
    fn test_part2_one_wraparound() {
        let input = vec![Input {
            direction: 'R',
            distance: 60,
        }];
        assert_eq!(solve_part2(&input), 1);
    }

    #[test]
    fn test_part2_multiple_wraparounds() {
        let input = vec![Input {
            direction: 'R',
            distance: 1000,
        }];
        assert_eq!(solve_part2(&input), 10);
    }

    #[test]
    fn test_part2_land_on_zero_right() {
        let input = vec![Input {
            direction: 'R',
            distance: 50,
        }];
        assert_eq!(solve_part2(&input), 1);
    }

    #[test]
    fn test_part2_land_on_zero_left() {
        let input = vec![Input {
            direction: 'L',
            distance: 50,
        }];
        assert_eq!(solve_part2(&input), 1);
    }
}
