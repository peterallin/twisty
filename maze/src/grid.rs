use crate::{cell::Configuration, MazeCell};

pub struct Grid {
    cells: Vec<Vec<MazeCell>>,
}

impl Grid {
    pub fn new(row_count: usize, column_count: usize) -> Self {
        let mut cells: Vec<Vec<MazeCell>> = (0..row_count)
            .map(|row| make_row(row, column_count))
            .collect();

        for row_number in 0..row_count {
            for column_number in 0..column_count {
                let north = if row_number > 0 {
                    Some(cells[row_number - 1][column_number].id())
                } else {
                    None
                };
                let south = if row_number < row_count - 1 {
                    Some(cells[row_number + 1][column_number].id())
                } else {
                    None
                };
                let west = if column_number > 0 {
                    Some(cells[row_number][column_number - 1].id())
                } else {
                    None
                };
                let east = if column_number < column_count - 1 {
                    Some(cells[row_number][column_number + 1].id())
                } else {
                    None
                };
                let configuration = Configuration {
                    north,
                    south,
                    west,
                    east,
                };
                cells[row_number][column_number].configure(configuration);
            }
        }
        Self { cells }
    }

    pub fn get(&self, row: i32, column: i32) -> &MazeCell {
&self.cells[row as usize][column as usize]
    }

    pub fn rows(&self) -> impl Iterator<Item = &Vec<MazeCell>> {
        self.cells.iter()
    }
}

fn make_row(row: usize, column_count: usize) -> Vec<MazeCell> {
    (0..column_count)
        .map(|col| MazeCell::new(row as i32, col as i32))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn has_right_dimensions() {
        let rows = 33;
        let columns = 51;
        let grid = Grid::new(rows, columns);
        grid.rows().next();
        assert_eq!(rows, grid.rows().count());
        for row in grid.rows() {
            assert_eq!(columns, row.len());
        }
    }

    #[test]
    fn gives_neighbors_to_cells() {
        let rows = 3;
        let columns = 3;
        let grid = Grid::new(rows, columns);
        let mut rows = grid.rows();

        let top_row = rows.next().unwrap();
        assert_eq!(2, top_row[0].neighbors().len());
        assert_eq!(3, top_row[1].neighbors().len());
        assert_eq!(2, top_row[2].neighbors().len());
        assert!(top_row[0].neighbors().contains(&grid.get(0, 1).id()));
        assert!(top_row[0].neighbors().contains(&grid.get(1, 0).id()));
        assert!(top_row[1].neighbors().contains(&grid.get(0, 0).id()));
        assert!(top_row[1].neighbors().contains(&grid.get(0, 2).id()));
        assert!(top_row[1].neighbors().contains(&grid.get(1, 1).id()));

        let middle_row = rows.next().unwrap();
        assert_eq!(3, middle_row[0].neighbors().len());
        assert_eq!(4, middle_row[1].neighbors().len());
        assert_eq!(3, middle_row[2].neighbors().len());
        assert!(middle_row[0].neighbors().contains(&grid.get(0, 0).id()));
        assert!(middle_row[0].neighbors().contains(&grid.get(2, 0).id()));
        assert!(middle_row[0].neighbors().contains(&grid.get(1, 1).id()));
        assert!(middle_row[1].neighbors().contains(&grid.get(0, 1).id()));
        assert!(middle_row[1].neighbors().contains(&grid.get(2, 1).id()));
        assert!(middle_row[1].neighbors().contains(&grid.get(1, 0).id()));
        assert!(middle_row[1].neighbors().contains(&grid.get(1, 2).id()));
        assert!(middle_row[2].neighbors().contains(&grid.get(0, 2).id()));
        assert!(middle_row[2].neighbors().contains(&grid.get(2, 2).id()));
        assert!(middle_row[2].neighbors().contains(&grid.get(1, 1).id()));

        let bottom_row = rows.next().unwrap();
        assert_eq!(2, bottom_row[0].neighbors().len());
        assert_eq!(3, bottom_row[1].neighbors().len());
        assert_eq!(2, bottom_row[2].neighbors().len());
        assert!(bottom_row[0].neighbors().contains(&grid.get(1, 0).id()));
        assert!(bottom_row[0].neighbors().contains(&grid.get(2, 1).id()));
        assert!(bottom_row[1].neighbors().contains(&grid.get(2, 0).id()));
        assert!(bottom_row[1].neighbors().contains(&grid.get(2, 2).id()));
        assert!(bottom_row[1].neighbors().contains(&grid.get(1, 1).id()));
    }
}
