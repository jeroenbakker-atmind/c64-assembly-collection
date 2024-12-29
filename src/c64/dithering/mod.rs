use crate::colors::SRGB;
pub mod dithering_mask;
pub mod no_dithering;

pub trait Dithering {
    fn dither(&self, pixel_x: usize, pixel_y: usize, color: SRGB) -> SRGB;
}
