pub(super) enum TimerTick {
    Normal,
    SoundTimerZero,
}

#[derive(Default)]
pub(super) struct Timer {
    pub delay: u8,
    pub sound: u8,
}

impl Timer {
    pub fn tick(&mut self) -> TimerTick {
        if self.delay > 0 {
            self.delay -= 1;
        }

        if self.sound > 0 {
            self.sound -= 1;
            if self.sound == 0 {
                return TimerTick::SoundTimerZero;
            }
        }

        TimerTick::Normal
    }
}
