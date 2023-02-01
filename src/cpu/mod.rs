mod alu;
mod opcode;
mod registers;

use crate::memory::Memory;

use opcode::Opcode;
use registers::{Flag, Flags, Registers};

pub struct Cpu {
    regs: Registers,
    state: State,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            regs: Registers::default(),
            state: State::Running,
        }
    }

    pub fn cycle(&mut self, mem: &mut Memory) -> usize {
        log::trace!("begin cycle - {}", self.regs);

        let opcode = Opcode {
            value: self.fetch8(mem),
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
                self.regs.set8(opcode.xxx(), self.regs.get8(opcode.yyy()));
                1
            }

            // LD r, n
            // 0b00xxx110
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E => {
                let n = self.fetch8(mem);
                self.regs.set8(opcode.xxx(), n);
                2
            }

            // LD r, [HL]
            // 0b01xxx110
            0x46 | 0x4E | 0x56 | 0x5E | 0x66 | 0x6E | 0x7E => {
                self.regs.set8(opcode.xxx(), mem.read8(self.regs.hl()));
                2
            }

            // LD [HL], r
            // 0b01110yyy
            0x70..=0x75 | 0x77 => {
                mem.write8(self.regs.hl(), self.regs.get8(opcode.yyy()));
                2
            }

            // LD [HL], n
            0x36 => {
                mem.write8(self.regs.hl(), self.fetch8(mem));
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
                let nn = self.fetch16(mem);
                self.regs.a = mem.read8(nn);
                4
            }

            // LD [nn], A
            0xEA => {
                let nn = self.fetch16(mem);
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
                let addr = 0xFF00 + (self.fetch8(mem) as u16);
                self.regs.a = mem.read8(addr);
                3
            }

            // LDH [n], A
            0xE0 => {
                let addr = 0xFF00 + (self.fetch8(mem) as u16);
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
                let nn = self.fetch16(mem);
                self.regs.set16_with_sp(opcode.reg16(), nn);
                3
            }

            // LD [nn], SP
            0x08 => {
                mem.write16(self.fetch16(mem), self.regs.sp);
                5
            }

            // LD HL, SP+n
            0xF8 => {
                let n = self.fetch8(mem); // TODO: signed
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
            0x80..=0x85 | 0x87 => self.arith_logic_instr_reg(opcode, alu::add8, true),

            // ADD [HL]
            0x86 => self.arith_logic_instr_hl(mem, alu::add8, true),

            // ADD n
            0xC6 => self.arith_logic_instr_immediate(mem, alu::add8, true),

            // ADC r
            // 0b10001yyy
            0x88..=0x8D | 0x8F => self.arith_logic_instr_reg(opcode, alu::adc8, true),

            // ADC [HL]
            0x8E => self.arith_logic_instr_hl(mem, alu::adc8, true),

            // ADC n
            0xCE => self.arith_logic_instr_immediate(mem, alu::adc8, true),

            // SUB r
            // 0b10010yyy
            0x90..=0x95 | 0x97 => self.arith_logic_instr_reg(opcode, alu::sub8, true),

            // SUB [HL]
            0x96 => self.arith_logic_instr_hl(mem, alu::sub8, true),

            // SUB n
            0xD6 => self.arith_logic_instr_immediate(mem, alu::sub8, true),

            // SBC r
            // 0b10011yyy
            0x98..=0x9D | 0x9F => self.arith_logic_instr_reg(opcode, alu::sbc8, true),

            // SBC [HL]
            0x9E => self.arith_logic_instr_hl(mem, alu::sbc8, true),

            // SBC n
            0xDE => self.arith_logic_instr_immediate(mem, alu::sbc8, true),

            // CP r
            // 0b10111yyy
            0xB8..=0xBD | 0xBF => self.arith_logic_instr_reg(opcode, alu::sub8, false),

            // CP [HL]
            0xBE => self.arith_logic_instr_hl(mem, alu::sub8, false),

            // CP n
            0xFE => self.arith_logic_instr_immediate(mem, alu::sub8, false),

            // INC r
            // 0b00xxx100
            0x04 | 0x14 | 0x24 | 0x0C | 0x1C | 0x2C | 0x3C => {
                self.inc_dec_instr_reg(opcode, alu::add8)
            }

            // INC [HL]
            0x34 => self.inc_dec_instr_hl(mem, alu::add8),

            // DEC r
            // 0b00xxx101
            0x05 | 0x15 | 0x25 | 0x0D | 0x1D | 0x2D | 0x3D => {
                self.inc_dec_instr_reg(opcode, alu::sub8)
            }

            // DEC [HL]
            0x35 => self.inc_dec_instr_hl(mem, alu::sub8),

            // AND r
            // 0b10100yyy
            0xA0..=0xA5 | 0xA7 => self.arith_logic_instr_reg(opcode, alu::bitwise_and, true),

            // AND [HL]
            0xA6 => self.arith_logic_instr_hl(mem, alu::bitwise_and, true),

            // AND n
            0xE6 => self.arith_logic_instr_immediate(mem, alu::bitwise_and, true),

            // OR r
            // 0b10110yyy
            0xB0..=0xB5 | 0xB7 => self.arith_logic_instr_reg(opcode, alu::bitwise_or, true),

            // OR [HL]
            0xB6 => self.arith_logic_instr_hl(mem, alu::bitwise_or, true),

            // OR n
            0xF6 => self.arith_logic_instr_immediate(mem, alu::bitwise_or, true),

            // XOR r
            // 0b10101yyy
            0xA8..=0xAD | 0xAF => self.arith_logic_instr_reg(opcode, alu::bitwise_xor, true),

            // XOR [HL]
            0xAE => self.arith_logic_instr_hl(mem, alu::bitwise_xor, true),

            // XOR n
            0xEE => self.arith_logic_instr_immediate(mem, alu::bitwise_xor, true),

            // DAA
            0x27 => {
                // TODO
                1
            }

            // CPL
            0x2F => {
                self.regs.a = alu::bitwise_not(&mut self.regs.flags, self.regs.a);
                1
            }

            // --- 16-BIT ARITHMETIC/LOGIC INSTRUCTIONS ---

            // ADD HL, rr
            // 0b00xx1001
            0x09 | 0x19 | 0x29 | 0x39 => {
                let hl = self.regs.hl();
                let rr = self.regs.get16_with_sp(opcode.reg16());
                let result = alu::add16(&mut self.regs.flags, hl, rr);
                self.regs.set_hl(result);
                2
            }

            // INC rr
            0x03 | 0x13 | 0x23 | 0x33 => {
                let rr = self.regs.get16_with_sp(opcode.reg16());
                self.regs.set16_with_sp(opcode.reg16(), rr.wrapping_add(1));
                2
            }

            // DEC rr
            0x0B | 0x1B | 0x2B | 0x3B => {
                let rr = self.regs.get16_with_sp(opcode.reg16());
                self.regs.set16_with_sp(opcode.reg16(), rr.wrapping_sub(1));
                2
            }

            // ADD SP, n
            0xE8 => {
                let n = self.fetch8(mem); // TODO: sign
                self.regs.sp = alu::add16(&mut self.regs.flags, self.regs.sp, n as u16);
                self.regs.flags.set(Flag::Zero, false); // `ADD SP, nn` needs setting zero flag but `ADD HL, rr` doesn't
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
                self.regs.pc = self.fetch16(mem);
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
                self.regs
                    .flags
                    .set(Flag::Subtraction, false)
                    .set(Flag::HalfCarry, false)
                    .toggle(Flag::Carry);
                1
            }

            // SCF
            0x37 => {
                self.regs
                    .flags
                    .set(Flag::Subtraction, false)
                    .set(Flag::HalfCarry, false)
                    .set(Flag::Carry, true);
                1
            }

            // NOP
            0x00 => 1,

            // HALT
            0x76 => {
                self.state = State::Halted;
                1
            }

            // STOP
            0x10 => {
                let n = self.fetch8(mem);
                if n != 0x00 {
                    log::warn!(
                        "STOP instruction not followed by null byte - instead encountered {:#04X}",
                        n
                    );
                }
                self.state = State::Stopped;
                1
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

            // CB prefix instructions
            0xCB => {
                let suffix = self.fetch8(mem);
                log::trace!("given 0xCB prefix, the full opcode is CB {:02X}", suffix);
                self.execute_cb(suffix)
            }

            _ => {
                log::warn!("unknown opcode {0:#04X} encountered", opcode.value);
                1
            }
        }
    }

    fn execute_cb(&mut self, suffix: u8) -> usize {
        unimplemented!() // TODO
    }

    fn fetch8(&mut self, mem: &Memory) -> u8 {
        let value = mem.read8(self.regs.pc);
        self.regs.pc += 1;
        value
    }

    fn fetch16(&mut self, mem: &Memory) -> u16 {
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

    fn arith_logic_instr_reg(
        &mut self,
        opcode: Opcode,
        alu_func: impl Fn(&mut Flags, u8, u8) -> u8,
        store_result: bool,
    ) -> usize {
        let r = self.regs.get8(opcode.yyy());
        let result = alu_func(&mut self.regs.flags, self.regs.a, r);
        if store_result {
            self.regs.a = result;
        }
        1
    }

    fn arith_logic_instr_hl(
        &mut self,
        mem: &Memory,
        alu_func: impl Fn(&mut Flags, u8, u8) -> u8,
        store_result: bool,
    ) -> usize {
        let value = mem.read8(self.regs.hl());
        let result = alu_func(&mut self.regs.flags, self.regs.a, value);
        if store_result {
            self.regs.a = result;
        }
        2
    }

    fn arith_logic_instr_immediate(
        &mut self,
        mem: &Memory,
        alu_func: impl Fn(&mut Flags, u8, u8) -> u8,
        store_result: bool,
    ) -> usize {
        let n = self.fetch8(mem);
        let result = alu_func(&mut self.regs.flags, self.regs.a, n);
        if store_result {
            self.regs.a = result;
        }
        2
    }

    fn inc_dec_instr_reg(
        &mut self,
        opcode: Opcode,
        alu_func: impl Fn(&mut Flags, u8, u8) -> u8,
    ) -> usize {
        let r = self.regs.get8(opcode.xxx());
        let result = alu_func(&mut self.regs.flags, r, 1);
        self.regs.set8(opcode.xxx(), result);
        1
    }

    fn inc_dec_instr_hl(
        &mut self,
        mem: &mut Memory,
        alu_func: impl Fn(&mut Flags, u8, u8) -> u8,
    ) -> usize {
        let value = mem.read8(self.regs.hl());
        mem.write8(self.regs.hl(), alu_func(&mut self.regs.flags, value, 1));
        3
    }
}

enum State {
    Running,
    Halted,
    Stopped,
}
