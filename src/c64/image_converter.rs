//! Convert image to be used in one of the text or graphics mode of the C64.

use crate::charset::{petscii_to_bits, Charset};
use crate::colors::Color;

pub trait Image {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get_pixel_luminosity(&self, x: usize, y: usize) -> u8;
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
    fn get_pixel_luminosity(&self, x: usize, y: usize) -> u8 {
        let offset = y * self.width() + x;
        self.buffer[offset * self.components_per_pixel]
    }
}

pub trait ImageConverter {
    type ResultType;

    fn convert(&self, input: &dyn Image) -> Self::ResultType;
}

pub struct StandardCharacterModeResult {
    pub characters: Vec<u8>,
    pub foreground_colors: Vec<Color>,
    pub background_color: Color,
}

/// Converter to convert an input image to standard character mode of the C64.
#[derive(Default)]
pub struct StandardCharacterMode {
    pub charset: Charset,
}

impl StandardCharacterMode {
    /// Distance function between the two given bitvecs.
    pub fn bitvec_difference(a: &Vec<bool>, b: &Vec<bool>) -> u32 {
        let mut difference = 0;
        for (ab, bb) in a.iter().zip(b) {
            if ab != bb {
                difference += 1
            }
        }
        difference
    }

    /// Return the petscii char that matches the input bitvec the closest.
    pub fn find_best_matching_petscii_char(&self, input_bits: &Vec<bool>) -> u8 {
        let mut checks = Vec::new();
        for petscii_char in 0..=255 {
            let petscii_bits = petscii_to_bits(self.charset, petscii_char);
            checks.push((petscii_char, petscii_bits));
        }

        let min = checks
            .iter()
            .min_by_key(|a| StandardCharacterMode::bitvec_difference(input_bits, &a.1))
            .unwrap();
        min.0
    }
}

impl ImageConverter for StandardCharacterMode {
    type ResultType = StandardCharacterModeResult;

    fn convert(&self, input: &dyn Image) -> Self::ResultType {
        assert_eq!(input.width() % 8, 0);
        assert_eq!(input.height() % 8, 0);

        // TODO: determine best background color.
        // TODO: determine best foregroung color for each character.

        let mut petscii_chars = Vec::new();
        let height = input.height();
        let width = input.width();

        for y in 0..height / 8 {
            for x in 0..width / 8 {
                let mut bits = Vec::new();
                for iy in 0..8 {
                    for ix in 0..8 {
                        let xo = ix + x * 8;
                        let yo = iy + y * 8;
                        let byte = input.get_pixel_luminosity(xo, yo);
                        let bit = if byte > 127 { false } else { true };
                        bits.push(bit);
                    }
                }
                let best_match = self.find_best_matching_petscii_char(&bits);
                petscii_chars.push(best_match);
            }
        }

        let mut foreground_colors = Vec::new();
        foreground_colors.resize(petscii_chars.len(), Color::White);
        let background_color = Color::Black;

        StandardCharacterModeResult {
            characters: petscii_chars,
             foreground_colors,
             background_color,
        }


    }
}
