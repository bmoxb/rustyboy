use crate::emulator::Emulator;

use std::path::PathBuf;

use clap::Parser;

use rustyboy_core::{cartridge::Cartridge, mbc, GameBoy};

pub async fn run() {
    env_logger::init();

    let args = Args::parse();

    let rom_path = args
        .rom
        .or_else(|| rfd::FileDialog::new().pick_file())
        .unwrap();

    let cart = Cartridge::from_file(rom_path).unwrap();
    println!("Loaded cartridge: {}", cart);

    let mbc = mbc::from_cartridge(cart).unwrap();

    let gb = GameBoy::new(mbc);

    if let Some(_path) = &args.serial_log {
        unimplemented!() // TODO
    }

    Emulator::new(gb, args.speed).await.run();
}

#[derive(Parser)]
pub struct Args {
    /// Path to a Game Boy ROM file to execute
    rom: Option<PathBuf>,
    // Speed multiplier at which to run the emulator
    #[arg(short, long, default_value = "1.0")]
    speed: f32,
    /// Write the text written to serial out by debugging/test ROMs to a given file
    #[arg(long)]
    serial_log: Option<PathBuf>,
}
