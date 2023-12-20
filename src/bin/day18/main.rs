// --- Day 18: Lavaduct Lagoon ---

use itertools::Itertools;
use advent_of_code_2023::read_lines;

fn main() {
    let time_start = std::time::Instant::now();
    let sum = part_1();
    println!("Part 1: {:?}, Time: {}μs", sum, time_start.elapsed().as_micros()
    );
}

fn part_1() -> usize {
    let instructions = read_lines("src/bin/day18/input.txt").unwrap();
    let mut map = map_from_instructions(instructions);
    dig_out(&mut map)
}

#[derive(Debug, PartialEq)]
struct Color(u8, u8, u8);

impl Color {
    fn from_hex(hex: &str) -> Color {
        let r = u8::from_str_radix(&hex[1..3], 16).unwrap();
        let g = u8::from_str_radix(&hex[3..5], 16).unwrap();
        let b = u8::from_str_radix(&hex[5..7], 16).unwrap();

        Color(r, g, b)
    }
}

fn map_from_instructions(instructions: Vec<String>) -> Vec<Vec<Option<Color>>> {
    let mut cur_pos = (0, 0);
    // parse the instructions into a vector of (position, color) tuples
    // the position is the coordinates of the cube relative to the starting position (0,0)
    let cubes: Vec<_> = instructions
        .iter()
        .flat_map(|i| {
            let (direction, distance, color_hex) = i.split(" ").collect_tuple().unwrap();
            let distance = distance.parse::<i16>().unwrap();
            let color_hex = color_hex.to_string().trim_matches(|c| c == '(' || c == ')').to_string();
            match direction {
                "U" => (0..distance).map(|_| {
                    cur_pos.0 += 1;
                    (cur_pos, Color::from_hex(&color_hex))
                }).collect::<Vec<_>>(),
                "D" => (0..distance).map(|_| {
                    cur_pos.0 -= 1;
                    (cur_pos, Color::from_hex(&color_hex))
                }).collect::<Vec<_>>(),
                "R" => (0..distance).map(|_| {
                    cur_pos.1 += 1;
                    (cur_pos, Color::from_hex(&color_hex))
                }).collect::<Vec<_>>(),
                "L" => (0..distance).map(|_| {
                    cur_pos.1 -= 1;
                    (cur_pos, Color::from_hex(&color_hex))
                }).collect::<Vec<_>>(),
                _ => vec![],
            }
        }).collect();
    // find the extreme coordinates of the cubes
    let ((min_x, min_y), (max_x, max_y)) = cubes.iter().fold(
        ((isize::MAX, isize::MAX), (isize::MIN, isize::MIN)),
        |((min_x, min_y), (max_x, max_y)), ((x, y), _)| {
            (
                (min_x.min(*x), min_y.min(*y)),
                (max_x.max(*x), max_y.max(*y))
            )
        },
    );
    // and build up the vector of vectors using the difference between the extreme coordinates as the size
    let mut map = (0..(max_x - min_x + 1))
        .map(|_| (0..(max_y - min_y + 1)).map(|_| None).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    // we need to move the cubes to the positive quadrant
    let offset_x = if min_x < 0 { min_x.abs() } else { 0 };
    let offset_y = if min_y < 0 { min_y.abs() } else { 0 };
    // and fill out the map with the cubes taking into account the offset
    for ((x, y), color) in cubes {
        map[(x + offset_x) as usize][(y + offset_y) as usize] = Some(color);
    }
    map
}

fn print_map(map: &Vec<Vec<Option<Color>>>) {
    for row in map.iter().rev() {
        for cell in row.iter() {
            match cell {
                Some(Color(_r, _g, _b)) => print!("#"),
                //Some(Color(r, g, b)) => print!("\x1b[48;2;{};{};{}m  \x1b[0m", r, g, b),
                None => print!("."),
            }
        }
        println!();
    }
}

fn dig_out(map: &mut Vec<Vec<Option<Color>>>) -> usize {
    let mut total_cells = 0;
    for row in map.iter() {
        let mut counting = false;

        for i in 0..row.len() {
            let cell = &row[i];
            match cell {
                Some(_) => {                    // Start of a new segment - first wall encountered
                    counting = true;
                    total_cells += 1;
                }
                None if counting => {
                    if i == row.len() - 1 || row[i + 1].is_none() {
                         counting = false;
                    } else {
                        total_cells += 1;
                    }
                }
                _ => {}
            }
        }
    }

    println!("Total cells in the area: {}", total_cells);
    total_cells
}

#[cfg(test)]
mod tests {
    use advent_of_code_2023::read_lines;
    use super::*;

    #[test]
    fn test_map_from_instructions() {
        let instructions = vec![
            "U 2 (#FF0000)".to_string(),
            "R 2 (#00FF00)".to_string(),
            "D 2 (#0000FF)".to_string(),
            "L 2 (#FFFFFF)".to_string(),
        ];
        let map = map_from_instructions(instructions);
        assert_eq!(map.len(), 3);
        assert_eq!(map[0].len(), 3);
        assert_eq!(map[1].len(), 3);
        assert_eq!(map[2].len(), 3);
        assert_eq!(map[0][0], Some(Color(255, 255, 255)));
        assert_eq!(map[0][1], Some(Color(255, 255, 255)));
        assert_eq!(map[0][2], Some(Color(0, 0, 255)));
        assert_eq!(map[1][0], Some(Color(255, 0, 0)));
        assert_eq!(map[1][1], None);
        assert_eq!(map[1][2], Some(Color(0, 0, 255)));
        assert_eq!(map[2][0], Some(Color(255, 0, 0)));
        assert_eq!(map[2][1], Some(Color(0, 255, 0)));
        assert_eq!(map[2][2], Some(Color(0, 255, 0)));
    }

    #[test]
    fn test_total_cubes() {
        let instructions = vec![
            "U 2 (#FF0000)".to_string(),
            "R 2 (#00FF00)".to_string(),
            "D 2 (#0000FF)".to_string(),
            "L 2 (#FFFFFF)".to_string(),
        ];
        let mut map = map_from_instructions(instructions);
        assert_eq!(dig_out(&mut map), 9);
    }

    #[test]
    fn test_map_from_input_instructions() {
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
        let map = map_from_instructions(instructions);
        assert_eq!(map.len(), 10);
        assert_eq!(map[0].len(), 7);
        assert_eq!(map[0][0], None);
        assert_eq!(map[1][0], None);
        assert_eq!(map[2][0], Some(Color::from_hex("#1b58a2")));
        assert_eq!(map[3][0], Some(Color::from_hex("#caa171")));
        assert_eq!(map[4][0], Some(Color::from_hex("#caa171")));
        assert_eq!(map[5][0], None);
        assert_eq!(map[6][0], None);
        assert_eq!(map[7][0], Some(Color::from_hex("#015232")));
        assert_eq!(map[8][0], Some(Color::from_hex("#7a21e3")));
        assert_eq!(map[9][0], Some(Color::from_hex("#7a21e3")));
        assert_eq!(map[0][1], Some(Color::from_hex("#8ceee2")));
        assert_eq!(map[1][1], Some(Color::from_hex("#caa173")));
        assert_eq!(map[2][1], Some(Color::from_hex("#caa173")));
        assert_eq!(map[3][1], None);
        assert_eq!(map[4][1], Some(Color::from_hex("#7807d2")));
        assert_eq!(map[5][1], None);
        assert_eq!(map[6][1], None);
        assert_eq!(map[7][1], Some(Color::from_hex("#015232")));
        assert_eq!(map[8][1], None);
        assert_eq!(map[9][1], Some(Color::from_hex("#70c710")));
        // find the number of cubes that are not None
        let num_cubes = map.iter().fold(0, |acc, row| {
            acc + row.iter().fold(0, |acc, cube| {
                if cube.is_some() {
                    acc + 1
                } else {
                    acc
                }
            })
        });
        assert_eq!(num_cubes, 38);
    }

    #[test]
    fn test_total_cubes_from_input() {
        let instructions = read_lines("src/bin/day18/test_input.txt").unwrap();
        let mut map = map_from_instructions(instructions);
        assert_eq!(dig_out(&mut map), 62);
    }
}
