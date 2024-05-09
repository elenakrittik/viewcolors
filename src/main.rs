#![feature(iter_next_chunk)]

use yansi::{Color, Paint};

const COLORS: [Color; 16] = [
    Color::Black,
    Color::BrightBlack,
    Color::Red,
    Color::BrightRed,
    Color::Green,
    Color::BrightGreen,
    Color::Yellow,
    Color::BrightYellow,
    Color::Blue,
    Color::BrightBlue,
    Color::Magenta,
    Color::BrightMagenta,
    Color::Cyan,
    Color::BrightCyan,
    Color::White,
    Color::BrightWhite,
];

fn main() {
    eprintln!(" Normal | Bright       ");
    eprintln!("------- | -------------");

    let mut colors = COLORS.into_iter();

    while let Ok([normal, bright]) = colors.next_chunk::<2>() {
        // HACK: workaround for std::fmt not respecting width in {:?}
        {
            let padding = 7 - format!("{:?}", normal).len();
            //            ^   ----------------------- easiest way to do ".to_debug_string()"
            //            |
            //            length of "Normal" underline

            eprint!("{}", " ".repeat(padding));
        }

        eprint!("{:?}", normal.fg(normal).bold());
        eprint!(" | ");
        eprint!("{:?}", bright.fg(bright).bold());
        eprintln!();
    }
}
