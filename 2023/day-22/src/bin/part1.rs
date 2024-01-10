use std::{cmp, collections::HashSet};

use itertools::Itertools;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

// Start and end are both included in the brick
#[derive(Debug, Clone)]
struct Brick {
    start: (i32, i32, i32),
    end: (i32, i32, i32),
}

fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

impl Brick {
    fn _intersects(&self, other: &Brick, z_offset: i32) -> bool {
        let (min_x, max_x) = min_max(self.start.0, self.end.0);
        let (other_min_x, other_max_x) = min_max(other.start.0, other.end.0);
        if other_max_x < min_x || other_min_x > max_x {
            return false;
        }

        let (min_y, max_y) = min_max(self.start.1, self.end.1);
        let (other_min_y, other_max_y) = min_max(other.start.1, other.end.1);
        if other_max_y < min_y || other_min_y > max_y {
            return false;
        }

        let (min_z, max_z) = min_max(self.start.2 + z_offset, self.end.2 + z_offset);
        let (other_min_z, other_max_z) = min_max(other.start.2, other.end.2);
        if other_max_z < min_z || other_min_z > max_z {
            return false;
        }

        true
    }

    fn is_resting_on(&self, other: &Brick) -> bool {
        self._intersects(other, -1)
    }

    fn is_on_ground(&self) -> bool {
        cmp::min(self.start.2, self.end.2) == 1
    }

    fn move_down(&mut self) {
        self.start.2 -= 1;
        self.end.2 -= 1;
    }

    fn min_z(&self) -> i32 {
        cmp::min(self.start.2, self.end.2)
    }
}

fn process(input: &str) -> u32 {
    let mut bricks = Vec::new();
    for line in input.trim().split('\n') {
        let (start, end) = line.split_once('~').unwrap();
        let start_ints = start.split(',').map(|n| n.parse().unwrap());
        let end_ints = end.split(',').map(|n| n.parse().unwrap());

        bricks.push(Brick {
            start: start_ints.take(3).collect_tuple().unwrap(),
            end: end_ints.take(3).collect_tuple().unwrap(),
        })
    }

    // Sort from lowest bottom to highest bottom
    bricks.sort_by(|b1, b2| b1.min_z().cmp(&b2.min_z()));

    // Settle all bricks
    for i in 0..bricks.len() {
        while !bricks[i].is_on_ground()
            && !bricks[..i]
                .iter()
                .any(|other| bricks[i].is_resting_on(other))
        {
            // Not settled
            bricks[i].move_down();
        }
    }

    // Check the bricks each brick supports
    let mut supporting: Vec<Vec<usize>> = Vec::new();

    bricks.sort_by(|b1, b2| b1.min_z().cmp(&b2.min_z()));
    for (i, brick) in bricks.iter().enumerate().rev().skip(1).rev() {
        supporting.push(
            bricks
                .iter()
                .enumerate()
                .skip(i + 1)
                .filter(|(_, other)| other.is_resting_on(brick))
                .map(|(j, _)| j)
                .collect(),
        );
    }

    // TODO: deal with top better
    supporting.push(Vec::new());

    // Check which ones can be disintegrated
    let mut bricks_to_disintegrate = HashSet::new();
    for (i, s) in supporting.iter().enumerate() {
        // If all supporting have another support, add i to bricks to disintegrate
        if s.iter().all(|other| {
            supporting
                .iter()
                .enumerate()
                .filter(|(j, _)| j != &i)
                .any(|(_, s2)| s2.contains(&other))
        }) {
            bricks_to_disintegrate.insert(i);
        }
    }

    bricks_to_disintegrate.len() as u32
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part1.txt");
        let result = process(input);
        assert_eq!(result, 5);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 418);
    }
}
