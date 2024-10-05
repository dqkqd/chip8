use anyhow::{Context, Result};

use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window, Sdl};

use crate::chip8::graphic::{HEIGHT, WIDTH};

const SCALE: usize = 10;

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
            .window("chip8", (WIDTH * SCALE) as u32, (HEIGHT * SCALE) as u32)
            .position_centered()
            .build()?;

        Ok(Display {
            canvas: window.into_canvas().present_vsync().build()?,
        })
    }

    pub(crate) fn render(&mut self, point_locations: &[(usize, usize)]) -> Result<()> {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        let rects = point_locations
            .iter()
            .map(|(x, y)| {
                Rect::new(
                    (x * SCALE) as i32,
                    (y * SCALE) as i32,
                    SCALE as u32,
                    SCALE as u32,
                )
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
