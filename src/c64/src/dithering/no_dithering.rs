use crate::colors::SRGB;

use super::Dithering;

pub struct NoDithering {}
impl Dithering for NoDithering {
    fn dither(&self, _pixel_x: usize, _pixel_y: usize, color: SRGB) -> SRGB {
        color
    }
}
