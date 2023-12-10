// --- Day 10: Pipe Maze ---
type Coordinates = [i32; 2];

#[derive(Default, Clone, Copy)]
struct Tile {
    entries: [Coordinates; 2],
    is_ground: bool,
    is_start: bool,
    position: Coordinates,
}

impl Tile {
    fn new(entries: [Coordinates; 2], position: Coordinates) -> Tile {
        Tile {
            entries,
            is_ground: true,
            is_start: false,
            position,
        }
    }

    fn start_tile(position: Coordinates) -> Tile {
        Tile {
            entries: [[0; 2], [0; 2]],
            is_ground: false,
            is_start: true,
            position,
        }
    }

    fn ground_tile(position: Coordinates) -> Tile {
        Tile {
            entries: Default::default(),
            is_ground: true,
            is_start: false,
            position,
        }
    }
    fn to(&self, from: Coordinates) -> Coordinates {
        if self.entries[0] == from {
            self.entries[1]
        } else {
            self.entries[0]
        }
    }
}

#[derive(Default)]
struct Maze {
    ground: Vec<Tile>,
    start: Tile,
    tiles: Vec<Vec<Tile>>,
}

impl Maze {
    fn find_start_entries(&mut self) {
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
                    .entries
                    .contains(&self.start.position)
                {
                    entries.push([j, i]);
                }
            }
        }

        // Update start entries only if entries vec has enough items
        if entries.len() > 1 {
            self.start.entries[0] = entries[0];
            self.start.entries[1] = entries[1];
        }
    }

    fn walk_loop(
        &mut self,
        from: Coordinates,
        to: Coordinates,
        mut next: Coordinates,
    ) -> Vec<Coordinates> {
        let mut positions = Vec::new();
        positions.push(next);
        let mut prev = from;
        while next != to {
            let tile = &mut self.tiles[next[1] as usize][next[0] as usize];
            tile.is_ground = false;
            let cur = next;
            next = tile.to(prev);
            prev = cur;
            positions.push(next);
        }
        positions
    }
}

fn parse_input(input: &Vec<String>) -> Maze {
    let ground_tiles: Vec<Tile> = Vec::new();
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
                '.' => Tile::ground_tile([x as i32, y as i32]),
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
    maze.ground = ground_tiles;
    maze.find_start_entries();
    maze
}

fn part_1(filename: &str) -> i32 {
    let lines = advent_of_code_2023::read_lines(filename).unwrap();
    let mut maze = parse_input(&lines);
    let walk = maze.walk_loop(
        maze.start.position,
        maze.start.position,
        maze.start.entries[0],
    );
    walk.len() as i32 / 2
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
        assert!(maze.start.entries.contains(&[1, 2]));
        assert!(maze.start.entries.contains(&[2, 1]));
    }

    #[test]
    fn test_walk() {
        let input = advent_of_code_2023::read_lines("src/bin/day10/test_input_1.txt").unwrap();
        let mut maze = parse_input(&input);
        let positions = maze.walk_loop(
            maze.start.position,
            maze.start.position,
            maze.start.entries[0],
        );
        assert_eq!(positions.len(), 8);
        assert_eq!(positions[0], [2, 1]);
        assert_eq!(positions[1], [3, 1]);
        assert_eq!(positions[2], [3, 2]);
        assert_eq!(positions[3], [3, 3]);
        assert_eq!(positions[4], [2, 3]);
        assert_eq!(positions[5], [1, 3]);
        assert_eq!(positions[6], [1, 2]);
        assert_eq!(positions[7], [1, 1]);
        for position in positions {
            assert!(!maze.tiles[position[1] as usize][position[0] as usize].is_ground);
        }
        let ground_tiles = maze.tiles.iter().flatten().filter(|t| t.is_ground).count();
        assert_eq!(ground_tiles, 17);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/bin/day10/test_input_1.txt"), 4);
        assert_eq!(part_1("src/bin/day10/test_input_2.txt"), 8);
    }
}
