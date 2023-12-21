// --- Day 18: Lavaduct Lagoon ---

use advent_of_code_2023::read_lines;
use itertools::Itertools;

fn main() {
    let time_start = std::time::Instant::now();
    let area = part_1("src/bin/day18/input.txt");
    println!(
        "Part 1: {:?} , Time: {}μs",
        area,
        time_start.elapsed().as_micros()
    );

    let time_start = std::time::Instant::now();
    let area = part_2("src/bin/day18/input.txt");
    println!(
        "Part 2: {:?} , Time: {}μs",
        area,
        time_start.elapsed().as_micros()
    );
}

fn part_1(filename: &str) -> i64 {
    let instructions = read_lines(filename).unwrap();
    let points = collect_points_part_1(instructions);
    let area = get_area(points.0);
    // the points in the grid are in the middle of the squares, so we cannot use the area formula directly,
    // because we would be losing 1/2 of the area of the squares
    // we use Pick's theorem instead: A = I + B/2 - 1
    // where A is the area of the grid, I is the number of points inside the grid, and B is the number of points on the boundary
    area - points.1 / 2 + 1 + points.1
}

fn part_2(filename: &str) -> i64 {
    let instructions = read_lines(filename).unwrap();
    let points = collect_points_part_2(instructions);
    let area = get_area(points.0);
    area - points.1 / 2 + 1 + points.1
}

fn collect_points_part_1(instructions: Vec<String>) -> (Vec<(i64, i64)>, i64) {
    let mut points = vec![(0, 0)];
    let mut boundary_points = 0;
    instructions.iter().for_each(|i| {
        let (direction, distance, _color_hex) = i.split(" ").collect_tuple().unwrap();
        let distance = distance.parse::<i64>().unwrap();
        boundary_points += distance;
        let (x, y) = points.last().unwrap();
        match direction {
            "U" => points.push((x - 1 * distance, *y)),
            "D" => points.push((x + 1 * distance, *y)),
            "R" => points.push((*x, y + 1 * distance)),
            "L" => points.push((*x, y - 1 * distance)),
            _ => {}
        }
    });
    (points, boundary_points)
}

fn collect_points_part_2(instructions: Vec<String>) -> (Vec<(i64, i64)>, i64) {
    let mut points = vec![(0, 0)];
    let mut boundary_points = 0;
    instructions.iter().for_each(|i| {
        let (_direction, _distance, color_hex) = i.split(" ").collect_tuple().unwrap();
        let hex = color_hex
            .to_string()
            .trim_matches(|c| c == '(' || c == '#' || c == ')')
            .to_string();

        let (direction, distance) = get_instruction_from_hex(&hex);
        boundary_points += distance;

        let (x, y) = points.last().unwrap();
        match direction {
            '3' => points.push((x - 1 * distance, *y)),
            '1' => points.push((x + 1 * distance, *y)),
            '0' => points.push((*x, y + 1 * distance)),
            '2' => points.push((*x, y - 1 * distance)),
            _ => {}
        }
    });
    (points, boundary_points)
}

fn get_instruction_from_hex(hex: &str) -> (char, i64) {
    let trimmed_instruction = hex.chars().collect::<Vec<char>>();
    let hex_distance: String = trimmed_instruction[0..5].into_iter().collect();
    let distance = i64::from_str_radix(&hex_distance, 16).unwrap();
    (trimmed_instruction[5], distance)
}

fn get_area(points: Vec<(i64, i64)>) -> i64 {
    let a = points
        .iter()
        .enumerate()
        .map(|(i, (x, _y))| {
            let prev_idx = if i == 0 { points.len() - 1 } else { i - 1 };
            let next_idx = if i == points.len() - 1 { 0 } else { i + 1 };
            // for each point we take the product of the x coordinate and the difference between
            // the y coordinates of the previous and next points
            x * (points[prev_idx].1 - points[next_idx].1)
        })
        .sum::<i64>();

    ((a.abs() as f64) / 2.0) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_points() {
        let instructions = vec![
            "U 2 (#FF0000)".to_string(),
            "R 2 (#00FF00)".to_string(),
            "D 2 (#0000FF)".to_string(),
            "L 2 (#FFFFFF)".to_string(),
        ];
        let points = collect_points_part_1(instructions);
        assert_eq!(points.0.len(), 5);
        assert_eq!(points.0[0], (0, 0));
        assert_eq!(points.0[1], (-2, 0));
        assert_eq!(points.0[2], (-2, 2));
        assert_eq!(points.0[3], (0, 2));
        assert_eq!(points.0[4], (0, 0));
    }

    #[test]
    fn test_get_area() {
        let instructions = vec![
            "U 2 (#FF0000)".to_string(),
            "R 2 (#00FF00)".to_string(),
            "D 2 (#0000FF)".to_string(),
            "L 2 (#FFFFFF)".to_string(),
        ];
        let points = collect_points_part_1(instructions);
        assert_eq!(points.1, 8);
        assert_eq!(get_area(points.0), 4);
    }

    #[test]
    fn test_get_area_2() {
        let instructions = vec![
            "R 6 (#70c710)".to_string(),
            "D 5 (#0dc571)".to_string(),
            "L 2 (#5713f0)".to_string(),
            "D 2 (#d2c081)".to_string(),
            "R 2 (#59c680)".to_string(),
            "D 2 (#411b91)".to_string(),
            "L 5 (#8ceee2)".to_string(),
            "U 2 (#caa173)".to_string(),
            "L 1 (#1b58a2)".to_string(),
            "U 2 (#caa171)".to_string(),
            "R 2 (#7807d2)".to_string(),
            "U 3 (#a77fa3)".to_string(),
            "L 2 (#015232)".to_string(),
            "U 2 (#7a21e3)".to_string(),
        ];
        let points = collect_points_part_1(instructions);
        assert_eq!(points.1, 38);
        assert_eq!(get_area(points.0), 42);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/bin/day18/test_input.txt"), 62);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("src/bin/day18/test_input.txt"), 952408144115);
    }
}
