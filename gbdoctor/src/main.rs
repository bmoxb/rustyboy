use std::env;
use std::fs::File;

use rustyboy_core::{cartridge::Cartridge, mbc, GameBoy};

fn main() {
    let args: Vec<String> = env::args().collect();

    if let [_, rom_path, log_path] = &args[..] {
        let cart = Cartridge::from_file(rom_path).unwrap();
        let mbc = mbc::from_cartridge(cart).unwrap();

        let mut gb = GameBoy::new(mbc);

        let file = File::create(log_path).unwrap();
        gb.enable_gb_doctor_logging(Box::new(file));

        println!("beginning execution - press Ctrl-C to stop");

        loop {
            gb.step();

            if let Some(b) = gb.take_serial_byte() {
                print!("{}", b as char);
            }
        }
    } else {
        println!("expected ROM path and output log file path as CLI arguments");
    }
}
