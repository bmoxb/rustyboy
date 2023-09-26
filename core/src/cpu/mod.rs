mod alu;
mod ime;
mod opcode;
mod registers;

use crate::bits::modify_bit;
use crate::bus::MemoryBus;
use crate::Cycles;

use ime::InterruptMasterEnable;
use opcode::Opcode;
use registers::{Flags, Registers};

pub struct Cpu {
    pub regs: Registers,
    halted: bool,
    ime: InterruptMasterEnable,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            regs: Registers {
                a: 0x01,
                flags: Flags::new(true, true, false, true),
                b: 0x00,
                c: 0x13,
                d: 0x00,
                e: 0xD8,
                h: 0x01,
                l: 0x4D,
                sp: 0xFFFE,
                pc: 0x0100,
            },
            halted: false,
            ime: InterruptMasterEnable::new(true),
        }
    }

    pub fn cycle(&mut self, bus: &mut MemoryBus) -> Cycles {
        log::trace!("begin cycle - {}, {}", self.regs, self.ime);

        let interrupt_cycles = self.handle_interrupts(bus);

        let cycles = if interrupt_cycles > 0 {
            interrupt_cycles
        } else if self.halted {
            4 // NOP
        } else {
            self.fetch_execute(bus)
        };

        self.ime.cycle();

        log::trace!("end cycle after {} - {}, {}", cycles, self.regs, self.ime);

        cycles
    }

    fn handle_interrupts(&mut self, bus: &mut MemoryBus) -> Cycles {
        let mut cycles = 0;

        if let Some(int) = bus.interrupts.next_triggered_interrupt() {
            if self.ime.enabled() {
                log::debug!("{int} triggered and calling handler");

                bus.interrupts.flag(int, false);
                self.ime.disable(0);

                self.stack_push(bus, self.regs.pc);
                self.regs.pc = int.handler_address();

                cycles += 16;
            } else if self.halted {
                // if IME=0 and halted, PC is incremented so as to continue execution after the HALT instruction
                self.regs.pc += 1;
            }

            if self.halted {
                log::trace!("no longer halted due to interrupt being triggered");
                self.halted = false;
                cycles += 4;
            }
        }

        cycles
    }

    fn fetch_execute(&mut self, bus: &mut MemoryBus) -> Cycles {
        let opcode = Opcode(self.fetch8(bus));

        log::debug!(
            "fetched opcode {} from address {:#04X}",
            opcode,
            self.regs.pc - 1
        );

        let cycles = self.execute(opcode, bus);

        log::trace!("executed instruction with opcode {opcode} in {cycles}");

        cycles
    }

    fn execute(&mut self, opcode: Opcode, bus: &mut MemoryBus) -> Cycles {
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
                4
            }

            // LD r, n
            // 0b00xxx110
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x3E => {
                let n = self.fetch8(bus);
                self.regs.set8(opcode.xxx(), n);
                8
            }

            // LD r, [HL]
            // 0b01xxx110
            0x46 | 0x4E | 0x56 | 0x5E | 0x66 | 0x6E | 0x7E => {
                self.regs.set8(opcode.xxx(), bus.read8(self.regs.hl()));
                8
            }

            // LD [HL], r
            // 0b01110yyy
            0x70..=0x75 | 0x77 => {
                bus.write8(self.regs.hl(), self.regs.get8(opcode.yyy()));
                8
            }

            // LD [HL], n
            0x36 => {
                bus.write8(self.regs.hl(), self.fetch8(bus));
                12
            }

            // LD A, [BC]
            0x0A => {
                self.regs.a = bus.read8(self.regs.bc());
                8
            }

            // LD A, [DE]
            0x1A => {
                self.regs.a = bus.read8(self.regs.de());
                8
            }

            // LD [BC], A
            0x02 => {
                bus.write8(self.regs.bc(), self.regs.a);
                8
            }

            // LD [DE], A
            0x12 => {
                bus.write8(self.regs.de(), self.regs.a);
                8
            }

            // LD A, [nn]
            0xFA => {
                let nn = self.fetch16(bus);
                self.regs.a = bus.read8(nn);
                16
            }

            // LD [nn], A
            0xEA => {
                let nn = self.fetch16(bus);
                bus.write8(nn, self.regs.a);
                16
            }

            // LDH A, [C]
            0xF2 => {
                let addr = 0xFF00 + (self.regs.c as u16);
                self.regs.a = bus.read8(addr);
                8
            }

            // LDH [C], A
            0xE2 => {
                let addr = 0xFF00 + (self.regs.c as u16);
                bus.write8(addr, self.regs.a);
                8
            }

            // LDH A, [n]
            0xF0 => {
                let addr = 0xFF00 + (self.fetch8(bus) as u16);
                self.regs.a = bus.read8(addr);
                12
            }

            // LDH [n], A
            0xE0 => {
                let addr = 0xFF00 + (self.fetch8(bus) as u16);
                bus.write8(addr, self.regs.a);
                12
            }

            // LD A, [HL-]
            0x3A => {
                self.regs.a = bus.read8(self.regs.hl());
                self.regs.set_hl(self.regs.hl() - 1);
                8
            }

            // LD [HL-], A
            0x32 => {
                bus.write8(self.regs.hl(), self.regs.a);
                self.regs.set_hl(self.regs.hl() - 1);
                8
            }

            // LD A, [HL+]
            0x2A => {
                self.regs.a = bus.read8(self.regs.hl());
                self.regs.set_hl(self.regs.hl() + 1);
                8
            }

            // LD [HL+], A
            0x22 => {
                bus.write8(self.regs.hl(), self.regs.a);
                self.regs.set_hl(self.regs.hl() + 1);
                8
            }

            // --- 16-BIT LOAD INSTRUCTIONS ---

            // LD rr, nn
            // 0b00xx0001
            0x01 | 0x11 | 0x21 | 0x31 => {
                let nn = self.fetch16(bus);
                self.regs.set16_with_sp(opcode.rr(), nn);
                12
            }

            // LD [nn], SP
            0x08 => {
                bus.write16(self.fetch16(bus), self.regs.sp);
                20
            }

            // LD HL, SP+n
            0xF8 => {
                let n = self.fetch8(bus);
                let result =
                    alu::add16_with_signed_byte_operand(&mut self.regs.flags, self.regs.sp, n);
                self.regs.set_hl(result);
                12
            }

            // LD SP, HL
            0xF9 => {
                self.regs.sp = self.regs.hl();
                8
            }

            // PUSH rr
            // 0b11xx0101
            0xC5 | 0xD5 | 0xE5 | 0xF5 => {
                self.stack_push(bus, self.regs.get16_with_af(opcode.rr()));
                16
            }

            // POP rr
            // 0b11xx0001
            0xC1 | 0xD1 | 0xE1 | 0xF1 => {
                let value = self.stack_pop(bus);
                self.regs.set16_with_af(opcode.rr(), value);
                12
            }

            // --- 8-BIT ARITHMETIC/LOGIC INSTRUCTIONS ---

            // ADD r
            // 0b10000yyy
            0x80..=0x85 | 0x87 => {
                let r = self.regs.get8(opcode.yyy());
                self.regs.a = alu::add8(&mut self.regs.flags, self.regs.a, r);
                4
            }

            // ADD [HL]
            0x86 => {
                let x = bus.read8(self.regs.hl());
                self.regs.a = alu::add8(&mut self.regs.flags, self.regs.a, x);
                8
            }

            // ADD n
            0xC6 => {
                let x = self.fetch8(bus);
                self.regs.a = alu::add8(&mut self.regs.flags, self.regs.a, x);
                8
            }

            // ADC r
            // 0b10001yyy
            0x88..=0x8D | 0x8F => {
                let r = self.regs.get8(opcode.yyy());
                self.regs.a = alu::adc8(&mut self.regs.flags, self.regs.a, r);
                4
            }

            // ADC [HL]
            0x8E => {
                let x = bus.read8(self.regs.hl());
                self.regs.a = alu::adc8(&mut self.regs.flags, self.regs.a, x);
                8
            }

            // ADC n
            0xCE => {
                let x = self.fetch8(bus);
                self.regs.a = alu::adc8(&mut self.regs.flags, self.regs.a, x);
                8
            }

            // SUB r
            // 0b10010yyy
            0x90..=0x95 | 0x97 => {
                let r = self.regs.get8(opcode.yyy());
                self.regs.a = alu::sub8(&mut self.regs.flags, self.regs.a, r);
                4
            }

            // SUB [HL]
            0x96 => {
                let x = bus.read8(self.regs.hl());
                self.regs.a = alu::sub8(&mut self.regs.flags, self.regs.a, x);
                8
            }

            // SUB n
            0xD6 => {
                let x = self.fetch8(bus);
                self.regs.a = alu::sub8(&mut self.regs.flags, self.regs.a, x);
                8
            }

            // SBC r
            // 0b10011yyy
            0x98..=0x9D | 0x9F => {
                let r = self.regs.get8(opcode.yyy());
                self.regs.a = alu::sbc8(&mut self.regs.flags, self.regs.a, r);
                4
            }

            // SBC [HL]
            0x9E => {
                let x = bus.read8(self.regs.hl());
                self.regs.a = alu::sbc8(&mut self.regs.flags, self.regs.a, x);
                8
            }

            // SBC n
            0xDE => {
                let x = self.fetch8(bus);
                self.regs.a = alu::sbc8(&mut self.regs.flags, self.regs.a, x);
                8
            }

            // CP r
            // 0b10111yyy
            0xB8..=0xBD | 0xBF => {
                let r = self.regs.get8(opcode.yyy());
                alu::sub8(&mut self.regs.flags, self.regs.a, r);
                4
            }

            // CP [HL]
            0xBE => {
                let value = bus.read8(self.regs.hl());
                alu::sub8(&mut self.regs.flags, self.regs.a, value);
                8
            }

            // CP n
            0xFE => {
                let value = self.fetch8(bus);
                alu::sub8(&mut self.regs.flags, self.regs.a, value);
                8
            }

            // INC r
            // 0b00xxx100
            0x04 | 0x14 | 0x24 | 0x0C | 0x1C | 0x2C | 0x3C => {
                self.update_reg(opcode.xxx(), alu::inc8);
                4
            }

            // INC [HL]
            0x34 => {
                self.update_ram_hl(bus, alu::inc8);
                12
            }

            // DEC r
            // 0b00xxx101
            0x05 | 0x15 | 0x25 | 0x0D | 0x1D | 0x2D | 0x3D => {
                self.update_reg(opcode.xxx(), alu::dec8);
                4
            }

            // DEC [HL]
            0x35 => {
                self.update_ram_hl(bus, alu::dec8);
                12
            }

            // AND r
            // 0b10100yyy
            0xA0..=0xA5 | 0xA7 => {
                let r = self.regs.get8(opcode.yyy());
                self.regs.a = alu::bitwise_and(&mut self.regs.flags, self.regs.a, r);
                4
            }

            // AND [HL]
            0xA6 => {
                let x = bus.read8(self.regs.hl());
                self.regs.a = alu::bitwise_and(&mut self.regs.flags, self.regs.a, x);
                8
            }

            // AND n
            0xE6 => {
                let x = self.fetch8(bus);
                self.regs.a = alu::bitwise_and(&mut self.regs.flags, self.regs.a, x);
                8
            }

            // OR r
            // 0b10110yyy
            0xB0..=0xB5 | 0xB7 => {
                let r = self.regs.get8(opcode.yyy());
                self.regs.a = alu::bitwise_or(&mut self.regs.flags, self.regs.a, r);
                4
            }

            // OR [HL]
            0xB6 => {
                let x = bus.read8(self.regs.hl());
                self.regs.a = alu::bitwise_or(&mut self.regs.flags, self.regs.a, x);
                8
            }

            // OR n
            0xF6 => {
                let x = self.fetch8(bus);
                self.regs.a = alu::bitwise_or(&mut self.regs.flags, self.regs.a, x);
                8
            }

            // XOR r
            // 0b10101yyy
            0xA8..=0xAD | 0xAF => {
                let r = self.regs.get8(opcode.yyy());
                self.regs.a = alu::bitwise_xor(&mut self.regs.flags, self.regs.a, r);
                4
            }

            // XOR [HL]
            0xAE => {
                let x = bus.read8(self.regs.hl());
                self.regs.a = alu::bitwise_xor(&mut self.regs.flags, self.regs.a, x);
                8
            }

            // XOR n
            0xEE => {
                let x = self.fetch8(bus);
                self.regs.a = alu::bitwise_xor(&mut self.regs.flags, self.regs.a, x);
                8
            }

            // DAA
            0x27 => {
                self.regs.a = alu::daa(&mut self.regs.flags, self.regs.a);
                4
            }

            // CPL
            0x2F => {
                self.regs.a = alu::bitwise_not(&mut self.regs.flags, self.regs.a);
                4
            }

            // --- 16-BIT ARITHMETIC/LOGIC INSTRUCTIONS ---

            // ADD HL, rr
            // 0b00xx1001
            0x09 | 0x19 | 0x29 | 0x39 => {
                let hl = self.regs.hl();
                let rr = self.regs.get16_with_sp(opcode.rr());
                let result = alu::add16(&mut self.regs.flags, hl, rr);
                self.regs.set_hl(result);
                8
            }

            // INC rr
            0x03 | 0x13 | 0x23 | 0x33 => {
                let rr = self.regs.get16_with_sp(opcode.rr());
                self.regs.set16_with_sp(opcode.rr(), alu::inc16(rr));
                8
            }

            // DEC rr
            0x0B | 0x1B | 0x2B | 0x3B => {
                let rr = self.regs.get16_with_sp(opcode.rr());
                self.regs.set16_with_sp(opcode.rr(), alu::dec16(rr));
                8
            }

            // ADD SP, n
            0xE8 => {
                let n = self.fetch8(bus);
                self.regs.sp =
                    alu::add16_with_signed_byte_operand(&mut self.regs.flags, self.regs.sp, n);
                16
            }

            // --- ROTATE AND SHIFT INSTRUCTIONS ---

            // RLCA
            0x07 => {
                self.regs.a = alu::rotate_left(&mut self.regs.flags, self.regs.a);
                self.regs.flags.set_zero(false); // for rotation instructions on register A, always zero flag = 0
                4
            }

            // RLA
            0x17 => {
                self.regs.a =
                    alu::rotate_left_through_carry_flag(&mut self.regs.flags, self.regs.a);
                self.regs.flags.set_zero(false);
                4
            }

            // RRCA
            0x0F => {
                self.regs.a = alu::rotate_right(&mut self.regs.flags, self.regs.a);
                self.regs.flags.set_zero(false);
                4
            }

            // RRA
            0x1F => {
                self.regs.a =
                    alu::rotate_right_through_carry_flag(&mut self.regs.flags, self.regs.a);
                self.regs.flags.set_zero(false);
                4
            }

            // --- JUMP INSTRUCTIONS ---

            // JP nn
            0xC3 => {
                self.regs.pc = self.fetch16(bus);
                16
            }

            // JP HL
            0xE9 => {
                self.regs.pc = self.regs.hl();
                4
            }

            // JP flag, nn
            0xC2 | 0xCA | 0xD2 | 0xDA => {
                let nn = self.fetch16(bus);

                if self.evaluate_flag_condition(opcode.ff()) {
                    self.regs.pc = nn;
                    16
                } else {
                    12
                }
            }

            // JR n
            0x18 => {
                let n = self.fetch8(bus) as i8 as i32;
                self.regs.pc = ((self.regs.pc as u32 as i32) + n) as u16;
                12
            }

            // JR flag, n
            0x20 | 0x28 | 0x30 | 0x38 => {
                let n = self.fetch8(bus) as i8 as i32;

                if self.evaluate_flag_condition(opcode.ff()) {
                    self.regs.pc = ((self.regs.pc as u32 as i32) + n) as u16;
                    12
                } else {
                    8
                }
            }

            // CALL nn
            0xCD => {
                let nn = self.fetch16(bus);
                self.stack_push(bus, self.regs.pc);
                self.regs.pc = nn;
                24
            }

            // CALL flag, nn
            0xC4 | 0xCC | 0xD4 | 0xDC => {
                let nn = self.fetch16(bus);

                if self.evaluate_flag_condition(opcode.ff()) {
                    self.stack_push(bus, self.regs.pc);
                    self.regs.pc = nn;
                    24
                } else {
                    12
                }
            }

            // RET
            0xC9 => {
                self.regs.pc = self.stack_pop(bus);
                16
            }

            // RET flag
            0xC0 | 0xC8 | 0xD0 | 0xD8 => {
                if self.evaluate_flag_condition(opcode.ff()) {
                    self.regs.pc = self.stack_pop(bus);
                    20
                } else {
                    8
                }
            }

            // RETI
            0xD9 => {
                self.regs.pc = self.stack_pop(bus);
                self.ime.enable(0);
                16
            }

            // RST n
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => {
                let n = opcode.0 - 0xC7;
                self.stack_push(bus, self.regs.pc);
                self.regs.pc = n as u16;
                16
            }

            // --- CPU CONTROL INSTRUCTIONS ---

            // CCF
            0x3F => {
                self.regs.flags.set_subtraction(false);
                self.regs.flags.set_half_carry(false);
                self.regs.flags.toggle_carry();
                4
            }

            // SCF
            0x37 => {
                self.regs.flags.set_subtraction(false);
                self.regs.flags.set_half_carry(false);
                self.regs.flags.set_carry(true);
                4
            }

            // NOP
            0x00 => 4,

            // HALT
            0x76 => {
                self.halted = true;
                4
            }

            // STOP
            0x10 => {
                let n = self.fetch8(bus);
                if n != 0x00 {
                    log::warn!(
                        "STOP instruction not followed by null byte - instead encountered {:#04X}",
                        n
                    );
                    self.regs.pc -= 1; // go back so that the fetched byte does get executed
                }
                self.halted = true; // TODO
                4
            }

            // DI
            0xF3 => {
                self.ime.disable(1);
                4
            }

            // EI
            0xFB => {
                self.ime.enable(1);
                4
            }

            // CB prefix instructions
            0xCB => {
                let suffix = Opcode(self.fetch8(bus));
                log::trace!("following the 0xCB prefix is {}", suffix);
                self.execute_cb(suffix, bus)
            }

            0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD => {
                log::warn!("unknown opcode {} encountered", opcode);
                4
            }
        }
    }

    fn execute_cb(&mut self, opcode: Opcode, bus: &mut MemoryBus) -> Cycles {
        match opcode.0 {
            // --- ROTATE AND SHIFT INSTRUCTIONS ---

            // RLC r
            // 0b00000yyy
            0x00..=0x05 | 0x07 => {
                self.update_reg(opcode.yyy(), alu::rotate_left);
                8
            }

            // RLC [HL]
            0x06 => {
                self.update_ram_hl(bus, alu::rotate_left);
                16
            }

            // RL r
            // 0b00010yyy
            0x10..=0x15 | 0x17 => {
                self.update_reg(opcode.yyy(), alu::rotate_left_through_carry_flag);
                8
            }

            // RL [HL]
            0x16 => {
                self.update_ram_hl(bus, alu::rotate_left_through_carry_flag);
                16
            }

            // RRC r
            // 0b00001yyy
            0x08..=0x0D | 0x0F => {
                self.update_reg(opcode.yyy(), alu::rotate_right);
                8
            }

            // RRC [HL]
            0x0E => {
                self.update_ram_hl(bus, alu::rotate_right);
                16
            }

            // RR r
            // 0b00011yyy
            0x18..=0x1D | 0x1F => {
                self.update_reg(opcode.yyy(), alu::rotate_right_through_carry_flag);
                8
            }

            // RR [HL]
            0x1E => {
                self.update_ram_hl(bus, alu::rotate_right_through_carry_flag);
                16
            }

            // SLA r
            // 0b00100yyy
            0x20..=0x25 | 0x27 => {
                self.update_reg(opcode.yyy(), alu::shift_left);
                8
            }

            // SLA [HL]
            0x26 => {
                self.update_ram_hl(bus, alu::shift_left);
                16
            }

            // SWAP r
            // 0b00110yyy
            0x30..=0x35 | 0x37 => {
                self.update_reg(opcode.yyy(), alu::swap_nibbles);
                8
            }

            // SWAP [HL]
            0x36 => {
                self.update_ram_hl(bus, alu::swap_nibbles);
                16
            }

            // SRA r
            // 0b00101yyy
            0x28..=0x2D | 0x2F => {
                self.update_reg(opcode.yyy(), alu::shift_right_leave_msb);
                8
            }

            // SRA [HL]
            0x2E => {
                self.update_ram_hl(bus, alu::shift_right_leave_msb);
                16
            }

            // SRL r
            // 0b00111yyy
            0x38..=0x3D | 0x3F => {
                self.update_reg(opcode.yyy(), alu::shift_right_clear_msb);
                8
            }

            // SRL [HL]
            0x3E => {
                self.update_ram_hl(bus, alu::shift_right_clear_msb);
                16
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
                alu::test_bit(&mut self.regs.flags, r, opcode.xxx());
                8
            }

            // BIT n, [HL]
            // 0b01xxx110
            0x46 | 0x4E | 0x56 | 0x5E | 0x66 | 0x6E | 0x76 | 0x7E => {
                let value = bus.read8(self.regs.hl());
                alu::test_bit(&mut self.regs.flags, value, opcode.xxx());
                12
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
                let result = modify_bit(r, opcode.xxx(), true);
                self.regs.set8(opcode.yyy(), result);
                8
            }

            // SET n, [HL]
            // 0b11xxx110
            0xC6 | 0xCE | 0xD6 | 0xDE | 0xE6 | 0xEE | 0xF6 | 0xFE => {
                let value = bus.read8(self.regs.hl());
                let result = modify_bit(value, opcode.xxx(), true);
                bus.write8(self.regs.hl(), result);
                16
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
                let result = modify_bit(r, opcode.xxx(), false);
                self.regs.set8(opcode.yyy(), result);
                8
            }

            // RES n, [HL]
            // 0b10xxx110
            0x86 | 0x8E | 0x96 | 0x9E | 0xA6 | 0xAE | 0xB6 | 0xBE => {
                let value = bus.read8(self.regs.hl());
                let result = modify_bit(value, opcode.xxx(), false);
                bus.write8(self.regs.hl(), result);
                16
            }
        }
    }

    fn fetch8(&mut self, bus: &MemoryBus) -> u8 {
        let value = bus.read8(self.regs.pc);
        self.regs.pc += 1;
        value
    }

    fn fetch16(&mut self, bus: &MemoryBus) -> u16 {
        let value = bus.read16(self.regs.pc);
        self.regs.pc += 2;
        value
    }

    fn stack_push(&mut self, bus: &mut MemoryBus, value: u16) {
        self.regs.sp -= 2;
        bus.write16(self.regs.sp, value);
    }

    fn stack_pop(&mut self, bus: &mut MemoryBus) -> u16 {
        let value = bus.read16(self.regs.sp);
        self.regs.sp += 2;
        value
    }

    fn evaluate_flag_condition(&self, ff: u8) -> bool {
        match ff {
            0 => !self.regs.flags.zero(),
            1 => self.regs.flags.zero(),
            2 => !self.regs.flags.carry(),
            3 => self.regs.flags.carry(),
            _ => panic!("{ff} is an unknown flag condition"),
        }
    }

    fn update_reg(&mut self, reg_index: u8, f: impl Fn(&mut Flags, u8) -> u8) {
        let x = self.regs.get8(reg_index);
        let result = f(&mut self.regs.flags, x);
        self.regs.set8(reg_index, result);
    }

    fn update_ram_hl(&mut self, bus: &mut MemoryBus, f: impl Fn(&mut Flags, u8) -> u8) {
        let x = bus.read8(self.regs.hl());
        let result = f(&mut self.regs.flags, x);
        bus.write8(self.regs.hl(), result);
    }
}
