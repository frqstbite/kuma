pub struct True {
    r: u8,
    g: u8,
    b: u8,
}

pub enum Ansi16Hue {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

struct Ansi16 {
    bright: bool,
    hue: Ansi16Hue,
}

pub enum Color {
    True(True),
    Ansi256(u8),
    Ansi16(Ansi16),
    Mono(bool),
}