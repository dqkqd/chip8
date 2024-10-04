use anyhow::{Context, Result};
use sdl2::{event::Event, keyboard::Keycode, EventPump, Sdl};

use super::Keymap;

pub(crate) enum PollResult {
    Stop,
    Keymap(Keymap),
}

pub(crate) struct Input {
    event_pump: EventPump,
}

impl Input {
    pub(crate) fn new(sdl_context: &Sdl) -> Result<Input> {
        let event_pump = sdl_context
            .event_pump()
            .ok()
            .context("Cannot open event pump")?;

        Ok(Input { event_pump })
    }

    pub(crate) fn poll(&mut self) -> PollResult {
        for event in self.event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                return PollResult::Stop;
            }
        }
        let keys = self
            .event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect::<Vec<Keycode>>();

        if keys.iter().any(|code| code == &Keycode::Escape) {
            return PollResult::Stop;
        }

        let mut keymap = Keymap::default();

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
            .for_each(|key_id| keymap.set(key_id));

        PollResult::Keymap(keymap)
    }
}
