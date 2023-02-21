#[cfg(not(target_arch = "wasm32"))]
mod desktop;
#[cfg(target_arch = "wasm32")]
mod web;

use macroquad::prelude as quad;

#[macroquad::main("rustyboy")]
async fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    let mbc = desktop::init();
    #[cfg(target_arch = "wasm32")]
    let mbc = web::init();

    let mut gb = rustyboy_core::GameBoy::new(mbc);

    quad::set_camera(&quad::Camera2D::default());

    loop {
        let delta = quad::get_frame_time();
        gb.update(delta);

        if let Some(b) = gb.take_serial_byte() {
            print!("{}", b as char);
        }

        quad::draw_circle(0.0, 0.0, 1.0, quad::BLUE);

        quad::next_frame().await
    }
}
