use rustyboy_core::{
    joypad::Button,
    mbc,
    screen::{Colour, SCREEN_HEIGHT, SCREEN_WIDTH},
    GameBoy,
};

use clap::Parser;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    style, terminal, ExecutableCommand, QueueableCommand,
};

use std::{
    io::Write,
    path::PathBuf,
    time::{Duration, Instant},
};

fn main() -> crossterm::Result<()> {
    let args = Args::parse();

    let mbc = mbc::from_rom_file(&args.rom).unwrap();
    let mut gb = GameBoy::new(mbc);

    let mut last_instant = Instant::now();

    let mut stdout = std::io::stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    terminal::enable_raw_mode()?;

    loop {
        let delta = (Instant::now() - last_instant).as_secs_f32();
        last_instant = Instant::now();

        gb.update(delta);

        for x in 0..SCREEN_WIDTH as u8 {
            for y in 0..SCREEN_HEIGHT as u8 {
                let col = gb_colour_to_term_colour(gb.screen().get(x, y), args.no_rgb);

                stdout
                    .queue(cursor::MoveTo(x as u16, y as u16))?
                    .queue(style::SetBackgroundColor(col))?
                    .queue(style::Print(' '))?;
            }
        }

        stdout.flush().unwrap();

        if event::poll(Duration::from_millis(10))? {
            match event::read()? {
                Event::Key(KeyEvent { code, kind, .. }) => {
                    let down = matches!(kind, KeyEventKind::Press | KeyEventKind::Release);
                    let jp = gb.joypad();

                    match code {
                        KeyCode::Char('x') => jp.set_button(Button::A, down),
                        KeyCode::Char('z') => jp.set_button(Button::B, down),
                        KeyCode::Enter => jp.set_button(Button::Start, down),
                        KeyCode::Backspace => jp.set_button(Button::Select, down),
                        KeyCode::Up => jp.set_button(Button::Up, down),
                        KeyCode::Down => jp.set_button(Button::Down, down),
                        KeyCode::Left => jp.set_button(Button::Left, down),
                        KeyCode::Right => jp.set_button(Button::Right, down),
                        _ => {}
                    }
                }

                Event::Resize(_, _) => {
                    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
                }

                _ => {}
            }
        }
    }
}

fn gb_colour_to_term_colour(gb_colour: Colour, no_rgb: bool) -> style::Color {
    if no_rgb {
        match gb_colour {
            Colour::Black => style::Color::Black,
            Colour::DarkGrey => style::Color::DarkGreen,
            Colour::LightGrey => style::Color::Green,
            Colour::White => style::Color::White,
        }
    } else {
        match gb_colour {
            Colour::Black => style::Color::Rgb {
                r: 15,
                g: 56,
                b: 15,
            },
            Colour::DarkGrey => style::Color::Rgb {
                r: 48,
                g: 98,
                b: 48,
            },
            Colour::LightGrey => style::Color::Rgb {
                r: 139,
                g: 172,
                b: 15,
            },
            Colour::White => style::Color::Rgb {
                r: 155,
                g: 188,
                b: 15,
            },
        }
    }
}

#[derive(Parser)]
pub struct Args {
    /// Path to a Game Boy ROM file to execute
    rom: PathBuf,
    /// Disable full RGB colours and use a more limited palette
    #[arg(long, default_value = "false")]
    no_rgb: bool,
}
