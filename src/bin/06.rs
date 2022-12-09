use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let chars: Vec<char> = input.chars().collect();

    for (idx, window) in chars.windows(4).enumerate() {
        if HashSet::<char>::from_iter(window.iter().cloned()).len() == 4 {
            return Some((idx + 4) as _);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let chars: Vec<char> = input.chars().collect();

    for (idx, window) in chars.windows(14).enumerate() {
        if HashSet::<char>::from_iter(window.iter().cloned()).len() == 14 {
            return Some((idx + 14) as _);
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
