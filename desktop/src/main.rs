use std::time::Instant;

use pixels::{Pixels, SurfaceTexture};
use rustyboy_core::{
    joypad::Button,
    mbc,
    screen::{Colour, SCREEN_HEIGHT, SCREEN_WIDTH},
    GameBoy,
};
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("rustyboy")
        .build(&event_loop)
        .unwrap();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture).unwrap()
    };

    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let mbc = mbc::from_rom_file(path).unwrap();
    let mut gb = GameBoy::new(mbc);

    let mut last_instant = Instant::now();

    event_loop.run(move |event, _, control_flow| match event {
        Event::MainEventsCleared => {
            let delta = (Instant::now() - last_instant).as_secs_f32();
            last_instant = Instant::now();

            gb.update(delta);

            window.request_redraw();
        }

        Event::RedrawRequested(window_id) if window_id == window.id() => {
            for (i, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
                let x = (i % SCREEN_WIDTH) as u8;
                let y = (i / SCREEN_WIDTH) as u8;

                let rgba = match gb.screen().get(x, y) {
                    Colour::Black => [15, 56, 15, 255],
                    Colour::DarkGrey => [48, 98, 48, 255],
                    Colour::LightGrey => [139, 172, 15, 255],
                    Colour::White => [155, 188, 15, 255],
                };

                pixel.copy_from_slice(&rgba);
            }

            pixels.render().unwrap();
        }

        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }

                WindowEvent::Resized(size) => {
                    pixels.resize_surface(size.width, size.height).unwrap();
                }

                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(code),
                            state,
                            ..
                        },
                    ..
                } => {
                    let jp = gb.joypad();
                    let state = matches!(state, ElementState::Pressed);
                    match code {
                        VirtualKeyCode::A => jp.set_button(Button::A, state),
                        VirtualKeyCode::B => jp.set_button(Button::B, state),
                        VirtualKeyCode::Return => jp.set_button(Button::Start, state),
                        VirtualKeyCode::RShift => jp.set_button(Button::Select, state),
                        VirtualKeyCode::Up => jp.set_button(Button::Up, state),
                        VirtualKeyCode::Down => jp.set_button(Button::Down, state),
                        VirtualKeyCode::Left => jp.set_button(Button::Left, state),
                        VirtualKeyCode::Right => jp.set_button(Button::Right, state),
                        _ => {}
                    };
                }

                _ => {}
            };
        }

        _ => {}
    });
}
