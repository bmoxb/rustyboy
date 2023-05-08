use crossterm::{cursor, style, terminal, ExecutableCommand, QueueableCommand};
use rustyboy_core::{
    mbc,
    screen::{Colour, SCREEN_HEIGHT, SCREEN_WIDTH},
    GameBoy,
};

use clap::Parser;

use std::{io::Write, path::PathBuf, time::Instant};

fn main() -> crossterm::Result<()> {
    let args = Args::parse();

    let mbc = mbc::from_rom_file(&args.rom).unwrap();
    let mut gb = GameBoy::new(mbc);

    let mut last_instant = Instant::now();

    let mut stdout = std::io::stdout();

    loop {
        let delta = (Instant::now() - last_instant).as_secs_f32();
        last_instant = Instant::now();

        gb.update(delta);

        stdout.execute(terminal::Clear(terminal::ClearType::All))?;

        for term_x in 0..(SCREEN_WIDTH as u16 / 2) {
            for term_y in 0..(SCREEN_HEIGHT as u16 / 2) {
                let (col, chr) = terminal_colour_and_character(&gb, term_x, term_y);

                stdout
                    .queue(cursor::MoveTo(term_x, term_y))?
                    .queue(style::SetForegroundColor(col))?
                    .queue(style::Print(chr))?;
            }
        }

        stdout.flush().unwrap();

        std::thread::sleep(std::time::Duration::from_secs_f32(0.1))
    }
}

fn terminal_colour_and_character(gb: &GameBoy, term_x: u16, term_y: u16) -> (style::Color, char) {
    let screen = gb.screen();

    let gb_x = term_x as u8 * 2;
    let gb_y = term_y as u8 * 2;

    let top_left = screen.get(gb_x, gb_y);
    let top_right = screen.get(gb_x + 1, gb_y);
    let bottom_left = screen.get(gb_x, gb_y + 1);
    let bottom_right = screen.get(gb_x + 1, gb_y + 1);
    let colours = [top_left, top_right, bottom_left, bottom_right];

    let filtered_rgb_values = colours.iter().map(|c| match c {
        Colour::Black => [15, 56, 15],
        Colour::DarkGrey => [48u16, 98, 48],
        Colour::LightGrey => [139, 172, 15],
        Colour::White => [155, 188, 15],
    });

    let mut interpolated = filtered_rgb_values.fold([0, 0, 0], |mut col, y| {
        for i in 0..3 {
            col[i] += y[i];
        }
        col
    });
    for i in 0..3 {
        interpolated[i] /= 3;
    }
    let col = style::Color::Rgb {
        r: interpolated[0] as u8,
        g: interpolated[1] as u8,
        b: interpolated[2] as u8,
    };

    let mut blacks: [bool; 4] = [false; 4];
    for (i, c) in colours.into_iter().enumerate() {
        blacks[i] = matches!(c, Colour::Black);
    }

    let chr = match blacks {
        // tl    tr    bl    br
        [true, false, false, false] => '▘',
        [false, true, false, false] => '▝',
        [false, false, true, false] => '▖',
        [false, false, false, true] => '▗',
        [true, false, true, false] => '▌',
        [false, true, false, true] => '▐',
        [true, true, false, false] => '▀',
        [false, false, true, true] => '▄',
        [true, true, true, false] => '▛',
        [true, true, false, true] => '▜',
        [true, false, true, true] => '▙',
        [false, true, true, true] => '▟',
        [true, false, false, true] => '▚',
        [false, true, true, false] => '▞',
        _ => '█',
    };

    (col, chr)
}

#[derive(Parser)]
pub struct Args {
    /// Path to a Game Boy ROM file to execute
    rom: PathBuf,
}
