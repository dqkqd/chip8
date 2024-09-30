use anyhow::Result;
use chip8::Chip8;

fn main() -> Result<()> {
    let mut chip8 = Chip8::load("tests/test_opcode.ch8")?;
    chip8.run()?;
    Ok(())
}
