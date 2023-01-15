use anyhow::Result;
use clap::Parser;
use twisty_maze::Grid;

fn main() -> Result<()> {
    let options = Options::parse();
    let maze = match options.algorithm {
        Algorithm::Sidewinder => sidewinder(options.rows, options.columns),
        Algorithm::BinaryTree => binary_tree(options.rows, options.columns),
    };
    twisty_gfx::run(maze)?;
    // println!("{maze}");
    Ok(())
}

#[derive(clap::Parser, Debug)]
struct Options {
    rows: usize,
    columns: usize,
    algorithm: Algorithm,
}

#[derive(Clone, clap::ValueEnum, Debug)]
enum Algorithm {
    Sidewinder,
    BinaryTree,
}

fn binary_tree(rows: usize, columns: usize) -> Grid {
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

fn sidewinder(rows: usize, columns: usize) -> Grid {
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

fn close_run(run: &mut Vec<twisty_maze::Id>, grid: &mut Grid) {
    use rand::seq::SliceRandom;
    let chosen = grid.get_by_id(*run.choose(&mut rand::thread_rng()).unwrap());
    grid.link_by_id(chosen.id(), chosen.north().unwrap());
    run.clear();
}
