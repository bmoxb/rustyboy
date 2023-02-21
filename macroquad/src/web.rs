use sapp_jsutils::JsObject;

extern "C" {
    fn get_rom_data() -> JsObject;
}

pub fn init() -> Box<dyn rustyboy_core::mbc::MemoryBankController> {
    let js_obj = unsafe { get_rom_data() };

    let mut buf = Vec::new();
    js_obj.to_byte_buffer(&mut buf);

    rustyboy_core::mbc::from_rom_data(&buf)
}
