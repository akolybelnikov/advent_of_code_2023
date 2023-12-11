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
        let empty_columns = self
            .cells[0]
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

fn main() {
    println!("Hello from day11!");
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
}
