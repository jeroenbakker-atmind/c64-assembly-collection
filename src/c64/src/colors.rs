use std::collections::HashMap;

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum Color {
    Black,
    White,
    Red,
    Cyan,
    Purple,
    Green,
    Blue,
    Yellow,
    Orange,
    Brown,
    LightRed,
    DarkGrey,
    Grey,
    LightGreen,
    LightBlue,
    LightGrey,
}

impl Color {
    pub fn all() -> [Color; 16] {
        [
            Color::Black,
            Color::White,
            Color::Red,
            Color::Cyan,
            Color::Purple,
            Color::Green,
            Color::Blue,
            Color::Yellow,
            Color::Orange,
            Color::Brown,
            Color::LightRed,
            Color::DarkGrey,
            Color::Grey,
            Color::LightGreen,
            Color::LightBlue,
            Color::LightGrey,
        ]
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<u8> for Color {
    fn from(src: u8) -> Self {
        match src {
            0 => Color::Black,
            1 => Color::White,
            2 => Color::Red,
            3 => Color::Cyan,
            4 => Color::Purple,
            5 => Color::Green,
            6 => Color::Blue,
            7 => Color::Yellow,
            8 => Color::Orange,
            9 => Color::Brown,
            10 => Color::LightRed,
            11 => Color::DarkGrey,
            12 => Color::Grey,
            13 => Color::LightGreen,
            14 => Color::LightBlue,
            15 => Color::LightGrey,
            _ => panic!(),
        }
    }
}

impl From<Color> for u8 {
    fn from(src: Color) -> Self {
        match src {
            Color::Black => 0,
            Color::White => 1,
            Color::Red => 2,
            Color::Cyan => 3,
            Color::Purple => 4,
            Color::Green => 5,
            Color::Blue => 6,
            Color::Yellow => 7,
            Color::Orange => 8,
            Color::Brown => 9,
            Color::LightRed => 10,
            Color::DarkGrey => 11,
            Color::Grey => 12,
            Color::LightGreen => 13,
            Color::LightBlue => 14,
            Color::LightGrey => 15,
        }
    }
}

impl SRGB {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> SRGB {
        SRGB { r, g, b }
    }

    pub fn average(colors: &[SRGB]) -> Option<SRGB> {
        if colors.len() == 0 {
            return None;
        }
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for color in colors {
            r += color.r as usize;
            g += color.g as usize;
            b += color.b as usize;
        }
        Some(SRGB::from_rgb(
            (r / colors.len()) as u8,
            (g / colors.len()) as u8,
            (b / colors.len()) as u8,
        ))
    }

    pub fn distance(&self, other: SRGB) -> usize {
        ((self.r as i32 - other.r as i32).abs()
            + (self.g as i32 - other.g as i32).abs()
            + (self.b as i32 - other.b as i32).abs()) as usize
    }
}

impl From<Color> for SRGB {
    fn from(src: Color) -> Self {
        match src {
            Color::Black => SRGB::from_rgb(0, 0, 0),
            Color::White => SRGB::from_rgb(255, 255, 255),
            Color::Red => SRGB::from_rgb(136, 0, 0),
            Color::Cyan => SRGB::from_rgb(170, 255, 238),
            Color::Purple => SRGB::from_rgb(204, 68, 204),
            Color::Green => SRGB::from_rgb(0, 204, 85),
            Color::Blue => SRGB::from_rgb(0, 0, 170),
            Color::Yellow => SRGB::from_rgb(238, 238, 119),
            Color::Orange => SRGB::from_rgb(221, 136, 85),
            Color::Brown => SRGB::from_rgb(102, 68, 0),
            Color::LightRed => SRGB::from_rgb(255, 119, 119),
            Color::DarkGrey => SRGB::from_rgb(51, 51, 51),
            Color::Grey => SRGB::from_rgb(119, 119, 119),
            Color::LightGreen => SRGB::from_rgb(170, 255, 102),
            Color::LightBlue => SRGB::from_rgb(0, 136, 255),
            Color::LightGrey => SRGB::from_rgb(187, 187, 187),
        }
    }
}

impl From<SRGB> for Color {
    fn from(a: SRGB) -> Self {
        (0_u8..16)
            .map(|v| Color::from(v))
            .min_by_key(|c| SRGB::from(*c).distance(a))
            .unwrap()
    }
}

#[derive(Debug, Default)]
pub struct Histogram {
    pub data: HashMap<Color, usize>,
}

impl Histogram {
    pub fn add(&mut self, srgb_color: SRGB) {
        let key = Color::from(srgb_color);
        *self.data.entry(key).or_default() += 1;
    }
}
