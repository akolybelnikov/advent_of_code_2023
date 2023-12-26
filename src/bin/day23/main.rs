// --- Day 23: A Long Walk ---

use std::collections::{HashMap, HashSet};

fn main() {
    let time_start = std::time::Instant::now();
    let count = part_1("src/bin/day23/input.txt");
    println!(
        "Part 1: {:?}  Time: {}μs",
        count,
        time_start.elapsed().as_micros()
    );

    let time_start = std::time::Instant::now();
    let count = part_2("src/bin/day23/input.txt");
    println!(
        "Part 2: {:?}  Time: {}μs",
        count,
        time_start.elapsed().as_micros()
    );
}

fn part_1(filename: &str) -> u16 {
    let input = advent_of_code_2023::read_lines(filename).unwrap();
    let mut map = Map::new(input);
    map.find_nodes();
    map.traverse_graph(true);
    let mut seen = HashSet::new();
    map.dfs(map.start, &mut seen).unwrap()
}

fn part_2(filename: &str) -> u16 {
    let input = advent_of_code_2023::read_lines(filename).unwrap();
    let mut map = Map::new(input);
    map.find_nodes();
    map.traverse_graph(false);
    let mut seen = HashSet::new();
    map.dfs(map.start, &mut seen).unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(i16, i16);

impl Pos {
    fn neighbours(&self) -> Vec<Pos> {
        let mut neighbours = Vec::new();
        for dx in -1i16..=1 {
            for dy in -1i16..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if (dx + dy).abs() != 1 {
                    continue;
                }
                let new_position = Pos(self.0 + dx, self.1 + dy);
                neighbours.push(new_position);
            }
        }
        neighbours
    }
}

struct Map {
    data: Vec<Vec<Option<u8>>>,
    end: Pos,
    height: u8,
    nodes: Vec<Pos>,
    start: Pos,
    width: u8,
    graph: HashMap<Pos, HashMap<Pos, u16>>,
}

impl Map {
    fn new(input: Vec<String>) -> Self {
        let height = input.len() as u8;
        let width = input[0].len() as u8;
        let start = Pos(0, 1);
        let end = Pos(height as i16 - 1, width as i16 - 2);
        let mut data = vec![vec![None; width as usize]; height as usize];
        for (y, line) in input.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                data[y][x] = match c {
                    '#' => None,
                    _ => Some(c as u8),
                };
            }
        }
        let mut nodes = Vec::new();
        nodes.push(start);
        Self {
            data,
            end,
            graph: HashMap::new(),
            height,
            nodes,
            start,
            width,
        }
    }

    fn find_nodes(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                if let Some(_) = self.data[x as usize][y as usize] {
                    let mut neighbours = 0;
                    for dx in -1i16..=1 {
                        for dy in -1i16..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }
                            if (dx + dy).abs() != 1 {
                                continue;
                            }
                            let new_position = Pos(x as i16 + dx, y as i16 + dy);
                            if new_position.0 < 0
                                || new_position.0 >= self.width.into()
                                || new_position.1 < 0
                                || new_position.1 >= self.height.into()
                            {
                                continue;
                            }
                            if let Some(_) =
                                self.data[new_position.0 as usize][new_position.1 as usize]
                            {
                                neighbours += 1;
                            }
                        }
                    }
                    if neighbours > 2 {
                        self.nodes.push(Pos(x as i16, y as i16));
                    }
                }
            }
        }
        let end = self.end;
        self.nodes.push(end);
    }

    fn traverse_graph(&mut self, slopes: bool) {
        let width = self.width;
        let height = self.height;
        for node in self.nodes.iter() {
            let mut stack = vec![(0, node.0, node.1)];
            let mut seen = HashSet::new();
            seen.insert(*node);
            while let Some((steps, x, y)) = stack.pop() {
                if steps != 0 && self.nodes.contains(&Pos(x, y)) {
                    self.graph
                        .entry(*node)
                        .or_insert(HashMap::new())
                        .insert(Pos(x, y), steps);
                    continue;
                }
                let self_value = self.data[x as usize][y as usize];
                let mut new_positions = Vec::new();
                if slopes {
                    match self_value {
                        Some(b'v') => new_positions.push(Pos(x + 1, y)),
                        Some(b'>') => new_positions.push(Pos(x, y + 1)),
                        Some(b'^') => new_positions.push(Pos(x - 1, y)),
                        Some(b'<') => new_positions.push(Pos(x, y - 1)),
                        _ => new_positions = Pos(x, y).neighbours(),
                    }
                } else {
                    new_positions = Pos(x, y).neighbours();
                }
                for pos in new_positions {
                    if 0 <= pos.0
                        && pos.0 < width.into()
                        && 0 <= pos.1
                        && pos.1 < height.into()
                        && !seen.contains(&pos)
                    {
                        if let Some(_) = self.data[pos.0 as usize][pos.1 as usize] {
                            stack.push((steps + 1, pos.0, pos.1));
                            seen.insert(pos);
                        }
                    }
                }
            }
        }
    }

    fn dfs(&self, pos: Pos, seen: &mut HashSet<Pos>) -> Option<u16> {
        let end = self.end;
        if pos == end {
            return Some(0);
        }
        let mut max_steps = None;
        seen.insert(pos);
        for next in self.graph.get(&pos).unwrap().keys() {
            if !seen.contains(next) {
                if let Some(steps) = self.dfs(*next, seen) {
                    if let Some(key) = self.graph.get(&pos) {
                        if let Some(value) = key.get(next) {
                            let steps = steps + value;
                            if max_steps.is_none() || steps > max_steps.unwrap() {
                                max_steps = Some(steps);
                            }
                        }
                    }
                }
            }
        }
        seen.remove(&pos);
        max_steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2023::read_lines;

    #[test]
    fn test_successors() {
        let lines = read_lines("src/bin/day23/test_input.txt").unwrap();
        let width = lines[0].len() as u8;
        let height = lines.len() as u8;
        let map = Map::new(lines);
        assert_eq!(map.width, width);
        assert_eq!(map.height, height);
        assert_eq!(
            map.data[map.start.0 as usize][map.start.1 as usize],
            Some(b'.')
        );
        assert_eq!(map.data[map.end.0 as usize][map.end.1 as usize], Some(b'.'));
    }

    #[test]
    fn test_nodes() {
        let lines = read_lines("src/bin/day23/test_input.txt").unwrap();
        let mut map = Map::new(lines);
        map.find_nodes();
        assert_eq!(map.nodes.len(), 9);
    }

    #[test]
    fn test_graph() {
        let lines = read_lines("src/bin/day23/test_input.txt").unwrap();
        let mut map = Map::new(lines);
        map.find_nodes();
        map.traverse_graph(false);
        assert_eq!(map.graph.len(), 9);
    }

    #[test]
    fn test_dfs_1() {
        let lines = read_lines("src/bin/day23/test_input.txt").unwrap();
        let mut map = Map::new(lines);
        map.find_nodes();
        map.traverse_graph(true);
        let mut seen = HashSet::new();
        let steps = map.dfs(map.start, &mut seen).unwrap();
        assert_eq!(steps, 94);
    }

    #[test]
    fn test_dfs_2() {
        let lines = read_lines("src/bin/day23/test_input.txt").unwrap();
        let mut map = Map::new(lines);
        map.find_nodes();
        map.traverse_graph(false);
        let mut seen = HashSet::new();
        let steps = map.dfs(map.start, &mut seen).unwrap();
        assert_eq!(steps, 154);
    }
}
