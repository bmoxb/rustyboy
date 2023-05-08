use super::Args;

use std::{fs::File, time::Instant};

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
    window::{Window, WindowBuilder},
};

pub struct Emulator {
    args: Args,
    event_loop: Option<EventLoop<()>>,
    window: Window,
    pixels: Pixels,
    gb: GameBoy,
    last_instant: Instant,
}

impl Emulator {
    pub fn new(args: Args) -> Self {
        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_title("rustyboy")
            .build(&event_loop)
            .expect("failed to create window");

        let pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture)
                .expect("failed to initialise pixels")
        };

        let mbc = mbc::from_rom_file(&args.rom).unwrap();

        let mut gb = GameBoy::new(mbc);

        if let Some(path) = &args.gb_doctor_log {
            let f = File::create(path).unwrap();
            gb.enable_gb_doctor_logging(Box::new(f));
        }

        if let Some(_path) = &args.serial_log {
            unimplemented!() // TODO
        }

        Emulator {
            args,
            event_loop: Some(event_loop),
            window,
            pixels,
            gb,
            last_instant: Instant::now(),
        }
    }

    pub fn run(mut self) {
        let e = self.event_loop.take().unwrap(); // the option is just allow us to move out of the struct so can safely unwrap here

        e.run(move |event, _, control_flow| match event {
            Event::MainEventsCleared => {
                self.update();
            }

            Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                self.draw();
            }

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == self.window.id() => {
                self.handle_window_event(event, control_flow);
            }

            _ => {}
        });
    }

    fn update(&mut self) {
        let delta = (Instant::now() - self.last_instant).as_secs_f32();
        self.last_instant = Instant::now();

        self.gb.update(delta * self.args.speed);

        self.window.request_redraw();
    }

    fn draw(&mut self) {
        for (i, pixel) in self.pixels.frame_mut().chunks_exact_mut(4).enumerate() {
            let x = (i % SCREEN_WIDTH) as u8;
            let y = (i / SCREEN_WIDTH) as u8;

            let rgba = match self.gb.screen().get(x, y) {
                Colour::Black => [15, 56, 15, 255],
                Colour::DarkGrey => [48, 98, 48, 255],
                Colour::LightGrey => [139, 172, 15, 255],
                Colour::White => [155, 188, 15, 255],
            };

            pixel.copy_from_slice(&rgba);
        }

        self.pixels.render().unwrap();
    }

    fn handle_window_event(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow) {
        match event {
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }

            WindowEvent::Resized(size) => {
                self.pixels.resize_surface(size.width, size.height).unwrap();
            }

            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    },
                ..
            } => {
                let down = matches!(state, ElementState::Pressed);
                self.handle_keyboard_input(key, down);
            }

            _ => {}
        };
    }

    fn handle_keyboard_input(&mut self, key: &VirtualKeyCode, down: bool) {
        let jp = self.gb.joypad();

        match key {
            VirtualKeyCode::X => jp.set_button(Button::A, down),
            VirtualKeyCode::Z => jp.set_button(Button::B, down),
            VirtualKeyCode::Return => jp.set_button(Button::Start, down),
            VirtualKeyCode::RShift => jp.set_button(Button::Select, down),
            VirtualKeyCode::Up => jp.set_button(Button::Up, down),
            VirtualKeyCode::Down => jp.set_button(Button::Down, down),
            VirtualKeyCode::Left => jp.set_button(Button::Left, down),
            VirtualKeyCode::Right => jp.set_button(Button::Right, down),
            _ => {}
        };
    }
}
