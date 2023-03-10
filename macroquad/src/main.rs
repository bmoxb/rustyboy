#[cfg(not(target_arch = "wasm32"))]
mod desktop;
#[cfg(not(target_arch = "wasm32"))]
use desktop as platform;

#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
use web as platform;

use rustyboy_core::screen::{Colour, SCREEN_HEIGHT, SCREEN_WIDTH};
use rustyboy_core::GameBoy;

use macroquad::prelude as quad;

const X_UINT: f32 = 1.0 / (SCREEN_WIDTH as f32);
const Y_UINT: f32 = 1.0 / (SCREEN_HEIGHT as f32);

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

        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                let colour = match gb.screen().get(x as u8, y as u8) {
                    Colour::Black => quad::BLACK,
                    Colour::DarkGrey => quad::GRAY,
                    Colour::LightGrey => quad::BLUE,
                    Colour::White => quad::WHITE,
                    Colour::Transparent => quad::GREEN,
                };

                quad::draw_rectangle(x as f32 * X_UINT, y as f32 * Y_UINT, X_UINT, Y_UINT, colour);
            }
        }

        if let Some(b) = gb.take_serial_byte() {
            print!("{}", b as char);
        }

        quad::next_frame().await
    }
}
