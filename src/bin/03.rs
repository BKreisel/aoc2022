use std::collections::HashSet;
use std::str::FromStr;

pub struct Rucksack {
    items: Vec<char>,
}

impl Rucksack {
    pub fn common_item(&self) -> Option<char> {
        let compartment_size = self.items.len() / 2;
        let left_compartment: HashSet<_> = self.items.iter().take(compartment_size).collect();
        let right_compartment: HashSet<_> = self.items.iter().skip(compartment_size).collect();

        let common: HashSet<_> = left_compartment.intersection(&right_compartment).collect();
        return match common.len() {
            0 => None,
            1 => Some(***common.iter().next().unwrap()),
            _ => panic!("Multiple common items"),
        };
    }
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Rucksack {
            items: input.chars().collect(),
        })
    }
}

impl From<&Rucksack> for HashSet<char> {
    fn from(item: &Rucksack) -> Self {
        HashSet::from_iter(item.items.iter().cloned())
    }
}

pub fn priority(item: char) -> u32 {
    if item.is_uppercase() {
        // Point value minus start of uppercase ASCII value
        return item as u32 + 27 - 65;
    }
    // Point value minus start of lowercase ASCII value
    item as u32 + 1 - 97
}

pub fn find_badge(sacks: &[Rucksack]) -> Option<char> {
    let mut items: HashSet<char> = sacks.first().unwrap().into();

    for sack in sacks.iter().skip(1) {
        items = items.intersection(&sack.into()).copied().collect();
    }

    return match items.len() {
        0 => None,
        1 => Some(*items.iter().next().unwrap()),
        _ => panic!("Multiple common items"),
    };
}

pub fn part_one(input: &str) -> Option<u32> {
    return Some(
        input
            .lines()
            .map(|x| Rucksack::from_str(x).unwrap())
            .map(|x| x.common_item().expect("Error: No Common Item"))
            .map(priority)
            .sum(),
    );
}

pub fn part_two(input: &str) -> Option<u32> {
    let rucksacks: Vec<Rucksack> = input
        .lines()
        .map(|x| Rucksack::from_str(x).unwrap())
        .collect();
    Some(
        rucksacks
            .as_slice()
            .chunks(3)
            .map(|x| find_badge(x).expect("Error: Badge Not Found"))
            .map(priority)
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
