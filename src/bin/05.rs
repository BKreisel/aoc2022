use itertools::Itertools;
use std::borrow::BorrowMut;
use std::str::FromStr;

struct Action {
    count: usize,
    src_idx: usize,
    dst_idx: usize,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut word_iter = input.split(' ');

        let count = word_iter
            .borrow_mut()
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let src_idx = word_iter
            .borrow_mut()
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap()
            - 1;

        let dst_idx = word_iter
            .borrow_mut()
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap()
            - 1;

        Ok(Action {
            count,
            src_idx,
            dst_idx,
        })
    }
}
pub fn parse_stacks(input: Vec<&str>) -> Vec<Vec<char>> {
    let stack_count = input
        .iter()
        .rev()
        .next()
        .unwrap()
        .chars()
        .filter(|x| x == &'[')
        .count();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..stack_count {
        stacks.push(Vec::new());
    }

    for line in input.iter().rev() {
        for (idx, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
            let name = chunk.nth(1).unwrap();
            if name != ' ' {
                stacks.get_mut(idx).unwrap().push(name);
            }
        }
    }

    stacks
}

pub fn part_one(input: &str) -> Option<String> {
    let stacks_input = input
        .lines()
        .take_while(|x| x.contains('['))
        .collect::<Vec<_>>();
    let actions_input: Vec<&str> = input
        .lines()
        .skip(stacks_input.len() + 2)
        .collect::<Vec<_>>();

    let mut stacks = parse_stacks(stacks_input);
    let actions: Vec<Action> = actions_input
        .iter()
        .map(|x| Action::from_str(x).unwrap())
        .collect::<Vec<_>>();

    for action in actions {
        for _ in 0..action.count {
            let value = stacks.get_mut(action.src_idx).unwrap().pop().unwrap();
            stacks.get_mut(action.dst_idx).unwrap().push(value);
        }
    }

    Some(
        stacks
            .iter()
            .map(|stack| stack.iter().rev().next().copied().unwrap())
            .collect::<String>(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let stacks_input = input
        .lines()
        .take_while(|x| x.contains('['))
        .collect::<Vec<_>>();
    let actions_input: Vec<&str> = input
        .lines()
        .skip(stacks_input.len() + 2)
        .collect::<Vec<_>>();

    let mut stacks = parse_stacks(stacks_input);
    let actions: Vec<Action> = actions_input
        .iter()
        .map(|x| Action::from_str(x).unwrap())
        .collect::<Vec<_>>();

    for action in actions {
        let new_len = stacks.get_mut(action.src_idx).unwrap().len() - action.count;
        let mut values: Vec<_> = stacks
            .get_mut(action.src_idx)
            .unwrap()
            .drain(new_len..)
            .collect();
        stacks.get_mut(action.dst_idx).unwrap().append(&mut values);
    }

    Some(
        stacks
            .iter()
            .map(|stack| stack.iter().rev().next().copied().unwrap())
            .collect::<String>(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
