use std::time::Instant;

// --- Day 14: Parabolic Reflector Dish ---
const STONE: u8 = b'O';
const EMPTY: u8 = b'.';

fn main() {
    let time_start = Instant::now();
    let load = part_1("src/bin/day14/input.txt");
    println!("Part 1: {:?}", load);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn part_1(filename: &str) -> u64 {
    let input = advent_of_code_2023::read_lines(filename).unwrap();
    let mut platform = Platform::parse(&input);
    platform.tilt_north();
    platform.calculate_load()
}

struct Platform {
    rows: Vec<Vec<u8>>,
    cols: Vec<Vec<u8>>,
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

        Self { rows, cols }
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
        self.rows = (0..self.cols[0].len())
            .map(|row_idx| self.cols.iter().map(|col| col[row_idx]).collect::<Vec<_>>())
            .collect::<Vec<_>>();
    }

    fn calculate_load(&self) -> u64 {
        self.rows
            .iter()
            .enumerate()
            .map(|(idx, row)| {
                row.iter().filter(|c| **c == STONE).count() as u64 * (self.rows.len() as u64
                    - idx as u64)
            })
            .sum()
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
}
