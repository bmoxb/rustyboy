mod opcode;
mod registers;

use crate::memory::Memory;

use opcode::Opcode;
use registers::{Flag, Registers};

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
        let opcode = Opcode {
            value: self.read_byte_pc(mem),
        };
        self.execute(opcode, mem)
    }

    fn execute(&mut self, opcode: Opcode, mem: &mut Memory) -> usize {
        match opcode.value {
            //
            // 8-bit load instructions
            //

            // ld r, r
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
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x3E => {
                let n = self.read_byte_pc(mem);
                self.regs.set8(opcode.dst8(), n);
                2
            }

            // LD r, [HL]
            0x46 | 0x4E | 0x56 | 0x5E | 0x66 | 0x6E | 0x7E => {
                self.regs.set8(opcode.dst8(), mem[self.regs.hl()]);
                2
            }

            // LD [HL], r
            0x70..=0x75 | 0x77 => {
                mem[self.regs.hl()] = self.regs.get8(opcode.src8());
                2
            }

            // LD [HL], n
            0x36 => {
                mem[self.regs.hl()] = self.read_byte_pc(mem);
                3
            }

            // LD A, [BC]
            0x0A => {
                self.regs.a = mem[self.regs.bc()];
                2
            }

            // LD A, [DE]
            0x1A => {
                self.regs.a = mem[self.regs.de()];
                2
            }

            // LD [BC], A
            0x02 => {
                mem[self.regs.bc()] = self.regs.a;
                2
            }

            // LD [DE], A
            0x12 => {
                mem[self.regs.de()] = self.regs.a;
                2
            }

            // LD A, [nn]
            0xFA => {
                let nn = self.read_word_pc(mem);
                self.regs.a = mem[nn];
                4
            }

            // LD [nn], A
            0xEA => {
                let nn = self.read_word_pc(mem);
                mem[nn] = self.regs.a;
                4
            }

            // LDH A, [C]
            0xF2 => {
                let addr = 0xFF00 + (self.regs.c as u16);
                self.regs.a = mem[addr];
                2
            }

            // LDH [C], A
            0xE2 => {
                let addr = 0xFF00 + (self.regs.c as u16);
                mem[addr] = self.regs.a;
                2
            }

            // LDH A, [n]
            0xF0 => {
                let addr = 0xFF00 + (self.read_byte_pc(mem) as u16);
                self.regs.a = mem[addr];
                3
            }

            // LDH [n], A
            0xE0 => {
                let addr = 0xFF00 + (self.read_byte_pc(mem) as u16);
                mem[addr] = self.regs.a;
                3
            }

            // LD A, [HL-]
            0x3A => {
                let hl = self.regs.hl();
                self.regs.a = mem[hl];
                self.regs.set_hl(hl - 1);
                2
            }

            // LD [HL-], A
            0x32 => {
                let hl = self.regs.hl();
                mem[hl] = self.regs.a;
                self.regs.set_hl(hl - 1);
                2
            }

            // LD A, [HL+]
            0x2A => {
                let hl = self.regs.hl();
                self.regs.a = mem[hl];
                self.regs.set_hl(hl + 1);
                2
            }

            // LD [HL+], A
            0x22 => {
                let hl = self.regs.hl();
                mem[hl] = self.regs.a;
                self.regs.set_hl(hl + 1);
                2
            }

            //
            // 16-bit load instructions
            //

            // LD rr, nn
            0x01 | 0x11 | 0x21 | 0x31 => {
                let nn = self.read_word_pc(mem);
                self.regs.set16(opcode.reg16(), nn);
                3
            }

            // LD [nn], SP
            0x08 => {
                mem.write16(self.read_word_pc(mem), self.regs.sp);
                5
            }

            // LD SP, HL
            0xF9 => {
                self.regs.sp = self.regs.hl();
                2
            }

            // PUSH rr
            0xC5 | 0xD5 | 0xE5 | 0xF5 => {
                let rr = self.regs.get16(opcode.reg16());
                self.regs.sp -= 2;
                mem.write16(self.regs.sp, rr);
                4
            }

            // POP rr
            0xC1 | 0xD1 | 0xE1 | 0xF1 => {
                self.regs.set16(opcode.reg16(), mem.read16(self.regs.sp));
                self.regs.sp += 2;
                3
            }

            //
            // 8-bit arithmetic and logical instructions
            //

            // ADD r
            0x80..=0x85 | 0x87 => {
                let r = self.regs.get8(opcode.src8());
                self.regs.a = self.add_with_flags_updated(self.regs.a, r);
                2
            }

            // ADD [HL]
            0x86 => {
                let value = mem[self.regs.hl()];
                self.regs.a = self.add_with_flags_updated(self.regs.a, value);
                2
            }

            // ADD n
            0xC6 => {
                let n = self.read_byte_pc(mem);
                self.regs.a = self.add_with_flags_updated(self.regs.a, n);
                2
            }

            // ADC r
            0 => {
                // TODO
                1
            }

            // ADC [HL]
            0x8E => {
                // TODO
                2
            }

            // ADC n
            0xCE => {
                // TODO
                2
            }

            // SUB r
            0 => {
                // TODO
                1
            }

            // SUB [HL]
            0x96 => {
                // TODO
                2
            }

            // SUB n
            0xD6 => {
                // TODO
                2
            }

            // SBC r
            0 => {
                // TODO
                1
            }

            // SBC [HL]
            0x9E => {
                // TODO
                2
            }

            // SBC n
            0xDE => {
                // TODO
                2
            }

            // CP r
            0 => {
                // TODO
                1
            }

            // CP [HL]
            0xBE => {
                // TODO
                2
            }

            // CP n
            0xFE => {
                // TODO
                2
            }

            // INC r
            0 => {
                // TODO
                1
            }

            // INC [HL]
            0x34 => {
                mem[self.regs.hl()] = self.add_with_flags_updated(mem[self.regs.hl()], 1);
                3
            }

            // DEC r
            0 => {
                // TODO
                1
            }

            // DEC [HL]
            0x35 => {
                // TODO
                3
            }

            // AND r
            0 => {
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
            0 => {
                // TODO
                1
            }

            // OR [HL]
            0x86 => {
                // TODO
                2
            }

            // OR n
            0xF6 => {
                // TODO
                2
            }

            // XOR r
            0 => {
                // TODO
                1
            }

            // XOR [HL]
            0xBE => {
                // TODO
                2
            }

            // XOR n
            0xEE => {
                // TODO
                2
            }

            // CCF
            0x3F => {
                self.regs.set_flag(Flag::Subtraction, false);
                self.regs.set_flag(Flag::HalfCarry, false);
                self.regs.toggle_flag(Flag::Carry);
                1
            }

            // SCF
            0x37 => {
                self.regs.set_flag(Flag::Subtraction, false);
                self.regs.set_flag(Flag::HalfCarry, false);
                self.regs.set_flag(Flag::Carry, true);
                1
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

            //
            // 16-bit arithmetic instructions
            //

            // TODO

            //
            // rotate, shift, bit operation instructions
            //

            // TODO

            //
            // control flow instructions
            //

            // TODO

            //
            // misc. instructions
            //

            // TODO

            // NOP
            0x00 => 1,

            _ => unimplemented!(),
        }
    }

    fn read_byte_pc(&mut self, mem: &Memory) -> u8 {
        let value = mem[self.regs.pc];
        self.regs.pc += 1;
        value
    }

    fn read_word_pc(&mut self, mem: &Memory) -> u16 {
        let lsb = self.read_byte_pc(mem) as u16;
        let msb = self.read_byte_pc(mem) as u16;
        (msb << 8) + lsb
    }

    fn add_with_flags_updated(&mut self, x: u8, y: u8) -> u8 {
        let (sum, carry) = x.overflowing_add(y);

        self.regs.set_flag(Flag::Zero, sum == 0);
        self.regs.set_flag(Flag::Subtraction, false);
        self.regs
            .set_flag(Flag::HalfCarry, (x & 0x0F) + (y & 0x0F) > 0x0F);
        self.regs.set_flag(Flag::Carry, carry);

        sum
    }
}
