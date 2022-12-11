use std::collections::HashSet;

const KNOT_COUNT: usize = 10;
const HEAD: usize = 0;
const TAIL: usize = KNOT_COUNT - 1_usize;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn x_dist(&self, other: &Point) -> i32 {
        (other.x - self.x).abs()
    }

    fn y_dist(&self, other: &Point) -> i32 {
        (other.y - self.y).abs()
    }

    fn adjacent(&self, other: &Point) -> bool {
        self.x_dist(other) <= 1 && self.y_dist(other) <= 1
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn from_char(value: char) -> Self {
        match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => panic!("Bad Direction: {}", value),
        }
    }

    pub fn value(&self) -> i32 {
        match self {
            Direction::Left | Direction::Down => -1,
            Direction::Right | Direction::Up => 1,
        }
    }
}

fn parse_instruction(line: &str) -> (Direction, usize) {
    (
        Direction::from_char(line.chars().next().unwrap()),
        line.split(' ').last().unwrap().parse::<usize>().unwrap(),
    )
}

fn knot_action(head_pos: &Point, tail_pos: &mut Point) {
    if tail_pos.adjacent(head_pos) {
        return;
    }
    let x_dist = tail_pos.x_dist(head_pos);
    let y_dist = tail_pos.y_dist(head_pos);

    if x_dist > 1 && y_dist == 0 {
        if head_pos.x > tail_pos.x {
            tail_pos.x += 1;
        } else {
            tail_pos.x -= 1;
        }
        return;
    }
    if y_dist > 1 && x_dist == 0 {
        if head_pos.y > tail_pos.y {
            tail_pos.y += 1;
        } else {
            tail_pos.y -= 1;
        }
        return;
    }

    // Brute force diagonal
    if head_pos.adjacent(&Point {
        x: tail_pos.x - 1,
        y: tail_pos.y - 1,
    }) {
        tail_pos.x -= 1;
        tail_pos.y -= 1;
        return;
    }
    if head_pos.adjacent(&Point {
        x: tail_pos.x - 1,
        y: tail_pos.y + 1,
    }) {
        tail_pos.x -= 1;
        tail_pos.y += 1;
        return;
    }
    if head_pos.adjacent(&Point {
        x: tail_pos.x + 1,
        y: tail_pos.y - 1,
    }) {
        tail_pos.x += 1;
        tail_pos.y -= 1;
        return;
    }
    if head_pos.adjacent(&Point {
        x: tail_pos.x + 1,
        y: tail_pos.y + 1,
    }) {
        tail_pos.x += 1;
        tail_pos.y += 1;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut head_pos = Point { x: 0, y: 0 };
    let mut tail_pos = Point { x: 0, y: 0 };

    let series = input.lines().map(parse_instruction);

    let mut history: HashSet<Point> = HashSet::new();

    for (direction, count) in series {
        for _ in 0..count {
            match direction {
                Direction::Left | Direction::Right => head_pos.x += direction.value(),
                Direction::Up | Direction::Down => head_pos.y += direction.value(),
            }
            knot_action(&head_pos, &mut tail_pos);
            history.insert(tail_pos);
        }
    }

    Some(history.len() as _)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut knots: Vec<Point> = (0..KNOT_COUNT).map(|_| Point::default()).collect();

    let series = input.lines().map(parse_instruction);

    let mut history: HashSet<Point> = HashSet::new();

    for (direction, count) in series {
        for _ in 0..count {
            match direction {
                Direction::Left | Direction::Right => {
                    knots.get_mut(HEAD).unwrap().x += direction.value()
                }
                Direction::Up | Direction::Down => {
                    knots.get_mut(HEAD).unwrap().y += direction.value()
                }
            }

            for idx in 1..KNOT_COUNT {
                let prev = *knots.get(idx - 1).unwrap();
                let knot = knots.get_mut(idx).unwrap();
                knot_action(&prev, knot);
            }

            let tail = knots.get(TAIL).unwrap();
            history.insert(*tail);
        }
    }

    Some(history.len() as _)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 91);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 92);
        assert_eq!(part_two(&input), Some(36));
    }
}
