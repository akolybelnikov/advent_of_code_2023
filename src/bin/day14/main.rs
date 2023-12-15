use std::collections::HashMap;
use std::time::Instant;

// --- Day 14: Parabolic Reflector Dish ---
const STONE: u8 = b'O';
const EMPTY: u8 = b'.';

fn main() {
    let time_start = Instant::now();
    let load = part_1("src/bin/day14/input.txt");
    println!("Part 1: {:?}", load);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let load = part_2("src/bin/day14/input.txt");
    println!("Part 2: {:?}", load);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn part_1(filename: &str) -> u64 {
    let input = advent_of_code_2023::read_lines(filename).unwrap();
    let mut platform = Platform::parse(&input);
    platform.tilt_north();
    platform.calculate_load()
}

fn part_2(filename: &str) -> u64 {
    let input = advent_of_code_2023::read_lines(filename).unwrap();
    let mut platform = Platform::parse(&input);
    platform.spin_until_repeat();
    for _ in 0..platform.rest_cycle_count {
        platform.full_cycle();
    }
    platform.calculate_load()
}

struct Platform {
    rows: Vec<Vec<u8>>,
    cols: Vec<Vec<u8>>,
    previous_states: HashMap<Vec<Vec<u8>>, usize>,
    cycle_count: usize,
    rest_cycle_count: usize,
}

impl Platform {
    fn parse(input: &Vec<String>) -> Self {
        let rows = input
            .iter()
            .map(|line| line.as_bytes().to_vec())
            .collect::<Vec<_>>();

        let cols = (0..rows[0].len())
            .map(|col_idx| rows.iter().map(|row| row[col_idx]).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self {
            rows,
            cols,
            previous_states: HashMap::new(),
            cycle_count: 0,
            rest_cycle_count: 0,
        }
    }

    fn tilt_north(&mut self) {
        // first, tilt the rows by rolling the stones in each column
        self.cols.iter_mut().for_each(|col| {
            for i in 0..col.len() {
                if col[i] == STONE {
                    let mut j = i;
                    while j > 0 && col[j - 1] == EMPTY {
                        col.swap(j, j - 1);
                        j -= 1;
                    }
                }
            }
        });
        // after we're done, recalculate the rows based off the new columns
        self.recalculate_rows();
    }

    fn tilt_west(&mut self) {
        // first, tilt the rows by rolling the stones in each column
        self.rows.iter_mut().for_each(|row| {
            for i in 0..row.len() {
                if row[i] == STONE {
                    let mut j = i;
                    while j > 0 && row[j - 1] == EMPTY {
                        row.swap(j, j - 1);
                        j -= 1;
                    }
                }
            }
        });
        // after we're done, recalculate the columns based off the new rows
        self.recalculate_cols();
    }

    fn tilt_south(&mut self) {
        // first, tilt the rows by rolling the stones in each column
        self.cols.iter_mut().for_each(|col| {
            for i in (0..col.len()).rev() {
                if col[i] == STONE {
                    let mut j = i;
                    while j < col.len() - 1 && col[j + 1] == EMPTY {
                        col.swap(j, j + 1);
                        j += 1;
                    }
                }
            }
        });
        // after we're done, recalculate the rows based off the new columns
        self.recalculate_rows();
    }

    fn tilt_east(&mut self) {
        // first, tilt the rows by rolling the stones in each column
        self.rows.iter_mut().for_each(|row| {
            for i in (0..row.len()).rev() {
                if row[i] == STONE {
                    let mut j = i;
                    while j < row.len() - 1 && row[j + 1] == EMPTY {
                        row.swap(j, j + 1);
                        j += 1;
                    }
                }
            }
        });
        // after we're done, recalculate the columns based off the new rows
        self.recalculate_cols();
    }

    fn recalculate_cols(&mut self) {
        self.cols = (0..self.rows[0].len())
            .map(|col_idx| self.rows.iter().map(|row| row[col_idx]).collect::<Vec<_>>())
            .collect::<Vec<_>>();
    }

    fn recalculate_rows(&mut self) {
        self.rows = (0..self.cols[0].len())
            .map(|row_idx| self.cols.iter().map(|col| col[row_idx]).collect::<Vec<_>>())
            .collect::<Vec<_>>();
    }

    fn calculate_load(&self) -> u64 {
        self.rows
            .iter()
            .enumerate()
            .map(|(idx, row)| {
                row.iter().filter(|c| **c == STONE).count() as u64
                    * (self.rows.len() as u64 - idx as u64)
            })
            .sum()
    }

    fn full_cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn spin_until_repeat(&mut self) {
        loop {
            self.full_cycle();
            self.cycle_count += 1;
            if let Some(prev_cycle) = self.previous_states.insert(self.rows.clone(), self.cycle_count) {
                println!("cycle_count: {}", self.cycle_count);
                println!("prev_cycle: {}", prev_cycle);
                self.rest_cycle_count = (1_000_000_000 - self.cycle_count) % (self.cycle_count - prev_cycle);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2023::read_lines;

    #[test]
    fn test_parse() {
        let input = read_lines("src/bin/day14/test_input.txt").unwrap();
        let platform = Platform::parse(&input);
        assert_eq!(platform.rows.len(), 10);
        assert_eq!(platform.cols.len(), 10);
    }

    #[test]
    fn test_tilt_north() {
        let input = read_lines("src/bin/day14/test_input.txt").unwrap();
        let mut platform = Platform::parse(&input);
        assert_eq!(platform.rows[0], b"O....#....".to_vec());
        assert_eq!(platform.rows[1], b"O.OO#....#".to_vec());
        assert_eq!(platform.rows[2], b".....##...".to_vec());
        assert_eq!(platform.rows[3], b"OO.#O....O".to_vec());
        assert_eq!(platform.rows[4], b".O.....O#.".to_vec());
        assert_eq!(platform.rows[5], b"O.#..O.#.#".to_vec());
        assert_eq!(platform.rows[6], b"..O..#O..O".to_vec());
        assert_eq!(platform.rows[7], b".......O..".to_vec());
        assert_eq!(platform.rows[8], b"#....###..".to_vec());
        assert_eq!(platform.rows[9], b"#OO..#....".to_vec());
        platform.tilt_north();
        assert_eq!(platform.rows[0], b"OOOO.#.O..".to_vec());
        assert_eq!(platform.rows[1], b"OO..#....#".to_vec());
        assert_eq!(platform.rows[2], b"OO..O##..O".to_vec());
        assert_eq!(platform.rows[3], b"O..#.OO...".to_vec());
        assert_eq!(platform.rows[4], b"........#.".to_vec());
        assert_eq!(platform.rows[5], b"..#....#.#".to_vec());
        assert_eq!(platform.rows[6], b"..O..#.O.O".to_vec());
        assert_eq!(platform.rows[7], b"..O.......".to_vec());
        assert_eq!(platform.rows[8], b"#....###..".to_vec());
        assert_eq!(platform.rows[9], b"#....#....".to_vec());
    }

    #[test]
    fn test_calculate_load() {
        let input = read_lines("src/bin/day14/test_input.txt").unwrap();
        let mut platform = Platform::parse(&input);
        platform.tilt_north();
        assert_eq!(platform.calculate_load(), 136);
    }

    #[test]
    fn test_full_cycle() {
        let input = read_lines("src/bin/day14/test_input.txt").unwrap();
        let mut platform = Platform::parse(&input);
        platform.tilt_north();
        platform.tilt_west();
        assert_eq!(platform.rows[0], b"OOOO.#O...".to_vec());
        assert_eq!(platform.rows[1], b"OO..#....#".to_vec());
        assert_eq!(platform.rows[2], b"OOO..##O..".to_vec());
        assert_eq!(platform.rows[3], b"O..#OO....".to_vec());
        assert_eq!(platform.rows[4], b"........#.".to_vec());
        assert_eq!(platform.rows[5], b"..#....#.#".to_vec());
        assert_eq!(platform.rows[6], b"O....#OO..".to_vec());
        assert_eq!(platform.rows[7], b"O.........".to_vec());
        assert_eq!(platform.rows[8], b"#....###..".to_vec());
        assert_eq!(platform.rows[9], b"#....#....".to_vec());
        platform.tilt_south();
        assert_eq!(platform.rows[0], b".....#....".to_vec());
        assert_eq!(platform.rows[1], b"....#.O..#".to_vec());
        assert_eq!(platform.rows[2], b"O..O.##...".to_vec());
        assert_eq!(platform.rows[3], b"O.O#......".to_vec());
        assert_eq!(platform.rows[4], b"O.O....O#.".to_vec());
        assert_eq!(platform.rows[5], b"O.#..O.#.#".to_vec());
        assert_eq!(platform.rows[6], b"O....#....".to_vec());
        assert_eq!(platform.rows[7], b"OO....OO..".to_vec());
        assert_eq!(platform.rows[8], b"#O...###..".to_vec());
        assert_eq!(platform.rows[9], b"#O..O#....".to_vec());
        platform.tilt_east();
        assert_eq!(platform.rows[0], b".....#....".to_vec());
        assert_eq!(platform.rows[1], b"....#...O#".to_vec());
        assert_eq!(platform.rows[2], b"...OO##...".to_vec());
        assert_eq!(platform.rows[3], b".OO#......".to_vec());
        assert_eq!(platform.rows[4], b".....OOO#.".to_vec());
        assert_eq!(platform.rows[5], b".O#...O#.#".to_vec());
        assert_eq!(platform.rows[6], b"....O#....".to_vec());
        assert_eq!(platform.rows[7], b"......OOOO".to_vec());
        assert_eq!(platform.rows[8], b"#...O###..".to_vec());
        assert_eq!(platform.rows[9], b"#..OO#....".to_vec());
        assert_eq!(platform.calculate_load(), 87);
    }

    #[test]
    fn test_part_2() {
        let input = read_lines("src/bin/day14/test_input.txt").unwrap();
        let mut platform = Platform::parse(&input);
        platform.spin_until_repeat();
        assert_eq!(platform.rest_cycle_count, 3);
        for _ in 0..platform.rest_cycle_count {
            platform.full_cycle();
        }
        assert_eq!(platform.calculate_load(), 64);
    }
}
