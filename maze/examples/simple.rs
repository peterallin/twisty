use twisty_maze::Grid;

fn main() {
    let mut grid = Grid::new(5, 10);
    grid.link((0, 0), (0, 1));
    grid.link((0, 0), (1, 0));
    grid.link((0, 2), (0, 1));
    grid.link((0, 3), (0, 2));
    grid.link((0, 3), (1, 3));
    grid.link((1, 3), (1, 4));
    println!("{}", grid.to_string());
}
