use crate::emulator::Emulator;

use std::rc::Rc;

use rustyboy_core::{cartridge::Cartridge, mbc, GameBoy};

use wasm_bindgen::{closure::Closure, JsCast};

use winit::{dpi::LogicalSize, platform::web::WindowExtWebSys, window::Window};

pub async fn run() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let cart = Cartridge::from_data(include_bytes!("/home/max/Downloads/Tetris.gb").to_vec());
    let mbc = mbc::from_cartridge(cart).unwrap();
    let gb = GameBoy::new(mbc);

    Emulator::new(gb, 1.0).await.run();
}

#[derive(Default)]
pub struct Timer {
    last_instant: f64,
}

impl Timer {
    pub fn delta(&mut self) -> f32 {
        let now = js_sys::Date::now() / 1000.0;
        let delta = now - self.last_instant;
        self.last_instant = now;
        delta as f32
    }
}

pub fn window_setup(window: Rc<Window>) {
    window.set_inner_size(get_client_window_size());

    let web_window = web_sys::window().unwrap();

    web_window
        .document()
        .and_then(|doc| doc.body())
        .and_then(|body| {
            body.append_child(&web_sys::Element::from(window.canvas()))
                .ok()
        })
        .expect("failed to add canvas to document body");

    let callback = Closure::wrap(Box::new(move |_: web_sys::Event| {
        window.set_inner_size(get_client_window_size())
    }) as Box<dyn FnMut(_)>);

    web_window
        .add_event_listener_with_callback("resize", callback.as_ref().unchecked_ref())
        .expect("failed to set window resize callback");

    callback.forget();
}

fn get_client_window_size() -> LogicalSize<f64> {
    let client_window = web_sys::window().unwrap();

    LogicalSize::new(
        client_window.inner_width().unwrap().as_f64().unwrap(),
        client_window.inner_height().unwrap().as_f64().unwrap(),
    )
}
