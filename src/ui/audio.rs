use std::time::{Duration, SystemTime};

use anyhow::{Context, Result};
use sdl2::{
    audio::{AudioCallback, AudioDevice, AudioSpecDesired},
    Sdl,
};

pub(crate) struct Audio {
    device: AudioDevice<SquareWave>,
    next_pause: Option<SystemTime>,
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

        Ok(Self {
            device: audio_device,
            next_pause: None,
        })
    }

    pub(crate) fn run(&mut self, command: AudioCommand) {
        match command {
            AudioCommand::Resume => self.resume(),
            AudioCommand::TryPause => self.try_pause(),
        }
    }

    fn resume(&mut self) {
        self.device.resume();
        self.next_pause = Some(SystemTime::now() + Duration::from_secs(1));
    }

    fn try_pause(&mut self) {
        if self.can_pause() {
            self.device.pause();
            self.next_pause = None;
        }
    }

    fn can_pause(&self) -> bool {
        match self.next_pause {
            Some(next_stop) => SystemTime::now() >= next_stop,
            None => false,
        }
    }
}

pub(crate) enum AudioCommand {
    Resume,
    TryPause,
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
