use c64_colors::colors::SRGB;

use super::Dithering;

pub trait DitherPattern {
    fn threshold(&self, pixel_x: usize, pixel_y: usize) -> u8;
}

pub struct Bayer1x1 {}
impl DitherPattern for Bayer1x1 {
    fn threshold(&self, _pixel_x: usize, _pixel_y: usize) -> u8 {
        128
    }
}

pub struct Bayer2x2 {}
impl DitherPattern for Bayer2x2 {
    fn threshold(&self, pixel_x: usize, pixel_y: usize) -> u8 {
        let dither_x = pixel_x % 2;
        let dither_y = pixel_y % 2;
        let dither_offset = dither_x + dither_y * 2;
        [32, 224, 160, 96][dither_offset]
    }
}

pub struct Bayer4x4 {}
impl DitherPattern for Bayer4x4 {
    fn threshold(&self, pixel_x: usize, pixel_y: usize) -> u8 {
        let dither_x = pixel_x % 4;
        let dither_y = pixel_y % 4;
        let dither_offset = dither_x + dither_y * 4;
        [8, 136, 40, 168, 200, 72, 232, 104, 56, 184, 24, 152, 248, 120, 216, 88][dither_offset]
    }
}

pub struct ThresholdDithering<Pattern>
where
    Pattern: DitherPattern,
{
    pub color_dark: SRGB,
    pub color_bright: SRGB,
    pub dither_pattern: Pattern,
}

impl<Pattern> Dithering for ThresholdDithering<Pattern>
where
    Pattern: DitherPattern,
{
    fn dither(&self, pixel_x: usize, pixel_y: usize, color: SRGB) -> SRGB {
        let threshold = self.dither_pattern.threshold(pixel_x, pixel_y);
        let intensity = ((color.r as u32 + color.g as u32 + color.b as u32) / 3) as u8;

        if intensity > threshold {
            self.color_bright
        } else {
            self.color_dark
        }
    }
}
