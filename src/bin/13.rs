use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Packet {
    Integer(u32),
    Packet(Vec<Packet>),
}

impl Packet {
    pub fn from_int(int: u32) -> Self {
        Packet::Packet(vec![Packet::Integer(int)])
    }

    pub fn from_str(input: &str) -> Self {
        if !input.starts_with('[') {
            return Packet::Integer(input.parse::<u32>().unwrap());
        }
        if input.len() == 2 {
            return Packet::Packet(vec![]);
        }

        let mut start_idx = 1;
        let mut idx = 1;
        let mut depth = 0;
        let mut items: Vec<Packet> = vec![];

        while idx < (input.len() - 1) {
            match input.chars().nth(idx).unwrap() {
                ',' => {
                    if depth == 0 {
                        items.push(Packet::from_str(&input[start_idx..idx]));
                        start_idx = idx + 1;
                    }
                }
                '[' => depth += 1,
                ']' => depth -= 1,
                _ => (),
            }
            idx += 1;
        }
        items.push(Packet::from_str(&input[start_idx..idx]));
        Packet::Packet(items)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Packet::Integer(x) => match other {
                Packet::Integer(y) => Some(x.cmp(y)),
                Packet::Packet(_) => Packet::from_int(*x).partial_cmp(other),
            },
            Packet::Packet(x) => match other {
                Packet::Integer(y) => self.partial_cmp(&Packet::from_int(*y)),
                Packet::Packet(y) => {
                    for (idx, x_val) in x.iter().enumerate() {
                        if (idx + 1) > y.len() {
                            return Some(Ordering::Greater);
                        }
                        let cmp_val = x_val.partial_cmp(y.get(idx).unwrap());
                        if cmp_val == Some(Ordering::Equal) {
                            continue;
                        }
                        return cmp_val;
                    }
                    x.len().partial_cmp(&y.len())
                }
            },
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n\n")
            .enumerate()
            .map(|(idx, x)| {
                let mut iter = x.lines();
                let p1 = Packet::from_str(iter.next().unwrap());
                let p2 = Packet::from_str(iter.next().unwrap());
                (idx + 1, p1 <= p2)
            })
            .filter(|(_, is_valid)| *is_valid)
            .map(|(idx, _)| idx as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let decoder: &str = "[[2]]\n[[6]]";
    let mut input_with_decoder = input.to_owned();
    input_with_decoder.push_str(decoder);

    let packets = input_with_decoder
        .lines()
        .filter(|x| !x.is_empty())
        .map(Packet::from_str)
        .sorted()
        .collect::<Vec<Packet>>();

    let key1 = Packet::from_str(decoder.lines().next().unwrap());
    let key2 = Packet::from_str(decoder.lines().last().unwrap());

    Some(
        ((packets.iter().position(|x| x == &key1).unwrap() + 1)
            * (packets.iter().position(|x| x == &key2).unwrap() + 1)) as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
