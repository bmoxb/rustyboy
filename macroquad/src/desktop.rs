use std::path::PathBuf;

use rustyboy_core::mbc;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(help = "The ROM file to load and execute")]
    rom_path: PathBuf,
}

pub fn init() -> Option<Box<dyn mbc::MemoryBankController>> {
    env_logger::init();

    let args = Args::parse();

    match mbc::from_rom_file(&args.rom_path) {
        Ok(mbc) => {
            println!(
                "ROM Title: {}\nMBC ROM size: {:#X} bytes\nMBC RAM size: {:#X} bytes",
                mbc.game_title(),
                mbc.rom_size(),
                mbc.ram_size()
            );

            Some(mbc)
        }
        Err(e) => {
            eprintln!(
                "ROM at \"{}\" could not be loaded: {}",
                args.rom_path.display(),
                e
            );
            None
        }
    }
}
