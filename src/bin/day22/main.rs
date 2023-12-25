// --- Day 22: Sand Slabs ---

use advent_of_code_2023::read_lines;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

fn main() {
    let time_start = std::time::Instant::now();
    let count = part_1("src/bin/day22/input.txt");
    println!(
        "Part 1: {:?}  Time: {}μs",
        count,
        time_start.elapsed().as_micros()
    );

    let time_start = std::time::Instant::now();
    let sum = part_2("src/bin/day22/input.txt");
    println!(
        "Part 2: {:?}  Time: {}μs",
        sum,
        time_start.elapsed().as_micros()
    );
}

fn part_1(filename: &str) -> usize {
    let input = read_lines(filename).unwrap();
    let input_len = input.len();
    let mut stack = Stack::new();
    stack.settle_bricks(input);
    assert_eq!(stack.bricks.len(), input_len);
    stack.count_disintegrateable()
}

fn part_2(filename: &str) -> usize {
    let input = read_lines(filename).unwrap();
    let mut stack = Stack::new();
    stack.settle_bricks(input);
    stack
        .bricks
        .iter()
        .map(|(id, _)| stack.chain_reaction(*id))
        .sum()
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
    input.iter().for_each(|line| {
        let brick = Brick::new(line);
        bricks.push(brick);
    });
    // Sort bricks by z coordinate
    bricks.sort_by(|a, b| a.ends.0 .2.cmp(&b.ends.0 .2));
    bricks.iter_mut().enumerate().for_each(|(i, brick)| {
        brick.id = i + 1;
    });
    bricks
}

struct Stack {
    bricks: BTreeMap<usize, Brick>,
    levels: HashMap<usize, Vec<Vec<usize>>>,
    max_x: usize,
    max_y: usize,
}

impl Stack {
    fn new() -> Stack {
        Stack {
            bricks: BTreeMap::new(),
            levels: HashMap::new(),
            max_x: 0,
            max_y: 0,
        }
    }

    fn settle_bricks(&mut self, input: Vec<String>) {
        let bricks = make_bricks(input);
        self.update_dimensions(&bricks);
        for mut brick in bricks {
            let z = brick.ends.0 .2;
            if z == 1 {
                self.update_levels(&brick, z);
                self.bricks.insert(brick.id, brick);
                continue;
            }
            let next_under = self.update_supported_by(&mut brick, z);
            self.update_levels(&brick, next_under + 1);
            self.bricks.insert(brick.id, brick.clone());
        }
    }

    fn update_dimensions(&mut self, bricks: &Vec<Brick>) {
        let (max_x, max_y) = bricks.iter().fold((0, 0), |acc, brick| {
            (acc.0.max(brick.ends.0 .0), acc.1.max(brick.ends.0 .1))
        });
        self.max_x = max_x;
        self.max_y = max_y;
    }

    fn update_supported_by(&mut self, brick: &mut Brick, z: usize) -> usize {
        let (start, end) = &brick.ends;
        let mut next_under = z - 1;
        while next_under > 0 {
            if let Some(level_map) = self.levels.get(&next_under) {
                for x in start.0..=end.0 {
                    for y in start.1..=end.1 {
                        if level_map[x][y] != 0 {
                            brick.supported_by.insert(level_map[x][y]);
                            self.bricks
                                .get_mut(&level_map[x][y])
                                .unwrap()
                                .supports
                                .insert(brick.id);
                        }
                    }
                }
                if brick.supported_by.len() > 0 {
                    break;
                }
            }
            next_under -= 1;
        }
        next_under
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

    fn disintegrateable(&self, id: usize) -> bool {
        let brick = self.bricks.get(&id).unwrap();
        brick
            .supports
            .iter()
            .all(|id| self.bricks[id].supported_by.len() > 1)
            || brick.supports.len() == 0
    }

    fn count_disintegrateable(&self) -> usize {
        let mut count = 0;
        for id in self.bricks.keys() {
            if self.disintegrateable(*id) {
                count += 1;
            }
        }
        count
    }

    fn chain_reaction(&self, id: usize) -> usize {
        let mut fallen = HashSet::new();
        if !self.disintegrateable(id) {
            let mut falling: VecDeque<Vec<usize>> = VecDeque::new();
            falling.push_back(vec![id]);
            while let Some(ids) = falling.pop_front() {
                for bid in ids.iter() {
                    let mut next_fallen = HashSet::new();
                    let brick = self.bricks.get(bid).unwrap();
                    for sid in brick.supports.iter() {
                        let supported = self.bricks.get(sid).unwrap();
                        if supported.supported_by.len() == 1 {
                            next_fallen.insert(*sid);
                            fallen.insert(*sid);
                        } else {
                            if supported.supported_by.iter().all(|id| fallen.contains(id)) {
                                next_fallen.insert(*sid);
                                fallen.insert(*sid);
                            }
                        }
                    }
                    falling.push_back(next_fallen.iter().cloned().collect());
                }
            }
        }
        fallen.len()
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

        assert_eq!(stack.count_disintegrateable(), 5);
    }

    #[test]
    fn test_chain_reaction() {
        let input = read_lines("src/bin/day22/test_input.txt").unwrap();
        let mut stack = Stack::new();
        stack.settle_bricks(input);
        assert_eq!(stack.chain_reaction(1), 6);
        assert_eq!(stack.chain_reaction(2), 0);
        assert_eq!(stack.chain_reaction(3), 0);
        assert_eq!(stack.chain_reaction(4), 0);
        assert_eq!(stack.chain_reaction(5), 0);
        assert_eq!(stack.chain_reaction(6), 1);
        assert_eq!(stack.chain_reaction(7), 0);
    }
}
