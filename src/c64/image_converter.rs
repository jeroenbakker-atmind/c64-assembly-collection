//! Convert image to be used in one of the text or graphics mode of the C64.

use crate::charset::{petscii_to_bits, Charset};
use crate::colors::{average_color, Color, Histogram, SRGB};

pub trait Image {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get_pixel_luminosity(&self, x: usize, y: usize) -> u8;
    fn get_pixel_color(&self, x: usize, y: usize) -> SRGB;
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

pub trait ImageConverter {
    type ResultType;

    fn convert(&self, input: &dyn Image) -> Self::ResultType;
}

pub struct StandardCharacterModeResult {
    pub characters: Vec<u8>,
    pub foreground_colors: Vec<Color>,
    pub background_color: Color,
}

pub enum ConversionMode {
    /// Convert color to a bit value based on its luminence.
    Bit,

    /// Reduce errors by finding the solution with least amount of errors.
    Distance,
}

impl Default for ConversionMode {
    fn default() -> Self {
        ConversionMode::Bit
    }
}

/// Converter to convert an input image to standard character mode of the C64.
#[derive(Default)]
pub struct StandardCharacterMode {
    pub charset: Charset,
    pub mode: ConversionMode,
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

    fn color_histogram(image: &dyn Image) -> Histogram {
        let mut result = Histogram::default();

        for x in 0..image.width() {
            for y in 0..image.height() {
                let srgb_color = image.get_pixel_color(x, y);
                result.add(srgb_color);
            }
        }

        result
    }

    pub fn find_best_background_color(image: &dyn Image) -> Color {
        let histogram = StandardCharacterMode::color_histogram(image);
        *(histogram.data.iter().max_by_key(|(_k, v)| *v).unwrap().0)
    }

    fn extract_luminance(&self, image: &dyn Image) -> <Self as ImageConverter>::ResultType {
        let background_color = StandardCharacterMode::find_best_background_color(image);

        let mut petscii_chars = Vec::new();
        let mut foreground_colors = Vec::new();
        let height = image.height();
        let width = image.width();

        for y in 0..height / 8 {
            for x in 0..width / 8 {
                let mut bits = Vec::new();
                let mut char_foreground_colors = Vec::new();
                for iy in 0..8 {
                    for ix in 0..8 {
                        let xo = ix + x * 8;
                        let yo = iy + y * 8;
                        let srgb_color = image.get_pixel_color(xo, yo);
                        let color = Color::from(srgb_color);
                        let bit = color != background_color;
                        bits.push(bit);
                        if bit {
                            char_foreground_colors.push(srgb_color);
                        }
                    }
                }
                let best_match = self.find_best_matching_petscii_char(&bits);

                petscii_chars.push(best_match);
                let foreground_color = if let Some(c) = average_color(&char_foreground_colors) {
                    Color::from(c)
                } else {
                    background_color
                };
                foreground_colors.push(foreground_color);
            }
        }

        StandardCharacterModeResult {
            characters: petscii_chars,
            foreground_colors,
            background_color,
        }
    }

    pub fn find_best_matching_petscii_char_and_color(
        &self,
        background_color: SRGB,
        input_colors: &Vec<SRGB>,
        colors_to_check: &[SRGB],
        chars_to_check: &[Vec<bool>],
    ) -> (u8, u8) {
        let mut permutations = Vec::new();
        for color_index in 0..colors_to_check.len() {
            for char_index in 0..chars_to_check.len() {
                permutations.push((color_index, char_index));
            }
        }

        permutations
            .iter()
            .map(|(color_index, char_index)| {
                (
                    *color_index,
                    &colors_to_check[*color_index],
                    *char_index,
                    &chars_to_check[*char_index],
                )
            })
            .map(|(color_index, color, char_index, char_bits)| {
                let mut test_colors = Vec::new();
                for bit in char_bits {
                    if *bit {
                        test_colors.push(*color);
                    } else {
                        test_colors.push(background_color);
                    }
                }
                (color_index, char_index, test_colors)
            })
            .map(|(color_index, char_index, test_colors)| {
                let mut distance = 0;
                for (a, b) in test_colors.iter().zip(input_colors) {
                    distance += a.distance(*b);
                }
                (color_index, char_index, distance)
            })
            .min_by_key(|(_color_index, _char_index, distance)| *distance)
            .map(|(color_index, char_index, _distance)| (color_index as u8, char_index as u8))
            .unwrap()
    }

    fn extract_distance(&self, image: &dyn Image) -> <Self as ImageConverter>::ResultType {
        let background_color = StandardCharacterMode::find_best_background_color(image);

        let mut petscii_chars = Vec::new();
        let mut foreground_colors = Vec::new();
        let height = image.height();
        let width = image.width();

        let mut all_colors_to_check = Vec::new();
        for i in 0..16 {
            let color = Color::from(i);
            all_colors_to_check.push(SRGB::from(color));
        }
        let mut all_chars_to_check = Vec::new();
        for i in 0..255 {
            let petscii_bits = petscii_to_bits(self.charset, i);
            all_chars_to_check.push(petscii_bits);
        }

        for y in 0..height / 8 {
            for x in 0..width / 8 {
                let mut colors = Vec::new();
                for iy in 0..8 {
                    for ix in 0..8 {
                        let xo = ix + x * 8;
                        let yo = iy + y * 8;
                        let srgb_color = image.get_pixel_color(xo, yo);
                        colors.push(srgb_color);
                    }
                }
                let best_match = self.find_best_matching_petscii_char_and_color(
                    SRGB::from(background_color),
                    &colors,
                    &all_colors_to_check,
                    &all_chars_to_check,
                );
                println!("{:?}", best_match);
                petscii_chars.push(best_match.0);
                foreground_colors.push(Color::from(best_match.1));
            }
        }

        StandardCharacterModeResult {
            characters: petscii_chars,
            foreground_colors,
            background_color,
        }
    }
}

impl ImageConverter for StandardCharacterMode {
    type ResultType = StandardCharacterModeResult;

    fn convert(&self, input: &dyn Image) -> Self::ResultType {
        assert_eq!(input.width() % 8, 0);
        assert_eq!(input.height() % 8, 0);
        match self.mode {
            ConversionMode::Bit => self.extract_luminance(input),
            ConversionMode::Distance => self.extract_distance(input),
        }
    }
}
