#[derive(Copy, Clone)]
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

pub struct SRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl SRGB {
    pub fn from_rgb(r: u8, g:u8, b:u8) -> SRGB {
        SRGB{r, g, b}
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