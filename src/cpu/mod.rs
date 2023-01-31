mod alu;
mod opcode;
mod registers;

use crate::memory::Memory;

use opcode::Opcode;
use registers::{Flag, Flags, Registers};

pub struct Cpu {
    halted: bool,
    regs: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            halted: false,
            regs: Registers::default(),
        }
    }

    pub fn cycle(&mut self, mem: &mut Memory) -> usize {
        log::trace!("begin cycle - {}", self.regs);

        let opcode = Opcode {
            value: self.read_byte_pc(mem),
        };

        log::trace!(
            "fetched opcode {0:#04X} ({0:#010b}) from address {1:#04X}",
            opcode.value,
            self.regs.pc - 1
        );

        let cycles_taken = self.execute(opcode, mem);

        log::trace!(
            "executed instruction with opcode {0:#04X} ({0:#010b}) taking {1} machine cycle(s)",
            opcode.value,
            cycles_taken
        );
        log::trace!("end cycle - {}", self.regs);

        cycles_taken
    }

    fn execute(&mut self, opcode: Opcode, mem: &mut Memory) -> usize {
        match opcode.value {
            // --- 8-BIT LOAD INSTRUCTIONS ---

            // LD r, r
            // 0b01xxxyyy
            0x40..=0x45
            | 0x47..=0x4D
            | 0x4F..=0x55
            | 0x57..=0x5D
            | 0x5F..=0x65
            | 0x67..=0x6D
            | 0x6F
            | 0x78..=0x7D
            | 0x7F => {
                self.regs.set8(opcode.dst8(), self.regs.get8(opcode.src8()));
                1
            }

            // LD r, n
            // 0b00xxx110
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E => {
                let n = self.read_byte_pc(mem);
                self.regs.set8(opcode.dst8(), n);
                2
            }

            // LD r, [HL]
            // 0b01xxx110
            0x46 | 0x4E | 0x56 | 0x5E | 0x66 | 0x6E | 0x7E => {
                self.regs.set8(opcode.dst8(), mem.read8(self.regs.hl()));
                2
            }

            // LD [HL], r
            // 0b01110yyy
            0x70..=0x75 | 0x77 => {
                mem.write8(self.regs.hl(), self.regs.get8(opcode.src8()));
                2
            }

            // LD [HL], n
            0x36 => {
                mem.write8(self.regs.hl(), self.read_byte_pc(mem));
                3
            }

            // LD A, [BC]
            0x0A => {
                self.regs.a = mem.read8(self.regs.bc());
                2
            }

            // LD A, [DE]
            0x1A => {
                self.regs.a = mem.read8(self.regs.de());
                2
            }

            // LD [BC], A
            0x02 => {
                mem.write8(self.regs.bc(), self.regs.a);
                2
            }

            // LD [DE], A
            0x12 => {
                mem.write8(self.regs.de(), self.regs.a);
                2
            }

            // LD A, [nn]
            0xFA => {
                let nn = self.read_word_pc(mem);
                self.regs.a = mem.read8(nn);
                4
            }

            // LD [nn], A
            0xEA => {
                let nn = self.read_word_pc(mem);
                mem.write8(nn, self.regs.a);
                4
            }

            // LDH A, [C]
            0xF2 => {
                let addr = 0xFF00 + (self.regs.c as u16);
                self.regs.a = mem.read8(addr);
                2
            }

            // LDH [C], A
            0xE2 => {
                let addr = 0xFF00 + (self.regs.c as u16);
                mem.write8(addr, self.regs.a);
                2
            }

            // LDH A, [n]
            0xF0 => {
                let addr = 0xFF00 + (self.read_byte_pc(mem) as u16);
                self.regs.a = mem.read8(addr);
                3
            }

            // LDH [n], A
            0xE0 => {
                let addr = 0xFF00 + (self.read_byte_pc(mem) as u16);
                mem.write8(addr, self.regs.a);
                3
            }

            // LD A, [HL-]
            0x3A => {
                let hl = self.regs.hl();
                self.regs.a = mem.read8(hl);
                self.regs.set_hl(hl - 1);
                2
            }

            // LD [HL-], A
            0x32 => {
                let hl = self.regs.hl();
                mem.write8(hl, self.regs.a);
                self.regs.set_hl(hl - 1);
                2
            }

            // LD A, [HL+]
            0x2A => {
                let hl = self.regs.hl();
                self.regs.a = mem.read8(hl);
                self.regs.set_hl(hl + 1);
                2
            }

            // LD [HL+], A
            0x22 => {
                let hl = self.regs.hl();
                mem.write8(hl, self.regs.a);
                self.regs.set_hl(hl + 1);
                2
            }

            // --- 16-BIT LOAD INSTRUCTIONS ---

            // LD rr, nn
            // 0b00xx0001
            0x01 | 0x11 | 0x21 | 0x31 => {
                let nn = self.read_word_pc(mem);
                self.regs.set16_with_sp(opcode.reg16(), nn);
                3
            }

            // LD [nn], SP
            0x08 => {
                mem.write16(self.read_word_pc(mem), self.regs.sp);
                5
            }

            // LD HL, SP+n
            0xF8 => {
                let n = self.read_byte_pc(mem); // TODO: signed
                self.regs.set_hl(self.regs.sp + n as u16); // TODO: flags
                3
            }

            // LD SP, HL
            0xF9 => {
                self.regs.sp = self.regs.hl();
                2
            }

            // PUSH rr
            // 0b11xx0101
            0xC5 | 0xD5 | 0xE5 | 0xF5 => {
                self.stack_push(mem, self.regs.get16_with_af(opcode.reg16()));
                4
            }

            // POP rr
            // 0b11xx0001
            0xC1 | 0xD1 | 0xE1 | 0xF1 => {
                let value = self.stack_pop(mem);
                self.regs.set16_with_af(opcode.reg16(), value);
                3
            }

            // --- 8-BIT ARITHMETIC/LOGIC INSTRUCTIONS ---

            // ADD r
            // 0b10000yyy
            0x80..=0x85 | 0x87 => {
                let r = self.regs.get8(opcode.src8());
                self.regs.a = alu::add8(&mut self.regs.flags, self.regs.a, r, false);
                1
            }

            // ADD [HL]
            0x86 => {
                let value = mem.read8(self.regs.hl());
                self.regs.a = alu::add8(&mut self.regs.flags, self.regs.a, value, false);
                2
            }

            // ADD n
            0xC6 => {
                let n = self.read_byte_pc(mem);
                self.regs.a = alu::add8(&mut self.regs.flags, self.regs.a, n, false);
                2
            }

            // ADC r
            0x88..=0x8D | 0x8F => {
                let r = self.regs.get8(opcode.src8());
                self.regs.a = alu::add8(&mut self.regs.flags, self.regs.a, r, true);
                1
            }

            // ADC [HL]
            0x8E => {
                let value = mem.read8(self.regs.hl());
                self.regs.a = alu::add8(&mut self.regs.flags, self.regs.a, value, true);
                2
            }

            // ADC n
            0xCE => {
                let n = self.read_byte_pc(mem);
                self.regs.a = alu::add8(&mut self.regs.flags, self.regs.a, n, true);
                2
            }

            // SUB r
            0x90..=0x95 | 0x97 => {
                let r = self.regs.get8(opcode.src8());
                self.regs.a = alu::sub8(&mut self.regs.flags, self.regs.a, r, false);
                1
            }

            // SUB [HL]
            0x96 => {
                let value = mem.read8(self.regs.hl());
                self.regs.a = alu::sub8(&mut self.regs.flags, self.regs.a, value, false);
                2
            }

            // SUB n
            0xD6 => {
                let n = self.read_byte_pc(mem);
                self.regs.a = alu::sub8(&mut self.regs.flags, self.regs.a, n, false);
                2
            }

            // SBC r
            0x98..=0x9D | 0x9F => {
                let r = self.regs.get8(opcode.src8());
                self.regs.a = alu::sub8(&mut self.regs.flags, self.regs.a, r, true);
                1
            }

            // SBC [HL]
            0x9E => {
                let value = mem.read8(self.regs.hl());
                self.regs.a = alu::sub8(&mut self.regs.flags, self.regs.a, value, true);
                2
            }

            // SBC n
            0xDE => {
                let n = self.read_byte_pc(mem);
                self.regs.a = alu::sub8(&mut self.regs.flags, self.regs.a, n, true);
                2
            }

            // CP r
            0xB8..=0xBD | 0xBF => {
                let r = self.regs.get8(opcode.src8());
                alu::sub8(&mut self.regs.flags, self.regs.a, r, false);
                1
            }

            // CP [HL]
            0xBE => {
                let value = mem.read8(self.regs.hl());
                alu::sub8(&mut self.regs.flags, self.regs.a, value, false);
                2
            }

            // CP n
            0xFE => {
                let n = self.read_byte_pc(mem);
                alu::sub8(&mut self.regs.flags, self.regs.a, n, false);
                2
            }

            // INC r
            0x04 | 0x14 | 0x24 | 0x0C | 0x1C | 0x2C | 0x3C => {
                let r = self.regs.get8(opcode.dst8());
                let result = alu::add8(&mut self.regs.flags, r, 1, false);
                self.regs.set8(opcode.dst8(), result);
                1
            }

            // INC [HL]
            0x34 => {
                let hl = self.regs.hl();
                mem.write8(hl, alu::add8(&mut self.regs.flags, mem.read8(hl), 1, false));
                3
            }

            // DEC r
            0x05 | 0x15 | 0x25 | 0x0D | 0x1D | 0x2D | 0x3D => {
                let r = self.regs.get8(opcode.dst8());
                let result = alu::sub8(&mut self.regs.flags, r, 1, false);
                self.regs.set8(opcode.dst8(), result);
                1
            }

            // DEC [HL]
            0x35 => {
                let hl = self.regs.hl();
                mem.write8(hl, alu::sub8(&mut self.regs.flags, mem.read8(hl), 1, false));
                3
            }

            // AND r
            0xA0..=0xA5 | 0xA7 => {
                // TODO
                1
            }

            // AND [HL]
            0xA6 => {
                // TODO
                2
            }

            // AND n
            0xE6 => {
                // TODO
                2
            }

            // OR n
            0xB0..=0xB5 | 0xB7 => {
                // TODO
                1
            }

            // OR [HL]
            0xB6 => {
                // TODO
                2
            }

            // OR n
            0xF6 => {
                // TODO
                2
            }

            // XOR r
            0xA8..=0xAD | 0xAF => {
                // TODO
                1
            }

            // XOR [HL]
            0xAE => {
                // TODO
                2
            }

            // XOR n
            0xEE => {
                // TODO
                2
            }

            // DAA
            0x27 => {
                // TODO
                1
            }

            // CPL
            0x2F => {
                // TODO
                1
            }

            // --- 16-BIT ARITHMETIC/LOGIC INSTRUCTIONS ---

            // ADD HL, rr
            0x09 | 0x19 | 0x29 | 0x39 => {
                // TODO
                2
            }

            // INC rr
            0x03 | 0x13 | 0x23 | 0x33 => {
                // TODO
                2
            }

            // DEC rr
            0x0B | 0x1B | 0x2B | 0x3B => {
                // TODO
                2
            }

            // ADD SP, n
            0xE8 => {
                // TODO: consider sign
                4
            }

            // --- ROTATE AND SHIFT INSTRUCTIONS ---

            // RLCA
            0x07 => {
                // TODO
                1
            }

            // RLA
            0x17 => {
                // TODO
                1
            }

            // RRCA
            0x0F => {
                // TODO
                1
            }

            // RRA
            0x1F => {
                // TODO
                1
            }

            // --- JUMP INSTRUCTIONS ---

            // JP nn
            0xC3 => {
                self.regs.pc = self.read_word_pc(mem);
                4
            }

            // JP HL
            0xE9 => {
                self.regs.pc = self.regs.hl();
                1
            }

            // TODO: more jump instructions

            // --- CPU CONTROL INSTRUCTIONS ---

            // CCF
            0x3F => {
                self.regs.flags.set(Flag::Subtraction, false);
                self.regs.flags.set(Flag::HalfCarry, false);
                self.regs.flags.toggle(Flag::Carry);
                1
            }

            // SCF
            0x37 => {
                self.regs.flags.set(Flag::Subtraction, false);
                self.regs.flags.set(Flag::HalfCarry, false);
                self.regs.flags.set(Flag::Carry, true);
                1
            }

            // NOP
            0x00 => 1,

            // HALT
            0x76 => {
                // TODO
                0
            }

            // STOP
            0x10 => {
                // TODO
                0
            }

            // DI
            0xF3 => {
                // TODO
                1
            }

            // EI
            0xFB => {
                // TODO
                1
            }

            0xCB => self.execute_cb(),

            // TODO
            _ => unimplemented!(),
        }
    }

    fn execute_cb(&mut self) -> usize {
        unimplemented!()
    }

    fn read_byte_pc(&mut self, mem: &Memory) -> u8 {
        let value = mem.read8(self.regs.pc);
        self.regs.pc += 1;
        value
    }

    fn read_word_pc(&mut self, mem: &Memory) -> u16 {
        let value = mem.read16(self.regs.pc);
        self.regs.pc += 2;
        value
    }

    fn stack_push(&mut self, mem: &mut Memory, value: u16) {
        self.regs.sp -= 2;
        mem.write16(self.regs.sp, value);
    }

    fn stack_pop(&mut self, mem: &mut Memory) -> u16 {
        let value = mem.read16(self.regs.sp);
        self.regs.sp += 2;
        value
    }
}
