use anyhow::{Context, Result};
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
    EventPump,
};

pub(crate) struct UI {
    canvas: Canvas<Window>,
    event: EventPump,
}

impl UI {
    pub(crate) fn new() -> Result<UI> {
        let sdl_context = sdl2::init().ok().context("Cannot open sdl")?;
        let video_subsystem = sdl_context
            .video()
            .ok()
            .context("Cannot open video system")?;

        let window = video_subsystem
            .window("chip8", 640, 320)
            .position_centered()
            .build()?;

        let event_pump = sdl_context
            .event_pump()
            .ok()
            .context("Cannot open event pump")?;

        Ok(UI {
            canvas: window.into_canvas().present_vsync().build()?,
            event: event_pump,
        })
    }

    pub(crate) fn init(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
    }

    pub(crate) fn render(&mut self, gfx: &[u8; 64 * 32]) -> Result<()> {
        for x in 0..64 {
            for y in 0..32 {
                let offset = x + y * 64;
                let bit = gfx[offset as usize];
                if bit == 1 {
                    self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                    self.canvas
                        .fill_rect(Rect::new(x * 10, y * 10, 10, 10))
                        .ok()
                        .context("Cannot draw rect")?;
                };
            }
        }
        self.canvas.present();
        Ok(())
    }

    pub(crate) fn should_stop(&mut self) -> bool {
        for event in self.event.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return true,
                _ => {}
            }
        }
        false
    }
}
