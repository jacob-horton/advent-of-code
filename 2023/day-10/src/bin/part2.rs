use std::collections::HashSet;

use day_10::{get_next, parse_input, Pipe};
use itertools::Itertools;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

#[derive(Debug, Clone)]
struct LineSegment {
    start: (u32, u32),
    end: (u32, u32),
}

fn get_vertical_line_segments(pipes: &Vec<(u32, u32)>) -> Vec<LineSegment> {
    let mut segments = Vec::new();
    let mut start = pipes[0];

    // Start at pipes[1], go up and loop back to pipes[0] at end to close loop
    let pipes_closed = pipes.iter().enumerate().cycle().skip(1).take(pipes.len());

    for (i, pipe) in pipes_closed {
        if start.0 != pipe.0 {
            // x changed, check if end of vertical segment
            let prev = pipes[(i + pipes.len() - 1) % pipes.len()];
            if start != prev {
                let min_y = u32::min(start.1, prev.1);
                let max_y = u32::max(start.1, prev.1);
                segments.push(LineSegment {
                    start: (start.0, min_y),
                    end: (start.0, max_y),
                });
            }
            start = *pipe;
        }
    }

    segments.into_iter().collect()
}

// TODO: allow any order, check perpendicular
fn intersects(vertical: &LineSegment, horizontal: &LineSegment) -> bool {
    assert_eq!(vertical.start.0, vertical.end.0);
    assert_eq!(horizontal.start.1, horizontal.end.1);

    // We have sorted segments earlier so start is always smaller than end
    let min_y = vertical.start.1;
    let max_y = vertical.end.1;

    let min_x = horizontal.start.0;
    let max_x = horizontal.end.0;

    vertical.start.0 < max_x
        && vertical.start.0 >= min_x
        && horizontal.start.1 < max_y
        && horizontal.start.1 >= min_y
}

fn get_pipe_positions(start: (u32, u32), grid: &Vec<Vec<Pipe>>) -> Vec<(u32, u32)> {
    let mut curr = start;
    let mut pipe_positions = vec![curr];

    // Prev only needs to know which way we came from. Can go any direction at start, so use (0, 0)
    let mut next = get_next((0, 0), start, grid);

    while next != start {
        pipe_positions.push(next);

        let new_next = get_next(curr, next, grid);
        curr = next;
        next = new_next;
    }

    pipe_positions
}

fn get_inside_count(
    line_segments: &[LineSegment],
    pipe_positions: &HashSet<(u32, u32)>,
    grid_size: (u32, u32),
) -> u32 {
    let mut inside_count = 0;

    // TODO: flood fill each "empty" cell, keep record of one cell inside and count, then just do a
    // check for each of those groups

    // Loop through all coords
    for (j, i) in (0..grid_size.0).cartesian_product(0..grid_size.1) {
        // Skip any that are on a pipe
        if pipe_positions.contains(&(i, j)) {
            continue;
        }

        let start = (0, j);
        let end = (i, j);
        let segment = LineSegment { start, end };

        // Check how many times line segment from edge to this position intersects a pipe
        let intersections = line_segments
            .iter()
            .filter(|s| intersects(s, &segment))
            .count();

        // If the number of intersections is odd, we are inside the pipe loop
        if intersections % 2 == 1 {
            inside_count += 1;
        }
    }

    inside_count
}

fn process(input: &str) -> u32 {
    let (start, grid) = parse_input(input);

    // Get list of positions that represent the pipe
    let pipe_positions = get_pipe_positions(start, &grid);

    // Get vertical line segments of pipe (we only care about vertical as we only check horizontal
    // line intersecting vertical pipes to see if a point is inside or out)
    let line_segments = get_vertical_line_segments(&pipe_positions);

    // Convert to hashset for lookup efficiency
    let pipe_positions: HashSet<(u32, u32)> = HashSet::from_iter(pipe_positions);

    get_inside_count(
        &line_segments,
        &pipe_positions,
        (grid.len() as u32, grid[0].len() as u32),
    )
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part2.txt");
        let result = process(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_input_2() {
        let input = include_str!("../inputs/test_part2_2.txt");
        let result = process(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_input_3() {
        let input = include_str!("../inputs/test_part2_3.txt");
        let result = process(input);
        assert_eq!(result, 8);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 417);
    }
}
