use std::ops::Range;

fn parse_pair(pair: &str) -> Range<u32> {
    let mut iter = pair.split('-');
    Range {
        start: iter.next().unwrap().parse::<u32>().unwrap(),
        end: iter.next().unwrap().parse::<u32>().unwrap(),
    }
}

fn parse_line(input: &str) -> (Range<u32>, Range<u32>) {
    let mut iter = input.split(',');
    (
        parse_pair(iter.next().unwrap()),
        parse_pair(iter.next().unwrap()),
    )
}

fn has_full_overlap(pairs: &(Range<u32>, Range<u32>)) -> bool {
    let (p1, p2) = pairs;
    // Check if p1 is subset of p2
    if p1.start >= p2.start && p1.end <= p2.end {
        return true;
    }

    // Check if p2 is subset of p1
    if p2.start >= p1.start && p2.end <= p1.end {
        return true;
    }
    false
}

fn has_partial_overlap(pairs: &(Range<u32>, Range<u32>)) -> bool {
    let (p1, p2) = pairs;

    // Check if p1 is overlaps p2
    if p1.start >= p2.start && p1.start <= p2.end {
        return true;
    }

    if p1.end >= p2.start && p1.end <= p2.end {
        return true;
    }

    // Check if p2 overlaps p1
    if p2.start >= p1.start && p2.start <= p1.end {
        return true;
    }

    if p2.end >= p1.start && p2.end <= p1.end {
        return true;
    }
    false
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(parse_line)
            .filter(has_full_overlap)
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(parse_line)
            .filter(has_partial_overlap)
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
