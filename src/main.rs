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

fn sidewinder(_rows: usize, _columns: usize) -> Grid {
    todo!()
}
