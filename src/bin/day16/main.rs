use advent_of_code_2023::read_lines;
use std::time::Instant;

#[derive(Clone)]
struct Tile {
    value: u8,
    state: [u8; 4], // [0] left, [1] up, [2] right, [3] down; 1 = visited, 0 = not visited
}

struct Contraption {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Contraption {
    fn new(lines: Vec<String>) -> Self {
        let width = lines[0].len();
        let height = lines.len();
        let mut tiles = vec![
            vec![
                Tile {
                    value: 0,
                    state: [0; 4]
                };
                width
            ];
            height
        ];
        for x in 0..lines.len() {
            for y in 0..lines[x].len() {
                let value = lines[x].as_bytes()[y];
                tiles[y][x].value = value;
            }
        }
        Self {
            tiles,
            width,
            height,
        }
    }
    fn next(&self, x: usize, y: usize) -> u8 {
        self.tiles[y][x].value
    }

    fn move_right(&mut self, x: usize, y: usize) {
        self.tiles[x][y].state[0] = 1;
        match self.next(x, y) {
            b'|' => {
                self.next_down(x, y);
                self.next_up(x, y);
            }
            b'/' => self.next_up(x, y),
            b'\\' => self.next_down(x, y),
            _ => self.next_right(x, y),
        }
    }

    fn next_right(&mut self, x: usize, prev_y: usize) {
        if prev_y == self.width - 1 {
            return;
        }
        let y = prev_y + 1;
        if self.tiles[x][y].state[0] == 1 {
            return;
        }
        self.tiles[x][prev_y].state[2] = 1; // prevent looping
        self.move_right(x, y);
    }

    fn move_up(&mut self, x: usize, y: usize) {
        self.tiles[x][y].state[3] = 1;
        match self.next(x, y) {
            b'-' => {
                self.next_right(x, y);
                self.next_left(x, y);
            }
            b'/' => self.next_right(x, y),
            b'\\' => self.next_left(x, y),
            _ => self.next_up(x, y),
        }
    }

    fn next_up(&mut self, prev_x: usize, y: usize) {
        if prev_x == 0 {
            return;
        }
        let x = prev_x - 1;
        if self.tiles[x][y].state[3] == 1 {
            return;
        }
        self.tiles[prev_x][y].state[1] = 1; // prevent looping
        self.move_up(x, y);
    }

    fn move_down(&mut self, x: usize, y: usize) {
        self.tiles[x][y].state[1] = 1;
        match self.next(x, y) {
            b'-' => {
                self.next_right(x, y);
                self.next_left(x, y);
            }
            b'/' => self.next_left(x, y),
            b'\\' => self.next_right(x, y),
            _ => self.next_down(x, y),
        }
    }

    fn next_down(&mut self, prev_x: usize, y: usize) {
        if prev_x == self.height - 1 {
            return;
        }
        let x = prev_x + 1;
        if self.tiles[x][y].state[1] == 1 {
            return;
        }
        self.tiles[prev_x][y].state[3] = 1; // prevent looping
        self.move_down(x, y);
    }

    fn move_left(&mut self, x: usize, y: usize) {
        self.tiles[x][y].state[2] = 1;
        match self.next(x, y) {
            b'|' => {
                self.next_down(x, y);
                self.next_up(x, y);
            }
            b'/' => self.next_down(x, y),
            b'\\' => self.next_up(x, y),
            _ => self.next_left(x, y),
        }
    }

    fn next_left(&mut self, x: usize, prev_y: usize) {
        if prev_y == 0 {
            return;
        }
        let y = prev_y - 1;
        if self.tiles[x][y].state[2] == 1 {
            return;
        }
        self.tiles[x][prev_y].state[0] = 1; // prevent looping
        self.move_left(x, y);
    }

    fn flash_state(&mut self) {
        for x in 0..self.height {
            for y in 0..self.width {
                self.tiles[x][y].state = [0; 4];
            }
        }
    }

    fn num_of_energized(&self) -> u64 {
        self.tiles
            .iter()
            .flatten()
            .filter(|t| t.state.iter().sum::<u8>() > 0)
            .count() as u64
    }
}

fn main() {
    let time_start = Instant::now();
    let sum = part_1("src/bin/day16/input.txt");
    println!("Part 1: {:?}", sum);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let sum = part_2("src/bin/day16/input.txt");
    println!("Part 2: {:?}", sum);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn part_1(filename: &str) -> u64 {
    let input = read_lines(filename).unwrap();
    let mut contraption = Contraption::new(input);
    contraption.move_right(0, 0);
    contraption.num_of_energized()
}

fn part_2(filename: &str) -> u64 {
    let input = read_lines(filename).unwrap();
    let mut contraption = Contraption::new(input);
    let mut max = 0;
    for x in 0..contraption.height {
        contraption.flash_state();
        contraption.move_right(x, 0);
        max = max.max(contraption.num_of_energized());
        contraption.flash_state();
        contraption.move_left(x, contraption.width - 1);
        max = max.max(contraption.num_of_energized());
    }
    for y in 0..contraption.width {
        contraption.flash_state();
        contraption.move_down(0, y);
        max = max.max(contraption.num_of_energized());
        contraption.flash_state();
        contraption.move_up(contraption.height - 1, y);
        max = max.max(contraption.num_of_energized());
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_contraption() {
        let input = read_lines("src/bin/day16/test_input.txt").unwrap();
        let contraption = Contraption::new(input);
        assert_eq!(contraption.width, 10);
        assert_eq!(contraption.height, 10);
        assert_eq!(contraption.tiles[0][0].value, b'.');
        assert_eq!(contraption.tiles[0][1].value, b'|');
        assert_eq!(contraption.tiles[9][9].value, b'.');
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_1("src/bin/day16/test_input.txt"), 46);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_2("src/bin/day16/test_input.txt"), 51);
    }
}
