use super::*;
use crate::memory::Memory;

macro_rules! test_rom {
    ($name:ident, $file:literal) => {
        #[test]
        fn $name() {
            let _ = env_logger::builder().is_test(true).try_init();

            let rom = include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/roms/cpu_instrs/individual/",
                $file
            ));

            let mut cpu = Cpu::new();

            let mut mem = Memory::new();
            mem.load(rom);

            let mut logged = String::new();

            loop {
                cpu.cycle(&mut mem);

                if let Some(c) = mem.take_logged_char() {
                    logged.push(c);
                }

                assert!(
                    !logged.contains("Failed"),
                    "{} failed - \"{}\"",
                    stringify!($name),
                    logged.trim()
                );

                if logged.contains("Done") {
                    log::info!("{} - \"{}\"", stringify!($name), logged.trim());
                    break;
                }
            }
        }
    };
}

test_rom!(blargg_rom_special, "01-special.gb");
test_rom!(blargg_rom_interrupts, "02-interrupts.gb");
test_rom!(blargg_rom_op_sp_hl, "03-op sp,hl.gb");
test_rom!(blargg_rom_op_r_imm, "04-op r,imm.gb");
test_rom!(blargg_rom_op_rp, "05-op rp.gb");
test_rom!(blargg_rom_ld_r_r, "06-ld r,r.gb");
test_rom!(blargg_rom_jr_jp_call_ret_rst, "07-jr,jp,call,ret,rst.gb");
test_rom!(blargg_rom_misc, "08-misc instrs.gb");
test_rom!(blargg_rom_op_r_r, "09-op r,r.gb");
test_rom!(blargg_rom_bit_ops, "10-bit ops.gb");
test_rom!(blargg_rom_op_a_hl, "11-op a,(hl).gb");
