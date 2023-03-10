use std::fmt::Display;

use crate::{cell::Configuration, MazeCell};
use itertools::Itertools;

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

    pub fn row_count(&self) -> usize {
        self.rows().count()
    }

    pub fn column_count(&self) -> usize {
        self.rows().next().unwrap().len()
    }

    pub fn get(&self, row: i32, column: i32) -> &MazeCell {
        &self.cells[row as usize][column as usize]
    }

    pub fn get_by_id(&self, id: crate::cell::Id) -> &MazeCell {
        &self.cells[id.row as usize][id.col as usize]
    }

    pub fn link(&mut self, (row1, col1): (i32, i32), (row2, col2): (i32, i32)) {
        let mut c1 = std::mem::take(&mut self.cells[row1 as usize][col1 as usize]);
        let mut c2 = std::mem::take(&mut self.cells[row2 as usize][col2 as usize]);
        c1.link(&mut c2);
        self.cells[row1 as usize][col1 as usize] = c1;
        self.cells[row2 as usize][col2 as usize] = c2;
    }

    pub fn link_by_id(&mut self, id1: crate::cell::Id, id2: crate::cell::Id) {
        let cell1 = self.cells[id1.row as usize][id1.col as usize].id();
        let cell2 = self.cells[id2.row as usize][id2.col as usize].id();
        self.link((cell1.row, cell1.col), (cell2.row, cell2.col));
    }

    pub fn get_mut(&mut self, row: i32, column: i32) -> &mut MazeCell {
        &mut self.cells[row as usize][column as usize]
    }

    pub fn rows(&self) -> impl Iterator<Item = &Vec<MazeCell>> {
        self.cells.iter()
    }

    pub fn cells(&self) -> impl Iterator<Item = &MazeCell> {
        self.cells.iter().flatten()
    }

    pub fn ids(&self) -> Vec<crate::cell::Id> {
        self.cells().map(|c| c.id()).collect()
    }

    pub fn ids_by_rows(&self) -> Vec<Vec<crate::cell::Id>> {
        self.cells
            .iter()
            .map(|r| r.iter().map(|c| c.id()).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result: String =
            Itertools::intersperse((0..self.column_count()).map(|_| "+---"), "").collect();
        result.push_str("+\n");

        for row in self.rows() {
            let mut line1 = "|".to_string();
            let mut line2 = "+".to_string();
            for cell in row {
                line1 += if cell.has_east_wall() { "   |" } else { "    " };
                line2 += if cell.has_south_wall() {
                    "---+"
                } else {
                    "    "
                };
            }
            result.push_str(&line1);
            result.push('\n');
            result.push_str(&line2);
            result.push('\n');
        }
        write!(f, "{}", result)
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

    #[test]
    fn can_link_by_ids() {
        let mut grid = Grid::new(1, 3);
        let row = grid.rows().next().unwrap();
        let cell0 = row[0].id();
        let cell1 = row[1].id();
        let cell2 = row[2].id();
        grid.link_by_id(cell0, cell1);
        grid.link_by_id(cell1, cell2);

        let row = grid.rows().next().unwrap();
        assert!(row[0].is_linked(&row[1].id()));
        assert!(row[1].is_linked(&row[2].id()));
        assert!(!row[0].is_linked(&row[2].id()));
    }

    #[test]
    fn can_get_ids_by_row() {
        let grid = Grid::new(3, 4);
        let ids_in_rows = grid.ids_by_rows();
        assert_eq!(3, ids_in_rows.len());
        for row in &ids_in_rows {
            assert_eq!(4, row.len());
        }
        assert_eq!(ids_in_rows[0][0], grid.get(0, 0).id());
        assert_eq!(ids_in_rows[0][1], grid.get(0, 1).id());
        assert_eq!(ids_in_rows[0][2], grid.get(0, 2).id());
        assert_eq!(ids_in_rows[0][3], grid.get(0, 3).id());
        assert_eq!(ids_in_rows[1][0], grid.get(1, 0).id());
        assert_eq!(ids_in_rows[1][1], grid.get(1, 1).id());
        assert_eq!(ids_in_rows[1][2], grid.get(1, 2).id());
        assert_eq!(ids_in_rows[1][3], grid.get(1, 3).id());
        assert_eq!(ids_in_rows[2][0], grid.get(2, 0).id());
        assert_eq!(ids_in_rows[2][1], grid.get(2, 1).id());
        assert_eq!(ids_in_rows[2][2], grid.get(2, 2).id());
        assert_eq!(ids_in_rows[2][3], grid.get(2, 3).id());
    }
}
