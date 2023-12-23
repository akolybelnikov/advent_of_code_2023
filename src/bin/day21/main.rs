use advent_of_code_2023::read_lines;
use std::collections::HashSet;

fn main() {
    let time_start = std::time::Instant::now();
    let count = part_1("src/bin/day21/input.txt", 64);
    println!(
        "Part 1: {:?}  Time: {}μs",
        count,
        time_start.elapsed().as_micros()
    );
}

fn part_1(filename: &str, num: usize) -> usize {
    let lines = read_lines(filename).unwrap();
    let mut garden = Garden::new(lines);
    for _ in 0..num {
        garden.make_step();
    }
    garden.print_garden();
    garden.tiles.len()
}

const PLOT: char = '.';
const START: char = 'S';
const TILE: char = 'O';

struct Garden {
    garden: Vec<Vec<char>>,
    start: (usize, usize),
    size: usize,
    tiles: HashSet<(usize, usize)>,
}

impl Garden {
    fn new(lines: Vec<String>) -> Self {
        let mut garden = Vec::new();
        let mut start = (0, 0);
        let mut size = 0;
        for (i, line) in lines.iter().enumerate() {
            let mut row = Vec::new();
            for (j, c) in line.chars().enumerate() {
                if c == START {
                    start = (i, j);
                }
                row.push(c);
            }
            size = row.len();
            garden.push(row);
        }
        Garden {
            garden,
            start,
            size,
            tiles: HashSet::new(),
        }
    }

    fn make_step(&mut self) {
        let mut new_garden = self.garden.clone();
        if self.tiles.len() == 0 {
            for t in self.update_tiles(self.start.0, self.start.1) {
                self.tiles.insert(t);
            }
            for (i, j) in &self.tiles {
                new_garden[*i][*j] = TILE;
            }
            new_garden[self.start.0][self.start.1] = PLOT;
        } else {
            let mut new_tiles = HashSet::new();
            let old_tiles = self.tiles.clone();
            for (i, j) in &old_tiles {
                let tile_updates = self.update_tiles(*i, *j);
                for (ni, nj) in tile_updates {
                    new_garden[ni][nj] = TILE;
                    new_tiles.insert((ni, nj));
                }
                new_garden[*i][*j] = PLOT;
            }
            self.tiles = new_tiles;
        }
        self.garden = new_garden;
    }

    fn update_tiles(&mut self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let next = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut new_tiles = Vec::new();
        for (x, y) in next {
            let (ni, nj) = (i as i32 + x, j as i32 + y);
            if ni >= 0 && ni < self.size as i32 && nj >= 0 && nj < self.size as i32 {
                if self.garden[ni as usize][nj as usize] == PLOT {
                    new_tiles.push((ni as usize, nj as usize));
                }
            }
        }
        new_tiles
    }

    fn print_garden(&self) {
        for row in &self.garden {
            for c in row {
                print!("{}", c);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_garden_new() {
        let lines = vec![
            String::from("S.#"),
            String::from(".#."),
            String::from("..."),
        ];
        let garden = Garden::new(lines);
        assert_eq!(garden.garden.len(), 3);
        assert_eq!(garden.garden[0].len(), 3);
        assert_eq!(garden.garden[0][0], START);
        assert_eq!(garden.garden[0][1], PLOT);
        assert_eq!(garden.garden[1][0], PLOT);
        assert_eq!(garden.garden[1][2], PLOT);
        assert_eq!(garden.garden[2][0], PLOT);
        assert_eq!(garden.garden[2][1], PLOT);
        assert_eq!(garden.garden[2][2], PLOT);
        assert_eq!(garden.start, (0, 0));
        assert_eq!(garden.size, 3);
    }

    #[test]
    fn test_steps() {
        let lines = vec![
            String::from("S.#"),
            String::from(".#."),
            String::from("..."),
        ];
        let mut garden = Garden::new(lines);
        garden.make_step();
        assert_eq!(garden.garden[0][0], PLOT);
        assert_eq!(garden.tiles.len(), 2);
        garden.make_step();
        assert_eq!(garden.tiles.len(), 2);
        garden.make_step();
        assert_eq!(garden.tiles.len(), 3);
        garden.make_step();
        assert_eq!(garden.tiles.len(), 3);
        garden.make_step();
        assert_eq!(garden.tiles.len(), 4);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/bin/day21/test_input.txt", 6), 16);
    }
}
