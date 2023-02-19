use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let [_, rom_path, log_path] = &args[..] {
        let mbc = rustyboy_core::mbc::from_rom_file(rom_path).unwrap();
        let mut gb = rustyboy_core::GameBoy::new(mbc);

        let file = File::create(log_path).unwrap();
        gb.gb_doctor_logging = Some(Box::new(file));

        println!("beginning execution - press Ctrl-C to stop");

        loop {
            gb.update(0.0);
        }
    } else {
        println!("expected ROM path and output log file path as CLI arguments");
    }
}
