use crate::colors::SRGB;

use super::Dithering;

pub struct DitheringMask {
    pub width: usize,
    pub height: usize,
    pub mask: Vec<i8>,
}

impl DitheringMask {
    pub fn dither_2x2() -> DitheringMask {
        DitheringMask {
            width: 2,
            height: 2,
            mask: vec![-24, 48, -48, 24],
        }
    }
}

impl Dithering for DitheringMask {
    fn dither(
        &self,
        pixel_x: usize,
        pixel_y: usize,
        color: crate::colors::SRGB,
    ) -> crate::colors::SRGB {
        let dither_x = pixel_x % self.width;
        let dither_y = pixel_y % self.height;

        let dither_offset = self.mask[dither_y * self.width + dither_x];
        if dither_offset < 0 {
            let dither_offset = (-dither_offset) as u8;
            let dither_channel_fn = |channel: u8| {
                if channel < dither_offset {
                    0
                } else {
                    channel - dither_offset
                }
            };
            SRGB {
                r: dither_channel_fn(color.r),
                g: dither_channel_fn(color.g),
                b: dither_channel_fn(color.b),
            }
        } else {
            let dither_offset = dither_offset as u8;
            let dither_channel_fn = |channel: u8| {
                if channel > 255 - dither_offset {
                    255
                } else {
                    channel + dither_offset
                }
            };
            SRGB {
                r: dither_channel_fn(color.r),
                g: dither_channel_fn(color.g),
                b: dither_channel_fn(color.b),
            }
        }
    }
}
