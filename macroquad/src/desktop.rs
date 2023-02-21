use std::env;

pub fn init() -> Box<dyn rustyboy_core::mbc::MemoryBankController> {
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

    mbc
}
