// --- Day 17: Clumsy Crucible ---

use advent_of_code_2023::read_lines;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    let time_start = std::time::Instant::now();
    let sum = part_1("src/bin/day17/input.txt");
    println!("Part 1: {:?}", sum);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn part_1(filename: &str) -> usize {
    let input = read_lines(filename).unwrap();
    let mut graph = Graph::new(input);
    let path = graph.walk(0, 0);
    path.get(&(graph.height - 1, graph.width - 1)).unwrap().1
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vertex {
    heat: usize,
    visited: [usize; 4],
    x: usize,
    y: usize,
}

struct Visit {
    v: Vertex,
    direction: usize,
    heat_loss: usize,
}

impl Ord for Visit {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd<Self> for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<Self> for Visit {
    fn eq(&self, other: &Self) -> bool {
        self.heat_loss.eq(&other.heat_loss)
    }
}

impl Eq for Visit {}

struct Graph {
    height: usize,
    nodes: Vec<Vec<usize>>,
    width: usize,
}

impl Graph {
    fn new(lines: Vec<String>) -> Self {
        let width = lines[0].len();
        let height = lines.len();
        let mut nodes = vec![vec![0; width]; height];
        for x in 0..height {
            for y in 0..width {
                let heat = lines[x].as_bytes()[y] - b'0';
                nodes[x][y] = heat as usize;
            }
        }
        Graph {
            height,
            nodes,
            width,
        }
    }

    fn walk(&mut self, x: usize, y: usize) -> HashMap<(usize, usize), ((usize, usize), usize)> {
        let mut cur_direction = 2;
        let mut steps: i8 = 0;
        let mut path = HashMap::new();
        let mut visited = HashMap::new();
        let start = Vertex {
            heat: self.nodes[x][y],
            visited: [0; 4],
            x,
            y,
        };
        path.insert((x, y), ((x, y), 0));
        let mut to_visit = BinaryHeap::new();
        to_visit.push(Visit {
            v: start,
            direction: cur_direction,
            heat_loss: 0,
        });

        while let Some(Visit {
                           v,
                           direction,
                           heat_loss,
                       }) = to_visit.pop()
        {
            if v.x == self.height - 1 && v.y == self.width - 1 {
                break;
            }
            let visited_vertex = visited.entry((v.x, v.y)).or_insert(v);
            let from = (direction + 2) % 4;
            if visited_vertex.visited[from] == 1 {
                continue;
            } else {
                visited_vertex.visited[from] = 1;
            }
            let neighbours = self.find_neighbours(v.x, v.y, direction, steps);
            for (xn, yn, dirn) in neighbours {
                let new_heat_loss = heat_loss + self.nodes[xn][yn];
                let is_less = path.get(&(xn, yn)).map_or(true, |&p| p.1 > new_heat_loss);
                if is_less {
                    path.entry((xn, yn))
                        .and_modify(|p| {
                            p.0 = (v.x, v.y);
                            p.1 = new_heat_loss;
                        })
                        .or_insert(((v.x, v.y), new_heat_loss));
                    to_visit.push(Visit {
                        v: Vertex {
                            heat: self.nodes[xn][yn],
                            visited: [0; 4],
                            x: xn,
                            y: yn,
                        },
                        direction: dirn,
                        heat_loss: new_heat_loss,
                    });
                }
            }
            if cur_direction != direction {
                steps = 1;
                cur_direction = direction;
            } else {
                steps += 1;
            }
        }

        path
    }

    fn find_neighbours(
        &self,
        x: usize,
        y: usize,
        direction: usize,
        steps: i8,
    ) -> Vec<(usize, usize, usize)> {
        let mut neighbours = vec![];
        let (left, straight, right) = directions(direction);
        if let Some((x, y)) = self.next(x, y, left) {
            neighbours.push((x, y, left));
        }
        if let Some((x, y)) = self.next(x, y, right) {
            neighbours.push((x, y, right));
        }
        if steps < 3 {
            if let Some((x, y)) = self.next(x, y, straight) {
                neighbours.push((x, y, straight));
            }
        }
        neighbours
    }

    fn next(&self, x: usize, y: usize, dir: usize) -> Option<(usize, usize)> {
        match dir {
            0 => {
                if y == 0 {
                    None
                } else {
                    Some((x, y - 1))
                }
            }
            1 => {
                if x == 0 {
                    None
                } else {
                    Some((x - 1, y))
                }
            }
            2 => {
                if y == self.width - 1 {
                    None
                } else {
                    Some((x, y + 1))
                }
            }
            3 => {
                if x == self.height - 1 {
                    None
                } else {
                    Some((x + 1, y))
                }
            }
            _ => None,
        }
    }
}

fn print_path(path: &HashMap<(usize, usize), ((usize, usize), usize)>, height: usize, width: usize) {
    let mut current_node = (height - 1, width - 1);
    let mut path_vec = Vec::new();

    while let Some(parent_node) = path.get(&current_node){
        path_vec.push((current_node, parent_node.1));
        current_node = parent_node.0;
        if current_node == (0,0){
            path_vec.push((current_node, 0));
            break;
        }
    }

    path_vec.reverse();

    for node in &path_vec{
        println!("{:?}",node);
    }
}

fn directions(current: usize) -> (usize, usize, usize) {
    let left = (current + 3) % 4;
    let straight = current;
    let right = (current + 1) % 4;

    (left, straight, right)
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
        assert_eq!(graph.nodes[1][0], 2);
        assert_eq!(graph.nodes[2][0], 3);
        assert_eq!(graph.nodes[0][1], 4);
        assert_eq!(graph.nodes[1][1], 5);
        assert_eq!(graph.nodes[2][1], 6);
        assert_eq!(graph.nodes[0][2], 7);
        assert_eq!(graph.nodes[1][2], 8);
        assert_eq!(graph.nodes[2][2], 9);
    }

    #[test]
    fn test_directions() {
        assert_eq!(directions(0), (3, 2, 1));
        assert_eq!(directions(1), (0, 3, 2));
        assert_eq!(directions(2), (1, 0, 3));
        assert_eq!(directions(3), (2, 1, 0));
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
        let path = graph.walk(0, 0);
        let result = path.get(&(graph.height - 1, graph.width - 1)).unwrap().1;
        assert_eq!(result, 28);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/bin/day17/test_input.txt"), 94);
    }

    #[test]
    fn test_print_path() {
        let lines = read_lines("src/bin/day17/test_input.txt").unwrap();
        let mut graph = Graph::new(lines);
        let path = graph.walk(0, 0);
        print_path(&path, graph.height, graph.width);
    }
}
