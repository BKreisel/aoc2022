struct Cpu {
    register: i32,
    cycles: u32,
}

enum Instruction {
    Add(i32),
    Nop,
}

impl Instruction {
    pub fn cost(&self) -> u32 {
        match self {
            Self::Add(_) => 2,
            Self::Nop => 1,
        }
    }

    pub fn parse_str(input: &str) -> Self {
        let mut iter = input.split(' ');
        match iter.next().unwrap() {
            "addx" => Self::Add(iter.next().unwrap().parse::<i32>().unwrap()),
            "noop" => Self::Nop,
            x => panic!("Bad Instruction: {}", x),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut cpu = Cpu {
        register: 1,
        cycles: 1,
    };
    let key_cycles = Vec::from([20, 60, 100, 140, 180, 220]);
    let mut signal_strength: i32 = 0;

    let instructions: Vec<Instruction> = input.lines().map(Instruction::parse_str).collect();

    for instr in instructions {
        for cycle in cpu.cycles..(cpu.cycles + instr.cost()) {
            if key_cycles.contains(&cycle) {
                signal_strength += cpu.register * cycle as i32;
            }
        }

        if let Instruction::Add(val) = instr {
            cpu.register += val;
        }
        cpu.cycles += instr.cost();
    }

    Some(signal_strength as _)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cpu = Cpu {
        register: 1,
        cycles: 1,
    };

    let instructions: Vec<Instruction> = input.lines().map(Instruction::parse_str).collect();

    for instr in instructions {
        for cycle in cpu.cycles..(cpu.cycles + instr.cost()) {
            let pos: i32 = (cycle as i32 - 1) % 40;
            if pos == 0 {
                println!()
            }
            if (cpu.register as i32 - pos).abs() <= 1 {
                print!("#");
            } else {
                print!(".");
            }
        }

        if let Instruction::Add(val) = instr {
            cpu.register += val;
        }
        cpu.cycles += instr.cost();
    }
    println!();
    Some(0)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(0));
    }
}
