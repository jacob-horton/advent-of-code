use std::{cmp, collections::HashMap};

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

#[derive(Debug, Clone)]
struct Intersection {
    // Neighbour -> distance
    neighbours: HashMap<(i32, i32), u32>,
}

fn _get_intersections(
    grid: &Vec<Vec<char>>,
    start: (i32, i32),
    came_from: (i32, i32),
    intersections: &mut HashMap<(i32, i32), Intersection>,
) {
    // Go along `.`s. Once find slope, next `.` will have surrounding slopes on each side
    // For each one (except one we came from), call _get_intersections with came_from as the `.`
    let mut prev_tile = came_from;
    let mut curr_tile = start;

    let mut next_is_intersection = false;
    let mut backwards_dir = (came_from.0 - start.0, came_from.1 - start.1);
    let mut dist = 1;

    // NOTE: relies on their only being intersections at `>/v/^/<`
    'outer: loop {
        // No change - no neighbours, therefore at end
        if curr_tile == prev_tile {
            // Only break, not return as we want the end added as an intersection
            break;
        }
        prev_tile = curr_tile;

        for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if dir == backwards_dir {
                continue;
            }

            let neighbour = (curr_tile.0 + dir.0, curr_tile.1 + dir.1);
            if neighbour.0 < 0
                || neighbour.1 < 0
                || neighbour.0 >= grid[0].len() as i32
                || neighbour.1 >= grid.len() as i32
            {
                continue;
            }

            let is_intersection = next_is_intersection;

            match grid[neighbour.1 as usize][neighbour.0 as usize] {
                '#' => continue,
                '>' | '<' | 'v' | '^' => next_is_intersection = true,
                '.' => (),
                _ => panic!("Unknown character"),
            }
            dist += 1;

            backwards_dir = (curr_tile.0 - neighbour.0, curr_tile.1 - neighbour.1);
            curr_tile = neighbour;

            if is_intersection {
                break 'outer;
            }
        }
    }

    // We are at an intersection here
    let came_from_neighbour = intersections.get_mut(&came_from);
    let mut intersection = Intersection {
        neighbours: HashMap::new(),
    };

    if let Some(came_from_neighbour) = came_from_neighbour {
        came_from_neighbour.neighbours.insert(curr_tile, dist);
        intersection.neighbours.insert(came_from, dist);
    }

    if intersections.contains_key(&curr_tile) {
        return;
    }

    intersections.insert(curr_tile, intersection);

    for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        if dir == backwards_dir {
            continue;
        }

        let neighbour = (curr_tile.0 + dir.0, curr_tile.1 + dir.1);
        if neighbour.0 < 0
            || neighbour.1 < 0
            || neighbour.0 >= grid[0].len() as i32
            || neighbour.1 >= grid.len() as i32
        {
            continue;
        }

        match grid[neighbour.1 as usize][neighbour.0 as usize] {
            '>' | '<' | 'v' | '^' => _get_intersections(grid, neighbour, curr_tile, intersections),
            '.' | '#' => (),
            _ => panic!("Unknown character"),
        }
    }
}

fn get_intersections(
    grid: &Vec<Vec<char>>,
    start: (i32, i32),
) -> HashMap<(i32, i32), Intersection> {
    let mut intersections = HashMap::new();
    intersections.insert(
        start,
        Intersection {
            neighbours: HashMap::new(),
        },
    );

    _get_intersections(grid, (start.0, 1), start, &mut intersections);
    intersections.to_owned()
}

fn process(input: &str) -> u32 {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in input.trim().split('\n') {
        grid.push(line.chars().collect());
    }

    let start_x = grid[0].iter().position(|c| c == &'.').unwrap() as i32;
    let start = (start_x, 0);

    // Get all the intersection points
    let intersections = get_intersections(&grid, start);
    let mut frontier = vec![vec![start]];
    let mut finished_paths = Vec::new();

    while let Some(path) = frontier.pop() {
        if path.last().unwrap().1 == grid.len() as i32 - 1 {
            finished_paths.push(path);
            continue;
        }

        let curr_intersection = path.last().unwrap();
        let next_intersections = intersections
            .get(curr_intersection)
            .unwrap()
            .neighbours
            .keys()
            .filter(|i| !path.contains(i));

        frontier.append(
            &mut next_intersections
                .map(|i| {
                    let mut new_path = path.clone();
                    new_path.push(*i);
                    new_path
                })
                .collect(),
        )
    }

    let mut max_length = 0;
    for path in finished_paths {
        let mut length = 0;
        for pair in path.windows(2) {
            // Add distance between pairs
            length += intersections
                .get(&pair[0])
                .unwrap()
                .neighbours
                .get(&pair[1])
                .unwrap();
        }

        max_length = cmp::max(max_length, length);
    }

    max_length
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part2.txt");
        let result = process(input);
        assert_eq!(result, 154);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 6590);
    }
}
