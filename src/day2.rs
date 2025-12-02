//=====================================================================
// Input
//=====================================================================

#[derive(Debug, Eq, PartialEq)]
pub struct Range {
    start: u64,
    end: u64,
}

fn entry_to_range(line: &str) -> Range {
    let parts: Vec<&str> = line.split('-').collect();
    Range {
        start: parts[0].parse().unwrap(),
        end: parts[1].parse().unwrap(),
    }
}

fn parse_input(input: &str) -> Vec<Range> {
    input.split(',').map(entry_to_range).collect()
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Range> {
    parse_input(input)
}

//=====================================================================
// Helpers
//=====================================================================

fn is_invalid_part1(n: &u64) -> bool {
    let n_as_string = n.to_string();
    let digit_count = n_as_string.len();

    if digit_count % 2 == 1 {
        // Odd number of digits cannot repeat
        return false;
    } else {
        // Split at midway and compare results.
        let (lhs, rhs) = n_as_string.split_at(digit_count / 2);
        return lhs == rhs;
    }
}

fn is_invalid_part2(n: &u64) -> bool {
    let n_as_string = n.to_string();
    let digit_count = n_as_string.len();

    // Consider repeats of 1, 2, 3, ... digit_count / 2
    for repeat_count in 1..(digit_count / 2) + 1 {
        // If repeat count doesn't divide evenly, skip
        if digit_count % repeat_count != 0 {
            continue;
        }

        // Split string at repeat count, multiply up to the correct digit count and compare
        let repeated_string = n_as_string
            .split_at(repeat_count)
            .0
            .repeat(digit_count / repeat_count);
        if repeated_string == n_as_string {
            return true;
        }
    }

    return false;
}

//=====================================================================
// Solvers
//=====================================================================

fn solve_part1(input: &Vec<Range>) -> u64 {
    input
        .iter()
        .flat_map(|input| (input.start..input.end + 1).filter(is_invalid_part1))
        .sum()
}

#[aoc(day2, part1)]
fn part1(input: &Vec<Range>) -> u64 {
    solve_part1(input)
}

fn solve_part2(input: &Vec<Range>) -> u64 {
    input
        .iter()
        .flat_map(|input| (input.start..input.end + 1).filter(is_invalid_part2))
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &Vec<Range>) -> u64 {
    solve_part2(input)
}

//=====================================================================
// Tests
//=====================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_invalid_part1() {
        assert!(!is_invalid_part1(&123));
        assert!(!is_invalid_part1(&1231));
        assert!(!is_invalid_part1(&12312));
        assert!(is_invalid_part1(&123123));
    }

    #[test]
    fn test_is_invalid_part2() {
        assert!(!is_invalid_part2(&123));
        assert!(!is_invalid_part2(&1231));
        assert!(!is_invalid_part2(&12312));
        assert!(is_invalid_part2(&123123));
        assert!(is_invalid_part2(&121212));
    }
}
