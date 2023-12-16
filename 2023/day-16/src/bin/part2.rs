use day_16::{get_energised_tiles, parse_input, Beam};

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn get_starting_beams(width: i32, height: i32) -> Vec<Beam> {
    let mut beams = Vec::new();

    for i in 0..width {
        beams.push(Beam {
            pos: (i, 0),
            dir: (0, 1),
        });
        beams.push(Beam {
            pos: (i, height - 1),
            dir: (0, -1),
        });
    }

    for j in 0..height {
        beams.push(Beam {
            pos: (0, j),
            dir: (1, 0),
        });
        beams.push(Beam {
            pos: (width - 1, j),
            dir: (-1, 0),
        });
    }

    beams
}

fn process(input: &str) -> u32 {
    let grid = parse_input(input);
    let mut max = 0;

    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    for beam in get_starting_beams(width, height) {
        max = u32::max(max, get_energised_tiles(&grid, beam));
    }

    max
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part2.txt");
        let result = process(input);
        assert_eq!(result, 51);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 8216);
    }
}
