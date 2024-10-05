use std::time::Duration;

use anyhow::{Context, Result};

const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
use crate::{
    opcode::Opcode,
    ui::{input::PollResult, Keymap, UI},
};

use super::timer::{Timer, TimerTick};

enum WaitingKeyStatus {
    NoAction,
    Waiting { reg_index: usize, keymap: Keymap },
}

pub struct VM {
    memory: [u8; 4096],
    v: [u8; 16],
    pc: u16,
    i: u16,
    gfx: [[u8; 64]; 32],
    stack: Vec<u16>,
    ui: UI,
    should_rerender: bool,
    timer: Timer,
    keymap: Keymap,
    waiting_key_status: WaitingKeyStatus,
}

impl VM {
    pub fn load<P: AsRef<std::path::Path>>(rom: P) -> Result<VM> {
        let mut chip8 = VM {
            memory: [0; 4096],
            v: Default::default(),
            pc: 0x200,
            i: 0,
            gfx: [[0; 64]; 32],
            stack: Vec::with_capacity(16),
            ui: UI::new()?,
            should_rerender: false,
            timer: Timer::default(),
            keymap: Default::default(),
            waiting_key_status: WaitingKeyStatus::NoAction,
        };

        // load font
        chip8.memory[..80].clone_from_slice(&FONT);

        // load content
        let content = std::fs::read(rom.as_ref())?;
        let start = 0x200;
        chip8.memory[start..start + content.len()].clone_from_slice(&content);

        Ok(chip8)
    }

    fn fetch_opcode(&mut self) -> Result<Opcode> {
        let first = *self
            .memory
            .get(self.pc as usize)
            .context("Invalid memory access")? as u16;
        let second = *self
            .memory
            .get(self.pc as usize + 1)
            .context("Invalid memory access")? as u16;

        let raw = (first << 8) | second;
        let opcode = Opcode::try_from(raw)?;

        self.pc += 2;
        Ok(opcode)
    }

    fn execute(&mut self) -> Result<()> {
        let opcode = self.fetch_opcode().context("Opcode should not be None")?;
        match opcode {
            Opcode::ClearScreen => {
                for row in &mut self.gfx {
                    row.fill(0);
                }
            }
            Opcode::Return => {
                self.pc = self.stack.pop().context("Invalid stack pointer")?;
            }
            Opcode::Jump { addr } => {
                self.pc = addr;
            }
            Opcode::CallSub { addr } => {
                self.stack.push(self.pc);
                self.pc = addr;
            }
            Opcode::SkipIfEqualConst { x, nn } => {
                if self.v[x] == nn {
                    self.pc += 2;
                }
            }
            Opcode::SkipIfNotEqualConst { x, nn } => {
                if self.v[x] != nn {
                    self.pc += 2;
                }
            }
            Opcode::SkipIfEqual { x, y } => {
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
            }
            Opcode::SetConst { x, nn } => {
                self.v[x] = nn;
            }
            Opcode::AddConst { x, nn } => {
                let res = (self.v[x] as u16) + (nn as u16);
                self.v[x] = (res & 0xFF) as u8;
            }
            Opcode::Assign { x, y } => {
                self.v[x] = self.v[y];
            }
            Opcode::AssignOr { x, y } => {
                self.v[x] |= self.v[y];
            }
            Opcode::AssignAnd { x, y } => {
                self.v[x] &= self.v[y];
            }
            Opcode::AssignXor { x, y } => {
                self.v[x] ^= self.v[y];
            }
            Opcode::AssignAdd { x, y } => {
                let res = (self.v[x] as u16) + (self.v[y] as u16);
                self.v[x] = (res & 0xFF) as u8;
                if res > 0xFF {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
            }
            Opcode::AssignSub { x, y } => {
                if self.v[x] >= self.v[y] {
                    self.v[x] -= self.v[y];
                    self.v[0xF] = 1;
                } else {
                    self.v[x] = 0xFF - (self.v[y] - self.v[x] - 1);
                    self.v[0xF] = 0;
                }
            }
            Opcode::AssignShift { x } => {
                let lsb = self.v[x] & 1;
                self.v[x] >>= 1;
                self.v[0xF] = lsb;
            }
            Opcode::AssignRevSub { x, y } => {
                if self.v[y] >= self.v[x] {
                    self.v[x] = self.v[y] - self.v[x];
                    self.v[0xF] = 1;
                } else {
                    self.v[x] = 0xFF - (self.v[x] - self.v[y] - 1);
                    self.v[0xF] = 0;
                }
            }
            Opcode::AssignRevShift { x } => {
                let msb = (self.v[x] & 0x80) >> 7;
                self.v[x] = (((self.v[x] as u16) << 1) & 0xFF) as u8;
                self.v[0xF] = msb;
            }
            Opcode::SkipIfNotEqual { x, y } => {
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            }
            Opcode::RegAssign { addr } => {
                self.i = addr;
            }
            Opcode::Draw { x, y, height } => {
                let vx = self.v[x] as usize;
                let vy = self.v[y] as usize;

                let vx = vx % 64;
                let vy = vy % 32;

                self.v[0xF] = 0;

                for dy in 0..height {
                    let pixel = self.memory[(self.i + dy as u16) as usize];
                    let y = vy + dy as usize;
                    if y >= 32 {
                        break;
                    }

                    for dx in 0..8 {
                        let x = vx + dx;
                        if x >= 64 {
                            break;
                        }

                        if pixel & (0x80 >> dx) != 0 {
                            let bit = &mut self.gfx[y][x];
                            if bit == &1 {
                                self.v[0xF] = 1;
                            }
                            *bit ^= 1;
                        }
                    }
                }

                self.should_rerender = true;
            }
            Opcode::SkipIfPress { x } => {
                let vx = self.v[x];
                if self.keymap.is_down(vx as usize) {
                    self.pc += 2;
                }
            }
            Opcode::SkipIfNotPress { x } => {
                let vx = self.v[x];
                if !self.keymap.is_down(vx as usize) {
                    self.pc += 2;
                }
            }
            Opcode::AssignDelayTimer { x } => {
                self.v[x] = self.timer.delay;
            }
            Opcode::AssignKey { x } => {
                self.waiting_key_status = WaitingKeyStatus::Waiting {
                    reg_index: x,
                    keymap: Keymap::default(),
                };
            }
            Opcode::DelayTimerAssign { x } => {
                self.timer.delay = self.v[x];
            }
            Opcode::SoundTimerAssign { x } => {
                self.timer.sound = self.v[x];
            }
            Opcode::RegAssignAdd { x } => {
                self.i += self.v[x] as u16;
            }
            Opcode::RegAssignFont { x } => {
                let vx = self.v[x];
                debug_assert!(vx < 16);
                self.i = (vx * 5) as u16;
            }
            Opcode::BinaryCodedDecimal { x } => {
                let vx = self.v[x];
                self.memory[self.i as usize] = vx / 100;
                self.memory[self.i as usize + 1] = (vx / 10) % 10;
                self.memory[self.i as usize + 2] = vx % 10;
            }
            Opcode::RegDump { x } => {
                for offset in 0..=x {
                    self.memory[self.i as usize + offset] = self.v[offset];
                }
                self.i += x as u16 + 1;
            }
            Opcode::RegLoad { x } => {
                for offset in 0..=x {
                    self.v[offset] = self.memory[self.i as usize + offset];
                }
                self.i += x as u16 + 1;
            }
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            match self.ui.input.poll() {
                PollResult::Stop => {
                    break;
                }
                PollResult::Keymap(keymap) => self.keymap = keymap,
            };

            let wait_key_done = self.wait_key();
            if wait_key_done {
                self.execute()?;
            }

            if self.should_rerender {
                self.ui.display.render(&self.gfx)?;
                self.should_rerender = false;
            }

            if matches!(self.timer.tick(), TimerTick::SoundTimerZero) {
                // self.ui.audio.beep();
            }

            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 1000));
        }

        Ok(())
    }

    fn wait_key(&mut self) -> bool {
        if let WaitingKeyStatus::Waiting { reg_index, keymap } = &mut self.waiting_key_status {
            if let Some(key_id) = keymap.down_to_up(&self.keymap) {
                self.v[*reg_index] = key_id as u8;
                self.waiting_key_status = WaitingKeyStatus::NoAction;
            } else {
                *keymap = keymap.or(&self.keymap);
            }
        }

        matches!(self.waiting_key_status, WaitingKeyStatus::NoAction)
    }
}
