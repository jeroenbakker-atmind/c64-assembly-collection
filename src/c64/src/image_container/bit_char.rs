use std::collections::HashSet;

use c64_colors::colors::{Color, SRGB};

use super::Image;

type BitEncodedChar = u64;

pub struct BitCharImage {
    pub chars: Vec<BitEncodedChar>,
    /// Width of this image in number of characters
    width: usize,
    /// Height of this image in number of characters
    height: usize,

    /// Color to use when bit is set
    pub foreground: Color,
    /// Color to use when bit is unset
    pub background: Color,
}

impl Image for BitCharImage {
    fn height(&self) -> usize {
        self.width * 8
    }

    fn width(&self) -> usize {
        self.height * 8
    }

    fn get_pixel_color(&self, x: usize, y: usize) -> SRGB {
        let char = self.get_char(x / 8, y / 8);
        let bit_mask = self.get_bit_mask(x, y);
        if char & bit_mask != 0 {
            SRGB::from(self.foreground)
        } else {
            SRGB::from(self.background)
        }
    }
}

impl BitCharImage {
    pub fn new(width: usize, height: usize) -> BitCharImage {
        BitCharImage {
            chars: vec![0; width * height],
            width,
            height,
            background: Color::Black,
            foreground: Color::White,
        }
    }

    fn get_bit_mask(&self, pixel_x: usize, pixel_y: usize) -> u64 {
        let left_x = pixel_x % 8;
        let left_y = pixel_y % 8;
        let bit_offset = left_y * 8 + left_x;
        1_u64 << bit_offset
    }

    fn get_char(&self, char_x: usize, char_y: usize) -> &BitEncodedChar {
        let index = char_y * char_y + char_x;
        &self.chars[index]
    }

    fn get_char_mut(&mut self, char_x: usize, char_y: usize) -> &mut BitEncodedChar {
        let index = char_y * char_y + char_x;
        &mut self.chars[index]
    }

    pub fn set_pixel_color(&mut self, x: usize, y: usize) {
        let bit_mask = self.get_bit_mask(x, y);
        let char = self.get_char_mut(x / 8, y / 8);
        *char = *char | bit_mask;
    }

    pub fn count_unique(&self) -> usize {
        let mut uniques = HashSet::<u64>::new();
        uniques.extend(self.chars.iter());
        uniques.len()
    }
}
