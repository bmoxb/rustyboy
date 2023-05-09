use rustyboy_core::{
    joypad::Button,
    mbc,
    screen::{Colour, SCREEN_HEIGHT, SCREEN_WIDTH},
    GameBoy,
};

use clap::Parser;
use crossterm::{
    cursor,
    event::{
        self, Event, KeyCode, KeyEvent, KeyEventKind, KeyboardEnhancementFlags,
        PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
    },
    style, terminal, ExecutableCommand, QueueableCommand,
};

use std::{
    io::{Stdout, Write},
    path::PathBuf,
    time::{Duration, Instant},
};

fn main() -> crossterm::Result<()> {
    let args = Args::parse();

    let mbc = mbc::from_rom_file(&args.rom).unwrap();
    let gb = GameBoy::new(mbc);

    terminal::enable_raw_mode()?;
    std::io::stdout()
        .execute(PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::REPORT_EVENT_TYPES,
        ))?
        .execute(terminal::Clear(terminal::ClearType::All))?;

    let mut emu = Emulator {
        args,
        stdout: std::io::stdout(),
        gb,
        continue_execution: true,
    };
    let result = emu.run();

    terminal::disable_raw_mode()?;
    std::io::stdout()
        .execute(PopKeyboardEnhancementFlags)?
        .execute(style::SetBackgroundColor(style::Color::Black))?
        .execute(style::SetForegroundColor(style::Color::White))?
        .execute(terminal::Clear(terminal::ClearType::All))?;

    result
}

#[derive(Parser)]
pub struct Args {
    /// Path to a Game Boy ROM file to execute
    rom: PathBuf,
    /// Disable full RGB colours and use a more limited palette
    #[arg(long, default_value = "false")]
    no_rgb: bool,
    /// Disable Unicode characters
    #[arg(long, default_value = "false")]
    no_unicode: bool,
}

struct Emulator {
    args: Args,
    stdout: Stdout,
    gb: GameBoy,
    continue_execution: bool,
}

impl Emulator {
    fn run(&mut self) -> crossterm::Result<()> {
        let mut last_instant = Instant::now();

        while self.continue_execution {
            let delta = (Instant::now() - last_instant).as_secs_f32();
            last_instant = Instant::now();

            self.gb.update(delta);
            self.draw()?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&mut self) -> crossterm::Result<()> {
        for term_x in 0..SCREEN_WIDTH as u16 {
            for term_y in 0..(SCREEN_HEIGHT / 2) as u16 {
                let (chr, fg, bg) = self.choose_character_and_colour(term_x, term_y);

                self.stdout
                    .queue(cursor::MoveTo(term_x, term_y))?
                    .queue(style::SetForegroundColor(fg))?
                    .queue(style::SetBackgroundColor(bg))?
                    .queue(style::Print(chr))?;
            }
        }

        self.stdout.flush()
    }

    fn choose_character_and_colour(
        &self,
        term_x: u16,
        term_y: u16,
    ) -> (char, style::Color, style::Color) {
        let up = self.gb.screen().get(term_x as u8, term_y as u8 * 2);
        let down = self.gb.screen().get(term_x as u8, term_y as u8 * 2 + 1);

        let chr = if self.args.no_unicode {
            colours_to_ascii(up, down)
        } else {
            colours_to_unicode(up, down)
        };

        let (up_col, down_col) = if self.args.no_rgb {
            (gb_colour_to_term_colour(up), gb_colour_to_term_colour(down))
        } else {
            (gb_colour_to_rgb_colour(up), gb_colour_to_rgb_colour(down))
        };

        let (fg, bg) = if up > down {
            (down_col, up_col)
        } else {
            (up_col, down_col)
        };

        (chr, fg, bg)
    }

    fn handle_events(&mut self) -> crossterm::Result<()> {
        if event::poll(Duration::from_millis(5))? {
            match event::read()? {
                Event::Key(KeyEvent { code, kind, .. }) => {
                    let down = !matches!(kind, KeyEventKind::Release);
                    self.handle_key_event(code, down);
                }

                Event::Resize(_, _) => {
                    self.stdout
                        .execute(terminal::Clear(terminal::ClearType::All))?;
                }

                _ => {}
            }
        }

        Ok(())
    }

    fn handle_key_event(&mut self, code: KeyCode, down: bool) {
        let jp = self.gb.joypad();

        match code {
            KeyCode::Char('x') => jp.set_button(Button::A, down),
            KeyCode::Char('z') => jp.set_button(Button::B, down),
            KeyCode::Enter => jp.set_button(Button::Start, down),
            KeyCode::Backspace => jp.set_button(Button::Select, down),
            KeyCode::Up => jp.set_button(Button::Up, down),
            KeyCode::Down => jp.set_button(Button::Down, down),
            KeyCode::Left => jp.set_button(Button::Left, down),
            KeyCode::Right => jp.set_button(Button::Right, down),
            KeyCode::Esc => {
                self.continue_execution = false;
            }
            _ => {}
        }
    }
}

fn colours_to_ascii(up: Colour, down: Colour) -> char {
    if up == down {
        ' '
    } else if up > down {
        'v'
    } else {
        '^'
    }
}

fn colours_to_unicode(up: Colour, down: Colour) -> char {
    if up == down {
        '█'
    } else if up > down {
        '▄'
    } else {
        '▀'
    }
}

fn gb_colour_to_term_colour(c: Colour) -> style::Color {
    match c {
        Colour::Black => style::Color::Black,
        Colour::DarkGrey => style::Color::DarkGreen,
        Colour::LightGrey => style::Color::Green,
        Colour::White => style::Color::White,
    }
}

fn gb_colour_to_rgb_colour(c: Colour) -> style::Color {
    match c {
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
