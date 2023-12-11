// --- Day 11: Cosmic Expansion ---
#[derive(Debug, PartialEq)]
enum CellType {
    Empty,
    Galaxy,
}

type Coordinates = (i32, i32);

struct Cell {
    cell_type: CellType,
    original_position: Coordinates,
    updated_position: Coordinates,
}

impl Cell {
    fn new(cell_type: CellType, coordinates: Coordinates) -> Cell {
        Cell {
            cell_type,
            original_position: coordinates,
            updated_position: coordinates,
        }
    }
}

struct Image {
    cells: Vec<Vec<Cell>>,
}

impl Image {
    fn new() -> Image {
        Image { cells: Vec::new() }
    }

    fn get_mut_cell(&mut self, coordinates: Coordinates) -> &mut Cell {
        &mut self.cells[coordinates.1 as usize][coordinates.0 as usize]
    }

    fn row_is_empty(&self, idx: usize) -> bool {
        self.cells[idx]
            .iter()
            .all(|cell| cell.cell_type == CellType::Empty)
    }

    fn column_is_empty(&self, idx: usize) -> bool {
        self.cells
            .iter()
            .all(|row| row[idx].cell_type == CellType::Empty)
    }

    fn empty_rows(&self) -> Vec<usize> {
        self.cells
            .iter()
            .enumerate()
            .filter(|(idx, _)| self.row_is_empty(*idx))
            .map(|(idx, _)| idx)
            .collect::<Vec<usize>>()
    }

    fn empty_columns(&self) -> Vec<usize> {
        self.cells[0]
            .iter()
            .enumerate()
            .filter(|(idx, _)| self.column_is_empty(*idx))
            .map(|(idx, _)| idx)
            .collect::<Vec<usize>>()
    }

    fn update_galaxy_cells_y(&mut self, factor: i32) {
        let empty_rows = self.empty_rows();
        let galaxies = self.all_galaxies(false);
        for y in empty_rows {
            for galaxy in &galaxies {
                if galaxy.1 > y as i32 {
                    let cell = self.get_mut_cell(*galaxy);
                    cell.updated_position.1 += factor;
                }
            }
        }
    }

    fn update_galaxy_cells_x(&mut self, factor: i32) {
        let empty_columns = self.empty_columns();
        let galaxies = self.all_galaxies(false);
        for x in empty_columns {
            for galaxy in &galaxies {
                if galaxy.0 > x as i32 {
                    let cell = self.get_mut_cell(*galaxy);
                    cell.updated_position.0 += factor;
                }
            }
        }
    }

    fn expand(&mut self, factor: i32) {
        self.update_galaxy_cells_y(factor);
        self.update_galaxy_cells_x(factor);
    }

    fn all_galaxies(&self, updated: bool) -> Vec<Coordinates> {
        let mut galaxies = Vec::new();
        for row in &self.cells {
            for cell in row {
                if cell.cell_type == CellType::Galaxy {
                    let coordinates = if updated {
                        cell.updated_position
                    } else {
                        cell.original_position
                    };
                    galaxies.push(coordinates);
                }
            }
        }
        galaxies
    }

    fn shortest_path_manhattan(&self, start: Coordinates, end: Coordinates) -> i32 {
        (start.0 - end.0).abs() + (start.1 - end.1).abs()
    }
}

fn parse_image(lines: Vec<String>) -> Image {
    let mut image = Image::new();

    for (y, line) in lines.iter().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let cell_type = match c {
                '.' => CellType::Empty,
                '#' => CellType::Galaxy,
                _ => panic!("Unknown cell type: {}", c),
            };
            row.push(Cell::new(cell_type, (x as i32, y as i32)));
        }
        image.cells.push(row);
    }

    image
}

fn part_1(filename: &str) -> i32 {
    let lines = advent_of_code_2023::read_lines(filename).unwrap();
    let mut image = parse_image(lines);
    image.expand(1);
    let galaxies = image.all_galaxies(true);
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let start = galaxies[i];
            let end = galaxies[j];
            let shortest_path = image.shortest_path_manhattan(start, end);
            sum += shortest_path;
        }
    }

    sum
}

fn main() {
    println!("Part 1: {}", part_1("src/bin/day11/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_image() {
        let lines = advent_of_code_2023::read_lines("src/bin/day11/test_input.txt").unwrap();
        let image = parse_image(lines);
        assert_eq!(image.cells.len(), 10);
        assert_eq!(image.cells[0].len(), 10);
        assert_eq!(image.cells[0][0].cell_type, CellType::Empty);
        assert_eq!(image.cells[0][0].original_position, (0, 0));
        assert_eq!(image.cells[0][3].cell_type, CellType::Galaxy);
        assert_eq!(image.cells[0][3].original_position, (3, 0));
        assert_eq!(image.cells[9][0].cell_type, CellType::Galaxy);
        assert_eq!(image.cells[9][0].original_position, (0, 9));
    }

    #[test]
    fn test_shortest_path() {
        let lines = advent_of_code_2023::read_lines("src/bin/day11/test_input.txt").unwrap();
        let mut image = parse_image(lines);
        image.expand(1);
        let start = (1, 6);
        let end = (5, 11);
        let shortest_path = image.shortest_path_manhattan(start, end);
        assert_eq!(shortest_path, 9);

        let start = (4, 0);
        let end = (9, 10);
        let shortest_path = image.shortest_path_manhattan(start, end);
        assert_eq!(shortest_path, 15);

        let start = (0, 2);
        let end = (12, 7);
        let shortest_path = image.shortest_path_manhattan(start, end);
        assert_eq!(shortest_path, 17);

        let start = (0, 11);
        let end = (5, 11);
        let shortest_path = image.shortest_path_manhattan(start, end);
        assert_eq!(shortest_path, 5);
    }

    #[test]
    fn test_part_1() {
        let result = part_1("src/bin/day11/test_input.txt");
        assert_eq!(result, 374);
    }

    #[test]
    fn test_real_input_pairs() {
        let lines = advent_of_code_2023::read_lines("src/bin/day11/input.txt").unwrap();
        let image = parse_image(lines);
        let galaxies = image.all_galaxies(false);
        assert_eq!(galaxies.len(), 449);
        fn get_unique_pairs(cell_count: usize) -> usize {
            cell_count * (cell_count - 1) / 2
        }
        assert_eq!(get_unique_pairs(galaxies.len()), 100576);
    }

    #[test]
    fn test_update_galaxies() {
        let lines = advent_of_code_2023::read_lines("src/bin/day11/test_input.txt").unwrap();
        let mut image = parse_image(lines);
        let galaxies_before = image.all_galaxies(false);
        assert_eq!(galaxies_before.len(), 9);
        assert!(galaxies_before.contains(&(3, 0)));
        assert!(galaxies_before.contains(&(7, 1)));
        assert!(galaxies_before.contains(&(0, 2)));
        assert!(galaxies_before.contains(&(6, 4)));
        assert!(galaxies_before.contains(&(1, 5)));
        assert!(galaxies_before.contains(&(9, 6)));
        assert!(galaxies_before.contains(&(7, 8)));
        assert!(galaxies_before.contains(&(0, 9)));
        assert!(galaxies_before.contains(&(4, 9)));

        image.update_galaxy_cells_y(10);
        let galaxies = image.all_galaxies(true);
        assert_eq!(galaxies.len(), 9);
        assert!(galaxies.contains(&(3, 0)));
        assert!(galaxies.contains(&(7, 1)));
        assert!(galaxies.contains(&(0, 2)));
        assert!(galaxies.contains(&(6, 14)));
        assert!(galaxies.contains(&(1, 15)));
        assert!(galaxies.contains(&(9, 16)));
        assert!(galaxies.contains(&(7, 28)));
        assert!(galaxies.contains(&(0, 29)));
        assert!(galaxies.contains(&(4, 29)));

        image.update_galaxy_cells_x(10);
        let galaxies = image.all_galaxies(true);
        assert_eq!(galaxies.len(), 9);
        assert!(galaxies.contains(&(13, 0)));
        assert!(galaxies.contains(&(27, 1)));
        assert!(galaxies.contains(&(0, 2)));
        assert!(galaxies.contains(&(26, 14)));
        assert!(galaxies.contains(&(1, 15)));
        assert!(galaxies.contains(&(39, 16)));
        assert!(galaxies.contains(&(27, 28)));
        assert!(galaxies.contains(&(0, 29)));
        assert!(galaxies.contains(&(14, 29)));
    }
}
