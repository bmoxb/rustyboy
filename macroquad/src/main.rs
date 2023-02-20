use std::env;

use macroquad::prelude as quad;

#[macroquad::main("rustyboy")]
async fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let path = args.last().unwrap();

    let mbc = rustyboy_core::mbc::from_rom_file(path).unwrap();

    println!(
        "ROM Title: {}\nMBC ROM size: {:#X} bytes\nMBC RAM size: {:#X} bytes",
        mbc.game_title(),
        mbc.rom_size(),
        mbc.ram_size()
    );

    let mut gb = rustyboy_core::GameBoy::new(mbc);

    loop {
        let delta = quad::get_frame_time();
        gb.update(delta);

        if let Some(b) = gb.take_serial_byte() {
            print!("{}", b as char);
        }

        quad::next_frame().await
    }
}
