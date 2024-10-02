use std::time::Duration;

use anyhow::{Context, Result};

use crate::{opcode::Opcode, ui::UI};

pub struct Chip8 {
    memory: [u8; 4096],
    v: [u8; 16],
    pc: u16,
    i: u16,
    gfx: [u8; 64 * 32],
    stack: Vec<u16>,
    ui: UI,
    should_rerender: bool,
    end_of_mem: usize,
}

impl Chip8 {
    pub fn load<P: AsRef<std::path::Path>>(rom: P) -> Result<Chip8> {
        let mut chip8 = Chip8 {
            memory: [0; 4096],
            v: Default::default(),
            pc: 0x200,
            i: 0,
            gfx: [0; 64 * 32],
            stack: Vec::with_capacity(16),
            ui: UI::new()?,
            should_rerender: false,
            end_of_mem: 0,
        };

        let content = std::fs::read(rom.as_ref())?;
        let start = 0x200;
        chip8.end_of_mem = start + content.len();
        chip8.memory[start..chip8.end_of_mem].clone_from_slice(&content);

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
        dbg!(&opcode);
        match opcode {
            Opcode::ClearScreen => self.ui.clear_screen(),
            Opcode::Return => {
                self.pc = self.stack.pop().context("Invalid stack pointer")?;
            }
            Opcode::Jump { addr } => {
                self.stack.push(self.pc);
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
                self.v[0xF] = 0;
                let res = (self.v[x] as u16) + (self.v[y] as u16);
                if res > 0xFF {
                    self.v[0xF] = 1;
                }
                self.v[x] = (res & 0xF) as u8;
            }
            Opcode::AssignSub { x, y } => {
                if self.v[x] >= self.v[y] {
                    self.v[0xF] = 1;
                    self.v[x] -= self.v[y];
                } else {
                    self.v[0xF] = 0;
                    self.v[x] = (((self.v[x] as u16) << 1) - self.v[y] as u16) as u8;
                }
            }
            Opcode::AssignShift { x } => {
                self.v[0xF] = self.v[x] & 1;
                self.v[x] >>= 1;
            }
            Opcode::AssignRevShift { x } => {
                self.v[0xF] = self.v[x] & 0x80;
                self.v[x] = (((self.v[x] as u16) << 1) & 0xF) as u8;
            }
            Opcode::SkipIfNotEqual { x, y } => {
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            }
            Opcode::AssignRegister { addr } => {
                self.i = addr;
            }
            Opcode::Draw { x, y, height } => {
                let vx = self.v[x] as usize;
                let vy = self.v[y] as usize;

                self.v[0xF] = 0;

                for dy in 0..height {
                    let pixel = self.memory[(self.i + dy as u16) as usize];
                    for dx in 0..8 {
                        if pixel & (0x80 >> dx) != 0 {
                            let offset = vx + dx + (vy + dy as usize) * 64;
                            let bit = &mut self.gfx[offset];
                            if bit == &1 {
                                self.v[0xF] = 1;
                            }
                            *bit ^= 1;
                        }
                    }
                }

                self.should_rerender = true;
            }
            Opcode::SkipIfNotPress { x } => {
                let vx = self.v[x];
                let keys_state = self.ui.consume_keys();
                if !keys_state[vx as usize] {
                    self.pc += 2;
                }
            }
            Opcode::RegAssignAdd { x } => {
                let vx = self.v[x];
                self.i += vx as u16;
            }
            Opcode::BinaryCodedDecimal { x } => {
                let vx = self.v[x];
                self.memory[self.i as usize] = vx / 100;
                self.memory[self.i as usize + 1] = (vx / 10) % 10;
                self.memory[self.i as usize + 2] = vx % 10;
            }
            Opcode::RegDump { x } => {
                for offset in 0..=x {
                    self.v[offset] = self.memory[self.i as usize + offset];
                }
                self.i += x as u16 + 1;
            }
            Opcode::RegLoad { x } => {
                for offset in 0..=x {
                    self.memory[self.i as usize + offset] = self.v[offset];
                }
                self.i += x as u16 + 1;
            }
            _ => unimplemented!(),
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        self.ui.init();

        loop {
            self.execute()?;
            self.ui.poll_events();

            if self.ui.should_stop {
                break;
            }

            if self.should_rerender {
                self.ui.render(&self.gfx)?;
                self.should_rerender = false;
            }

            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }
}
