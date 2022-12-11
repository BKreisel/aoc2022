fn is_edge(x: usize, y: usize, width: usize, height: usize) -> bool {
    x == 0 || y == 0 || x == (width - 1) || y == (height - 1)
}

fn is_visible(x: usize, y: usize, width: usize, height: usize, trees: &[usize]) -> bool {
    let tree_height = trees.get(y * width + x).unwrap();

    // Row Check Right
    for w in (x + 1)..width {
        if trees.get((y * width) + w).unwrap() >= tree_height {
            break;
        }
        if w == (width - 1) {
            return true;
        }
    }

    // Row Check Left
    for w in 0..x {
        if trees.get((y * width) + w).unwrap() >= tree_height {
            break;
        }
        if w == (x - 1) {
            return true;
        }
    }

    // Col Check Down
    for z in (y + 1)..height {
        if trees.get((z * width) + x).unwrap() >= tree_height {
            break;
        }
        if z == (height - 1) {
            return true;
        }
    }

    // Col Check Up
    for z in 0..y {
        if trees.get((z * width) + x).unwrap() >= tree_height {
            break;
        }
        if z == (y - 1) {
            return true;
        }
    }
    false
}

fn scenic_score(x: usize, y: usize, width: usize, height: usize, trees: &[usize]) -> u32 {
    if is_edge(x, y, width, height) {
        return 0;
    }
    let tree_height = trees.get(y * width + x).unwrap();

    // Left Score
    let mut lscore: u32 = 0;
    for w in (0..x).rev() {
        lscore += 1;
        if trees.get((y * width) + w).unwrap() >= tree_height {
            break;
        }
    }

    // Right Score
    let mut rscore: u32 = 0;
    for w in (x + 1)..width {
        rscore += 1;
        if trees.get((y * width) + w).unwrap() >= tree_height {
            break;
        }
    }

    // Up Score
    let mut uscore: u32 = 0;
    for z in (0..y).rev() {
        uscore += 1;
        if trees.get((z * width) + x).unwrap() >= tree_height {
            break;
        }
    }

    // Down Score
    let mut dscore: u32 = 0;
    for z in (y + 1)..height {
        dscore += 1;
        if trees.get((z * width) + x).unwrap() >= tree_height {
            break;
        }
    }

    lscore * rscore * uscore * dscore
}

pub fn part_one(input: &str) -> Option<u32> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let trees: Vec<usize> = input
        .chars()
        .filter(|x| *x != '\n')
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();

    let mut visible = 0;
    for y in 0..height {
        for x in 0..width {
            if is_edge(x, y, width, height) {
                visible += 1;
                continue;
            }
            if is_visible(x, y, width, height, &trees) {
                visible += 1;
            }
        }
    }

    Some(visible)
}

pub fn part_two(input: &str) -> Option<u32> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let trees: Vec<usize> = input
        .chars()
        .filter(|x| *x != '\n')
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();

    Some(
        (0..height)
            .into_iter()
            .map(|y| {
                (0..width)
                    .into_iter()
                    .map(|x| scenic_score(x, y, width, height, &trees))
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
