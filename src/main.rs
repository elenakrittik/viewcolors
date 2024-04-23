use yansi::{Color, Paint};

const PAIRS: [(&str, Color); 9] = [
    ("Black", Color::Black),
    ("Primary", Color::Primary),
    ("Red", Color::Red),
    ("Green", Color::Green),
    ("Yellow", Color::Yellow),
    ("Blue", Color::Blue),
    ("Magenta", Color::Magenta),
    ("Cyan", Color::Cyan),
    ("White", Color::White),
];

fn main() {
    for (text, color) in PAIRS {
        eprintln!("This is what {} looks like!", text.fg(color));
    }
}
