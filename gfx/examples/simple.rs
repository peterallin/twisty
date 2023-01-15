use anyhow::Result;
use twisty_maze::Grid;

fn main() ->Result<()>{
    let maze = make_maze();
    twisty_gfx::run(maze)?;
    Ok(())
}

fn make_maze() -> Grid {
    let mut grid = Grid::new(10, 10);
    grid.link((0, 0), (0, 1));
    grid.link((0, 0), (1, 0));
    grid.link((0, 2), (0, 1));
    grid.link((0, 3), (0, 2));
    grid.link((0, 3), (1, 3));
    grid.link((1, 3), (1, 4));
    grid
}