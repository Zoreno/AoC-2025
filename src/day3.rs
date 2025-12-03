//=====================================================================
// Input
//=====================================================================

fn line_to_battery_bank(line: &str) -> BatteryBank {
    BatteryBank::from_string(line)
}

fn parse_input(input: &str) -> Vec<BatteryBank> {
    input.lines().map(line_to_battery_bank).collect()
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<BatteryBank> {
    parse_input(input)
}

//=====================================================================
// Helpers
//=====================================================================

struct BatteryBank {
    batteries: Vec<u64>,
}

impl BatteryBank {
    fn from_string(s: &str) -> Self {
        BatteryBank {
            batteries: s.chars().map(|c| c.to_digit(10).unwrap() as u64).collect(),
        }
    }
}

impl BatteryBank {
    fn joltage_helper(&self, offset: usize, n: usize) -> u64 {
        // Find the first value in the range, skipping the first offset values
        // and the last n - 1 values. Next recursion, find the first max value
        // from the values after the last max value and consider one more value
        // at the end.

        // Note: max_by would normally return the last max value found, but
        // since the iterator is reversed, the first one is returned instead.
        let (i, max) = self
            .batteries
            .iter()
            .enumerate()
            .skip(offset)
            .rev()
            .skip(n - 1)
            .max_by(|x, y| x.1.cmp(y.1))
            .unwrap();

        if n == 1 {
            *max
        } else {
            (*max * u64::pow(10, n as u32 - 1)) + self.joltage_helper(i + 1, n - 1)
        }
    }

    fn joltage_part1(&self) -> u64 {
        self.joltage_helper(0, 2)
    }

    fn joltage_part2(&self) -> u64 {
        self.joltage_helper(0, 12)
    }
}

//=====================================================================
// Solvers
//=====================================================================

fn solve_part1(input: &[BatteryBank]) -> u64 {
    input.iter().map(BatteryBank::joltage_part1).sum()
}

#[aoc(day3, part1)]
fn part1(input: &[BatteryBank]) -> u64 {
    solve_part1(input)
}

fn solve_part2(input: &[BatteryBank]) -> u64 {
    input.iter().map(BatteryBank::joltage_part2).sum()
}

#[aoc(day3, part2)]
fn part2(input: &[BatteryBank]) -> u64 {
    solve_part2(input)
}

//=====================================================================
// Tests
//=====================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joltage_part1() {
        assert_eq!(
            BatteryBank::from_string("987654321111111").joltage_part1(),
            98
        );
        assert_eq!(
            BatteryBank::from_string("811111111111119").joltage_part1(),
            89
        );
        assert_eq!(
            BatteryBank::from_string("234234234234278").joltage_part1(),
            78
        );
        assert_eq!(
            BatteryBank::from_string("818181911112111").joltage_part1(),
            92
        );

        assert_eq!(BatteryBank::from_string("4942223224223134312221222433336324234433314222333723222642441142184541322622221421243432273241422334").joltage_part1(), 98);
    }

    #[test]
    fn test_joltage_part2() {
        assert_eq!(
            BatteryBank::from_string("987654321111111").joltage_part2(),
            987654321111
        );
    }
}
