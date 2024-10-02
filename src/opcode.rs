use anyhow::{bail, Error};

#[derive(Debug)]
pub(crate) enum Opcode {
    // 00E0
    ClearScreen,
    // 00EE
    Return,
    // 1NNN
    Jump { addr: u16 },
    // 2NNN
    CallSub { addr: u16 },
    // 3XNN
    SkipIfEqualConst { x: usize, nn: u8 },
    // 4XNN
    SkipIfNotEqualConst { x: usize, nn: u8 },
    // 5XY0
    SkipIfEqual { x: usize, y: usize },
    // 6XNN
    SetConst { x: usize, nn: u8 },
    // 7XNN
    AddConst { x: usize, nn: u8 },
    // 8XY0
    Assign { x: usize, y: usize },
    // 8XY1
    AssignOr { x: usize, y: usize },
    // 8XY2
    AssignAnd { x: usize, y: usize },
    // 8XY3
    AssignXor { x: usize, y: usize },
    // 8XY4
    AssignAdd { x: usize, y: usize },
    // 8XY5
    AssignSub { x: usize, y: usize },
    // 8XY6
    AssignShift { x: usize },
    // 8XY7
    AssignRevSub { x: usize, y: usize },
    // 8XYE
    AssignRevShift { x: usize },
    // 9XY0
    SkipIfNotEqual { x: usize, y: usize },
    // ANNN
    RegAssign { addr: u16 },
    // DXYN
    Draw { x: usize, y: usize, height: u8 },
    // E09E
    SkipIfPress { x: usize },
    // EXA1
    SkipIfNotPress { x: usize },
    // FX0A
    AssignKey { x: usize },
    // FX07
    AssignDelayTimer { x: usize },
    // FX15
    DelayTimerAssign { x: usize },
    // FX18
    SoundTimerAssign { x: usize },
    // FX1E
    RegAssignAdd { x: usize },
    // FX29
    RegAssignFont { x: usize },
    // FX33
    BinaryCodedDecimal { x: usize },
    // FX55
    RegDump { x: usize },
    // FX65
    RegLoad { x: usize },
}

impl TryFrom<u16> for Opcode {
    type Error = Error;

    fn try_from(raw: u16) -> Result<Opcode, Error> {
        let code = raw & 0xF000;
        let x = ((raw & 0x0F00) >> 8) as usize;
        let y = ((raw & 0x00F0) >> 4) as usize;
        let nnn = raw & 0x0FFF;
        let nn = (raw & 0x00FF) as u8;
        let n = (raw & 0x000F) as u8;

        let invalid_opcode = || bail!(format!("Invalid opcode dec={}, hex={:X}", raw, raw));

        match code {
            0x0000 => match nnn {
                0x00E0 => Ok(Opcode::ClearScreen),
                0x00EE => Ok(Opcode::Return),
                _ => invalid_opcode(),
            },
            0x1000 => Ok(Opcode::Jump { addr: nnn }),
            0x2000 => Ok(Opcode::CallSub { addr: nnn }),
            0x3000 => Ok(Opcode::SkipIfEqualConst { x, nn }),
            0x4000 => Ok(Opcode::SkipIfNotEqualConst { x, nn }),
            0x5000 => Ok(Opcode::SkipIfEqual { x, y }),
            0x6000 => Ok(Opcode::SetConst { x, nn }),
            0x7000 => Ok(Opcode::AddConst { x, nn }),
            0x8000 => match raw & 0x000F {
                0x0000 => Ok(Opcode::Assign { x, y }),
                0x0001 => Ok(Opcode::AssignOr { x, y }),
                0x0002 => Ok(Opcode::AssignAnd { x, y }),
                0x0003 => Ok(Opcode::AssignXor { x, y }),
                0x0004 => Ok(Opcode::AssignAdd { x, y }),
                0x0005 => Ok(Opcode::AssignSub { x, y }),
                0x0006 => Ok(Opcode::AssignShift { x }),
                0x000E => Ok(Opcode::AssignRevShift { x }),
                _ => invalid_opcode(),
            },
            0x9000 => Ok(Opcode::SkipIfNotEqual { x, y }),
            0xA000 => Ok(Opcode::RegAssign { addr: nnn }),
            0xD000 => Ok(Opcode::Draw { x, y, height: n }),
            0xE000 => match nn {
                0x009E => Ok(Opcode::SkipIfPress { x }),
                0x00A1 => Ok(Opcode::SkipIfNotPress { x }),
                _ => invalid_opcode(),
            },
            0xF000 => match nn {
                0x0007 => Ok(Opcode::AssignDelayTimer { x }),
                0x000A => Ok(Opcode::AssignKey { x }),
                0x0015 => Ok(Opcode::DelayTimerAssign { x }),
                0x0018 => Ok(Opcode::SoundTimerAssign { x }),
                0x001E => Ok(Opcode::RegAssignAdd { x }),
                0x0029 => Ok(Opcode::RegAssignFont { x }),
                0x0033 => Ok(Opcode::BinaryCodedDecimal { x }),
                0x0055 => Ok(Opcode::RegDump { x }),
                0x0065 => Ok(Opcode::RegLoad { x }),
                _ => invalid_opcode(),
            },
            _ => invalid_opcode(),
        }
    }
}
