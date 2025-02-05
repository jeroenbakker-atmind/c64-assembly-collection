use c64::image_container::Image;
use c64_colors::colors::{Color, SRGB};

#[derive(Debug, Default, Clone)]
pub struct State {
    pub charmap: CharMap,
    pub text_screen: TextScreen,
}

#[derive(Debug, Clone)]
pub struct CharMap {
    pub chars: [u64; 256],
}
impl Default for CharMap {
    fn default() -> Self {
        Self {
            chars: [0xDEADBEEFDEADBEEF; 256],
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextScreen {
    pub bytes: [u8; 1000],
}
impl Default for TextScreen {
    fn default() -> Self {
        Self { bytes: [0; 1000] }
    }
}

impl Image for State {
    fn width(&self) -> usize {
        320 + 200
    }

    fn height(&self) -> usize {
        200
    }

    fn get_pixel_color(&self, x: usize, y: usize) -> c64_colors::colors::SRGB {
        let mut result = SRGB::from(Color::Black);
        if (0..320).contains(&x) && (0..200).contains(&y) {
            result = self.text_screen.get_pixel_color(x, y, &self.charmap);
        }
        if (320..(320 + 128 + 15)).contains(&x) && (0..(128 + 15)).contains(&y) {
            result = self.charmap.get_pixel_color(x - 320, y);
        }
        result
    }
}

impl TextScreen {
    fn get_pixel_color(&self, x: usize, y: usize, char_map: &CharMap) -> SRGB {
        let index = (x / 8) + (y / 8) * 40;
        let charcode = self.bytes[index];
        let charmap_x = (charcode as usize % 16) * 9 + (x % 8);
        let charmap_y = (charcode as usize / 16) * 9 + (y % 8);
        char_map.get_pixel_color(charmap_x, charmap_y)
    }
}

impl Image for CharMap {
    fn width(&self) -> usize {
        128 + 15
    }

    fn height(&self) -> usize {
        128 + 15
    }

    fn get_pixel_color(&self, x: usize, y: usize) -> c64_colors::colors::SRGB {
        let char_code = (x / 9) + (y / 9) * 16;
        let char_bits = self.chars[char_code];
        let char_x = x % 9;
        let char_y = y % 9;
        if char_x == 8 || char_y == 8 {
            return SRGB::from(Color::Red);
        }
        let bit_mask = 1_u64 << ((7 - char_x) + (7 - char_y) * 8);
        if char_bits & bit_mask != 0 {
            SRGB::from(Color::White)
        } else {
            SRGB::from(Color::Black)
        }
    }
}
