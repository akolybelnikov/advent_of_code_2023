use advent_of_code_2023::read_lines;
use std::collections::{HashMap, HashSet};

fn main() {
    let time_start = std::time::Instant::now();
    let count = part_1("src/bin/day22/input.txt");
    println!(
        "Part 1: {:?}  Time: {}μs",
        count,
        time_start.elapsed().as_micros()
    );
}

fn part_1(filename: &str) -> usize {
    let input = read_lines(filename).unwrap();
    let input_len = input.len();
    let mut stack = Stack::new();
    stack.settle_bricks(input);
    assert_eq!(stack.bricks.len(), input_len);
    stack.disintegrateable()
}

#[derive(Clone, Debug, PartialEq)]
struct Coordinates(usize, usize, usize);

impl Coordinates {
    fn new(input: &str) -> Coordinates {
        let mut parts = input.split(",");
        let x = parts.next().unwrap().parse::<usize>().unwrap();
        let y = parts.next().unwrap().parse::<usize>().unwrap();
        let z = parts.next().unwrap().parse::<usize>().unwrap();
        Coordinates(x, y, z)
    }
}

#[derive(Clone, Debug)]
struct Brick {
    id: usize,
    ends: (Coordinates, Coordinates),
    supports: HashSet<usize>,
    supported_by: HashSet<usize>,
}

impl Brick {
    fn new(input: &str) -> Brick {
        let mut parts = input.split("~");
        let start = Coordinates::new(parts.next().unwrap());
        let end = Coordinates::new(parts.next().unwrap());
        // make sure bricks ends are sorted by x,y coordinates
        let (start, end) = if start.0 > end.0 || start.1 > end.1 {
            (end, start)
        } else {
            (start, end)
        };
        Brick {
            id: 0,
            ends: (start, end),
            supports: HashSet::new(),
            supported_by: HashSet::new(),
        }
    }

    fn is_vertical(&self) -> bool {
        self.ends.0 .2 != self.ends.1 .2
    }
}

fn make_bricks(input: Vec<String>) -> Vec<Brick> {
    let mut bricks = Vec::new();
    input.iter().enumerate().for_each(|(i, line)| {
        let mut brick = Brick::new(line);
        brick.id = i + 1;
        bricks.push(brick);
    });
    // Sort bricks by z coordinate
    bricks.sort_by(|a, b| a.ends.0 .2.cmp(&b.ends.0 .2));
    bricks
}

struct Stack {
    bricks: HashMap<usize, Brick>,
    levels: HashMap<usize, Vec<Vec<usize>>>,
    max_x: usize,
    max_y: usize,
}

impl Stack {
    fn new() -> Stack {
        Stack {
            bricks: HashMap::new(),
            levels: HashMap::new(),
            max_x: 0,
            max_y: 0,
        }
    }

    fn settle_bricks(&mut self, input: Vec<String>) {
        let bricks = make_bricks(input);
        let (max_x, max_y) = bricks.iter().fold((0, 0), |acc, brick| {
            (acc.0.max(brick.ends.0 .0), acc.1.max(brick.ends.0 .1))
        });
        self.max_x = max_x;
        self.max_y = max_y;
        for mut brick in bricks {
            let z = brick.ends.0 .2;
            if z == 1 {
                self.update_levels(&brick, z);
                self.bricks.insert(brick.id, brick);
                continue;
            }
            let (start, end) = &brick.ends;
            let mut next_under = z - 1;
            while next_under > 0 {
                let level_exists = if let Some(_) = self.levels.get(&next_under) {
                    true
                } else {
                    false
                };
                if level_exists {
                    let level_map = self.levels.get(&next_under).unwrap().clone();
                    for x in start.0..=end.0 {
                        for y in start.1..=end.1 {
                            if level_map[x][y] != 0 {
                                brick.supported_by.insert(level_map[x][y]);
                                let s = self.bricks.get_mut(&level_map[x][y]).unwrap();
                                s.supports.insert(brick.id);
                            }
                        }
                    }
                    if brick.supported_by.len() > 0 {
                        break;
                    }
                }
                next_under -= 1;
            }
            self.update_levels(&brick, next_under + 1);
            self.bricks.insert(brick.id, brick.clone());
        }
    }

    fn update_levels(&mut self, brick: &Brick, z: usize) {
        let (start, end) = &brick.ends;
        if brick.is_vertical() {
            for i in z..=z + end.2 - start.2 {
                let level_map = self
                    .levels
                    .entry(i)
                    .or_insert(vec![vec![0; self.max_y + 1]; self.max_x + 1]);
                for x in start.0..=end.0 {
                    for y in start.1..=end.1 {
                        level_map[x][y] = brick.id;
                    }
                }
            }
        } else {
            let level_map = self
                .levels
                .entry(z)
                .or_insert(vec![vec![0; self.max_y + 1]; self.max_x + 1]);
            for x in start.0..=end.0 {
                for y in start.1..=end.1 {
                    level_map[x][y] = brick.id;
                }
            }
        }
    }

    fn disintegrateable(&self) -> usize {
        let mut count = 0;
        for brick in self.bricks.values() {
            let valid = brick
                .supports
                .iter()
                .all(|id| self.bricks[id].supported_by.len() > 1)
                || brick.supports.len() == 0;
            if valid {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bricks() {
        let input = read_lines("src/bin/day22/test_input.txt").unwrap();
        let bricks = make_bricks(input);
        assert_eq!(bricks.len(), 7);
        assert_eq!(bricks[0].ends.0, Coordinates(1, 0, 1));
        assert_eq!(bricks[0].ends.1, Coordinates(1, 2, 1));
        assert_eq!(bricks[6].ends.0, Coordinates(1, 1, 8));
        assert_eq!(bricks[6].ends.1, Coordinates(1, 1, 9));
        assert!(bricks[6].is_vertical());
    }

    #[test]
    fn test_stack() {
        let input = read_lines("src/bin/day22/test_input.txt").unwrap();
        let mut stack = Stack::new();
        stack.settle_bricks(input);
        assert_eq!(stack.levels.len(), 6);
        assert_eq!(stack.levels[&1][1][0], 1);
        assert_eq!(stack.levels[&1][1][1], 1);
        assert_eq!(stack.levels[&1][1][2], 1);

        assert_eq!(stack.levels[&2][0][0], 2);
        assert_eq!(stack.levels[&2][1][0], 2);
        assert_eq!(stack.levels[&2][2][0], 2);

        assert_eq!(stack.levels[&2][0][2], 3);
        assert_eq!(stack.levels[&2][1][2], 3);
        assert_eq!(stack.levels[&2][2][2], 3);

        assert_eq!(stack.levels[&3][0][0], 4);
        assert_eq!(stack.levels[&3][0][1], 4);
        assert_eq!(stack.levels[&3][0][2], 4);

        assert_eq!(stack.levels[&3][2][0], 5);
        assert_eq!(stack.levels[&3][2][1], 5);
        assert_eq!(stack.levels[&3][2][2], 5);

        assert_eq!(stack.levels[&4][0][1], 6);
        assert_eq!(stack.levels[&4][1][1], 6);
        assert_eq!(stack.levels[&4][2][1], 6);

        assert_eq!(stack.levels[&5][1][1], 7);
        assert_eq!(stack.levels[&6][1][1], 7);

        assert_eq!(stack.bricks[&1].supported_by.len(), 0);
        assert!(stack.bricks[&1].supports.contains(&2));
        assert!(stack.bricks[&1].supports.contains(&3));
        assert!(stack.bricks[&2].supported_by.contains(&1));
        assert!(stack.bricks[&2].supports.contains(&4));
        assert!(stack.bricks[&2].supports.contains(&5));
        assert!(stack.bricks[&3].supported_by.contains(&1));
        assert!(stack.bricks[&3].supports.contains(&4));
        assert!(stack.bricks[&3].supports.contains(&5));
        assert!(stack.bricks[&4].supported_by.contains(&2));
        assert!(stack.bricks[&4].supported_by.contains(&3));
        assert!(stack.bricks[&4].supports.contains(&6));
        assert!(stack.bricks[&5].supported_by.contains(&2));
        assert!(stack.bricks[&5].supported_by.contains(&3));
        assert!(stack.bricks[&5].supports.contains(&6));
        assert!(stack.bricks[&6].supported_by.contains(&4));
        assert!(stack.bricks[&6].supported_by.contains(&5));
        assert!(stack.bricks[&6].supports.contains(&7));
        assert!(stack.bricks[&7].supported_by.contains(&6));
        assert_eq!(stack.bricks[&7].supports.len(), 0);

        assert_eq!(stack.disintegrateable(), 5);
    }
}
