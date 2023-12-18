use day_18::parse_input_part2;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

// Finds the area if the points were infinitely small on a continuous grid
// This means it essentially finds the area inside the top left of each tile (as it doesn't account
// for the tiles having width/height/area themselves)
fn area(perimeter: &[(i64, i64)]) -> u64 {
    let mut sum = 0;
    for pair in perimeter.windows(2) {
        sum += pair[0].0 * pair[1].1 - pair[0].1 * pair[1].0;
    }

    sum.unsigned_abs() / 2
}

#[derive(PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn get_direction(p1: (i64, i64), p2: (i64, i64)) -> Direction {
    if p2.0 > p1.0 {
        Direction::Right
    } else if p2.0 < p1.0 {
        Direction::Left
    } else if p2.1 > p1.1 {
        Direction::Down
    } else if p2.1 < p1.1 {
        Direction::Up
    } else {
        panic!("Not a horizontal or vertical line");
    }
}

// Gets the length of the perimeter that is "outside" the region determined by the top left of
// each vertex. This gives us the area not included by the area() function.
// For example, O represents outside, # is inside:
// ######O
// #.....O
// OO#...O
// ..#...O
// ..#...O
// ###.OOO
// #...O..
// O#..##O
// .#....O
// .OOOOOO
fn perimeter_outside_tl(perimeter: &[(i64, i64)]) -> u64 {
    let mut sum = 0;
    for pair in perimeter.windows(3) {
        match get_direction(pair[0], pair[1]) {
            Direction::Left => {
                // Add length of stretch
                sum += pair[0].0 - pair[1].0;

                // If going left and next is down, don't count corner twice
                if get_direction(pair[1], pair[2]) == Direction::Down {
                    sum -= 1;
                }
            }
            Direction::Down => {
                sum += pair[1].1 - pair[0].1;

                // If going down and next is left, make sure to include corner
                if get_direction(pair[1], pair[2]) == Direction::Left {
                    sum += 1;
                }
            }
            _ => (),
        }
    }

    sum as u64
}

fn process(input: &str) -> u64 {
    let instructions = parse_input_part2(input);
    let mut current = (0, 0);
    let mut perimeter = vec![current];

    for instruction in instructions {
        current = (
            current.0 + instruction.dir.0 * instruction.distance,
            current.1 + instruction.dir.1 * instruction.distance,
        );
        perimeter.push(current);
    }

    let area = area(&perimeter);
    let perimeter_outside_tl = perimeter_outside_tl(&perimeter);

    area + perimeter_outside_tl
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part2.txt");
        let result = process(input);
        assert_eq!(result, 952408144115);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 92556825427032);
    }
}
