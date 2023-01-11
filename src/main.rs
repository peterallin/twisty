use anyhow::{anyhow, Result};
use sdl2::event::Event;

mod maze_cell;

fn main() -> Result<()> {
    let context = sdl2::init().map_err(|e| anyhow!(e))?;
    let video = context.video().map_err(|e| anyhow!(e))?;
    let window = video.window("Twisty", 800, 600).build()?;
    let mut canvas = window.into_canvas().accelerated().present_vsync().build()?;
    let mut keep_running = true;

    canvas.clear();
    for i in 0..255 {
        if i % 7 != 0 {
            continue;
        }
        canvas.set_draw_color((255, i as u8, i / 1.8 as u8));
        canvas
            .draw_line((100, 100), (200, 200 + i as i32))
            .map_err(|e| anyhow!(e))?;
    }
    canvas.present();

    while keep_running {
        for event in context.event_pump().map_err(|e| anyhow!(e))?.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    keep_running = false;
                }
                _ => {}
            }
        }
        canvas.present();
    }
    Ok(())
}
