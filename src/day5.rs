#[derive(Clone, Debug, Eq, PartialEq)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Range { start, end }
    }

    fn contains(&self, val: &u64) -> bool {
        *val >= self.start && *val <= self.end
    }

    fn size(&self) -> u64 {
        self.end + 1 - self.start
    }
}

fn parse_range(line: &str) -> Range {
    let parts: Vec<u64> = line.split('-').map(|s| s.parse().unwrap()).collect();
    Range::new(parts[0], parts[1])
}

fn parse_ranges(input: &str) -> Vec<Range> {
    input.lines().map(parse_range).collect()
}

fn parse_available(input: &str) -> Vec<u64> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

struct Input {
    ranges: Vec<Range>,
    available: Vec<u64>,
}

impl Input {
    fn new(ranges: Vec<Range>, available: Vec<u64>) -> Self {
        Input { ranges, available }
    }
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Input {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let ranges = parse_ranges(parts[0]);
    let available = parse_available(parts[1]);

    Input::new(ranges, available)
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> usize {
    input
        .available
        .iter()
        .filter(|n| input.ranges.iter().any(|r| r.contains(n)))
        .count()
}

fn non_overlapping_ranges(ranges: &[Range]) -> Vec<Range> {
    let mut ranges_sorted: Vec<Range> = ranges.to_owned();
    ranges_sorted.sort_by(|a, b| a.start.cmp(&b.start));

    let mut non_overlapping: Vec<Range> = Vec::new();
    let mut last: Option<Range> = None;

    for range in ranges_sorted {
        if let Some(last_range) = last {
            if range.start > last_range.end + 1 {
                // No overlap, so push and start on new range
                non_overlapping.push(last_range);
                last = Some(range.clone());
            } else {
                // Merge with last range
                last = Some(Range::new(
                    last_range.start,
                    std::cmp::max(last_range.end, range.end),
                ))
            }
        } else {
            last = Some(range.clone());
        }
    }

    if let Some(last) = last {
        non_overlapping.push(last);
    }

    non_overlapping
}

fn non_overlapping_interval_size(ranges: &[Range]) -> u64 {
    non_overlapping_ranges(ranges)
        .iter()
        .map(|r| r.size())
        .sum()
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> u64 {
    non_overlapping_interval_size(&input.ranges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_overlapping_ranges() {
        let ranges = vec![Range::new(2, 4), Range::new(1, 3), Range::new(6, 8)];
        assert_eq!(
            non_overlapping_ranges(&ranges),
            vec![Range::new(1, 4), Range::new(6, 8)]
        );
    }

    #[test]
    fn test_non_overlapping_ranges2() {
        let ranges = vec![Range::new(1, 3), Range::new(3, 4), Range::new(6, 8)];
        assert_eq!(
            non_overlapping_ranges(&ranges),
            vec![Range::new(1, 4), Range::new(6, 8)]
        );
    }

    #[test]
    fn test_non_overlapping_ranges3() {
        let ranges = vec![Range::new(1, 3), Range::new(3, 4), Range::new(5, 8)];
        assert_eq!(non_overlapping_ranges(&ranges), vec![Range::new(1, 8)]);
    }

    #[test]
    fn test_non_overlapping_interval_size() {
        let ranges = vec![Range::new(1, 3), Range::new(2, 4), Range::new(6, 8)];
        assert_eq!(non_overlapping_interval_size(&ranges), 7);
    }

    #[test]
    fn test_non_overlapping_interval_size2() {
        let ranges = vec![Range::new(1, 3), Range::new(3, 4), Range::new(5, 8)];
        assert_eq!(non_overlapping_interval_size(&ranges), 8);
    }
}
