// --- Day 13: Point of Incidence ---

use advent_of_code_2023::read_lines;

fn main() {
    println!("Part 1: {}", part_1("src/bin/day13/input.txt"));
}

fn part_1(filename: &str) -> u64 {
    let input = read_lines(filename).unwrap();
    let mirrors = parse_mirrors(input);

    mirrors.iter().map(|m| m.score_reflection()).sum()
}

struct Mirror {
    lines: Vec<Vec<char>>,
}

impl Mirror {
    fn new(input: Vec<String>) -> Self {
        let mut lines = Vec::new();
        for line in input {
            lines.push(line.chars().collect());
        }

        Self { lines }
    }

    fn row_equals(&self, row: usize, other_row: usize) -> bool {
        self.lines[row] == self.lines[other_row]
    }

    fn col_equals(&self, col: usize, other_col: usize) -> bool {
        let mut result = true;
        for row in &self.lines {
            result &= row[col] == row[other_col];
        }
        result
    }

    fn find_horizontal_symmetry(&self) -> Option<(usize, usize)> {
        for row in 0..self.lines.len() - 1 {
            if self.row_equals(row, row + 1) {
                let mut upper = row;
                let mut lower = row + 1;
                while upper > 0 && lower < self.lines.len() - 1 {
                    upper -= 1;
                    lower += 1;
                    if !self.row_equals(upper, lower) {
                        return None;
                    }
                }
                return Some((row, row + 1));
            }
        }
        None
    }

    fn find_vertical_symmetry(&self) -> Option<(usize, usize)> {
        for col in 0..self.lines[0].len() - 1 {
            if self.col_equals(col, col + 1) {
                let mut left = col;
                let mut right = col + 1;
                while left > 0 && right < self.lines[0].len() - 1 {
                    left -= 1;
                    right += 1;
                    if !self.col_equals(left, right) {
                        return None;
                    }
                }
                return Some((col, col + 1));
            }
        }
        None
    }

    fn score_reflection(&self) -> u64 {
        let mut score = 0;
        let h_symmetry = self.find_horizontal_symmetry();
        match h_symmetry {
            Some((upper, _lower)) => score += 100 * (upper as u64 + 1),
            None => score += 0,
        }
        let v_symmetry = self.find_vertical_symmetry();
        match v_symmetry {
            Some((left, _right)) => score += left as u64 + 1,
            None => score += 0,
        }

        score
    }
}

fn parse_mirrors(input: Vec<String>) -> Vec<Mirror> {
    let mut mirrors = Vec::new();
    let mut mirror = Vec::new();
    for line in input {
        if line.is_empty() {
            mirrors.push(Mirror::new(mirror));
            mirror = Vec::new();
        } else {
            mirror.push(line);
        }
    }
    mirrors.push(Mirror::new(mirror));

    mirrors
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2023::read_lines;

    #[test]
    fn test_parse_mirrors() {
        let input = read_lines("src/bin/day13/test_input.txt").unwrap();

        let mirrors = parse_mirrors(input);

        assert_eq!(mirrors.len(), 2);
        assert_eq!(mirrors[0].lines.len(), 7);
        assert_eq!(mirrors[1].lines.len(), 7);
        assert_eq!(mirrors[0].lines[0].len(), 9);
        assert_eq!(mirrors[1].lines[0].len(), 9);
    }

    #[test]
    fn test_find_symmetry() {
        let input = read_lines("src/bin/day13/test_input.txt").unwrap();
        let mirrors = parse_mirrors(input);

        let h_symmetry = mirrors[0].find_horizontal_symmetry();
        assert_eq!(h_symmetry, None);

        let v_symmetry = mirrors[0].find_vertical_symmetry();
        assert_eq!(v_symmetry, Some((4, 5)));

        let h_symmetry = mirrors[1].find_horizontal_symmetry();
        assert_eq!(h_symmetry, Some((3, 4)));

        let v_symmetry = mirrors[1].find_vertical_symmetry();
        assert_eq!(v_symmetry, None);
    }

    #[test]
    fn test_score_reflections() {
        let input = read_lines("src/bin/day13/test_input.txt").unwrap();
        let mirrors = parse_mirrors(input);

        let score = mirrors[0].score_reflection();
        assert_eq!(score, 5);

        let score = mirrors[1].score_reflection();
        assert_eq!(score, 400);
    }

    #[test]
    fn test_part_1() {
        let input = read_lines("src/bin/day13/input.txt").unwrap();
        let mirrors = parse_mirrors(input);
        for mirror in &mirrors {
            println!("{:?}", mirror.score_reflection());
        }
    }
}
