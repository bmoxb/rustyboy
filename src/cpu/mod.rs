mod alu;
mod ime;
mod opcode;
mod registers;

use crate::memory::Memory;

use ime::InterruptMasterEnable;
use opcode::Opcode;
use registers::{Flag, Flags, Registers};

pub struct Cpu {
    regs: Registers,
    state: State,
    ime: InterruptMasterEnable,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            regs: Registers::default(),
            state: State::Running,
            ime: InterruptMasterEnable::default(),
        }
    }

    pub fn cycle(&mut self, mem: &mut Memory) -> usize {
        log::trace!("begin cycle - {}", self.regs);

        let opcode = Opcode(self.fetch8(mem));

        log::trace!(
            "fetched opcode {} from address {:#04X}",
            opcode,
            self.regs.pc - 1
        );

        let cycles_taken = self.execute(opcode, mem);

        log::trace!(
            "executed instruction with opcode {} taking {} machine cycle(s)",
            opcode,
            cycles_taken
        );

        self.ime.cycle();

        log::trace!("end cycle - {}", self.regs);

        cycles_taken
    }

    fn execute(&mut self, opcode: Opcode, mem: &mut Memory) -> usize {
        match opcode.0 {
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
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x3E => {
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
                self.regs.a = mem.read8(self.regs.hl());
                self.regs.set_hl(self.regs.hl() - 1);
                2
            }

            // LD [HL-], A
            0x32 => {
                mem.write8(self.regs.hl(), self.regs.a);
                self.regs.set_hl(self.regs.hl() - 1);
                2
            }

            // LD A, [HL+]
            0x2A => {
                self.regs.a = mem.read8(self.regs.hl());
                self.regs.set_hl(self.regs.hl() + 1);
                2
            }

            // LD [HL+], A
            0x22 => {
                mem.write8(self.regs.hl(), self.regs.a);
                self.regs.set_hl(self.regs.hl() + 1);
                2
            }

            // --- 16-BIT LOAD INSTRUCTIONS ---

            // LD rr, nn
            // 0b00xx0001
            0x01 | 0x11 | 0x21 | 0x31 => {
                let nn = self.fetch16(mem);
                self.regs.set16_with_sp(opcode.rr(), nn);
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
                self.stack_push(mem, self.regs.get16_with_af(opcode.rr()));
                4
            }

            // POP rr
            // 0b11xx0001
            0xC1 | 0xD1 | 0xE1 | 0xF1 => {
                let value = self.stack_pop(mem);
                self.regs.set16_with_af(opcode.rr(), value);
                3
            }

            // --- 8-BIT ARITHMETIC/LOGIC INSTRUCTIONS ---

            // ADD r
            // 0b10000yyy
            0x80..=0x85 | 0x87 => {
                let r = self.regs.get8(opcode.yyy());
                self.regs.a = alu::add8(&mut self.regs.flags, self.regs.a, r);
                1
            }

            // ADD [HL]
            0x86 => {
                let x = mem.read8(self.regs.hl());
                self.regs.a = alu::add8(&mut self.regs.flags, self.regs.a, x);
                2
            }

            // ADD n
            0xC6 => {
                let x = self.fetch8(mem);
                self.regs.a = alu::add8(&mut self.regs.flags, self.regs.a, x);
                2
            }

            // ADC r
            // 0b10001yyy
            0x88..=0x8D | 0x8F => {
                let r = self.regs.get8(opcode.yyy());
                self.regs.a = alu::adc8(&mut self.regs.flags, self.regs.a, r);
                1
            }

            // ADC [HL]
            0x8E => {
                let x = mem.read8(self.regs.hl());
                self.regs.a = alu::adc8(&mut self.regs.flags, self.regs.a, x);
                2
            }

            // ADC n
            0xCE => {
                let x = self.fetch8(mem);
                self.regs.a = alu::adc8(&mut self.regs.flags, self.regs.a, x);
                2
            }

            // SUB r
            // 0b10010yyy
            0x90..=0x95 | 0x97 => {
                let r = self.regs.get8(opcode.yyy());
                self.regs.a = alu::sub8(&mut self.regs.flags, self.regs.a, r);
                1
            }

            // SUB [HL]
            0x96 => {
                let x = mem.read8(self.regs.hl());
                self.regs.a = alu::sub8(&mut self.regs.flags, self.regs.a, x);
                2
            }

            // SUB n
            0xD6 => {
                let x = self.fetch8(mem);
                self.regs.a = alu::sub8(&mut self.regs.flags, self.regs.a, x);
                2
            }

            // SBC r
            // 0b10011yyy
            0x98..=0x9D | 0x9F => {
                let r = self.regs.get8(opcode.yyy());
                self.regs.a = alu::sbc8(&mut self.regs.flags, self.regs.a, r);
                1
            }

            // SBC [HL]
            0x9E => {
                let x = mem.read8(self.regs.hl());
                self.regs.a = alu::sbc8(&mut self.regs.flags, self.regs.a, x);
                2
            }

            // SBC n
            0xDE => {
                let x = self.fetch8(mem);
                self.regs.a = alu::sbc8(&mut self.regs.flags, self.regs.a, x);
                2
            }

            // CP r
            // 0b10111yyy
            0xB8..=0xBD | 0xBF => {
                let r = self.regs.get8(opcode.yyy());
                alu::sub8(&mut self.regs.flags, self.regs.a, r);
                1
            }

            // CP [HL]
            0xBE => {
                let value = mem.read8(self.regs.hl());
                alu::sub8(&mut self.regs.flags, self.regs.a, value);
                2
            }

            // CP n
            0xFE => {
                let value = self.fetch8(mem);
                alu::sub8(&mut self.regs.flags, self.regs.a, value);
                2
            }

            // INC r
            // 0b00xxx100
            0x04 | 0x14 | 0x24 | 0x0C | 0x1C | 0x2C | 0x3C => {
                self.update_reg(opcode.xxx(), alu::inc8);
                1
            }

            // INC [HL]
            0x34 => {
                self.update_mem_hl(mem, alu::inc8);
                3
            }

            // DEC r
            // 0b00xxx101
            0x05 | 0x15 | 0x25 | 0x0D | 0x1D | 0x2D | 0x3D => {
                self.update_reg(opcode.xxx(), alu::dec8);
                1
            }

            // DEC [HL]
            0x35 => {
                self.update_mem_hl(mem, alu::dec8);
                3
            }

            // AND r
            // 0b10100yyy
            0xA0..=0xA5 | 0xA7 => {
                let r = self.regs.get8(opcode.yyy());
                self.regs.a = alu::bitwise_and(&mut self.regs.flags, self.regs.a, r);
                1
            }

            // AND [HL]
            0xA6 => {
                let x = mem.read8(self.regs.hl());
                self.regs.a = alu::bitwise_and(&mut self.regs.flags, self.regs.a, x);
                2
            }

            // AND n
            0xE6 => {
                let x = self.fetch8(mem);
                self.regs.a = alu::bitwise_and(&mut self.regs.flags, self.regs.a, x);
                2
            }

            // OR r
            // 0b10110yyy
            0xB0..=0xB5 | 0xB7 => {
                let r = self.regs.get8(opcode.yyy());
                self.regs.a = alu::bitwise_or(&mut self.regs.flags, self.regs.a, r);
                1
            }

            // OR [HL]
            0xB6 => {
                let x = mem.read8(self.regs.hl());
                self.regs.a = alu::bitwise_or(&mut self.regs.flags, self.regs.a, x);
                2
            }

            // OR n
            0xF6 => {
                let x = self.fetch8(mem);
                self.regs.a = alu::bitwise_or(&mut self.regs.flags, self.regs.a, x);
                2
            }

            // XOR r
            // 0b10101yyy
            0xA8..=0xAD | 0xAF => {
                let r = self.regs.get8(opcode.yyy());
                self.regs.a = alu::bitwise_xor(&mut self.regs.flags, self.regs.a, r);
                1
            }

            // XOR [HL]
            0xAE => {
                let x = mem.read8(self.regs.hl());
                self.regs.a = alu::bitwise_xor(&mut self.regs.flags, self.regs.a, x);
                2
            }

            // XOR n
            0xEE => {
                let x = self.fetch8(mem);
                self.regs.a = alu::bitwise_xor(&mut self.regs.flags, self.regs.a, x);
                2
            }

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
                let rr = self.regs.get16_with_sp(opcode.rr());
                let result = alu::add16(&mut self.regs.flags, hl, rr);
                self.regs.set_hl(result);
                2
            }

            // INC rr
            0x03 | 0x13 | 0x23 | 0x33 => {
                let rr = self.regs.get16_with_sp(opcode.rr());
                self.regs.set16_with_sp(opcode.rr(), alu::inc16(rr));
                2
            }

            // DEC rr
            0x0B | 0x1B | 0x2B | 0x3B => {
                let rr = self.regs.get16_with_sp(opcode.rr());
                self.regs.set16_with_sp(opcode.rr(), alu::dec16(rr));
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
                self.regs.a = alu::rotate_left(&mut self.regs.flags, self.regs.a);
                1
            }

            // RLA
            0x17 => {
                self.regs.a =
                    alu::rotate_left_through_carry_flag(&mut self.regs.flags, self.regs.a);
                1
            }

            // RRCA
            0x0F => {
                self.regs.a = alu::rotate_right(&mut self.regs.flags, self.regs.a);
                1
            }

            // RRA
            0x1F => {
                self.regs.a =
                    alu::rotate_right_through_carry_flag(&mut self.regs.flags, self.regs.a);
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

            // JP flag, nn
            0xC2 | 0xCA | 0xD2 | 0xDA => {
                let nn = self.fetch16(mem);

                if self.evaluate_flag_condition(opcode.ff()) {
                    self.regs.pc = nn;
                    4
                } else {
                    3
                }
            }

            // JR n
            0x18 => {
                let n = self.fetch8(mem); // TODO: signed
                self.regs.pc += n as u16;
                3
            }

            // JR flag, n
            0x20 | 0x28 | 0x30 | 0x38 => {
                let n = self.fetch8(mem); // TODO: signed

                if self.evaluate_flag_condition(opcode.ff()) {
                    self.regs.pc += n as u16;
                    3
                } else {
                    2
                }
            }

            // CALL nn
            0xCD => {
                let nn = self.fetch16(mem);
                self.stack_push(mem, self.regs.pc);
                self.regs.pc = nn;
                6
            }

            // CALL flag, nn
            0xC4 | 0xCC | 0xD4 | 0xDC => {
                let nn = self.fetch16(mem);

                if self.evaluate_flag_condition(opcode.ff()) {
                    self.stack_push(mem, self.regs.pc);
                    self.regs.pc = nn;
                    6
                } else {
                    4
                }
            }

            // RET
            0xC9 => {
                self.regs.pc = self.stack_pop(mem);
                4
            }

            // RET flag
            0xC0 | 0xC8 | 0xD0 | 0xD8 => {
                if self.evaluate_flag_condition(opcode.ff()) {
                    self.regs.pc = self.stack_pop(mem);
                    5
                } else {
                    2
                }
            }

            // RETI
            0xD9 => {
                self.regs.pc = self.stack_pop(mem);
                self.ime.enable(0);
                4
            }

            // RST n
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => {
                self.stack_push(mem, self.regs.pc);
                self.regs.pc = (0xC7 - opcode.0) as u16;
                4
            }

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
                    self.regs.pc -= 1; // go back so that the fetched byte does get executed
                }
                self.state = State::Stopped;
                1
            }

            // DI
            0xF3 => {
                self.ime.disable(1);
                1
            }

            // EI
            0xFB => {
                self.ime.enable(1);
                1
            }

            // CB prefix instructions
            0xCB => {
                let suffix = Opcode(self.fetch8(mem));
                log::trace!("following the 0xCB prefix is {}", suffix);
                self.execute_cb(suffix, mem)
            }

            0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD => {
                log::warn!("unknown opcode {} encountered", opcode);
                1
            }
        }
    }

    fn execute_cb(&mut self, opcode: Opcode, mem: &mut Memory) -> usize {
        match opcode.0 {
            // --- ROTATE AND SHIFT INSTRUCTIONS ---

            // RLC r
            // 0b00000yyy
            0x00..=0x05 | 0x07 => {
                self.update_reg(opcode.yyy(), alu::rotate_left);
                2
            }

            // RLC [HL]
            0x06 => {
                self.update_mem_hl(mem, alu::rotate_left);
                4
            }

            // RL r
            // 0b00010yyy
            0x10..=0x15 | 0x17 => {
                self.update_reg(opcode.yyy(), alu::rotate_left_through_carry_flag);
                2
            }

            // RL [HL]
            0x16 => {
                self.update_mem_hl(mem, alu::rotate_left_through_carry_flag);
                4
            }

            // RRC r
            // 0b00001yyy
            0x08..=0x0D | 0x0F => {
                self.update_reg(opcode.yyy(), alu::rotate_right);
                2
            }

            // RRC [HL]
            0x0E => {
                self.update_mem_hl(mem, alu::rotate_right);
                4
            }

            // RR r
            // 0b00011yyy
            0x18..=0x1D | 0x1F => {
                self.update_reg(opcode.yyy(), alu::rotate_right_through_carry_flag);
                2
            }

            // RR [HL]
            0x1E => {
                self.update_mem_hl(mem, alu::rotate_right_through_carry_flag);
                4
            }

            // SLA r
            // 0b00100yyy
            0x20..=0x25 | 0x27 => {
                self.update_reg(opcode.yyy(), alu::shift_left);
                2
            }

            // SLA [HL]
            0x26 => {
                self.update_mem_hl(mem, alu::shift_left);
                4
            }

            // SWAP r
            // 0b00110yyy
            0x30..=0x35 | 0x37 => {
                self.update_reg(opcode.yyy(), alu::swap_nibbles);
                2
            }

            // SWAP [HL]
            0x36 => {
                self.update_mem_hl(mem, alu::swap_nibbles);
                4
            }

            // SRA r
            // 0b00101yyy
            0x28..=0x2D | 0x2F => {
                self.update_reg(opcode.yyy(), alu::shift_right_leave_msb);
                2
            }

            // SRA [HL]
            0x2E => {
                self.update_mem_hl(mem, alu::shift_right_leave_msb);
                4
            }

            // SRL r
            // 0b00111yyy
            0x38..=0x3D | 0x3F => {
                self.update_reg(opcode.yyy(), alu::shift_right_clear_msb);
                2
            }

            // SRL [HL]
            0x3E => {
                self.update_mem_hl(mem, alu::shift_right_clear_msb);
                4
            }

            // --- SINGLE-BIT OPERATION INSTRUCTIONS ---

            // BIT n, r
            // 0b01xxxyyy
            0x40..=0x45
            | 0x47..=0x4D
            | 0x4F..=0x55
            | 0x57..=0x5D
            | 0x5F..=0x65
            | 0x67..=0x6D
            | 0x6F..=0x75
            | 0x77..=0x7D
            | 0x7F => {
                let r = self.regs.get8(opcode.yyy());
                alu::test_bit(&mut self.regs.flags, opcode.xxx(), r);
                2
            }

            // BIT n, [HL]
            // 0b01xxx110
            0x46 | 0x4E | 0x56 | 0x5E | 0x66 | 0x6E | 0x76 | 0x7E => {
                let value = mem.read8(self.regs.hl());
                alu::test_bit(&mut self.regs.flags, opcode.xxx(), value);
                3
            }

            // SET n, r
            // 0b11xxxyyy
            0xC0..=0xC5
            | 0xC7..=0xCD
            | 0xCF..=0xD5
            | 0xD7..=0xDD
            | 0xDF..=0xE5
            | 0xE7..=0xED
            | 0xEF..=0xF5
            | 0xF7..=0xFD
            | 0xFF => {
                let r = self.regs.get8(opcode.yyy());
                let result = alu::set_bit(opcode.xxx(), r);
                self.regs.set8(opcode.yyy(), result);
                2
            }

            // SET n, [HL]
            // 0b11xxx110
            0xC6 | 0xCE | 0xD6 | 0xDE | 0xE6 | 0xEE | 0xF6 | 0xFE => {
                let value = mem.read8(self.regs.hl());
                let result = alu::set_bit(opcode.xxx(), value);
                mem.write8(self.regs.hl(), result);
                4
            }

            // RES n, r
            // 0b10xxxyyy
            0x80..=0x85
            | 0x87..=0x8D
            | 0x8F..=0x95
            | 0x97..=0x9D
            | 0x9F..=0xA5
            | 0xA7..=0xAD
            | 0xAF..=0xB5
            | 0xB7..=0xBD
            | 0xBF => {
                let r = self.regs.get8(opcode.yyy());
                let result = alu::reset_bit(opcode.xxx(), r);
                self.regs.set8(opcode.yyy(), result);
                2
            }

            // RES n, [HL]
            // 0b10xxx110
            0x86 | 0x8E | 0x96 | 0x9E | 0xA6 | 0xAE | 0xB6 | 0xBE => {
                let value = mem.read8(self.regs.hl());
                let result = alu::reset_bit(opcode.xxx(), value);
                mem.write8(self.regs.hl(), result);
                4
            }
        }
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

    fn evaluate_flag_condition(&self, ff: u8) -> bool {
        match ff {
            0 => !self.regs.flags.get(Flag::Zero),
            1 => !self.regs.flags.get(Flag::Zero),
            2 => self.regs.flags.get(Flag::Carry),
            _ => panic!("{ff} is an unknown flag condition"),
        }
    }

    fn update_reg(&mut self, reg_index: u8, f: impl Fn(&mut Flags, u8) -> u8) {
        let x = self.regs.get8(reg_index);
        let result = f(&mut self.regs.flags, x);
        self.regs.set8(reg_index, result);
    }

    fn update_mem_hl(&mut self, mem: &mut Memory, f: impl Fn(&mut Flags, u8) -> u8) {
        let x = mem.read8(self.regs.hl());
        let result = f(&mut self.regs.flags, x);
        mem.write8(self.regs.hl(), result);
    }
}

enum State {
    Running,
    Halted,
    Stopped,
}
