use rustworkx_core::{
    connectivity::stoer_wagner_min_cut,
    petgraph::graph::{NodeIndex, UnGraph},
    Result,
};
use std::collections::HashMap;

fn main() {
    let time_start = std::time::Instant::now();
    println!(
        "Min Cut: {:?}  Time: {}μs",
        find_mul("src/bin/day25/input.txt"),
        time_start.elapsed().as_micros()
    );
}

fn find_mul(filename: &str) -> usize {
    let input = advent_of_code_2023::read_lines(filename).unwrap();
    let graph = make_graph(input);
    let (min_cut, partition) = min_cut(&graph);
    assert_eq!(min_cut, 3);
    partition.len() * (graph.node_count() - partition.len())
}

fn min_cut(graph: &UnGraph<(), ()>) -> (usize, Vec<NodeIndex>) {
    let min_cut_res: Result<Option<(usize, Vec<NodeIndex>)>> = stoer_wagner_min_cut(&graph, |_| Ok(1));
    min_cut_res.unwrap().unwrap()
}

fn make_graph(input: Vec<String>) -> UnGraph<(), ()> {
    let mut graph = UnGraph::<(), ()>::new_undirected();
    let mut nodes: HashMap<String, NodeIndex> = HashMap::new();
    for line in input {
        let parts: Vec<&str> = line.split(":").collect();
        let from = parts[0].to_string();
        if let Some(adjacent) = parts.get(1) {
            let node = *nodes.entry(from).or_insert_with(|| graph.add_node(()));
            for to in adjacent.split_whitespace() {
                let to_node = *nodes
                    .entry(to.to_string())
                    .or_insert_with(|| graph.add_node(()));
                graph.add_edge(node, to_node, ());
            }
        }
    }
    graph
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2023::read_lines;

    #[test]
    fn test_make_graph() {
        let input = read_lines("src/bin/day25/test_input.txt").unwrap();
        let graph = make_graph(input);
        assert_eq!(graph.node_count(), 15);
        assert_eq!(graph.edge_count(), 33);
    }

    #[test]
    fn test_min_cut() {
        let input = read_lines("src/bin/day25/test_input.txt").unwrap();
        let graph = make_graph(input);
        let (min_cut, partition) = min_cut(&graph);
        assert_eq!(min_cut, 3);
        assert_eq!(partition.len(), 9);
    }

    #[test]
    fn test_find_mul() {
        assert_eq!(find_mul("src/bin/day25/test_input.txt"), 54);
    }
}
