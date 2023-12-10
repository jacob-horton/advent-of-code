use std::collections::HashSet;

use day_10::{get_next, parse_input, DIRECTIONS};

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

// fn print_grid(pipes: &Vec<(u32, u32)>) {
//     let max_x = pipes.iter().max_by(|(x1, _), (x2, _)| x1.cmp(x2)).unwrap().0;
//     let max_y = pipes.iter().max_by(|(_, y1), (_, y2)| y1.cmp(y2)).unwrap().1;
//
//     for y in 0..=max_y {
//         for x in 0..=max_x {
//             if pipes.contains(&(x, y)) {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         }
//         println!();
//     }
// }

// NOTE: only works if pipes are in order
fn expand_pipes(pipes: &Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut expanded = Vec::new();
    let mut new_grid = pipes.clone();
    new_grid.push(pipes[0]);

    for window in new_grid.windows(2) {
        let (curr, next) = (window[0], window[1]);
        expanded.push((curr.0 * 2, curr.1 * 2));

        // Add half way point
        expanded.push((
            (curr.0 as i32 * 2 + (next.0 as i32 - curr.0 as i32)) as u32,
            (curr.1 as i32 * 2 + (next.1 as i32 - curr.1 as i32)) as u32,
        ));
    }

    expanded
}

fn get_inside_point(expanded_pipes: &Vec<(u32, u32)>) -> (u32, u32) {
    let max_x = expanded_pipes
        .iter()
        .max_by(|(x1, _), (x2, _)| x1.cmp(x2))
        .unwrap()
        .0;
    let max_y = expanded_pipes
        .iter()
        .max_by(|(_, y1), (_, y2)| y1.cmp(y2))
        .unwrap()
        .1;

    for y in 0..max_y {
        for window in (0..max_x).collect::<Vec<_>>().windows(3) {
            let (before, current, after) = (window[0], window[1], window[2]);
            if expanded_pipes.contains(&(current, y)) {
                if !expanded_pipes.contains(&(before, y)) && !expanded_pipes.contains(&(after, y)) {
                    return (after, y);
                } else {
                    // We have found a wall, but it also has a wall next to it, so we don't want to
                    // keep searching this row as we won't know if the next is inside or out
                    break;
                }
            }
        }
    }

    panic!("Inside not found");
}

fn get_num_inside(pipes: &Vec<(u32, u32)>) -> u32 {
    // "zoom in" on grid - multiply positions by 2, connect pipes
    // Then flood fill on inside (start), and count up points that land on original grid.
    // This is all the points that have an even x and y value
    let expanded = expand_pipes(pipes);
    let start = get_inside_point(&expanded);
    let expanded_set: HashSet<(u32, u32)> = HashSet::from_iter(expanded);

    let mut explored = HashSet::new();
    let mut frontier = vec![start];
    while !frontier.is_empty() {
        let curr = frontier.pop().unwrap();
        if explored.contains(&curr) {
            continue;
        }

        explored.insert(curr);

        let next = DIRECTIONS
            .map(|(dx, dy)| ((curr.0 as i32 + dx) as u32, (curr.1 as i32 + dy) as u32))
            .into_iter()
            .filter(|pos| !expanded_set.contains(pos));

        frontier.append(&mut next.collect());
    }

    explored
        .into_iter()
        .filter(|(x, y)| x % 2 == 0 && y % 2 == 0)
        .count() as u32
}

fn process(input: &str) -> u32 {
    let (start, grid) = parse_input(input);
    let mut curr = start;
    let mut pipe_positions = vec![curr];

    // Prev only needed to know which way we came from. We can go any direction at start, so just
    // put (0, 0)
    let mut next = get_next((0, 0), start, &grid);

    while next != start {
        pipe_positions.push(next);

        let new_next = get_next(curr, next, &grid);
        curr = next;
        next = new_next;
    }

    get_num_inside(&pipe_positions)
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
