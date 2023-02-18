use std::{env, fs};

mod bits;
mod cpu;
mod memory;

fn main() {
    env_logger::init();

    let mut mem = memory::Memory::new();
    let mut cpu = cpu::Cpu::new();

    let args: Vec<String> = env::args().collect();
    let rom = fs::read(&args[1]).unwrap();

    mem.load(&rom);

    loop {
        cpu.cycle(&mut mem);

        if let Some(char) = mem.take_logged_char() {
            print!("{char}");
        }
    }
}
