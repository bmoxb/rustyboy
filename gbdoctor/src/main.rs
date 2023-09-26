use std::fs::File;
use std::{env, io::Write};

use rustyboy_core::{cartridge::Cartridge, mbc, GameBoy};

fn main() {
    let args: Vec<String> = env::args().collect();

    if let [_, rom_path, log_path] = &args[..] {
        let cart = Cartridge::from_file(rom_path).unwrap();
        let mbc = mbc::from_cartridge(cart).unwrap();

        let mut gb = GameBoy::new(mbc);

        let mut file = File::create(log_path).unwrap();

        println!("beginning execution - press Ctrl-C to stop");

        loop {
            gb.step();

            writeln!(
                file,
                "A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}",
                gb.cpu.regs.a,
                gb.cpu.regs.flags.0,
                gb.cpu.regs.b,
                gb.cpu.regs.c,
                gb.cpu.regs.d,
                gb.cpu.regs.e,
                gb.cpu.regs.h,
                gb.cpu.regs.l,
                gb.cpu.regs.sp,
                gb.cpu.regs.pc,
                gb.bus.read8(gb.cpu.regs.pc),
                gb.bus.read8(gb.cpu.regs.pc+1),
                gb.bus.read8(gb.cpu.regs.pc+2),
                gb.bus.read8(gb.cpu.regs.pc+3),
            ).unwrap();

            if let Some(b) = gb.bus.serial.take_byte() {
                print!("{}", b as char);
            }
        }
    } else {
        println!("expected ROM path and output log file path as CLI arguments");
    }
}
