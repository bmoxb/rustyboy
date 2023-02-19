use std::env;

mod bits;
mod gb;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    let mbc = gb::mbc::from_rom_file(&args[1]).unwrap();

    println!(
        "ROM Title: {}\nMBC ROM size: {:#X} bytes\nMBC RAM size: {:#X} bytes",
        mbc.game_title(),
        mbc.rom_size(),
        mbc.ram_size()
    );

    let mut gb = gb::GameBoy::new(mbc, true);

    loop {
        gb.update(0.0);
    }
}
