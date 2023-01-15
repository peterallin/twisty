use crate::Grid;

pub fn sidewinder(rows: usize, columns: usize) -> Grid {
    let mut grid = Grid::new(rows, columns);
    for ids in grid.ids_by_rows() {
        let mut run = vec![];
        for id in ids {
            let north = grid.get_by_id(id).north();
            let east = grid.get_by_id(id).east();
            run.push(id);
            match (north, east) {
                (Some(_n), Some(e)) => {
                    if rand::random() {
                        close_run(&mut run, &mut grid);
                    } else {
                        grid.link_by_id(id, e);
                    }
                }
                (Some(_n), None) => {
                    close_run(&mut run, &mut grid);
                }
                (None, Some(e)) => {
                    grid.link_by_id(id, e);
                }
                (None, None) => {}
            }
        }
    }
    grid
}

fn close_run(run: &mut Vec<crate::Id>, grid: &mut Grid) {
    use rand::seq::SliceRandom;
    let chosen = grid.get_by_id(*run.choose(&mut rand::thread_rng()).unwrap());
    grid.link_by_id(chosen.id(), chosen.north().unwrap());
    run.clear();
}
