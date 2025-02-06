use crate::image_container::bit_char::BitCharImage;

use super::ImageConverter;

pub struct DitheredText {}

impl ImageConverter for DitheredText {
    type ResultType = BitCharImage;

    fn convert(&self, input: &dyn crate::image_container::Image) -> Self::ResultType {
        assert!(
            input.width() == 160 && input.height() == 100,
            "Only images with resolution of 160x100 are supported."
        );
        let mut bit_char_image = BitCharImage::new(320 / 8, 200 / 8);

        for y in 0..200 {
            for x in 0..320 {
                let sx = x / 2;
                let sy = y / 2;
                let c = input.get_pixel_color(sx, sy);
                if c.r > 0x7F {
                    bit_char_image.set_pixel_color(x, y);
                }
            }
        }

        bit_char_image
    }
}
