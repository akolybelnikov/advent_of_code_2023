use std::collections::HashSet;

// --- Day 10: Pipe Maze ---
type Coordinates = [i32; 2];

#[derive(Default, Clone, Copy)]
struct Tile {
    gates: [Coordinates; 2],
    is_loop: bool,
    is_start: bool,
    position: Coordinates,
}

impl Tile {
    fn new(entries: [Coordinates; 2], position: Coordinates) -> Tile {
        Tile {
            gates: entries,
            is_loop: false,
            is_start: false,
            position,
        }
    }

    fn start_tile(position: Coordinates) -> Tile {
        Tile {
            gates: [[0; 2], [0; 2]],
            is_loop: true,
            is_start: true,
            position,
        }
    }

    fn to(&self, from: Coordinates) -> Coordinates {
        if self.gates[0] == from {
            self.gates[1]
        } else {
            self.gates[0]
        }
    }
}

#[derive(Default)]
struct Maze {
    start: Tile,
    tiles: Vec<Vec<Tile>>,
}

impl Maze {
    fn find_start_gates(&mut self) {
        // Define the ranges around the start tile
        let x_range = [
            (self.start.position[0] - 1).max(0),
            (self.start.position[0] + 1).min(self.tiles[0].len() as i32 - 1),
        ];
        let y_range = [
            (self.start.position[1] - 1).max(0),
            (self.start.position[1] + 1).min(self.tiles.len() as i32 - 1),
        ];

        let mut entries: Vec<Coordinates> = Vec::new();

        // Enumerate over every tile in the tiles Vec that is within our defined range
        for i in y_range[0]..=y_range[1] {
            for j in x_range[0]..=x_range[1] {
                // Skip the start tile itself
                if i == self.start.position[1] && j == self.start.position[0] {
                    continue;
                }
                // Check if the tile's list of entries contains the start position
                if self.tiles[i as usize][j as usize]
                    .gates
                    .contains(&self.start.position)
                {
                    entries.push([j, i]);
                }
            }
        }

        // Update start entries only if entries vec has enough items
        if entries.len() > 1 {
            self.start.gates[0] = entries[0];
            self.start.gates[1] = entries[1];
        }
    }

    fn find_flood_gates(&mut self) -> Vec<Coordinates> {
        let mut entries: Vec<Coordinates> = Vec::new();
        let y = self.start.position[1];
        let x = self.start.position[0];
        // tile to the north of start
        if y > 0 {
            let north = self.tiles[(y - 1) as usize][x as usize].position;
            if !self.start.gates.contains(&north) {
                entries.push(north);
            }
        }
        // tile to the south of start
        if y < (self.tiles.len() - 1) as i32 {
            let south = self.tiles[(y + 1) as usize][x as usize].position;
            if !self.start.gates.contains(&south) {
                entries.push(south);
            }
        }
        // tile to the east of start
        if x < (self.tiles[0].len() - 1) as i32 {
            let east = self.tiles[y as usize][(x + 1) as usize].position;
            if !self.start.gates.contains(&east) {
                entries.push(east);
            }
        }
        // tile to the west of start
        if x > 0 {
            let west = self.tiles[y as usize][(x - 1) as usize].position;
            if !self.start.gates.contains(&west) {
                entries.push(west);
            }
        }

        entries
    }

    fn walk_loop(
        &mut self,
        from: Coordinates,
        to: Coordinates,
        mut next: Coordinates,
    ) -> Vec<Coordinates> {
        let mut loop_tiles = Vec::new();
        loop_tiles.push(next);
        let mut prev = from;
        while next != to {
            let tile = &mut self.tiles[next[1] as usize][next[0] as usize];
            tile.is_loop = true;
            let cur = next;
            next = tile.to(prev);
            prev = cur;
            loop_tiles.push(next);
        }
        loop_tiles
    }

    fn fill_flood(&mut self, from: Coordinates) -> (HashSet<Coordinates>, bool) {
        let mut visited: HashSet<Coordinates> = HashSet::new();
        let offsets: [Coordinates; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];
        let mut queue: Vec<Coordinates> = Vec::new();
        queue.push(from);

        while !queue.is_empty() {
            let current = queue.pop().unwrap();
            // if current is on the border, we're done
            if current[0] == 0
                || current[1] == 0
                || current[0] == (self.tiles[0].len() - 1) as i32
                || current[1] == (self.tiles.len() - 1) as i32
            {
                return (visited, false);
            }
            visited.insert(current);

            for offset in offsets.iter() {
                let next = [current[0] + offset[0], current[1] + offset[1]];

                let is_valid = next[0] >= 0
                    && next[1] >= 0
                    && next[0] <= (self.tiles[0].len() - 1) as i32
                    && next[1] <= (self.tiles.len() - 1) as i32;
                if is_valid && !visited.contains(&next) {
                    let tile = &self.tiles[next[1] as usize][next[0] as usize];
                    if !tile.is_loop {
                        queue.push(next);
                    }
                }
            }
        }
        (visited, true)
    }
}

fn parse_input(input: &Vec<String>) -> Maze {
    let mut tiles: Vec<Vec<Tile>> = Vec::new();
    let mut maze = Maze::default();
    for (y, line) in input.iter().enumerate() {
        let mut row: Vec<Tile> = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '-' => Tile::new(
                    [[x as i32 - 1, y as i32], [x as i32 + 1, y as i32]],
                    [x as i32, y as i32],
                ),
                '.' => Tile::new(Default::default(), [x as i32, y as i32]),
                '7' => Tile::new(
                    [[x as i32, y as i32 + 1], [x as i32 - 1, y as i32]],
                    [x as i32, y as i32],
                ),
                'F' => Tile::new(
                    [[x as i32, y as i32 + 1], [x as i32 + 1, y as i32]],
                    [x as i32, y as i32],
                ),
                'J' => Tile::new(
                    [[x as i32, y as i32 - 1], [x as i32 - 1, y as i32]],
                    [x as i32, y as i32],
                ),
                'L' => Tile::new(
                    [[x as i32, y as i32 - 1], [x as i32 + 1, y as i32]],
                    [x as i32, y as i32],
                ),
                'S' => Tile::start_tile([x as i32, y as i32]),
                '|' => Tile::new(
                    [[x as i32, y as i32 - 1], [x as i32, y as i32 + 1]],
                    [x as i32, y as i32],
                ),
                _ => panic!("Unknown tile type: {}", c),
            };
            if tile.is_start {
                maze.start = tile;
            }
            row.push(tile);
        }
        tiles.push(row);
    }
    maze.tiles = tiles;
    maze.find_start_gates();
    maze
}

fn part_1(filename: &str) -> i32 {
    let lines = advent_of_code_2023::read_lines(filename).unwrap();
    let mut maze = parse_input(&lines);
    let walk = maze.walk_loop(
        maze.start.position,
        maze.start.position,
        maze.start.gates[0],
    );
    walk.len() as i32 / 2
}

fn part_2(filename: &str) -> i32 {
    let lines = advent_of_code_2023::read_lines(filename).unwrap();
    let mut maze = parse_input(&lines);
    maze.walk_loop(
        maze.start.position,
        maze.start.position,
        maze.start.gates[0],
    );
    let flood_gates = maze.find_flood_gates();
    let (flood, inner) = maze.fill_flood([0, 0]);
    if inner {
        flood.iter().count() as i32
    } else {
        let (flood, inner) = maze.fill_flood([2, 2]);
        flood.iter().count() as i32
    }
}

fn main() {
    println!("Part 1: {}", part_1("src/bin/day10/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = advent_of_code_2023::read_lines("src/bin/day10/test_input_1.txt").unwrap();
        let maze = parse_input(&input);
        assert_eq!(maze.tiles.len(), 5);
        assert_eq!(maze.tiles[0].len(), 5);
        assert_eq!(maze.start.position, [1, 1]);
        assert!(maze.start.gates.contains(&[1, 2]));
        assert!(maze.start.gates.contains(&[2, 1]));
    }

    #[test]
    fn test_walk() {
        let input = advent_of_code_2023::read_lines("src/bin/day10/test_input_1.txt").unwrap();
        let mut maze = parse_input(&input);
        let loop_tiles = maze.walk_loop(
            maze.start.position,
            maze.start.position,
            maze.start.gates[0],
        );
        assert_eq!(loop_tiles.len(), 8);
        assert_eq!(loop_tiles[0], [2, 1]);
        assert_eq!(loop_tiles[1], [3, 1]);
        assert_eq!(loop_tiles[2], [3, 2]);
        assert_eq!(loop_tiles[3], [3, 3]);
        assert_eq!(loop_tiles[4], [2, 3]);
        assert_eq!(loop_tiles[5], [1, 3]);
        assert_eq!(loop_tiles[6], [1, 2]);
        assert_eq!(loop_tiles[7], [1, 1]);
        for position in loop_tiles {
            assert!(maze.tiles[position[1] as usize][position[0] as usize].is_loop);
        }
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/bin/day10/test_input_1.txt"), 4);
        assert_eq!(part_1("src/bin/day10/test_input_2.txt"), 8);
    }

    #[test]
    fn test_fill_flood() {
        let input = advent_of_code_2023::read_lines("src/bin/day10/test_input_3.txt").unwrap();
        let mut maze = parse_input(&input);
        let _loop_tiles = maze.walk_loop(
            maze.start.position,
            maze.start.position,
            maze.start.gates[0],
        );
        let flood = maze.fill_flood([1, 0]);
        assert!(!flood.1);
    }

    #[test]
    fn test_part_2() {
        //assert_eq!(part_2("src/bin/day10/test_input_1.txt"), 4);
        // assert_eq!(part_2("src/bin/day10/test_input_2.txt"), 8);
        assert_eq!(part_2("src/bin/day10/test_input_3.txt"), 24);
    }
}
