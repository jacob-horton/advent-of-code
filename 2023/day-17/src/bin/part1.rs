use std::collections::{BinaryHeap, HashMap, HashSet};

fn main() {
    // 914 too high
    let input = include_str!("../inputs/input.txt");
    // let input = include_str!("../inputs/test_part1.txt");
    // let input = include_str!("../inputs/test.txt");
    let result = process(input);
    println!("{result}");
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut grid = Vec::new();

    for line in input.trim().split('\n') {
        let row = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        grid.push(row);
    }

    grid
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NodeDist {
    node: (i32, i32),
    last_move: (i32, i32),
    heuristic: u32,
    // prev: (i32, i32),
    dist: u32,
}

impl std::hash::Hash for NodeDist {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Only want hash to depend on node and move, not distance or prev
        self.node.hash(state);
        self.last_move.hash(state);
        // self.dist.hash(state);
    }
}

impl Ord for NodeDist {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse comparison as binary heap will pop highest value
        (other.dist + other.heuristic).cmp(&(self.dist + self.heuristic))
        // other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for NodeDist {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// NOTE: only works if only x or only y changes, not both
fn cost_between(grid: &Vec<Vec<u32>>, start: (i32, i32), end: (i32, i32)) -> u32 {
    assert!(start.0 == end.0 || start.1 == end.1);
    let mut cost = 0;

    let mut x = start.0;
    let mut y = start.1;
    while x != end.0 || y != end.1 {
        x += (end.0 - x).signum();
        y += (end.1 - y).signum();
        cost += grid[y as usize][x as usize];
    }

    cost
}

fn heuristic(start: (i32, i32), end: (i32, i32)) -> u32 {
    // Manhattan distance
    end.0.abs_diff(start.0) + end.1.abs_diff(start.1)
}

fn find_shortest_path(grid: &Vec<Vec<u32>>, start: (i32, i32), end: (i32, i32)) -> u32 {
    // TODO: come up with heuristic. Must always be less or equal to true cheapest cost
    // Also must never overestimate step cost (h(n+1) - h(n) <= real(n+1) - real(n))
    let mut frontier = BinaryHeap::new();
    frontier.push(NodeDist {
        node: start,
        last_move: (1, 0),
        heuristic: heuristic(start, end),
        dist: 0,
    });
    frontier.push(NodeDist {
        node: start,
        last_move: (0, 1),
        heuristic: heuristic(start, end),
        dist: 0,
    });

    let mut distances = HashMap::new();
    for n in &frontier {
        distances.insert(n.clone(), 0);
    }

    let mut visited = HashSet::new();
    let mut count = 0;
    while !frontier.is_empty() {
        count += 1;
        if count % 10000 == 0 {
            println!("{:?}", frontier.peek().unwrap());
            println!("{}", frontier.len());
        }
        let curr = frontier.pop().unwrap();

        if curr.node == end {
            println!("Done");
            return curr.dist;
        }

        if visited.contains(&curr) {
            continue;
        }

        visited.insert(curr.clone());

        for dir in [(0, 1), (1, 0)] {
            // Skip parallel
            if curr.last_move == dir {
                continue;
            }

            // Explore 3 spaces perp to last direction
            // TODO: maybe
            // If at start, only 2 (as all in line will be same direction)
            // let mut num = 3;
            // if curr.node == start {
            //     num -= 1;
            // }

            for i in -3..=3 {
                if i == 0 {
                    continue;
                }

                let neighbour = (curr.node.0 + dir.0 * i, curr.node.1 + dir.1 * i);
                if neighbour.0 < 0
                    || neighbour.0 >= grid[0].len() as i32
                    || neighbour.1 < 0
                    || neighbour.1 >= grid.len() as i32
                {
                    continue;
                }

                let new_dist = curr.dist + cost_between(&grid, curr.node, neighbour);
                let neighbour_node = NodeDist {
                    node: neighbour,
                    last_move: (
                        // TODO: make better
                        // Trying to get either (0,1) or (1,0)
                        (neighbour.0 - curr.node.0).signum().abs(),
                        (neighbour.1 - curr.node.1).signum().abs(),
                    ),
                    heuristic: heuristic(neighbour, end),
                    // prev: curr.node,
                    dist: new_dist,
                };

                if visited.contains(&neighbour_node) {
                    continue;
                }

                let mut replace = true;
                if let Some(d) = distances.get(&neighbour_node) {
                    if new_dist >= *d {
                        replace = false;
                    }
                }

                if replace {
                    distances.insert(neighbour_node.clone(), new_dist);
                    frontier.push(neighbour_node);
                }
            }
        }
    }

    panic!("Couldn't find end")
}

fn process(input: &str) -> u32 {
    let grid = parse_input(input);
    find_shortest_path(
        &grid,
        (0, 0),
        (grid[0].len() as i32 - 1, grid.len() as i32 - 1),
    )
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part1.txt");
        let result = process(input);
        assert_eq!(result, 102);
    }
}
