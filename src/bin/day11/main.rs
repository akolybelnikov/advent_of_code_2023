// --- Day 11: Cosmic Expansion ---
#[derive(Debug, PartialEq)]
enum CellType {
    Empty,
    Galaxy,
}

type Coordinates = (i32, i32);

struct Cell {
    cell_type: CellType,
    coordinates: Coordinates,
}

impl Cell {
    fn new(cell_type: CellType, coordinates: Coordinates) -> Cell {
        Cell {
            cell_type,
            coordinates,
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

    fn insert_empty_row(&mut self, idx: usize) {
        let mut row = Vec::new();
        for x in 0..self.cells[0].len() {
            row.push(Cell::new(CellType::Empty, (x as i32, idx as i32)));
        }
        self.cells.insert(idx, row);
    }

    fn insert_empty_column(&mut self, idx: usize) {
        for (y, row) in self.cells.iter_mut().enumerate() {
            row.insert(idx, Cell::new(CellType::Empty, (idx as i32, y as i32)));
        }
    }

    fn expand_rows(&mut self) {
        let empty_rows = self
            .cells
            .iter()
            .enumerate()
            .filter(|(idx, _)| self.row_is_empty(*idx))
            .map(|(idx, _)| idx)
            .collect::<Vec<usize>>();
        let mut count = 0;
        for y in empty_rows {
            self.insert_empty_row(y + 1 + count);
            count += 1;
        }
    }

    fn expand_columns(&mut self) {
        let empty_columns = self.cells[0]
            .iter()
            .enumerate()
            .filter(|(idx, _)| self.column_is_empty(*idx))
            .map(|(idx, _)| idx)
            .collect::<Vec<usize>>();
        let mut count = 0;
        for x in empty_columns {
            self.insert_empty_column(x + 1 + count);
            count += 1;
        }
    }

    fn update_all_cells(&mut self) {
        for (y, row) in self.cells.iter_mut().enumerate() {
            for (x, c) in row.iter_mut().enumerate() {
                c.coordinates = (x as i32, y as i32);
            }
        }
    }

    fn expand(&mut self) {
        self.expand_rows();
        self.expand_columns();
    }

    fn all_galaxies(&self) -> Vec<Coordinates> {
        let mut galaxies = Vec::new();
        for row in &self.cells {
            for cell in row {
                if cell.cell_type == CellType::Galaxy {
                    galaxies.push(cell.coordinates);
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
    image.expand();
    image.update_all_cells();
    let galaxies = image.all_galaxies();
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
        assert_eq!(image.cells[0][0].coordinates, (0, 0));
        assert_eq!(image.cells[0][3].cell_type, CellType::Galaxy);
        assert_eq!(image.cells[0][3].coordinates, (3, 0));
        assert_eq!(image.cells[9][0].cell_type, CellType::Galaxy);
        assert_eq!(image.cells[9][0].coordinates, (0, 9));
    }

    #[test]
    fn test_expand_rows() {
        let lines = advent_of_code_2023::read_lines("src/bin/day11/test_input.txt").unwrap();
        let mut image = parse_image(lines);
        assert_eq!(image.cells.len(), 10);
        assert!(image.row_is_empty(3));
        assert!(image.row_is_empty(7));
        image.expand_rows();
        assert_eq!(image.cells.len(), 12);
        assert!(image.row_is_empty(3));
        assert!(image.row_is_empty(4));
        assert!(image.row_is_empty(8));
        assert!(image.row_is_empty(9));
    }

    #[test]
    fn test_expand_columns() {
        let lines = advent_of_code_2023::read_lines("src/bin/day11/test_input.txt").unwrap();
        let mut image = parse_image(lines);
        assert_eq!(image.cells[0].len(), 10);
        assert!(image.column_is_empty(2));
        assert!(image.column_is_empty(5));
        assert!(image.column_is_empty(8));
        image.expand_columns();
        assert_eq!(image.cells[0].len(), 13);
        assert!(image.column_is_empty(2));
        assert!(image.column_is_empty(3));
        assert!(image.column_is_empty(6));
        assert!(image.column_is_empty(7));
        assert!(image.column_is_empty(10));
        assert!(image.column_is_empty(11));
    }

    #[test]
    fn test_all_galaxies_after_update() {
        let lines = advent_of_code_2023::read_lines("src/bin/day11/test_input.txt").unwrap();
        let mut image = parse_image(lines);
        image.expand();
        image.update_all_cells();
        let galaxies = image.all_galaxies();
        assert_eq!(galaxies.len(), 9);
        assert!(galaxies.contains(&(4, 0)));
        assert!(galaxies.contains(&(9, 1)));
        assert!(galaxies.contains(&(0, 2)));
        assert!(galaxies.contains(&(8, 5)));
        assert!(galaxies.contains(&(1, 6)));
        assert!(galaxies.contains(&(12, 7)));
        assert!(galaxies.contains(&(9, 10)));
        assert!(galaxies.contains(&(0, 11)));
        assert!(galaxies.contains(&(5, 11)));
    }

    #[test]
    fn test_shortest_path() {
        let lines = advent_of_code_2023::read_lines("src/bin/day11/test_input.txt").unwrap();
        let mut image = parse_image(lines);
        image.expand();
        image.update_all_cells();
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
        let mut image = parse_image(lines);
        image.expand();
        image.update_all_cells();
        let galaxies = image.all_galaxies();
        assert_eq!(galaxies.len(), 449);

        fn get_unique_pairs(cell_count: usize) -> usize {
            cell_count * (cell_count - 1) / 2
        }

        assert_eq!(get_unique_pairs(galaxies.len()), 100576);
    }
}
