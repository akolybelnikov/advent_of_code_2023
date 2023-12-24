use advent_of_code_2023::read_lines;
use std::collections::{HashSet, VecDeque};

fn main() {
    let time_start = std::time::Instant::now();
    let count = part_1("src/bin/day21/input.txt", 64);
    println!(
        "Part 1: {:?}  Time: {}μs",
        count,
        time_start.elapsed().as_micros()
    );

    let time_start = std::time::Instant::now();
    let count = part_2();
    println!(
        "Part 2: {:?}  Time: {}μs",
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

fn part_2() -> usize {
    // watch the explanation here: https://youtu.be/9UOMZSL0JTg?si=fRbkrVPlbZm1_TJ8
    let steps = 26501365;
    let lines = read_lines("src/bin/day21/input.txt").unwrap();
    let garden = Garden::new(lines);
    // assuming start tile is in the middle
    assert_eq!(garden.start.0, garden.size / 2);
    assert_eq!(garden.start.1, garden.size / 2);
    // assuming steps are a multiple of the size of the garden
    assert_eq!(steps % garden.size, garden.size / 2);
    // fun fact :)
    assert_eq!(steps / garden.size, 202300);
    // the following is possible because of the assumption that the row and column of the start tile are empty
    let garden_width = (steps / garden.size) - 1;
    let odd = (garden_width / 2 * 2 + 1).pow(2);
    let even = ((garden_width + 1) / 2 * 2).pow(2);
    let odd_points = garden.fill(garden.start, garden.size * 2 + 1);
    let even_points = garden.fill(garden.start, garden.size * 2);

    let size = garden.size - 1;
    let top_corner = garden.fill((size, garden.start.1), size);
    let right_corner = garden.fill((garden.start.0, 0), size);
    let bottom_corner = garden.fill((0, garden.start.1), size);
    let left_corner = garden.fill((garden.start.0, size), size);

    let sm_segment_steps = garden.size / 2 - 1;
    let sm_tr_segment = garden.fill((size, 0), sm_segment_steps);
    let sm_tl_segment = garden.fill((size, size), sm_segment_steps);
    let sm_br_segment = garden.fill((0, 0), sm_segment_steps);
    let sm_bl_segment = garden.fill((0, size), sm_segment_steps);

    let lg_segment_steps = garden.size * 3 / 2 - 1;
    let lg_tr_segment = garden.fill((size, 0), lg_segment_steps);
    let lg_tl_segment = garden.fill((size, size), lg_segment_steps);
    let lg_br_segment = garden.fill((0, 0), lg_segment_steps);
    let lg_bl_segment = garden.fill((0, size), lg_segment_steps);

    (odd * odd_points)
        + (even * even_points)
        + top_corner
        + right_corner
        + bottom_corner
        + left_corner
        + ((garden_width + 1) * (sm_tr_segment + sm_tl_segment + sm_br_segment + sm_bl_segment))
        + (garden_width * (lg_tr_segment + lg_tl_segment + lg_br_segment + lg_bl_segment))
}

const PLOT: char = '.';
const ROCK: char = '#';
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

    // Part 2
    fn fill(&self, pos: (usize, usize), steps: usize) -> usize {
        let mut queue = VecDeque::new();
        queue.push_back((pos, steps));
        let mut seen = HashSet::new();
        seen.insert(pos);
        let mut ans = HashSet::new();

        while let Some(((r, c), num)) = queue.pop_front() {
            if num % 2 == 0 {
                ans.insert((r, c));
            }
            if num == 0 {
                continue;
            }
            for (nr, nc) in vec![
                (r as i32 + 1, c as i32),
                (r as i32 - 1, c as i32),
                (r as i32, c as i32 + 1),
                (r as i32, c as i32 - 1),
            ] {
                if nr < 0
                    || nr >= self.size as i32
                    || nc < 0
                    || nc >= self.size as i32
                    || self.garden[nr as usize][nc as usize] == ROCK
                    || seen.contains(&(nr as usize, nc as usize))
                {
                    continue;
                }
                seen.insert((nr as usize, nc as usize));
                queue.push_back(((nr as usize, nc as usize), num - 1));
            }
        }

        ans.len()
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
