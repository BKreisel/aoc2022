use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};

const START: usize = 0;
const END: usize = ('z' as usize) - ('a' as usize) + 2;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Grid<T> {
    width: usize,
    height: usize,
    items: Vec<T>,
}

impl<T: PartialEq> Grid<T> {
    pub fn from_str<F: Fn(char) -> T>(input: &str, closure: F) -> Self {
        let height = input.lines().count();

        let width = input.lines().next().unwrap().len();

        let items = input
            .chars()
            .filter(|x| x != &'\n')
            .map(closure)
            .collect::<Vec<T>>();

        Grid {
            width,
            height,
            items,
        }
    }

    pub fn get(&self, point: &Point) -> &T {
        match self.items.get(self.width * point.y + point.x) {
            Some(x) => x,
            None => panic!("({},{}) OOB for Grid", point.x, point.y),
        }
    }

    pub fn find(&self, item: T) -> Option<Point> {
        let result = self.items.iter().enumerate().find(|(_, x)| x == &&item);

        result.map(|(idx, _)| {
            Point::new(
                idx % self.width, // x
                idx / self.width, // y
            )
        })
    }

    pub fn find_all(&self, item: T) -> Vec<Point> {
        self.items
            .iter()
            .enumerate()
            .filter(|(_, x)| x == &&item)
            .map(|(idx, _)| Point::new(idx % self.width, idx / self.width))
            .collect()
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, item) in self.items.iter().enumerate() {
            if idx % self.width == 0 {
                writeln!(f).unwrap();
            }
            write!(f, "{:2} ", item).unwrap();
        }
        Ok(())
    }
}

fn valid_elevation(src: &usize, dst: &usize) -> bool {
    if src >= dst || dst - src == 1 {
        return true;
    }
    false
}

fn adjacent(map: &Grid<usize>, cur: &Point) -> Vec<Point> {
    let mut adjacent = Vec::new();
    let elevation = map.get(cur);

    if cur.x > 0 {
        let left = Point::new(cur.x - 1, cur.y);
        if valid_elevation(elevation, map.get(&left)) {
            adjacent.push(left);
        }
    }

    if cur.y > 0 {
        let top = Point::new(cur.x, cur.y - 1);
        if valid_elevation(elevation, map.get(&top)) {
            adjacent.push(top);
        }
    }

    if cur.y < map.height - 1 {
        let bottom = Point::new(cur.x, cur.y + 1);
        if valid_elevation(elevation, map.get(&bottom)) {
            adjacent.push(bottom);
        }
    }

    if cur.x < map.width - 1 {
        let right = Point::new(cur.x + 1, cur.y);
        if valid_elevation(elevation, map.get(&right)) {
            adjacent.push(right);
        }
    }
    adjacent
}

pub fn part_one(input: &str) -> Option<u32> {
    let map_func = |c: char| -> usize {
        match c {
            'S' => START,
            'E' => END,
            x => (x as usize) - ('a' as usize) + 1,
        }
    };

    let map = Grid::<usize>::from_str(input, map_func);
    let end = map.find(END).unwrap();
    let start = map.find(START).unwrap();

    let mut visited: HashSet<Point> = HashSet::from([start.clone()]);
    let mut queue: VecDeque<Point> = VecDeque::from([start.clone()]);
    let mut depths: HashMap<Point, usize> = HashMap::from([(start, 0)]);

    while !queue.is_empty() {
        let p = queue.pop_front().unwrap();
        let depth = *depths.get(&p).unwrap();
        if p == end {
            return Some(*depths.get(&end).unwrap() as u32);
        }

        for edge in adjacent(&map, &p) {
            if !visited.contains(&edge) {
                visited.insert(edge.clone());
                depths.insert(edge.clone(), depth + 1);
                queue.push_back(edge);
            }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let map_func = |c: char| -> usize {
        match c {
            'S' => 1,
            'E' => END,
            x => (x as usize) - ('a' as usize) + 1,
        }
    };

    let map = Grid::<usize>::from_str(input, map_func);
    let end = map.find(END).unwrap();
    let start_points = map.find_all(1);
    let mut shortest_path: usize = map.height * map.width + 1;

    for start in start_points {
        let mut visited: HashSet<Point> = HashSet::from([start.clone()]);
        let mut queue: VecDeque<Point> = VecDeque::from([start.clone()]);
        let mut depths: HashMap<Point, usize> = HashMap::from([(start.clone(), 0)]);

        while !queue.is_empty() {
            let p = queue.pop_front().unwrap();
            let depth = *depths.get(&p).unwrap();
            if p == end {
                let path_length = *depths.get(&end).unwrap();
                if path_length < shortest_path {
                    shortest_path = path_length;
                }
            }

            for edge in adjacent(&map, &p) {
                if !visited.contains(&edge) {
                    visited.insert(edge.clone());
                    depths.insert(edge.clone(), depth + 1);
                    queue.push_back(edge);
                }
            }
        }
    }
    Some(shortest_path as _)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
