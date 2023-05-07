use super::*;

const CYCLES_WITHOUT_LOG_THRESHOLD: usize = 10_000_000;

macro_rules! test_rom {
    ($name:ident, $file:literal) => {
        #[test]
        fn $name() {
            let _ = env_logger::builder().is_test(true).try_init();

            let rom = include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../gb-test-roms/",
                $file
            ));
            let mbc = mbc::from_rom_data(rom);

            let mut gb = GameBoy::new(mbc);

            let mut logged = String::new();
            let mut cycles_since_last_log = 0;

            // continue executing instructions until enough cycles have passed without any output being produced
            while cycles_since_last_log < CYCLES_WITHOUT_LOG_THRESHOLD {
                gb.step();

                cycles_since_last_log += 1;

                if let Some(b) = gb.take_serial_byte() {
                    logged.push(b as char);
                    cycles_since_last_log = 0;
                }
            }

            assert!(
                logged.contains("Passed"),
                "Blargg test ROM \"{}\" failed: {}",
                $file,
                logged.trim().replace("\n", " ")
            );

            log::info!("{} - \"{}\"", stringify!($name), logged.trim());
        }
    };
}

test_rom!(cpu_instrs_special, "cpu_instrs/individual/01-special.gb");
test_rom!(
    cpu_instrs_interrupts,
    "cpu_instrs/individual/02-interrupts.gb"
);
test_rom!(cpu_instrs_op_sp_hl, "cpu_instrs/individual/03-op sp,hl.gb");
test_rom!(cpu_instrs_op_r_imm, "cpu_instrs/individual/04-op r,imm.gb");
test_rom!(cpu_instrs_op_rp, "cpu_instrs/individual/05-op rp.gb");
test_rom!(cpu_instrs_ld_r_r, "cpu_instrs/individual/06-ld r,r.gb");
test_rom!(
    cpu_instrs_jr_jp_call_ret_rst,
    "cpu_instrs/individual/07-jr,jp,call,ret,rst.gb"
);
test_rom!(cpu_instrs_misc, "cpu_instrs/individual/08-misc instrs.gb");
test_rom!(cpu_instrs_op_r_r, "cpu_instrs/individual/09-op r,r.gb");
test_rom!(cpu_instrs_bit_ops, "cpu_instrs/individual/10-bit ops.gb");
test_rom!(cpu_instrs_op_a_hl, "cpu_instrs/individual/11-op a,(hl).gb");

test_rom!(instr_timing, "instr_timing/instr_timing.gb");
