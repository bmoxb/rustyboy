use crate::emulator::Emulator;

use std::{fs::File, path::PathBuf};

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

    let mut gb = GameBoy::new(mbc);

    if let Some(path) = &args.gb_doctor_log {
        let f = File::create(path).unwrap();
        gb.enable_gb_doctor_logging(Box::new(f));
    }

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
    /// Write Game Boy Doctor log lines to a given file - this option should be used with a very low speed setting
    /// (e.g., 0.01) so as to avoid crashes
    #[arg(long)]
    gb_doctor_log: Option<PathBuf>,
}
