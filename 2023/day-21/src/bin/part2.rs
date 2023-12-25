#![feature(int_roundings)]
use std::collections::HashSet;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input, 26501365);
    println!("{result}");
}

fn print_positions(grid: &Vec<Vec<bool>>, current_positions: &HashSet<(i64, i64)>) {
    let left = 1;
    let right = 2;
    for j in -(grid.len() as i64) * left as i64..(grid.len() as i64 * right as i64) {
        let wrapped_y = j.rem_euclid(grid.len() as i64) as usize;
        for i in -(grid[0].len() as i64) * left..(grid[0].len() as i64 * right) {
            let wrapped_x = i.rem_euclid(grid[0].len() as i64) as usize;

            let tile = grid[wrapped_y][wrapped_x];
            if tile {
                if current_positions.contains(&(i, j)) {
                    print!("OO");
                } else {
                    print!("  ");
                }
            } else {
                if current_positions.contains(&(i, j)) {
                    print!("XX");
                } else {
                    print!("██");
                }
            }
        }
        println!();
    }
}

fn step(grid: &Vec<Vec<bool>>, current_positions: &HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    let mut new_positions = HashSet::new();

    for pos in current_positions {
        for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if new_pos.0 < 0
                || new_pos.1 < 0
                || new_pos.0 >= grid[0].len() as i64
                || new_pos.1 >= grid.len() as i64
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

fn get_num_positions(grid: &Vec<Vec<bool>>, start: (i64, i64), num_steps: u64) -> u64 {
    let mut current_positions = HashSet::from_iter([start]);

    for _ in 0..num_steps {
        current_positions = step(&grid, &current_positions);
    }

    current_positions.len() as u64
}

// Assumptions for this to work:
// - S is in the middle of the input
// - There is a diamon in the middle of the grid, with a 4 tile gap, then 4 quater diamonds at the
// edges
// - extra_steps is half the width of the grid - meaning that the diamond will end up in the gaps
// between the diamonds (in the input)
fn process(input: &str, num_steps: u64) -> u64 {
    // TODO: optimise using prev explored so don't have to keep processing them?
    let mut grid: Vec<Vec<bool>> = Vec::new();

    for line in input.trim().split('\n') {
        grid.push(
            line.chars()
                .map(|c| match c {
                    '#' => false,
                    '.' | 'S' => true,
                    _ => panic!("Unknown tile"),
                })
                .collect(),
        );
    }

    let extra_steps = num_steps % grid.len() as u64;
    let half_grid = grid.len() as i64 / 2;
    assert_eq!(grid.len(), grid[0].len());
    assert_eq!(extra_steps, half_grid as u64);

    // Assume half_grid is odd for now
    let mut centre_diamond_positions_odd =
        get_num_positions(&grid, (half_grid, half_grid), half_grid as u64);
    let mut centre_diamond_positions_even =
        get_num_positions(&grid, (half_grid, half_grid), half_grid as u64 - 1);

    // Swap if half_grid is even
    if half_grid % 2 == 0 {
        (centre_diamond_positions_even, centre_diamond_positions_odd) =
            (centre_diamond_positions_odd, centre_diamond_positions_even);
    }

    let full_grid_positions_even =
        get_num_positions(&grid, (half_grid, half_grid), grid.len() as u64 * 2 + 2);
    let full_grid_positions_odd =
        get_num_positions(&grid, (half_grid, half_grid), grid.len() as u64 * 2 + 1);

    let corner_diamond_positions_even = full_grid_positions_even - centre_diamond_positions_even;
    let corner_diamond_positions_odd = full_grid_positions_odd - centre_diamond_positions_odd;

    // Diamonds are like blocks in a bigger diamond
    // For example, @ is a centre diamond, # is a corner diamond (corner diamonds are named that
    // because they are actually made from the 4 corners of the original grid)
    //
    //   @
    //  # #
    // @ @ @
    //  # #
    //   @

    // Get diamonds along one length of the bigger diamond
    let number_diamonds_one_length = (num_steps * 2) / grid.len() as u64 + 1;

    // Square diamonds along length to get total number of diamonds
    let number_diamonds = number_diamonds_one_length.pow(2);

    let number_corner_diamonds = number_diamonds / 2;
    let number_centre_diamonds = number_diamonds - number_corner_diamonds;

    // Find the number of centre diamonds that are odd. This is every other concentric ring of
    // centre diamonds, which works out to be the number of centre diamonds along the outside edge
    // squared. Using the example from above, but centre diamonds are replaced with O for odd and E
    // for even:
    //
    //   O
    //  # #
    // O E O
    //  # #
    //   O
    let number_centre_diamonds_odd = (number_diamonds_one_length / 2 + 1).pow(2);
    let number_centre_diamonds_even = number_centre_diamonds - number_centre_diamonds_odd;

    // Equal number of odd and even corner diamonds, so just divide by 2
    (number_corner_diamonds / 2) * corner_diamond_positions_odd
        + (number_corner_diamonds / 2) * corner_diamond_positions_even
        + number_centre_diamonds_odd * centre_diamond_positions_odd
        + number_centre_diamonds_even * centre_diamond_positions_even
}

#[cfg(test)]
pub mod tests {
    use super::*;

    // NOTE: my code heavily relies on the specific input given
    // This means that test inputs don't work
    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input, 26501365);
        assert_eq!(result, 630129824772393);
    }
}
