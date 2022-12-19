use std::cmp::PartialEq;
use std::collections::HashSet;
use std::fmt::Debug;

use itertools::Itertools;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn from_str(input: &str) -> Self {
        let (x, y) = input
            .split(',')
            .map(|z| z.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        Point { x, y }
    }

    pub fn line(&self, other: &Self) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::from([other.clone()]);

        for x in (self.x..other.x).chain((other.x + 1)..=self.x) {
            points.push(Point { x, y: self.y })
        }
        for y in (self.y..other.y).chain((other.y + 1)..=self.y) {
            points.push(Point { x: self.x, y })
        }
        points
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map: HashSet<Point> = HashSet::new();

    for line in input.lines() {
        for (p1, p2) in line.split(" -> ").map(Point::from_str).tuple_windows() {
            for point in p1.line(&p2) {
                map.insert(point);
            }
        }
    }

    let max_depth = map.iter().map(|p| p.y).max().unwrap();

    let rock_count = map.len();

    let mut infinite: bool = false;

    while !infinite {
        let mut sand = Point { x: 500, y: 0 };
        map.insert(sand.clone());
        loop {
            if sand.y > max_depth {
                infinite = true;
                break;
            }
            if !map.contains(&Point {
                x: sand.x,
                y: sand.y + 1,
            }) {
                map.remove(&sand);
                sand.y += 1;
                map.insert(sand.clone());
            } else if !map.contains(&Point {
                x: sand.x - 1,
                y: sand.y + 1,
            }) {
                map.remove(&sand);
                sand.x -= 1;
                sand.y += 1;
                map.insert(sand.clone());
            } else if !map.contains(&Point {
                x: sand.x + 1,
                y: sand.y + 1,
            }) {
                map.remove(&sand);
                sand.x += 1;
                sand.y += 1;
                map.insert(sand.clone());
            } else {
                break;
            }
        }
    }

    Some((map.len() - (rock_count + 1)) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map: HashSet<Point> = HashSet::new();

    for line in input.lines() {
        for (p1, p2) in line.split(" -> ").map(Point::from_str).tuple_windows() {
            for point in p1.line(&p2) {
                map.insert(point);
            }
        }
    }

    let max_depth = map.iter().map(|p| p.y).max().unwrap() + 2;

    let rock_count = map.len();

    loop {
        let mut sand = Point { x: 500, y: 0 };
        if map.contains(&sand) {
            break;
        }
        map.insert(sand.clone());
        loop {
            if sand.y == (max_depth - 1) {
                break;
            }
            if !map.contains(&Point {
                x: sand.x,
                y: sand.y + 1,
            }) {
                map.remove(&sand);
                sand.y += 1;
                map.insert(sand.clone());
            } else if !map.contains(&Point {
                x: sand.x - 1,
                y: sand.y + 1,
            }) {
                map.remove(&sand);
                sand.x -= 1;
                sand.y += 1;
                map.insert(sand.clone());
            } else if !map.contains(&Point {
                x: sand.x + 1,
                y: sand.y + 1,
            }) {
                map.remove(&sand);
                sand.x += 1;
                sand.y += 1;
                map.insert(sand.clone());
            } else {
                break;
            }
        }
    }
    Some((map.len() - rock_count) as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
