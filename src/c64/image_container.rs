use std::path::Path;

use crate::{
    chars::Chars,
    colors::{Color, SRGB},
    palette::{Palette4, PaletteIndex},
};

pub trait Image {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get_pixel_color(&self, x: usize, y: usize) -> SRGB;
    fn sub_image(
        &self,
        x_start: usize,
        y_start: usize,
        width: usize,
        height: usize,
    ) -> SRGBImageContainer {
        let mut colors = Vec::new();
        let x_end = x_start + width;
        let y_end = y_start + height;
        for iy in y_start..y_end {
            for ix in x_start..x_end {
                let srgb_color = self.get_pixel_color(ix, iy);
                colors.push(srgb_color);
            }
        }
        SRGBImageContainer {
            width,
            height,
            buffer: colors,
        }
    }

    // TODO: RowView?
    fn extract_row(&self, y: usize) -> Vec<SRGB> {
        let mut result = Vec::new();
        for x in 0..self.width() {
            result.push(self.get_pixel_color(x, y));
        }
        result
    }
}

impl Image for Vec<SRGB> {
    fn width(&self) -> usize {
        self.len()
    }
    fn height(&self) -> usize {
        1
    }
    fn get_pixel_color(&self, x: usize, _y: usize) -> SRGB {
        self[x]
    }
}

pub fn difference(a: &dyn Image, b: &dyn Image) -> usize {
    assert_eq!(a.width(), b.width());
    assert_eq!(a.height(), b.height());

    let mut result = 0;
    for x in 0..a.width() {
        for y in 0..a.height() {
            let ac = a.get_pixel_color(x, y);
            let bc = b.get_pixel_color(x, y);
            result += ac.distance(bc);
        }
    }
    result
}

pub struct DefaultImageContainer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u8>,
    pub components_per_pixel: usize,
}

impl Image for DefaultImageContainer {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get_pixel_color(&self, x: usize, y: usize) -> SRGB {
        let offset = y * self.width() + x;
        let offset = offset * self.components_per_pixel;
        SRGB::from_rgb(
            self.buffer[offset],
            self.buffer[offset + 1],
            self.buffer[offset + 2],
        )
    }
}

pub struct SRGBImageContainer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<SRGB>,
}

impl Image for SRGBImageContainer {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get_pixel_color(&self, x: usize, y: usize) -> SRGB {
        let offset = y * self.width() + x;
        self.buffer[offset]
    }
}

pub struct StandardCharacterImage {
    pub width: usize,
    pub height: usize,
    pub characters: Vec<u8>,
    pub foreground_colors: Vec<Color>,
    pub background_color: Color,
    pub charset: Chars,
}

impl Image for StandardCharacterImage {
    fn height(&self) -> usize {
        self.height * 8
    }
    fn width(&self) -> usize {
        self.width * 8
    }

    fn get_pixel_color(&self, x: usize, y: usize) -> SRGB {
        let char_x = x / 8;
        let char_y = y / 8;
        let char_offset = char_y * self.width + char_x;
        let char_index = self.characters[char_offset];
        let bits = Vec::<bool>::from(self.charset.get_char(char_index as usize));
        let char_x_rest = x % 8;
        let char_y_rest = y % 8;
        let bit_offset = char_y_rest * 8 + char_x_rest;
        let bit = bits[bit_offset];
        if bit {
            SRGB::from(self.foreground_colors[char_offset])
        } else {
            SRGB::from(self.background_color)
        }
    }
}

pub struct StandardBitmapImage {
    pub width: usize,
    pub height: usize,
    pub bitmap: Vec<u8>,
    pub colors: Vec<u8>,
}

#[derive(Debug)]
pub struct Palette4BitmapImage {
    pub width: usize,
    pub height: usize,
    pub bitmap: Vec<u8>,
    pub palette: Palette4,
}

impl Image for Palette4BitmapImage {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get_pixel_color(&self, x: usize, y: usize) -> SRGB {
        self.palette
            .get_srgb_color(self.get_pixel_palette_index(x, y))
    }
}

impl Palette4BitmapImage {
    pub fn new(width: usize, height: usize, palette: Palette4) -> Palette4BitmapImage {
        Palette4BitmapImage {
            width,
            height,
            bitmap: vec![0; width * height / 4],
            palette,
        }
    }

    fn bitmap_index(&self, x: usize, y: usize) -> usize {
        (y * self.width / 4) + (x / 4)
    }
    fn bitmap_index_shift(&self, x: usize) -> usize {
        (3 - (x % 4)) * 2
    }
    fn bitmap_index_mask(&self, bitmap_index_shift: usize) -> u8 {
        3 << bitmap_index_shift
    }

    pub fn set_pixel_palette_index(&mut self, x: usize, y: usize, palette_index: PaletteIndex) {
        assert!((0..4).contains(&palette_index));
        let bitmap_index = self.bitmap_index(x, y);
        let bitmap_index_shift = self.bitmap_index_shift(x);
        let mask = 0xff - self.bitmap_index_mask(bitmap_index_shift);
        self.bitmap[bitmap_index] &= mask;
        self.bitmap[bitmap_index] |= palette_index << bitmap_index_shift;
    }
    pub fn get_pixel_palette_index(&self, x: usize, y: usize) -> PaletteIndex {
        let bitmap_index = self.bitmap_index(x, y);
        let bitmap_index_shift = self.bitmap_index_shift(x);
        let mask = self.bitmap_index_mask(bitmap_index_shift);

        (self.bitmap[bitmap_index] & mask) >> bitmap_index_shift
    }
}
