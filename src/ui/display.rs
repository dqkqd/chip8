use anyhow::{Context, Result};

use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window, Sdl};

pub(crate) struct Display {
    canvas: Canvas<Window>,
}

impl Display {
    pub(crate) fn new(sdl_context: &Sdl) -> Result<Display> {
        let video_subsystem = sdl_context
            .video()
            .ok()
            .context("Cannot open video system")?;

        let window = video_subsystem
            .window("chip8", 640, 320)
            .position_centered()
            .build()?;

        Ok(Display {
            canvas: window.into_canvas().present_vsync().build()?,
        })
    }

    pub(crate) fn render(&mut self, gfx: &[[u8; 64]; 32]) -> Result<()> {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for (y, row) in gfx.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                if pixel == &1 {
                    self.canvas
                        .fill_rect(Rect::new((x as i32) * 10, (y as i32) * 10, 10, 10))
                        .ok()
                        .context("Cannot draw rect")?;
                };
            }
        }
        self.canvas.present();
        Ok(())
    }
}
