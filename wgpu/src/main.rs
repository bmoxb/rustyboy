mod emulator;

#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(not(target_arch = "wasm32"))]
mod desktop;

fn main() {
    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_futures::spawn_local(web::run());
    #[cfg(not(target_arch = "wasm32"))]
    pollster::block_on(desktop::run());
}
