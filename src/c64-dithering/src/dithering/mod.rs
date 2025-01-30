use c64_colors::colors::SRGB;

pub mod bayer;
pub mod dithering_mask;
pub mod no_dithering;

pub trait Dithering {
    fn dither(&self, pixel_x: usize, pixel_y: usize, color: SRGB) -> SRGB;
}
// https://tannerhelland.com/2012/12/28/dithering-eleven-algorithms-source-code.html