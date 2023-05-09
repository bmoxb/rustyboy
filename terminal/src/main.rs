use rustyboy_core::{
    joypad::Button,
    mbc,
    screen::{Colour, Screen, SCREEN_HEIGHT, SCREEN_WIDTH},
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

    let mut stdout = std::io::stdout();

    terminal::enable_raw_mode()?;
    stdout.execute(PushKeyboardEnhancementFlags(
        KeyboardEnhancementFlags::REPORT_EVENT_TYPES,
    ))?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let result = run(&mut stdout, args);

    terminal::disable_raw_mode()?;
    stdout.execute(PopKeyboardEnhancementFlags)?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    result
}

fn run(stdout: &mut Stdout, args: Args) -> crossterm::Result<()> {
    let mbc = mbc::from_rom_file(&args.rom).unwrap();
    let mut gb = GameBoy::new(mbc);

    let mut last_instant = Instant::now();

    loop {
        let delta = (Instant::now() - last_instant).as_secs_f32();
        last_instant = Instant::now();

        gb.update(delta);

        for term_x in 0..SCREEN_WIDTH as u16 {
            for term_y in 0..(SCREEN_HEIGHT / 2) as u16 {
                let (chr, col) = choose_character_and_colour(gb.screen(), term_x, term_y, &args);

                stdout
                    .queue(cursor::MoveTo(term_x, term_y))?
                    .queue(style::SetForegroundColor(col))?
                    .queue(style::Print(chr))?;
            }
        }

        stdout.flush().unwrap();

        if event::poll(Duration::from_millis(1))? {
            match event::read()? {
                Event::Key(KeyEvent { code, kind, .. }) => {
                    let down = !matches!(kind, KeyEventKind::Release);
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
                        KeyCode::Esc => {
                            return Ok(());
                        }
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

fn choose_character_and_colour(
    screen: &Screen,
    term_x: u16,
    term_y: u16,
    args: &Args,
) -> (char, style::Color) {
    let up = screen.get(term_x as u8, term_y as u8 * 2);
    let down = screen.get(term_x as u8, term_y as u8 * 2 + 1);

    let up_black = matches!(up, Colour::Black);
    let down_black = matches!(down, Colour::Black);

    let chr = if args.no_unicode {
        match (up_black, down_black) {
            (true, true) => ' ',
            (true, false) => 'v',
            (false, true) => '^',
            (false, false) => '#',
        }
    } else {
        match (up_black, down_black) {
            (true, true) => ' ',
            (true, false) => '▄',
            (false, true) => '▀',
            (false, false) => '█',
        }
    };

    let gb_col = match (up, down) {
        (Colour::Black, _) => down,
        (_, Colour::Black) => up,
        (Colour::DarkGrey, Colour::White) | (Colour::White, Colour::DarkGrey) => Colour::LightGrey,
        (Colour::White, _) | (_, Colour::White) => Colour::White,
        (Colour::LightGrey, _) | (_, Colour::LightGrey) => Colour::LightGrey,
        _ => Colour::DarkGrey,
    };

    let col = if args.no_rgb {
        match gb_col {
            Colour::Black => style::Color::Black,
            Colour::DarkGrey => style::Color::DarkGreen,
            Colour::LightGrey => style::Color::Green,
            Colour::White => style::Color::White,
        }
    } else {
        match gb_col {
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
    };

    (chr, col)
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
