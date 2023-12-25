use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use priority_queue::PriorityQueue;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

type Graph = HashMap<String, Vec<String>>;

fn get_graph(input: &str) -> Graph {
    let mut graph = HashMap::new();

    for line in input.trim().split('\n') {
        let (node, neighbours) = line.split_once(": ").unwrap();
        let neighbours: Vec<_> = neighbours
            .split_whitespace()
            .map(|n| n.to_string())
            .collect();

        for neighbour in &neighbours {
            let graph_neighbour: &mut Vec<String> = graph.entry(neighbour.to_owned()).or_default();
            graph_neighbour.push(node.to_string());
        }

        let graph_node_neighbours: &mut Vec<String> = graph.entry(node.to_string()).or_default();
        graph_node_neighbours.extend(neighbours);
    }

    graph
}

fn get_connected_nodes(graph: &Graph, node: &str) -> HashSet<String> {
    let mut explored = HashSet::new();
    let intitial = node.to_string();
    let mut frontier = vec![&intitial];

    while let Some(curr) = frontier.pop() {
        if explored.contains(curr) {
            continue;
        }

        explored.insert(curr.to_string());
        frontier.extend(graph.get(curr).unwrap());
    }

    explored
}

fn disconnect_wire(graph: &Graph, a: &str, b: &str) -> Graph {
    let mut new_graph = graph.to_owned();

    let a_node = new_graph.get_mut(a).unwrap();
    a_node.retain(|n| n != b);

    if a_node.len() == 0 {
        new_graph.remove(a);
    }

    let b_node = new_graph.get_mut(b).unwrap();
    b_node.retain(|n| n != a);

    if b_node.len() == 0 {
        new_graph.remove(b);
    }

    new_graph
}

fn dijkstra(graph: &Graph, start: &str) -> HashMap<String, Vec<String>> {
    let mut distance: PriorityQueue<&str, Reverse<i32>> = PriorityQueue::new();
    let mut explored = HashSet::new();
    let mut prev = HashMap::new();

    distance.push(start, Reverse(0));

    while let Some((curr, dist)) = distance.pop() {
        if explored.contains(curr) {
            continue;
        }
        explored.insert(curr);

        for neighbour in graph.get(curr).unwrap() {
            if explored.contains(neighbour.as_str()) {
                continue;
            }

            let alt = dist.0 + 1;
            let (_, existing_dist) = distance
                .get(neighbour.as_str())
                .unwrap_or((&"", &Reverse(i32::MAX)));

            if &alt < &existing_dist.0 {
                distance.push(neighbour.as_str(), Reverse(alt));
                prev.insert(neighbour, curr);
            }
        }
    }

    let mut paths = HashMap::new();

    for node in graph.keys() {
        if node == start {
            continue;
        }

        let mut path = Vec::new();
        let mut curr = node.to_string();

        while let Some(next) = prev.get(&curr) {
            path.push(curr);
            curr = next.to_string();

            if curr == start {
                path.push(curr);
                break;
            }
        }

        path.reverse();
        paths.insert(node.to_string(), path);
    }

    paths
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct Link {
    lower: String,
    higher: String,
}

impl Link {
    fn new(a: &str, b: &str) -> Self {
        let lower = std::cmp::min(a, b).to_string();
        let higher = std::cmp::max(a, b).to_string();

        Link { lower, higher }
    }
}

fn find_edge_importances(graph: &Graph) -> PriorityQueue<Link, i32> {
    let mut importance = PriorityQueue::new();

    for start in graph.keys() {
        let paths = dijkstra(&graph, start);

        for path in paths.values() {
            for pair in path.windows(2) {
                let link = Link::new(&pair[0], &pair[1]);
                let imp = importance.get(&link).map(|n| n.1).unwrap_or(&0);

                importance.push_increase(link, imp + 1);
            }
        }
    }

    importance
}

// NOTE: this assumes the most "important" wires (ones with the most paths groing through them) are
// the ones that need to be disconnected. I'm not sure if this is the case
fn disconnect_wires(mut graph: Graph, n: i32) -> Graph {
    let mut importances = find_edge_importances(&graph);

    for _ in 0..n {
        let (link, _) = importances.pop().unwrap();
        graph = disconnect_wire(&graph, &link.lower, &link.higher);
    }

    graph
}

fn get_sections(disconnected_graph: &Graph) -> (HashSet<String>, HashSet<String>) {
    let random = disconnected_graph.keys().next().unwrap();
    let half1 = get_connected_nodes(&disconnected_graph, random);

    // Find node not in the first half
    let mut other_node = None;
    for node in disconnected_graph.keys() {
        if !half1.contains(node) {
            other_node = Some(node);
        }
    }

    let half2 = get_connected_nodes(
        &disconnected_graph,
        other_node.expect("Couldn't find other half"),
    );

    (half1, half2)
}

fn process(input: &str) -> u32 {
    let graph = get_graph(input);
    let disconnected_graph = disconnect_wires(graph, 3);
    let (half1, half2) = get_sections(&disconnected_graph);

    (half1.len() * half2.len()) as u32
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part1.txt");
        let result = process(input);
        assert_eq!(result, 54);
    }
}
