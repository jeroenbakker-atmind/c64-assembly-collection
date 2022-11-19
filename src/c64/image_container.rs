use crate::colors::{Color, SRGB};

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

pub struct StandardCharacterImage {
    pub width: usize,
    pub height: usize,
    pub characters: Vec<u8>,
    pub foreground_colors: Vec<Color>,
    pub background_color: Color,
}
