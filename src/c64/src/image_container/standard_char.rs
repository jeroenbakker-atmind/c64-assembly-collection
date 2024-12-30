use c64_charset::chars::Char;
use c64_colors::colors::{Color, SRGB};

use super::Image;

pub struct StandardCharImage {
    ch: Char,
    foreground: Color,
    background: Color,
}
impl StandardCharImage {
    pub fn get_char(&self) -> Char {
        self.ch
    }
    pub fn get_foreground_color(&self) -> Color {
        self.foreground
    }
    pub fn get_background_color(&self) -> Color {
        self.background
    }
}

impl StandardCharImage {
    pub fn new(ch: Char, foreground: Color, background: Color) -> StandardCharImage {
        StandardCharImage {
            ch,
            foreground,
            background,
        }
    }
}

impl Image for StandardCharImage {
    fn height(&self) -> usize {
        8
    }
    fn width(&self) -> usize {
        8
    }
    fn get_pixel_color(&self, x: usize, y: usize) -> SRGB {
        SRGB::from(if self.ch.is_bit_set(x, y) {
            self.foreground
        } else {
            self.background
        })
    }
}
