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

        let rects = gfx
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, pixel)| pixel == &&1)
                    .map(|(x, _)| Rect::new((x as i32) * 10, (y as i32) * 10, 10, 10))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        self.canvas
            .fill_rects(&rects)
            .ok()
            .context("Cannot draw rect")?;

        self.canvas.present();

        Ok(())
    }
}
