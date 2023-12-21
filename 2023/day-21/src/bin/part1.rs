use std::collections::HashSet;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input, 64);
    println!("{result}");
}

fn step(grid: &Vec<Vec<bool>>, current_positions: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut new_positions = HashSet::new();

    for pos in current_positions {
        for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if new_pos.0 < 0
                || new_pos.1 < 0
                || new_pos.0 >= grid[0].len() as i32
                || new_pos.1 >= grid.len() as i32
            {
                continue;
            }

            // Skip rocks
            if !grid[new_pos.1 as usize][new_pos.0 as usize] {
                continue;
            }

            new_positions.insert(new_pos);
        }
    }

    new_positions
}

fn process(input: &str, num_steps: u32) -> u32 {
    let mut start = None;
    let mut grid: Vec<Vec<bool>> = Vec::new();

    for (j, line) in input.trim().split('\n').enumerate() {
        grid.push(
            line.chars()
                .map(|c| match c {
                    '#' => false,
                    '.' | 'S' => true,
                    _ => panic!("Unknown tile"),
                })
                .collect(),
        );

        if let Some(pos) = line.chars().position(|c| c == 'S') {
            start = Some((pos as i32, j as i32));
        }
    }

    let start = start.expect("Could not find starting position");
    let mut current_positions = HashSet::from_iter([start]);
    for _ in 0..num_steps {
        current_positions = step(&grid, &current_positions);
    }

    current_positions.len() as u32
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part1.txt");
        let result = process(input, 6);
        assert_eq!(result, 16);
    }
}
