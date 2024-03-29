use std::rc::Rc;

use instant::Instant;
use pixels::{Pixels, SurfaceTexture};
use rustyboy_core::{
    joypad::Button,
    screen::{Colour, SCREEN_HEIGHT, SCREEN_WIDTH},
    GameBoy,
};
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub struct Emulator {
    gb: GameBoy,
    event_loop: Option<EventLoop<()>>,
    window: Rc<Window>,
    pixels: Pixels,
    timer: Timer,
    emulation_speed: f32,
}

impl Emulator {
    pub async fn new(gb: GameBoy, emulation_speed: f32) -> Self {
        let event_loop = EventLoop::new();

        let window = Rc::new(
            WindowBuilder::new()
                .with_title("rustyboy")
                .build(&event_loop)
                .expect("failed to create window"),
        );

        #[cfg(target_arch = "wasm32")]
        crate::web::window_setup(Rc::clone(&window));

        let pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, window.as_ref());
            Pixels::new_async(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture)
                .await
                .expect("failed to initialise pixels")
        };

        Emulator {
            gb,
            event_loop: Some(event_loop),
            window,
            pixels,
            timer: Timer {
                last_instant: Instant::now(),
            },
            emulation_speed,
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
        let delta = self.timer.delta();

        self.gb.update(delta * self.emulation_speed);

        self.window.request_redraw();
    }

    fn draw(&mut self) {
        for (i, pixel) in self.pixels.frame_mut().chunks_exact_mut(4).enumerate() {
            let x = (i % SCREEN_WIDTH) as u8;
            let y = (i / SCREEN_WIDTH) as u8;

            let rgba = match self.gb.bus.gpu.screen.get(x, y) {
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
        match key {
            VirtualKeyCode::X => self.gb.bus.joypad.set_button(Button::A, down),
            VirtualKeyCode::Z => self.gb.bus.joypad.set_button(Button::B, down),
            VirtualKeyCode::Return => self.gb.bus.joypad.set_button(Button::Start, down),
            VirtualKeyCode::RShift => self.gb.bus.joypad.set_button(Button::Select, down),
            VirtualKeyCode::Up => self.gb.bus.joypad.set_button(Button::Up, down),
            VirtualKeyCode::Down => self.gb.bus.joypad.set_button(Button::Down, down),
            VirtualKeyCode::Left => self.gb.bus.joypad.set_button(Button::Left, down),
            VirtualKeyCode::Right => self.gb.bus.joypad.set_button(Button::Right, down),
            _ => {}
        };
    }
}

struct Timer {
    last_instant: Instant,
}

impl Timer {
    fn delta(&mut self) -> f32 {
        let now = Instant::now();
        let delta = (now - self.last_instant).as_secs_f32();
        self.last_instant = now;
        delta
    }
}
