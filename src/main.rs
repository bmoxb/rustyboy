mod cpu;
mod memory;

fn main() {
    env_logger::init();

    let mut mem = memory::Memory::new();
    let mut cpu = cpu::Cpu::new();

    cpu.cycle(&mut mem);
}
