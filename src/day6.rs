use crate::transpose_ref;

#[derive(Debug)]
enum Operation {
    Plus,
    Times,
}

impl Operation {
    fn from_string(s: &str) -> Operation {
        match s {
            "+" => Operation::Plus,
            "*" => Operation::Times,
            _ => panic!("Unknown operation: {s}"),
        }
    }
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<u64>,
    op: Operation,
}

impl Problem {
    fn from_column(column: &Vec<&&str>) -> Problem {
        let op = Operation::from_string(column[0]);
        let numbers: Vec<u64> = column
            .iter()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .rev()
            .collect();
        Problem { numbers, op }
    }

    fn from_group(group: &str) -> Problem {
        let op = if group.contains('*') {
            Operation::Times
        } else {
            Operation::Plus
        };

        let filtered_group: String = group
            .chars()
            .filter(|c| !c.is_whitespace() && *c != '+' && *c != '*')
            .collect();

        let numbers: Vec<u64> = filtered_group
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse().unwrap())
            .collect();

        Problem { numbers, op }
    }

    fn solve(&self) -> u64 {
        match self.op {
            Operation::Plus => self.numbers.iter().sum(),
            Operation::Times => self.numbers.iter().product(),
        }
    }
}

#[derive(Debug)]
struct Input {
    problems: Vec<Problem>,
}

impl Input {
    fn from_string_part1(input: &str) -> Input {
        let rows: Vec<Vec<&str>> = input
            .lines()
            .rev()
            .map(|s| s.split_whitespace().collect())
            .collect();

        let problems = transpose_ref(&rows)
            .iter()
            .map(Problem::from_column)
            .collect();
        Input { problems }
    }

    fn from_string_part2(input: &str) -> Input {
        let rows: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();

        let new_lines = transpose_ref(&rows)
            .iter()
            .map(|v| v.iter().cloned().collect())
            .collect::<Vec<String>>()
            .join("\n");

        let problems: Vec<Problem> = new_lines
            .split("\n     \n") // Kind of yanky but seems to work
            .map(Problem::from_group)
            .collect();

        Input { problems }
    }
}

#[aoc_generator(day6, part1)]
fn parse_part1(input: &str) -> Input {
    Input::from_string_part1(input)
}

#[aoc_generator(day6, part2)]
fn parse_part2(input: &str) -> Input {
    Input::from_string_part2(input)
}

#[aoc(day6, part1)]
fn part1(input: &Input) -> u64 {
    input.problems.iter().map(|p| p.solve()).sum()
}

#[aoc(day6, part2)]
fn part2(input: &Input) -> u64 {
    input.problems.iter().map(|p| p.solve()).sum()
}
