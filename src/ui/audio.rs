use anyhow::{Context, Result};
use sdl2::{
    audio::{AudioCallback, AudioDevice, AudioSpecDesired},
    Sdl,
};

pub(crate) struct Audio {
    audio_device: AudioDevice<SquareWave>,
}

impl Audio {
    pub(crate) fn new(sdl_context: &Sdl) -> Result<Audio> {
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

        Ok(Self { audio_device })
    }

    pub(crate) fn beep(&self) {
        self.audio_device.resume();
    }
    pub(crate) fn stop(&self) {
        self.audio_device.pause();
    }
}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
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
