use rand::prelude::IteratorRandom;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello from day25!");
}

#[derive(Debug, Clone)]
struct Node {
    edges: HashSet<String>,
}

impl Node {
    fn intersection(&self, other: &Self) -> HashSet<String> {
        self.edges.intersection(&other.edges).cloned().collect()
    }
}

struct Graph {
    nodes: HashMap<String, Node>,
}

impl Graph {
    // Contract the graph
    fn contract(&mut self) -> Option<Self> {
        let mut graph = Graph {
            nodes: self.nodes.clone(),
        };
        while graph.nodes.len() > 2 {
            let keys: Vec<&String> = graph.nodes.keys().collect();
            let node_key = *keys.choose(&mut thread_rng()).unwrap();
            let node = graph.nodes.get(node_key).unwrap();

            // Randomly choose an edge
            if let Some(another) = node.edges.iter().choose(&mut thread_rng()) {
                // Merging the two nodes
                graph.merge_nodes(node_key.clone(), another.clone());
            }
        }

        let intersection = graph
            .nodes
            .values()
            .next()
            .unwrap()
            .intersection(graph.nodes.values().last().unwrap());
        if intersection.len() > 0 {
            graph.nodes.iter_mut().for_each(|(_, node)| {
                node.edges.clear();
            });
            for edge in intersection {
                graph.nodes.iter_mut().for_each(|(_, node)| {
                    node.edges.insert(edge.clone());
                });
                let mut new_node = Node {
                    edges: HashSet::new(),
                };
                for key in graph.nodes.keys() {
                    if *key != edge {
                        new_node.edges.insert(key.clone());
                    }
                }
                graph.nodes.insert(edge.clone(), new_node);
            }
            return Some(graph);
        }
        None
    }

    // Merge two nodes into one
    fn merge_nodes(&mut self, name1: String, name2: String) {
        let node2_edges = {
            self.nodes
                .get(&name2)
                .map(|node| node.edges.clone())
                .unwrap_or_else(HashSet::new)
        };

        for edge in &node2_edges {
            if let Some(edge_node) = self.nodes.get_mut(edge) {
                edge_node.edges.remove(&name2);
                edge_node.edges.insert(name1.clone());
            }
        }

        if let Some(node1) = self.nodes.get_mut(&name1) {
            node1.edges.extend(node2_edges);
            node1.edges.remove(&name1);
            node1.edges.remove(&name2);
        }

        self.nodes.remove(&name2);
    }

    fn print_graph(&self) {
        for (key, node) in &self.nodes {
            println!("{}: {:?}", key, node.edges);
        }
    }
}

fn make_nodes(input: Vec<String>) -> HashMap<String, Node> {
    let mut nodes = HashMap::new();

    for line in input {
        let mut split_line = line.split(": ");
        let name = split_line.next().unwrap();
        let edges = split_line.next().unwrap().split(" ");

        let mut node = nodes
            .entry(name.to_string())
            .or_insert(Node {
                edges: HashSet::new(),
            })
            .clone();

        for edge in edges {
            node.edges.insert(edge.to_string());
            if let Some(edge_node) = nodes.get_mut(edge) {
                edge_node.edges.insert(name.to_string());
            } else {
                nodes.insert(
                    edge.to_string(),
                    Node {
                        edges: [name.to_string()].iter().cloned().collect(),
                    },
                );
            }
        }
    }
    nodes
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2023::read_lines;

    #[test]
    fn test_make_nodes() {
        let input = read_lines("src/bin/day25/test_input.txt").unwrap();

        let nodes = make_nodes(input);
        assert_eq!(nodes.len(), 15);
        let hfx = nodes.get("hfx").unwrap();
        assert_eq!(hfx.edges.len(), 5);
        assert!(hfx.edges.contains("xhk"));
    }

    #[test]
    fn test_merge_nodes() {
        let input = read_lines("src/bin/day25/test_input.txt").unwrap();
        let nodes = make_nodes(input);
        let mut graph = Graph { nodes };
        graph.merge_nodes("hfx".to_string(), "xhk".to_string());
        assert_eq!(graph.nodes.len(), 14);
        let hfx = graph.nodes.get("hfx").unwrap();
        assert_eq!(hfx.edges.len(), 5);
        assert!(graph.nodes.get("xhk").is_none());
    }

    #[test]
    fn test_contract() {
        let input = read_lines("src/bin/day25/test_input.txt").unwrap();

        let nodes = make_nodes(input);
        let mut graph = Graph { nodes };

        if let Some(new_graph) = graph.contract() {
            new_graph.print_graph();
        }
    }
}
