use crate::Grid;

pub fn binary_tree(rows: usize, columns: usize) -> Grid {
    let mut grid = Grid::new(rows, columns);
    for id in grid.ids() {
        let north = grid.get_by_id(id).north();
        let east = grid.get_by_id(id).east();
        match (north, east) {
            (Some(north), Some(east)) => {
                if rand::random() {
                    grid.link_by_id(id, north)
                } else {
                   grid.link_by_id(id, east)
                }
            }
            (Some(n), None) => grid.link_by_id(id, n),
            (None, Some(e)) => grid.link_by_id(id, e),
            (None, None) => {}
        }
    }
    grid
}