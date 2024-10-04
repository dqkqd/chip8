use anyhow::{Context, Result};
use audio::Audio;
use display::Display;
use input::Input;

pub(crate) mod audio;
pub(crate) mod display;
pub(crate) mod input;

pub(crate) struct UI {
    pub audio: Audio,
    pub display: Display,
    pub input: Input,
}

impl UI {
    pub(crate) fn new() -> Result<UI> {
        let sdl_context = sdl2::init().ok().context("Cannot open sdl")?;

        let audio = Audio::new(&sdl_context)?;
        let display = Display::new(&sdl_context)?;
        let input = Input::new(&sdl_context)?;

        Ok(UI {
            audio,
            display,
            input,
        })
    }
}

#[derive(Default)]
pub(crate) struct Keymap {
    inner: [bool; 16],
}

impl Keymap {
    pub fn set(&mut self, index: usize) {
        self.inner[index] = true;
    }
    pub fn is_down(&self, index: usize) -> bool {
        self.inner[index]
    }
    pub fn or(&self, other: &Keymap) -> Keymap {
        let inner: [bool; 16] = self
            .inner
            .iter()
            .zip(other.inner)
            .map(|(key, other_key)| key | other_key)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Keymap { inner }
    }
    pub fn down_to_up(&self, other: &Keymap) -> Option<usize> {
        self.inner
            .iter()
            .zip(other.inner)
            .enumerate()
            .find(|(_, (key, other_key))| key == &&true && other_key == &false)
            .map(|(index, _)| index)
    }
}
