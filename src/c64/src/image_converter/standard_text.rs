//! Convert image to be used in one of the text or graphics mode of the C64.

use c64_charset::{
    chars::{Char, Chars},
    charset::{petscii_to_bits, Charset},
};
use c64_colors::colors::{Color, Histogram, SRGB};

use crate::image_container::{
    difference, standard_char::StandardCharImage, Image, SRGBImageContainer, StandardCharacterImage,
};

use super::ImageConverter;

pub enum ConversionQuality {
    /// Convert color to a bit value based on its luminance.
    EachChar,

    /// Reduce errors by finding the solution with least amount of errors by going over each Character and Foreground color.
    EachCharAndForegroundColor,

    EachCharAndColor,

    CustomCharAndColor,
}

impl Default for ConversionQuality {
    fn default() -> Self {
        ConversionQuality::EachChar
    }
}

/// Converter to convert an input image to standard character mode of the C64.
#[derive(Default)]
pub struct StandardCharacterMode {
    pub charset: Charset,
    pub quality: ConversionQuality,
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

    fn extract_each_char(&self, image: &dyn Image) -> <Self as ImageConverter>::ResultType {
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
                let foreground_color = if let Some(c) = SRGB::average(&char_foreground_colors) {
                    Color::from(c)
                } else {
                    background_color
                };
                foreground_colors.push(foreground_color);
            }
        }

        StandardCharacterImage {
            height: height / 8,
            width: width / 8,
            characters: petscii_chars,
            foreground_colors,
            background_color,
            charset: Chars::from(self.charset),
        }
    }

    pub fn find_best_matching_petscii_char_and_color(
        &self,
        background_color: SRGB,
        input_colors: &SRGBImageContainer,
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
                for (a, b) in test_colors.iter().zip(&input_colors.buffer) {
                    distance += a.distance(*b);
                }
                (color_index, char_index, distance)
            })
            .min_by_key(|(_color_index, _char_index, distance)| *distance)
            .map(|(color_index, char_index, _distance)| (char_index as u8, color_index as u8))
            .unwrap()
    }

    fn extract_each_char_and_foreground_color_with_background(
        &self,
        image: &dyn Image,
        background_color: Color,
    ) -> <Self as ImageConverter>::ResultType {
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
        let chars = Chars::from(self.charset);
        for char_index in 0..255 {
            let petscii_bits = Vec::<bool>::from(chars.get_char(char_index));
            all_chars_to_check.push(petscii_bits);
        }

        for y in 0..height / 8 {
            for x in 0..width / 8 {
                let tile = image.sub_image(x * 8, y * 8, 8, 8);
                let best_match = self.find_best_matching_petscii_char_and_color(
                    SRGB::from(background_color),
                    &tile,
                    &all_colors_to_check,
                    &all_chars_to_check,
                );
                petscii_chars.push(best_match.0);
                foreground_colors.push(Color::from(best_match.1));
            }
        }

        StandardCharacterImage {
            height: height / 8,
            width: width / 8,
            characters: petscii_chars,
            foreground_colors,
            background_color,
            charset: Chars::from(self.charset),
        }
    }

    fn extract_each_char_and_foreground_color(&self, image: &dyn Image) -> <Self as ImageConverter>::ResultType {
        let background_color = StandardCharacterMode::find_best_background_color(image);
        self.extract_each_char_and_foreground_color_with_background(image, background_color)
    }

    fn extract_each_char_and_color(&self, image: &dyn Image) -> <Self as ImageConverter>::ResultType {
        (0_u8..16)
            .map(|background_index| Color::from(background_index))
            .map(|background_color| {
                self.extract_each_char_and_foreground_color_with_background(image, background_color)
            })
            .min_by_key(|result| difference(image, result))
            .unwrap()
    }

    fn generate_char_for(
        tile_to_match: &SRGBImageContainer,
        background_color: Color,
        foreground_color: Color,
    ) -> StandardCharImage {
        assert_eq!(tile_to_match.width(), 8);
        assert_eq!(tile_to_match.height(), 8);

        let mut ch = Char::default();
        let foreground_srgb = SRGB::from(foreground_color);
        let background_srgb = SRGB::from(background_color);

        for y in 0..8 {
            let row = tile_to_match.extract_row(y);
            let mut row_b = 0_u8;
            for c in row {
                let bit = if c.distance(foreground_srgb) < c.distance(background_srgb) {
                    1
                } else {
                    0
                };
                row_b = (row_b << 1) | bit;
            }
            ch.bytes[y] = row_b;
        }
        StandardCharImage::new(ch, foreground_color, background_color)
    }

    fn extract_custom_char_and_foreground_color_with_background(
        &self,
        image: &dyn Image,
        background_color: Color,
    ) -> <Self as ImageConverter>::ResultType {
        let mut char_list = Vec::new();
        let mut foreground_colors = Vec::new();
        let mut chars = Chars::default();
        let height = image.height();
        let width = image.width();

        let all_colors_to_check = Color::all();

        for y in 0..height / 8 {
            for x in 0..width / 8 {
                let tile = image.sub_image(x * 8, y * 8, 8, 8);
                let best_char = all_colors_to_check
                    .iter()
                    .map(|foreground_color| {
                        StandardCharacterMode::generate_char_for(&tile, background_color, *foreground_color)
                    })
                    .min_by_key(|ch| difference(&tile, ch))
                    .unwrap();

                let ch = best_char.get_char();
                if !chars.contains(ch) {
                    chars.add_char(ch);
                }
                let char_index = chars.index_of(ch).unwrap();
                char_list.push(char_index);
                foreground_colors.push(best_char.get_foreground_color());
            }
        }

        println!("{:?}, {}", background_color, chars.len());
        let vicii_char_list = chars.compress(&mut char_list, 256);

        StandardCharacterImage {
            height: height / 8,
            width: width / 8,
            characters: vicii_char_list,
            foreground_colors,
            background_color,
            charset: chars,
        }
    }

    fn extract_custom_char_and_color(&self, image: &dyn Image) -> <Self as ImageConverter>::ResultType {
        (0_u8..16)
            .map(|background_index| Color::from(background_index))
            .map(|background_color| {
                self.extract_custom_char_and_foreground_color_with_background(image, background_color)
            })
            .min_by_key(|result| difference(image, result))
            .unwrap()
    }
}

impl ImageConverter for StandardCharacterMode {
    type ResultType = StandardCharacterImage;

    fn convert(&self, input: &dyn Image) -> Self::ResultType {
        assert_eq!(input.width() % 8, 0);
        assert_eq!(input.height() % 8, 0);
        match self.quality {
            ConversionQuality::EachChar => self.extract_each_char(input),
            ConversionQuality::EachCharAndForegroundColor => self.extract_each_char_and_foreground_color(input),
            ConversionQuality::EachCharAndColor => self.extract_each_char_and_color(input),
            ConversionQuality::CustomCharAndColor => self.extract_custom_char_and_color(input),
        }
    }
}
