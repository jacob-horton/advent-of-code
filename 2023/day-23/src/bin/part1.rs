fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

#[derive(Debug, Clone)]
struct Path {
    tiles: Vec<(i32, i32)>,
}

fn process(input: &str) -> u32 {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in input.trim().split('\n') {
        grid.push(line.chars().collect());
    }

    let start_x = grid[0].iter().position(|c| c == &'.').unwrap() as i32;

    let mut frontier: Vec<Path> = vec![Path {
        tiles: vec![(start_x, 0)],
    }];
    let mut finished_paths = Vec::new();
    while let Some(curr) = frontier.pop() {
        let curr_tile = curr.tiles.last().unwrap();
        if curr_tile.1 == grid.len() as i32 - 1 {
            finished_paths.push(curr);
            continue;
        }

        for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let neighbour = (curr_tile.0 + dir.0, curr_tile.1 + dir.1);
            if neighbour.0 < 0
                || neighbour.1 < 0
                || neighbour.0 >= grid[0].len() as i32
                || neighbour.1 >= grid.len() as i32
            {
                continue;
            }

            match grid[neighbour.1 as usize][neighbour.0 as usize] {
                '#' => continue,
                '>' => {
                    if dir != (1, 0) {
                        continue;
                    }
                }
                '<' => {
                    if dir != (-1, 0) {
                        continue;
                    }
                }
                'v' => {
                    if dir != (0, 1) {
                        continue;
                    }
                }
                '^' => {
                    if dir != (0, -1) {
                        continue;
                    }
                }
                '.' => (),
                _ => panic!("Unknown character"),
            }

            if let Some(last) = curr.tiles.iter().rev().nth(1) {
                if last == &neighbour {
                    continue;
                }
            }

            let mut new_tiles = curr.tiles.clone();
            new_tiles.push(neighbour);
            frontier.push(Path { tiles: new_tiles });
        }
    }

    let lengths: Vec<_> = finished_paths
        .into_iter()
        .map(|p| p.tiles.len() as u32)
        .collect();

    lengths.into_iter().max().expect("No finished paths found") - 1
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part1.txt");
        let result = process(input);
        assert_eq!(result, 94);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 2222);
    }
}
