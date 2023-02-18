use std::{env, fs};

mod bits;
mod cpu;
mod memory;

use memory::mbc::NoMBC;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    let rom = fs::read(&args[1]).unwrap();
    let mbc = Box::new(NoMBC::new(&rom));

    let mut mem = memory::Memory::new(mbc);
    let mut cpu = cpu::Cpu::new();

    loop {
        cpu.cycle(&mut mem);
    }
}
