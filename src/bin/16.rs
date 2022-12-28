use std::cmp;
use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::alpha1,
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

fn parse_valve(input: &str) -> IResult<&str, (String, u32, Vec<String>)> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, name) = is_not(" ")(input)?;
    let (input, _) = tag(" has flow rate=")(input)?;
    let (input, flow_rate) = map_res(is_not(";"), |s: &str| s.parse::<u32>())(input)?;
    let (input, _) = alt((tag("; tunnel "), tag("; tunnels ")))(input)?;
    let (input, _) = alt((tag("lead "), tag("leads ")))(input)?;
    let (input, _) = alt((tag("to valve "), tag("to valves ")))(input)?;
    let (input, conn_str) = separated_list1(tag(", "), alpha1)(input)?;
    let connections = conn_str.iter().map(|x| String::from(*x)).collect();

    Ok((input, (name.to_owned(), flow_rate, connections)))
}

struct ValveGraph {
    flow_rates: HashMap<String, u32>,
    state_masks: HashMap<String, u32>,
    distances: HashMap<String, HashMap<String, u32>>,
}

impl ValveGraph {
    pub fn from_str(input: &str) -> Self {
        let mut names: Vec<String> = vec![];
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut flow_rates: HashMap<String, u32> = HashMap::new();
        let mut state_masks: HashMap<String, u32> = HashMap::new();

        for line in input.lines() {
            let (_, (name, flow_rate, tunnels)) = parse_valve(line).unwrap();

            names.push(name.clone());
            graph.insert(name.clone(), tunnels);
            if flow_rate > 0 {
                flow_rates.insert(name.clone(), flow_rate);
                state_masks.insert(name, 1 << state_masks.len());
            }
        }

        // Floyd-Warshall
        let mut distances: HashMap<String, HashMap<String, u32>> = names
            .iter()
            .map(|name| {
                let dists = names
                    .iter()
                    .map(|other_name| {
                        let cost = match graph.get(other_name).unwrap().contains(name) {
                            true => 1,
                            false => 99,
                        };
                        (other_name.to_owned(), cost)
                    })
                    .collect();
                (name.to_owned(), dists)
            })
            .collect();

        for k in names.iter() {
            for i in names.iter() {
                for j in names.iter() {
                    let value = distances.get(i).unwrap().get(k).unwrap()
                        + distances.get(k).unwrap().get(j).unwrap();
                    if &value < distances.get(i).unwrap().get(j).unwrap() {
                        *distances.get_mut(i).unwrap().get_mut(j).unwrap() = value;
                    }
                }
            }
        }
        ValveGraph {
            flow_rates,
            state_masks,
            distances,
        }
    }

    fn simulate(&self, start_node: &str, start_time: u32) -> HashMap<u32, u32> {
        let mut pressure_map: HashMap<u32, u32> = HashMap::new();
        self._simulate_valve(start_node, 0, start_time, 0, &mut pressure_map);
        pressure_map
    }

    fn _simulate_valve(
        &self,
        current: &str,
        current_state: u32,
        time: u32,
        flow_rate: u32,
        pressure_map: &mut HashMap<u32, u32>,
    ) {
        pressure_map.insert(
            current_state,
            cmp::max(*pressure_map.get(&current_state).unwrap_or(&0), flow_rate),
        );
        for valve in self.flow_rates.keys() {
            let new_time: i32 = time as i32
                - (*self.distances.get(current).unwrap().get(valve).unwrap() as i32)
                - 1;
            if (self.state_masks.get(valve).unwrap() & current_state) > 0 || new_time <= 0 {
                continue;
            }
            let new_rate = flow_rate + new_time as u32 * self.flow_rates.get(valve).unwrap();
            self._simulate_valve(
                valve,
                self.state_masks.get(valve).unwrap() | current_state,
                new_time as u32,
                new_rate,
                pressure_map,
            );
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        *ValveGraph::from_str(input)
            .simulate("AA", 30)
            .iter()
            .map(|(_, v)| v)
            .max()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let pressure_map = ValveGraph::from_str(input).simulate("AA", 26);
    let mut max_pressure = 0;

    for (state, value) in pressure_map.iter() {
        for (other_state, other_value) in pressure_map.iter() {
            if state & other_state == 0 && value + other_value > max_pressure {
                max_pressure = value + other_value;
            }
        }
    }
    Some(max_pressure)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
