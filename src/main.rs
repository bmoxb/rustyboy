mod cpu;
mod memory;

fn main() {
    let mut mem = memory::Memory::new();
    let mut cpu = cpu::Cpu::new();
    cpu.cycle(&mut mem);
}
