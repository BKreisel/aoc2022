use itertools::Itertools;
use nom::{
    bytes::complete::{is_not, tag},
    combinator::map_res,
    IResult,
};
use std::cmp;

#[derive(Clone, Debug, Default, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn mdist(&self, other: &Self) -> i32 {
        ((self.x as i32 - other.x as i32).abs()) + ((self.y as i32 - other.y as i32).abs())
    }
}

#[derive(Debug, PartialEq)]
struct Sensor {
    location: Point,
    beacon: Point,
    mdist: i32,
}

impl Sensor {
    pub fn from_str(input: &str) -> Self {
        match parse_sensor(input) {
            Ok(s) => s.1,
            Err(e) => panic!("{}", e),
        }
    }
}

fn parse_i32(input: &str) -> IResult<&str, i32> {
    map_res(is_not(",:"), |s: &str| s.parse::<i32>())(input)
}

fn parse_sensor(input: &str) -> IResult<&str, Sensor> {
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, loc_x) = parse_i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, loc_y) = parse_i32(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, beacon_x) = parse_i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, beacon_y) = parse_i32(input)?;

    let location = Point { x: loc_x, y: loc_y };

    let beacon = Point {
        x: beacon_x,
        y: beacon_y,
    };

    let mdist = location.mdist(&beacon);
    Ok((
        input,
        Sensor {
            location,
            beacon,
            mdist,
        },
    ))
}

fn part_one_with_y(input: &str, y: i32) -> Option<u32> {
    let sensors: Vec<Sensor> = input.lines().map(Sensor::from_str).collect();

    let min_x = sensors
        .iter()
        .map(|s| s.location.x - s.mdist as i32)
        .min()
        .unwrap();

    let max_x = sensors
        .iter()
        .map(|s| s.location.x + s.mdist as i32)
        .max()
        .unwrap();

    let potential = (min_x..=max_x)
        .filter(|x| {
            sensors
                .iter()
                .any(|s| s.location.mdist(&Point { x: *x, y }) <= s.mdist)
        })
        .count();

    let beacons = sensors
        .iter()
        .map(|s| s.beacon.clone())
        .filter(|b| b.y == y)
        .unique()
        .count();

    Some((potential - beacons) as _)
}

fn part_two_with_max(input: &str, max: u32) -> Option<u64> {
    let sensors: Vec<Sensor> = input.lines().map(Sensor::from_str).collect();

    let mut circles: Vec<Point> = Vec::new();

    for sensor in sensors.iter() {
        let min_y = cmp::max(sensor.location.y - sensor.mdist - 1, 0);
        let max_y = cmp::min(sensor.location.y + sensor.mdist, max as _);
        for y in min_y..=max_y {
            let x_abs = (sensor.mdist + 1) - (y - sensor.location.y).abs();
            let x1 = sensor.location.x - x_abs;
            let x2 = sensor.location.x + x_abs;

            if x1 >= 0 && x1 <= max as _ {
                circles.push(Point { x: x1, y });
            }
            if x2 >= 0 && x2 <= max as _ {
                circles.push(Point { x: x2, y });
            }
        }
    }

    let mut valid: bool;
    for point in circles.iter() {
        valid = true;
        for sensor in sensors.iter() {
            if sensor.location.mdist(point) <= sensor.mdist {
                valid = false;
                break;
            }
        }
        if valid {
            return Some((point.x as u64 * 4000000) + point.y as u64);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    part_one_with_y(input, 2000000)
}

pub fn part_two(input: &str) -> Option<u64> {
    part_two_with_max(input, 4000000)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let location = Point { x: 2, y: 18 };
        let beacon = Point { x: -2, y: 15 };
        let mdist = location.mdist(&beacon);
        let input_str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";
        assert_eq!(
            Sensor::from_str(input_str),
            Sensor {
                location,
                beacon,
                mdist
            }
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one_with_y(&input, 10), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two_with_max(&input, 20), Some(56000011));
    }
}
