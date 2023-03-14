#[cfg(not(target_arch = "wasm32"))]
mod desktop;
#[cfg(not(target_arch = "wasm32"))]
use desktop as platform;

#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
use web as platform;

use rustyboy_core::joypad::Button;
use rustyboy_core::screen::{Colour, SCREEN_HEIGHT, SCREEN_WIDTH};
use rustyboy_core::GameBoy;

use macroquad::prelude as quad;
use quad::Color;

#[macroquad::main("rustyboy")]
async fn main() {
    if let Some(mbc) = platform::init() {
        let gb = GameBoy::new(mbc);

        game(gb).await;
    }
}

async fn game(mut gb: GameBoy) {
    loop {
        let delta = quad::get_frame_time();
        gb.update(delta);

        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                let x_unit = quad::screen_width() / SCREEN_WIDTH as f32;
                let y_unit = quad::screen_height() / SCREEN_HEIGHT as f32;

                let colour = match gb.screen().get(x as u8, y as u8) {
                    Colour::Black => quad::color_u8!(15, 56, 15, 255),
                    Colour::DarkGrey => quad::color_u8!(48, 98, 48, 255),
                    Colour::LightGrey => quad::color_u8!(139, 172, 15, 255),
                    Colour::White => quad::color_u8!(155, 188, 15, 255),
                };

                quad::draw_rectangle(x as f32 * x_unit, y as f32 * y_unit, x_unit, y_unit, colour);
            }
        }

        let jp = gb.joypad();
        jp.set_button(Button::A, quad::is_key_down(quad::KeyCode::X));
        jp.set_button(Button::B, quad::is_key_down(quad::KeyCode::Z));
        jp.set_button(Button::Start, quad::is_key_down(quad::KeyCode::Enter));
        jp.set_button(Button::Select, quad::is_key_down(quad::KeyCode::RightShift));
        jp.set_button(Button::Up, quad::is_key_down(quad::KeyCode::Up));
        jp.set_button(Button::Down, quad::is_key_down(quad::KeyCode::Down));
        jp.set_button(Button::Left, quad::is_key_down(quad::KeyCode::Left));
        jp.set_button(Button::Right, quad::is_key_down(quad::KeyCode::Right));

        if let Some(b) = gb.take_serial_byte() {
            print!("{}", b as char);
        }

        quad::next_frame().await
    }
}
