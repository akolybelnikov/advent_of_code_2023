// --- Day 10: Pipe Maze ---

type Coordinates = [i32; 2];

#[allow(dead_code)]
#[derive(Default, Clone, Copy)]
struct Tile {
    entries: [Coordinates; 2],
    is_start: bool,
    position: Coordinates,
    visited: i32,
}
#[allow(dead_code)]
impl Tile {
    fn new(entries: [Coordinates; 2], position: Coordinates) -> Tile {
        Tile { entries, is_start: false, position, visited: 0 }
    }

    fn start_tile(position: Coordinates) -> Tile {
        Tile { entries: [[0;2], [0;2]], is_start: true, position, visited: 0 }
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
    start: Tile,
    tiles: Vec<Vec<Tile>>,
}

impl Maze {
    fn find_start_entries(&mut self) {
        let x = self.start.position[0];
        let y = self.start.position[1];
        let mut entries: Vec<Coordinates> = Vec::new();
        match x {
            0 => entries.push([x + 1, y]),
            _ if x == self.tiles.len() as i32 - 1 => entries.push([x - 1, y]),
            _ => {}
        }

        match y {
            0 => entries.push([x, y + 1]),
            _ if y == self.tiles[0].len() as i32 - 1 => entries.push([x, y - 1]),
            _ => {}
        }

        if self.tiles[y as usize - 1][x as usize].entries.contains(&self.start.position) {
            entries.push([x, y - 1]);
        }
        if self.tiles[y as usize + 1][x as usize].entries.contains(&self.start.position) {
            entries.push([x, y + 1]);
        }
        if self.tiles[y as usize][x as usize + 1].entries.contains(&self.start.position) {
            entries.push([x + 1, y]);
        }
        if self.tiles[y as usize][x as usize - 1].entries.contains(&self.start.position) {
            entries.push([x - 1, y]);
        }
        self.start.entries[0] = [entries[0][0], entries[0][1]];
        self.start.entries[1] = [entries[1][0], entries[1][1]];
    }
}

fn parse_input(input: &Vec<String>) -> Maze {
    let mut tiles: Vec<Vec<Tile>> = Vec::new();
    let mut maze = Maze::default();
    for (y, line) in input.iter().enumerate() {
        let mut row: Vec<Tile> = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '-' => Tile::new([[x as i32 - 1, y as i32], [x as i32 + 1, y as i32]], [x as i32, y as i32]),
                '.' => Tile::default(),
                '7' => Tile::new([[x as i32, y as i32 + 1], [x as i32 - 1, y as i32]], [x as i32, y as i32]),
                'F' => Tile::new([[x as i32, y as i32 + 1], [x as i32 + 1, y as i32]], [x as i32, y as i32]),
                'J' => Tile::new([[x as i32, y as i32 - 1], [x as i32 - 1, y as i32]], [x as i32, y as i32]),
                'L' => Tile::new([[x as i32, y as i32 - 1], [x as i32 + 1, y as i32]], [x as i32, y as i32]),
                'S' => Tile::start_tile([x as i32, y as i32]),
                '|' => Tile::new([[x as i32, y as i32 - 1], [x as i32, y as i32 + 1]], [x as i32, y as i32]),
                _ => panic!("Unknown tile type: {}", c),
            };
            row.push(tile);
            if tile.is_start {
                maze.start = tile;
            }
        }
        tiles.push(row);
    }
    maze.tiles = tiles;
    maze.find_start_entries();
    maze
}

fn main() {
    println!("Hello from day10!");
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
}
