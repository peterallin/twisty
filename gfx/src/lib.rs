use anyhow::{anyhow, Result};
use sdl2::{event::Event, render::Canvas, video::Window};
use twisty_maze::Grid;

pub fn run(grid: Grid) -> Result<()> {
    let context = sdl2::init().map_err(|e| anyhow!(e))?;
    let video = context.video().map_err(|e| anyhow!(e))?;
    let window = video.window("Twisty", 1600, 1600).build()?;
    let mut canvas = window.into_canvas().accelerated().present_vsync().build()?;
    let mut keep_running = true;

    while keep_running {
        for event in context.event_pump().map_err(|e| anyhow!(e))?.poll_iter() {
            #[allow(clippy::single_match)] // I am expecting more events to be handled later
            match event {
                Event::Quit { .. } => {
                    keep_running = false;
                }
                _ => {}
            }
        }

        canvas.set_draw_color((0, 0, 0));
        canvas.clear();
        draw_maze(&mut canvas, &grid).map_err(|e| anyhow!(e))?;
        canvas.present();
    }
    Ok(())
}

fn draw_maze(canvas: &mut Canvas<Window>, grid: &Grid) -> Result<(), String> {
    let size = canvas.output_size().unwrap();
    let canvas_width = size.0 as i32;
    let canvas_height = size.1 as i32;

    let percentage_use = 90;
    let width = percentage_use * canvas_width / 100;
    let height = percentage_use * canvas_height / 100;

    let cell_width = width / grid.column_count() as i32;
    let cell_height = height / grid.row_count() as i32;
    let cell_size = std::cmp::min(cell_width, cell_height);

    let width = grid.row_count() as i32 * cell_size;
    let height = grid.column_count() as i32 * cell_size;

    let left = (canvas_width - width) / 2;
    let top = (canvas_height - height) / 2;


    canvas.set_draw_color((20, 20, 20));
    canvas.fill_rect(sdl2::rect::Rect::new(
        left,
        top,
        width as u32,
        height as u32,
    ))?;

    canvas.set_draw_color((127, 127, 127));

    for (row_number, row) in grid.rows().enumerate() {
        for (column_number, cell) in row.iter().enumerate() {
            let cell_left = left + (column_number as i32 * cell_size);
            let cell_top = top + (row_number as i32 * cell_size);
            if cell.has_south_wall() {
                canvas.draw_line(
                    (cell_left, cell_top + cell_size),
                    (cell_left + cell_size, cell_top + cell_size),
                )?;
            }
            if cell.has_east_wall() {
                canvas.draw_line(
                    (cell_left + cell_size, cell_top),
                    (cell_left + cell_size, cell_top + cell_size),
                )?;
            }
        }
    }
    canvas.draw_line((left, top), (left + width, top))?;
    canvas.draw_line((left, top), (left, top + height))?;

    Ok(())
}
