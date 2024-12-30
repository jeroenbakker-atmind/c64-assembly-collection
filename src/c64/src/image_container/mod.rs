use c64_charset::chars::Chars;
use c64_colors::colors::{Color, SRGB};


pub mod palette4;
pub mod standard_char;

#[cfg(test)]
mod palette4_test;

pub trait Image {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get_pixel_color(&self, x: usize, y: usize) -> SRGB;
    fn sub_image(
        &self,
        x_start: usize,
        y_start: usize,
        width: usize,
        height: usize,
    ) -> SRGBImageContainer {
        let mut colors = Vec::new();
        let x_end = x_start + width;
        let y_end = y_start + height;
        for iy in y_start..y_end {
            for ix in x_start..x_end {
                let srgb_color = self.get_pixel_color(ix, iy);
                colors.push(srgb_color);
            }
        }
        SRGBImageContainer {
            width,
            height,
            buffer: colors,
        }
    }

    // TODO: RowView?
    fn extract_row(&self, y: usize) -> Vec<SRGB> {
        let mut result = Vec::new();
        for x in 0..self.width() {
            result.push(self.get_pixel_color(x, y));
        }
        result
    }
}

impl Image for Vec<SRGB> {
    fn width(&self) -> usize {
        self.len()
    }
    fn height(&self) -> usize {
        1
    }
    fn get_pixel_color(&self, x: usize, _y: usize) -> SRGB {
        self[x]
    }
}

pub fn difference(a: &dyn Image, b: &dyn Image) -> usize {
    assert_eq!(a.width(), b.width());
    assert_eq!(a.height(), b.height());

    let mut result = 0;
    for x in 0..a.width() {
        for y in 0..a.height() {
            let ac = a.get_pixel_color(x, y);
            let bc = b.get_pixel_color(x, y);
            result += ac.distance(bc);
        }
    }
    result
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

pub struct SRGBImageContainer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<SRGB>,
}

impl Image for SRGBImageContainer {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get_pixel_color(&self, x: usize, y: usize) -> SRGB {
        let offset = y * self.width() + x;
        self.buffer[offset]
    }
}

pub struct StandardCharacterImage {
    pub width: usize,
    pub height: usize,
    pub characters: Vec<u8>,
    pub foreground_colors: Vec<Color>,
    pub background_color: Color,
    pub charset: Chars,
}

impl Image for StandardCharacterImage {
    fn height(&self) -> usize {
        self.height * 8
    }
    fn width(&self) -> usize {
        self.width * 8
    }

    fn get_pixel_color(&self, x: usize, y: usize) -> SRGB {
        let char_x = x / 8;
        let char_y = y / 8;
        let char_offset = char_y * self.width + char_x;
        let char_index = self.characters[char_offset];
        let bits = Vec::<bool>::from(self.charset.get_char(char_index as usize));
        let char_x_rest = x % 8;
        let char_y_rest = y % 8;
        let bit_offset = char_y_rest * 8 + char_x_rest;
        let bit = bits[bit_offset];
        if bit {
            SRGB::from(self.foreground_colors[char_offset])
        } else {
            SRGB::from(self.background_color)
        }
    }
}

pub struct StandardBitmapImage {
    pub width: usize,
    pub height: usize,
    pub bitmap: Vec<u8>,
    pub colors: Vec<u8>,
}
