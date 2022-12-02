#[derive(Debug)]
enum Action {
    Win,
    Lose,
    Tie,
}

impl Action {
    pub fn score(&self) -> u32 {
        match self {
            Action::Win => 6,
            Action::Tie => 3,
            Action::Lose => 0,
        }
    }

    fn from_char(input: char) -> Action {
        match input {
            'X' => Action::Lose,
            'Y' => Action::Tie,
            'Z' => Action::Win,
            _ => panic!("Bad Action: {input}"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_char(input: char) -> Move {
        match input {
            'A' | 'X' => Move::Rock,
            'B' | 'Y' => Move::Paper,
            'C' | 'Z' => Move::Scissors,
            _ => panic!("Bad Move: {input}"),
        }
    }

    pub fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn win(&self) -> u32 {
        self.score() + Action::Win.score()
    }

    pub fn tie(&self) -> u32 {
        self.score() + Action::Tie.score()
    }

    pub fn lose(&self) -> u32 {
        self.score() + Action::Lose.score()
    }
}

fn score_round(moves: &(Move, Move)) -> (u32, u32) {
    let (p1_move, p2_move) = moves;

    match p1_move {
        Move::Rock => match p2_move {
            Move::Rock => (p1_move.tie(), p2_move.tie()),
            Move::Paper => (p1_move.lose(), p2_move.win()),
            Move::Scissors => (p1_move.win(), p2_move.lose()),
        },
        Move::Paper => match p2_move {
            Move::Rock => (p1_move.win(), p2_move.lose()),
            Move::Paper => (p1_move.tie(), p2_move.tie()),
            Move::Scissors => (p1_move.lose(), p2_move.win()),
        },
        Move::Scissors => match p2_move {
            Move::Rock => (p1_move.lose(), p2_move.win()),
            Move::Paper => (p1_move.win(), p2_move.lose()),
            Move::Scissors => (p1_move.tie(), p2_move.tie()),
        },
    }
}

fn parse_line_part1(line: &str) -> (Move, Move) {
    if line.len() != 3 {
        panic!("Bad Line: {line}");
    }
    (
        Move::from_char(line.chars().next().unwrap()),
        Move::from_char(line.chars().nth(2).unwrap()),
    )
}

fn parse_line_part2(line: &str) -> (Move, Move) {
    if line.len() != 3 {
        panic!("Bad Line: {line}");
    }
    let p1_move = Move::from_char(line.chars().next().unwrap());
    let p2_action = Action::from_char(line.chars().nth(2).unwrap());

    let p2_move: Move = match p2_action {
        Action::Win => match p1_move {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        },
        Action::Lose => match p1_move {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        },
        Action::Tie => p1_move,
    };

    (p1_move, p2_move)
}

pub fn part_one(input: &str) -> Option<u32> {
    let moves: Vec<(Move, Move)> = input.lines().map(parse_line_part1).collect();

    let scores: Vec<(u32, u32)> = moves.iter().map(score_round).collect();

    let p2_score = scores.iter().map(|&(.., x)| x).sum();

    Some(p2_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let moves: Vec<(Move, Move)> = input.lines().map(parse_line_part2).collect();

    let scores: Vec<(u32, u32)> = moves.iter().map(score_round).collect();

    let p2_score = scores.iter().map(|&(.., x)| x).sum();

    Some(p2_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
