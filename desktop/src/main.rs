mod emulator;

use std::path::PathBuf;

use clap::Parser;

fn main() {
    env_logger::init();
    let args = Args::parse();
    let emu = emulator::Emulator::new(args);
    emu.run()
}

#[derive(Parser)]
pub struct Args {
    /// Path to a Game Boy ROM file to execute
    rom: PathBuf,
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
