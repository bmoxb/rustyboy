use std::env;

mod bits;
mod cpu;
mod mbc;
mod memory;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    let mbc = mbc::from_rom_file(&args[1]).unwrap();

    println!(
        "ROM Title: {}\nMBC ROM size: {:#X} bytes\nMBC RAM size: {:#X} bytes",
        mbc.game_title(),
        mbc.rom_size(),
        mbc.ram_size()
    );

    let mut mem = memory::Memory::new(mbc);
    let mut cpu = cpu::Cpu::new();

    loop {
        cpu.cycle(&mut mem);
    }
}
