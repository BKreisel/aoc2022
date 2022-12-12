use itertools::{Chunk, Itertools};
use std::str::Lines;

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Action {
    item: u64,
    dst: usize,
}

impl Operation {
    pub fn from_str(value: &str) -> Self {
        match value {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            x => panic!("Bad Operation {}", x),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Argument {
    Literal(u64),
    OldValue,
}

impl Argument {
    pub fn from_str(value: &str) -> Self {
        match value {
            "old" => Argument::OldValue,
            x => Argument::Literal(x.parse::<u64>().unwrap()),
        }
    }
}

#[derive(Debug)]
struct Expression {
    arg1: Argument,
    op: Operation,
    arg2: Argument,
}

impl Expression {
    pub fn from_str(line: &str) -> Self {
        let mut items = line.split("= ").nth(1).unwrap().split(' ');
        Self {
            arg1: Argument::from_str(items.next().unwrap()),
            op: Operation::from_str(items.next().unwrap()),
            arg2: Argument::from_str(items.next().unwrap()),
        }
    }

    pub fn evaluate(&self, old_value: u64) -> u64 {
        let arg1 = match self.arg1 {
            Argument::OldValue => old_value,
            Argument::Literal(x) => x,
        };
        let arg2 = match self.arg2 {
            Argument::OldValue => old_value,
            Argument::Literal(x) => x,
        };
        match self.op {
            Operation::Add => arg1 + arg2,
            Operation::Multiply => arg1 * arg2,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    inspect_count: u64,
    items: Vec<u64>,
    expression: Expression,
    divisor: u64,
    true_dst: usize,
    false_dst: usize,
}

impl Monkey {
    pub fn from_lines(mut lines: Chunk<Lines>) -> Self {
        let items: Vec<u64> = lines
            .nth(1)
            .unwrap()
            .split(':')
            .last()
            .unwrap()
            .split(',')
            .map(|x| x.trim().parse::<u64>().unwrap())
            .collect();

        let expression = Expression::from_str(lines.next().unwrap());

        let divisor = lines
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let true_dst = lines
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let false_dst = lines
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Self {
            items,
            expression,
            divisor,
            true_dst,
            false_dst,
            inspect_count: 0,
        }
    }

    pub fn test(&self, value: u64) -> usize {
        match value % self.divisor {
            0 => self.true_dst,
            _ => self.false_dst,
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut monkies: Vec<Monkey> = input
        .lines()
        .chunks(7)
        .into_iter()
        .map(Monkey::from_lines)
        .collect();

    for _ in 0..20 {
        for monkey_idx in 0..monkies.len() {
            let monkey = monkies.get_mut(monkey_idx).unwrap();
            let items = monkey.items.drain(..).collect::<Vec<u64>>();
            monkey.inspect_count += items.len() as u64;

            let actions = items
                .iter()
                .map(|x| {
                    let new_value = monkey.expression.evaluate(*x) / 3;
                    Action {
                        item: new_value,
                        dst: monkey.test(new_value),
                    }
                })
                .collect::<Vec<Action>>();

            for action in actions {
                monkies.get_mut(action.dst).unwrap().items.push(action.item);
            }
        }
    }

    let counts = monkies
        .iter()
        .map(|x| x.inspect_count)
        .sorted()
        .collect::<Vec<u64>>();

    Some(counts.iter().rev().take(2).product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut monkies: Vec<Monkey> = input
        .lines()
        .chunks(7)
        .into_iter()
        .map(Monkey::from_lines)
        .collect();

    let modulo: u64 = monkies.iter().map(|x| x.divisor).product();

    for _ in 0..10000 {
        for monkey_idx in 0..monkies.len() {
            let monkey = monkies.get_mut(monkey_idx).unwrap();
            let items = monkey.items.drain(..).collect::<Vec<u64>>();
            monkey.inspect_count += items.len() as u64;

            let actions = items
                .iter()
                .map(|x| {
                    let new_value = monkey.expression.evaluate(*x) % modulo;
                    Action {
                        item: new_value,
                        dst: monkey.test(new_value),
                    }
                })
                .collect::<Vec<Action>>();

            for action in actions {
                monkies.get_mut(action.dst).unwrap().items.push(action.item);
            }
        }
    }

    let counts = monkies
        .iter()
        .map(|x| x.inspect_count)
        .sorted()
        .collect::<Vec<u64>>();

    Some(counts.iter().rev().take(2).product())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
