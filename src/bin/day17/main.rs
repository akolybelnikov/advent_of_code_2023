// --- Day 17: Clumsy Crucible ---

use advent_of_code_2023::read_lines;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

fn main() {
    let time_start = std::time::Instant::now();
    let sum = part_1("src/bin/day17/input.txt");
    println!(
        "Part 1: {:?}, Time: {}μs",
        sum,
        time_start.elapsed().as_micros()
    );

    let time_start = std::time::Instant::now();
    let sum = part_2("src/bin/day17/input.txt");
    println!(
        "Part 2: {:?}, Time: {}μs",
        sum,
        time_start.elapsed().as_micros()
    );
}

fn part_1(filename: &str) -> i32 {
    let input = read_lines(filename).unwrap();
    let mut graph = Graph::new(input);
    graph.walk(0, 0, 0, 3)
}

fn part_2(filename: &str) -> i32 {
    let input = read_lines(filename).unwrap();
    let mut graph = Graph::new(input);
    graph.walk(0, 0, 4, 10)
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
    height: i32,
    nodes: Vec<Vec<i32>>,
    width: i32,
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
            height: height as i32,
            nodes,
            width: width as i32,
        }
    }

    fn walk(&mut self, x: i32, y: i32, n_min: usize, n_max: usize) -> i32 {
        let mut heat_loss = 0;
        let mut visited = HashSet::new();
        let mut priority_queue = BinaryHeap::new();
        priority_queue.push(State(0, x, y, 0, 0, 0));

        while let Some(State(cur_heat_loss, cur_row, cur_col, dir_row, dir_col, steps_same_dir)) =
            priority_queue.pop()
        {
            if cur_row == (self.height - 1)
                && cur_col == (self.width - 1)
                && steps_same_dir >= n_min
            {
                heat_loss = cur_heat_loss;
                break;
            }

            if visited.contains(&(cur_row, cur_col, dir_row, dir_col, steps_same_dir)) {
                continue;
            }

            visited.insert((cur_row, cur_col, dir_row, dir_col, steps_same_dir));

            if steps_same_dir < n_max && (dir_row, dir_col) != (0, 0) {
                let new_row = cur_row + dir_row;
                let new_col = cur_col + dir_col;
                if 0 <= new_row && new_row < self.height && 0 <= new_col && new_col < self.width {
                    priority_queue.push(State(
                        cur_heat_loss + self.nodes[new_row as usize][new_col as usize],
                        new_row,
                        new_col,
                        dir_row,
                        dir_col,
                        steps_same_dir + 1,
                    ));
                }
            }

            if steps_same_dir >= n_min || (dir_row, dir_col) == (0, 0) {
                for (new_dir_row, new_dir_col) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    if (*new_dir_row, *new_dir_col) != (-dir_row, -dir_col) && (*new_dir_row, *new_dir_col) != (dir_row, dir_col) {
                        let new_row = cur_row + new_dir_row;
                        let new_col = cur_col + new_dir_col;
                        if 0 <= new_row && new_row < self.height && 0 <= new_col && new_col < self.width {
                            priority_queue.push(State(
                                cur_heat_loss + self.nodes[new_row as usize][new_col as usize],
                                new_row,
                                new_col,
                                *new_dir_row,
                                *new_dir_col,
                                1,
                            ));
                        }
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
        let heat_loss = graph.walk(0, 0, 0, 3);
        assert_eq!(heat_loss, 28);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/bin/day17/test_input.txt"), 102);
    }

    #[test]
    fn test_part_2() {
        let lines = read_lines("src/bin/day17/test_input.txt").unwrap();
        let mut graph = Graph::new(lines);
        let heat_loss = graph.walk(0, 0, 4, 10);
        assert_eq!(heat_loss, 94);
    }

    #[test]
    fn test_part_2_another_input() {
        let lines = vec![
            String::from("111111111111"),
            String::from("999999999991"),
            String::from("999999999991"),
            String::from("999999999991"),
            String::from("999999999991"),
        ];
        let mut graph = Graph::new(lines);
        let heat_loss = graph.walk(0, 0, 4, 10);
        assert_eq!(heat_loss, 71);
    }
}
