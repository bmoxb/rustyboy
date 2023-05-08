use crossterm::{cursor, style, terminal, ExecutableCommand, QueueableCommand};
use rustyboy_core::{
    mbc,
    screen::{Colour, SCREEN_HEIGHT, SCREEN_WIDTH},
    GameBoy,
};

use clap::Parser;

use std::{io::Write, path::PathBuf, time::Instant};

fn main() -> crossterm::Result<()> {
    let args = Args::parse();

    let mbc = mbc::from_rom_file(&args.rom).unwrap();
    let mut gb = GameBoy::new(mbc);

    let mut last_instant = Instant::now();

    let mut stdout = std::io::stdout();

    loop {
        let delta = (Instant::now() - last_instant).as_secs_f32();
        last_instant = Instant::now();

        gb.update(delta);

        stdout.execute(terminal::Clear(terminal::ClearType::All))?;

        for x in 0..SCREEN_WIDTH as u8 {
            for y in 0..SCREEN_HEIGHT as u8 {
                let colour = match gb.screen().get(x, y) {
                    Colour::Black => style::Color::Black,
                    Colour::DarkGrey => style::Color::DarkGreen,
                    Colour::LightGrey => style::Color::Green,
                    Colour::White => style::Color::White,
                };

                stdout
                    .queue(cursor::MoveTo(x as u16, y as u16))?
                    .queue(style::SetBackgroundColor(colour))?
                    .queue(style::Print(" "))?;
            }
        }

        stdout.flush().unwrap();

        std::thread::sleep(std::time::Duration::from_secs_f32(0.1))
    }
}

#[derive(Parser)]
pub struct Args {
    /// Path to a Game Boy ROM file to execute
    rom: PathBuf,
}
