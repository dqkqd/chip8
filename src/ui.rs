use anyhow::{Context, Result};
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
    EventPump,
};

pub(crate) struct UI {
    canvas: Canvas<Window>,
    event: EventPump,
    pub should_stop: bool,
    pub keys: Vec<Keycode>,
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
            should_stop: false,
            keys: Vec::new(),
        })
    }

    pub(crate) fn clear_screen(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();
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

    pub(crate) fn poll_events(&mut self) {
        for event in self.event.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    self.should_stop = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.should_stop = true;
                }
                Event::KeyDown { keycode, .. } => self.keys.extend(keycode),
                _ => {}
            }
        }
    }

    pub(crate) fn consume_keys(&mut self) -> [bool; 16] {
        let mut pressed = [false; 16];

        let keys = std::mem::take(&mut self.keys);
        keys.into_iter()
            .filter_map(|key| match key {
                Keycode::Num1 => Some(0),
                Keycode::Num2 => Some(1),
                Keycode::Num3 => Some(2),
                Keycode::Num4 => Some(3),
                Keycode::Q => Some(4),
                Keycode::W => Some(5),
                Keycode::E => Some(6),
                Keycode::R => Some(7),
                Keycode::A => Some(8),
                Keycode::S => Some(9),
                Keycode::D => Some(10),
                Keycode::F => Some(11),
                Keycode::Z => Some(12),
                Keycode::X => Some(13),
                Keycode::C => Some(14),
                Keycode::V => Some(15),
                _ => None,
            })
            .for_each(|id| pressed[id] = true);

        pressed
    }
}
