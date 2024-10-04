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
