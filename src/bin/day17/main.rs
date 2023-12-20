// --- Day 17: Clumsy Crucible ---

use advent_of_code_2023::read_lines;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

fn main() {
    let time_start = std::time::Instant::now();
    let sum = part_1("src/bin/day17/input.txt");
    println!("Part 1: {:?}, Time: {}μs", sum, time_start.elapsed().as_micros());
}

fn part_1(filename: &str) -> i32 {
    let input = read_lines(filename).unwrap();
    let mut graph = Graph::new(input);
    graph.walk(0, 0)
}

struct State(i32, i32, i32, i32, i32, usize);

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<Self> for State {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for State {}

struct Graph {
    height: usize,
    nodes: Vec<Vec<i32>>,
    width: usize,
}

impl Graph {
    fn new(lines: Vec<String>) -> Self {
        let width = lines[0].len();
        let height = lines.len();
        let mut nodes: Vec<Vec<i32>> = vec![vec![0; width]; height];
        for x in 0..height {
            for y in 0..width {
                let heat = lines[x].as_bytes()[y] - b'0';
                nodes[x][y] = heat as i32;
            }
        }
        Graph {
            height,
            nodes,
            width,
        }
    }

    fn walk(&mut self, x: i32, y: i32) -> i32 {
        let mut heat_loss = 0;
        let mut visited = HashSet::new();
        let mut pq = BinaryHeap::new();
        pq.push(State(0, x, y, 0, 0, 0));

        while let Some(State(hl, r, c, dr, dc, n)) = pq.pop() {
            if r == (self.height - 1) as i32 && c == (self.width - 1) as i32 {
                heat_loss = hl;
                break;
            }

            if visited.contains(&(r, c, dr, dc, n)) {
                continue;
            }

            visited.insert((r, c, dr, dc, n));

            if n < 3 && (dr, dc) != (0, 0) {
                let nr = r + dr;
                let nc = c + dc;
                if 0 <= nr && nr < self.height as i32 && 0 <= nc && nc < self.width as i32 {
                    pq.push(State(hl + self.nodes[nr as usize][nc as usize], nr, nc, dr, dc, n + 1));
                }
            }

            for (ndr, ndc) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                if (*ndr, *ndc) != (-dr, -dc) && (*ndr, *ndc) != (dr, dc) {
                    let nr = r + ndr;
                    let nc = c + ndc;
                    if 0 <= nr && nr < self.height as i32 && 0 <= nc && nc < self.width as i32 {
                        pq.push(State(hl + self.nodes[nr as usize][nc as usize], nr, nc, *ndr, *ndc, 1));
                    }
                }
            }
        }
        heat_loss
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_graph() {
        let lines = vec![
            String::from("123"),
            String::from("456"),
            String::from("789"),
        ];
        let graph = Graph::new(lines);
        assert_eq!(graph.nodes[0][0], 1);
        assert_eq!(graph.nodes[1][0], 4);
        assert_eq!(graph.nodes[2][0], 7);
        assert_eq!(graph.nodes[0][1], 2);
        assert_eq!(graph.nodes[1][1], 5);
        assert_eq!(graph.nodes[2][1], 8);
        assert_eq!(graph.nodes[0][2], 3);
        assert_eq!(graph.nodes[1][2], 6);
        assert_eq!(graph.nodes[2][2], 9);
    }

    #[test]
    fn test_walk() {
        let lines = vec![
            String::from("24134"),
            String::from("32154"),
            String::from("32552"),
            String::from("34465"),
            String::from("45466"),
        ];
        let mut graph = Graph::new(lines);
        let heat_loss = graph.walk(0, 0);
        assert_eq!(heat_loss, 28);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/bin/day17/test_input.txt"), 102);
    }
}
