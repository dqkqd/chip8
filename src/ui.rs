use std::time::{Duration, SystemTime};

use anyhow::{Context, Result};

use sdl2::{
    audio::{AudioCallback, AudioDevice, AudioSpecDesired},
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::Canvas,
    video::Window,
    EventPump,
};

pub(crate) struct UI {
    canvas: Canvas<Window>,
    event: EventPump,
    audio_device: AudioDevice<SquareWave>,
    sound_stop_at: Option<SystemTime>,
    pub should_stop: bool,
}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
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

        let audio_subsystem = sdl_context
            .audio()
            .ok()
            .context("Cannot open audio system")?;
        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1), // mono
            samples: None,     // default sample size
        };
        let audio_device = audio_subsystem
            .open_playback(None, &desired_spec, |spec| {
                // initialize the audio callback
                SquareWave {
                    phase_inc: 440.0 / spec.freq as f32,
                    phase: 0.0,
                    volume: 0.25,
                }
            })
            .expect("Cannot open audio device");

        Ok(UI {
            canvas: window.into_canvas().present_vsync().build()?,
            event: event_pump,
            audio_device,
            sound_stop_at: None,
            should_stop: false,
        })
    }

    pub(crate) fn render(&mut self, gfx: &[u8; 64 * 32]) -> Result<()> {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for x in 0..64 {
            for y in 0..32 {
                let offset = x + y * 64;
                let bit = gfx[offset as usize];
                if bit == 1 {
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

    pub(crate) fn play_sound(&mut self) {
        self.audio_device.resume();
        self.sound_stop_at = Some(SystemTime::now() + Duration::from_millis(600));
    }
    pub(crate) fn stop_sound(&mut self) {
        if let Some(ref sound_stop_at) = self.sound_stop_at {
            let now = SystemTime::now();
            if &now >= sound_stop_at {
                self.audio_device.pause();
            }
        }
    }

    pub(crate) fn poll_keys(&mut self) -> [bool; 16] {
        for event in self.event.poll_iter() {
            if let Event::Quit { .. } = event {
                self.should_stop = true;
            }
        }
        let keys = self
            .event
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect::<Vec<Keycode>>();

        if keys.iter().any(|code| code == &Keycode::Escape) {
            self.should_stop = true;
        }

        let mut keymap = [false; 16];
        keys.into_iter()
            .filter_map(|key| match key {
                Keycode::Num1 => Some(0x1),
                Keycode::Num2 => Some(0x2),
                Keycode::Num3 => Some(0x3),
                Keycode::Num4 => Some(0xC),
                Keycode::Q => Some(0x4),
                Keycode::W => Some(0x5),
                Keycode::E => Some(0x6),
                Keycode::R => Some(0xD),
                Keycode::A => Some(0x7),
                Keycode::S => Some(0x8),
                Keycode::D => Some(0x9),
                Keycode::F => Some(0xE),
                Keycode::Z => Some(0xA),
                Keycode::X => Some(0x0),
                Keycode::C => Some(0xB),
                Keycode::V => Some(0xF),
                _ => None,
            })
            .for_each(|key_id| keymap[key_id] = true);

        keymap
    }
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}
