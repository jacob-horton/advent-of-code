use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    rc::Rc,
};

pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut grid = Vec::new();

    for line in input.trim().split('\n') {
        let row = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        grid.push(row);
    }

    grid
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    pos: (i32, i32),
    last_move: (i32, i32),
    heuristic: u32,
    cost_from_start: u32,
}

impl std::hash::Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Only want hash to depend on node and move
        self.pos.hash(state);
        self.last_move.hash(state);
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse comparison as binary heap will pop highest value
        (other.cost_from_start + other.heuristic).cmp(&(self.cost_from_start + self.heuristic))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// NOTE: only works if only x or only y changes, not both
fn cost_between(grid: &[Vec<u32>], start: (i32, i32), end: (i32, i32)) -> u32 {
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

// NOTE: this heuristic actually provides VERY minimal (only a few ms) speed improvements
fn heuristic(start: (i32, i32), end: (i32, i32)) -> u32 {
    // Manhattan distance
    end.0.abs_diff(start.0) + end.1.abs_diff(start.1)
}

pub fn find_shortest_path(
    grid: &Vec<Vec<u32>>,
    start: (i32, i32),
    end: (i32, i32),
    min_path: i32,
    max_path: i32,
) -> u32 {
    let path_lengths: Vec<_> = (-max_path..=max_path)
        .filter(|n| n.abs() >= min_path)
        .collect();

    // Nodes is hashmap from pos -> HashSet<Rc<Node>>
    // Each hashset is the possible paths to that point (i.e. horizontal vs vertical)
    let mut nodes = HashMap::new();
    let start_nodes = HashSet::from_iter([
        Rc::new(Node {
            pos: start,
            last_move: (0, 1),
            heuristic: heuristic(start, end),
            cost_from_start: 0,
        }),
        Rc::new(Node {
            pos: start,
            last_move: (1, 0),
            heuristic: heuristic(start, end),
            cost_from_start: 0,
        }),
    ]);

    nodes.insert(start, start_nodes);

    // Add start to frontier
    let mut frontier: BinaryHeap<Rc<Node>> = BinaryHeap::new();
    for ns in nodes.values() {
        for n in ns {
            frontier.push(n.clone());
        }
    }

    let mut visited = HashSet::new();
    while !frontier.is_empty() {
        let curr = frontier.pop().unwrap();

        if curr.pos == end {
            return curr.cost_from_start;
        }

        if visited.contains(&curr) {
            continue;
        }

        visited.insert(curr.clone());

        // Horizontal and vertical
        for dir in [(0, 1), (1, 0)] {
            // Skip parallel to direction came from
            if curr.last_move == dir {
                continue;
            }

            // Check different lengths in each direction parallel to dir
            // E.g. for (0, 1), search (0, -2), (0, -1), (0, 0), (0, 1), (0, 2)
            for i in &path_lengths {
                let neighbour = (curr.pos.0 + dir.0 * i, curr.pos.1 + dir.1 * i);
                if neighbour.0 < 0
                    || neighbour.0 >= grid[0].len() as i32
                    || neighbour.1 < 0
                    || neighbour.1 >= grid.len() as i32
                {
                    continue;
                }

                let new_cost = curr.cost_from_start + cost_between(grid, curr.pos, neighbour);
                let neighbour_node = Node {
                    pos: neighbour,
                    last_move: (
                        // Get either (0,1) or (1,0)
                        (neighbour.0 - curr.pos.0).signum().abs(),
                        (neighbour.1 - curr.pos.1).signum().abs(),
                    ),
                    heuristic: heuristic(neighbour, end),
                    cost_from_start: new_cost,
                };

                // Check if new cost to get to that node less than existing cost
                let mut replace = true;
                if let Some(ns) = nodes.get(&neighbour_node.pos) {
                    for n in ns {
                        if n.last_move == neighbour_node.last_move && new_cost >= n.cost_from_start
                        {
                            replace = false;
                        }
                    }
                }

                // If smaller cost or no cost yet, keep track of node and add to frontier
                if replace {
                    let rc = Rc::new(neighbour_node);
                    nodes
                        .entry(rc.pos)
                        .or_insert(HashSet::new())
                        .insert(rc.clone());

                    frontier.push(rc.clone());
                }
            }
        }
    }

    panic!("Couldn't find end")
}
