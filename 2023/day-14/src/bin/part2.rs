#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Round,
    Cube,
}

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    let mut grid = Vec::new();
    for line in input.trim().split('\n') {
        grid.push(
            line.chars()
                .map(|c| match c {
                    '#' => Tile::Cube,
                    'O' => Tile::Round,
                    '.' => Tile::Empty,
                    _ => panic!("Unknown character {c}"),
                })
                .collect(),
        );
    }

    grid
}

enum Direction {
    North,
    East,
    South,
    West,
}

fn tilt(grid: &mut Vec<Vec<Tile>>, dir: Direction) {
    let (shift, mut obstacles) = match dir {
        Direction::North => (1, vec![0i32; grid[0].len()]),
        Direction::South => (-1, vec![grid.len() as i32 - 1; grid[0].len()]),
        Direction::East => (-1, vec![grid[0].len() as i32 - 1; grid.len()]),
        Direction::West => (1, vec![0i32; grid.len()]),
    };

    let mut js: Vec<_> = (0..grid.len()).collect();
    if let Direction::South = dir {
        js = js.into_iter().rev().collect();
    }

    let mut is: Vec<_> = (0..grid[0].len()).collect();
    if let Direction::East = dir {
        is = is.into_iter().rev().collect();
    }

    match dir {
        Direction::North | Direction::South => {
            for j in js {
                for i in is.iter() {
                    match grid[j][*i] {
                        Tile::Cube => obstacles[*i] = j as i32 + shift,
                        Tile::Round => {
                            if obstacles[*i] as usize != j {
                                grid[obstacles[*i] as usize][*i] = Tile::Round;
                                grid[j][*i] = Tile::Empty;
                            }
                            obstacles[*i] += shift;
                        }
                        _ => (),
                    }
                }
            }
        }
        Direction::East | Direction::West => {
            for i in is {
                for j in js.iter() {
                    match grid[*j][i] {
                        Tile::Cube => obstacles[*j] = i as i32 + shift,
                        Tile::Round => {
                            if obstacles[*j] as usize != i {
                                grid[*j][obstacles[*j] as usize] = Tile::Round;
                                grid[*j][i] = Tile::Empty;
                            }
                            obstacles[*j] += shift;
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

fn north_load(grid: &Vec<Vec<Tile>>) -> u32 {
    let mut load = 0;
    for (i, row) in grid.iter().enumerate() {
        for tile in row {
            match tile {
                Tile::Round => load += grid.len() - i,
                _ => (),
            }
        }
    }

    load as u32
}

fn process(input: &str) -> u32 {
    let mut grid = parse_input(input);
    let mut prev = vec![grid.clone()];

    let mut loop_len = 0;
    let mut loop_start = 0;
    let max = 1_000_000_000;

    for i in 0..max {
        for dir in [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            tilt(&mut grid, dir);
        }

        if let Some(prev_i) = prev.iter().position(|p| &grid == p) {
            loop_start = prev_i;
            loop_len = i - prev_i + 1;
            break;
        }

        prev.push(grid.clone());
    }

    let offset = (max - loop_start) % loop_len;
    let grid_at_end = &prev[loop_start + offset];

    north_load(grid_at_end)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part2.txt");
        let result = process(input);
        assert_eq!(result, 64);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 102829);
    }
}
