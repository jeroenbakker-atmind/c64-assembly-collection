//! Convert image to be used in one of the text or graphics mode of the C64.

use crate::{
    colors::{Color, SRGB},
    image_container::{difference, Image, StandardBitmapImage},
};

use super::ImageConverter;

/// Converter to convert an input image to standard character mode of the C64.
#[derive(Default)]
pub struct StandardBitmapMode {
    pub double_colors: bool,
}

impl StandardBitmapMode {
    fn convert_single_color(&self, input: &dyn Image) -> StandardBitmapImage {
        let width = input.width();
        let height = input.height();

        #[derive(Clone)]
        struct Solution {
            foreground_color: Color,
            background_color: Color,
            bits: Vec<bool>,
            distance: usize,
        }

        let mut image_solution = Vec::new();

        for y1 in 0..height / 8 {
            for x1 in 0..width / 8 {
                let mut input_colors = Vec::new();
                for y in 0..8 {
                    for x in 0..8 {
                        let color = input.get_pixel_color(x1 * 8 + x, y1 * 8 + y);
                        input_colors.push(color);
                    }
                }

                let mut solutions = Vec::new();

                for foreground_index in 0..16 {
                    for background_index in 0..16 {
                        let foreground_color = Color::from(foreground_index);
                        let foreground_srgb = SRGB::from(foreground_color);
                        let background_color = Color::from(background_index);
                        let background_srgb = SRGB::from(background_color);

                        let mut bits = Vec::new();
                        for color in &input_colors {
                            let distance_to_fg = foreground_srgb.distance(*color);
                            let distance_to_bg = background_srgb.distance(*color);
                            bits.push(distance_to_fg < distance_to_bg);
                        }
                        let output_colors: Vec<SRGB> = bits
                            .iter()
                            .map(|b| if *b { foreground_srgb } else { background_srgb })
                            .collect();

                        let distance = difference(&input_colors, &output_colors);
                        solutions.push(Solution {
                            foreground_color,
                            background_color,
                            bits,
                            distance,
                        });
                    }
                }

                // Find best solution.
                image_solution.push(solutions.iter().min_by_key(|s| s.distance).unwrap().clone());
            }
        }

        let colors = image_solution
            .iter()
            .map(|s| u8::from(s.foreground_color) << 4 | u8::from(s.background_color))
            .collect();

        let mut bitmap = Vec::new();
        for y1 in 0..height / 8 {
            for x1 in 0..width / 8 {
                for y2 in 0..8 {
                    let mut bitmask = 0;
                    let offset = y1 * (width / 8) + x1;
                    let solution = &image_solution[offset];

                    for x2 in 0..8 {
                        let bit = solution.bits[y2 * 8 + x2];
                        bitmask = bitmask << 1 | if bit { 1 } else { 0 };
                    }
                    bitmap.push(bitmask);
                }
            }
        }
        StandardBitmapImage {
            bitmap,
            colors_1: colors,
            colors_2: Vec::new(),
            height,
            width,
        }
    }

    fn convert_double_color(&self, input: &dyn Image) -> StandardBitmapImage {
        let width = input.width();
        let height = input.height();

        #[derive(Clone)]
        struct Solution {
            foreground_color_1: Color,
            foreground_color_2: Color,
            background_color_1: Color,
            background_color_2: Color,
            bits: Vec<bool>,
            distance: usize,
        }

        let mut image_solution = Vec::new();

        for y1 in 0..height / 8 {
            for x1 in 0..width / 8 {
                let mut input_colors = Vec::new();
                for y in 0..8 {
                    for x in 0..8 {
                        let color = input.get_pixel_color(x1 * 8 + x, y1 * 8 + y);
                        input_colors.push(color);
                    }
                }

                let mut solutions = Vec::new();

                for foreground_index_1 in 0..16 {
                    for foreground_index_2 in 0..16 {
                        for background_index_1 in 0..16 {
                            for background_index_2 in 0..16 {
                                let foreground_color_1 = Color::from(foreground_index_1);
                                let foreground_color_2 = Color::from(foreground_index_2);
                                let foreground_srgb = SRGB::average(&[
                                    SRGB::from(foreground_color_1),
                                    SRGB::from(foreground_color_2),
                                ])
                                .unwrap();
                                let background_color_1 = Color::from(background_index_1);
                                let background_color_2 = Color::from(background_index_2);
                                let background_srgb = SRGB::average(&[
                                    SRGB::from(background_color_1),
                                    SRGB::from(background_color_2),
                                ])
                                .unwrap();

                                let mut bits = Vec::new();
                                for color in &input_colors {
                                    let distance_to_fg = foreground_srgb.distance(*color);
                                    let distance_to_bg = background_srgb.distance(*color);
                                    bits.push(distance_to_fg < distance_to_bg);
                                }
                                let output_colors: Vec<SRGB> = bits
                                    .iter()
                                    .map(|b| if *b { foreground_srgb } else { background_srgb })
                                    .collect();

                                let distance = difference(&input_colors, &output_colors);
                                solutions.push(Solution {
                                    foreground_color_1,
                                    foreground_color_2,
                                    background_color_1,
                                    background_color_2,
                                    bits,
                                    distance,
                                });
                            }
                        }
                    }
                }

                // Find best solution.
                image_solution.push(solutions.iter().min_by_key(|s| s.distance).unwrap().clone());
            }
        }

        let colors_1 = image_solution
            .iter()
            .map(|s| u8::from(s.foreground_color_1) << 4 | u8::from(s.background_color_1))
            .collect();
        let colors_2 = image_solution
            .iter()
            .map(|s| u8::from(s.foreground_color_2) << 4 | u8::from(s.background_color_2))
            .collect();

        let mut bitmap = Vec::new();
        for y1 in 0..height / 8 {
            for x1 in 0..width / 8 {
                for y2 in 0..8 {
                    let mut bitmask = 0;
                    let offset = y1 * (width / 8) + x1;
                    let solution = &image_solution[offset];

                    for x2 in 0..8 {
                        let bit = solution.bits[y2 * 8 + x2];
                        bitmask = bitmask << 1 | if bit { 1 } else { 0 };
                    }
                    bitmap.push(bitmask);
                }
            }
        }
        StandardBitmapImage {
            bitmap,
            colors_1: colors_1,
            colors_2: colors_2,
            height,
            width,
        }
    }
}

impl ImageConverter for StandardBitmapMode {
    type ResultType = StandardBitmapImage;

    fn convert(&self, input: &dyn Image) -> Self::ResultType {
        assert_eq!(input.width() % 8, 0);
        assert_eq!(input.height() % 8, 0);

        match self.double_colors {
            false => self.convert_single_color(input),
            true => self.convert_double_color(input),
        }
    }
}
