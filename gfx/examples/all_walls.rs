use anyhow::Result;
use twisty_maze::Grid;

fn main() ->Result<()>{
    let maze = Grid::new(10, 10);
    twisty_gfx::run(maze)?;
    Ok(())
}
