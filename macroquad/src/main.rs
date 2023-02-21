#[cfg(not(target_arch = "wasm32"))]
mod desktop;
#[cfg(not(target_arch = "wasm32"))]
use desktop as platform;

#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
use web as platform;

use rustyboy_core::GameBoy;

use macroquad::prelude as quad;

#[macroquad::main("rustyboy")]
async fn main() {
    if let Some(mbc) = platform::init() {
        let gb = GameBoy::new(mbc);

        game(gb).await;
    }
}

async fn game(mut gb: GameBoy) {
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
