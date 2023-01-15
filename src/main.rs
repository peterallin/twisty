use anyhow::Result;
use clap::Parser;
use twisty_maze::{sidewinder, binary_tree};

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
