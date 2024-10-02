use std::path::PathBuf;

use anyhow::Result;
use chip8::Chip8;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    file: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let mut chip8 = Chip8::load(args.file)?;
    chip8.run()?;
    Ok(())
}
