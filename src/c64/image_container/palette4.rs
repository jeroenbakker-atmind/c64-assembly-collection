use crate::{
    colors::SRGB,
    palette::{Palette4, PaletteIndex},
};

use super::Image;

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
