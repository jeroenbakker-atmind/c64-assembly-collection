use c64::image_container::Image;
use c64_colors::colors::{Color, SRGB};

#[derive(Debug, Default, Clone)]
pub struct State {
    pub charset: CharSet,
    pub text_screen: TextScreen,
}

impl State {
    pub fn mark_used(&mut self) {
        for screen_char in &self.text_screen.screen_chars {
            self.charset.mark_used(*screen_char);
        }
    }
    pub fn reset(&mut self) {
        self.charset.reset();
    }
}

#[derive(Debug, Clone)]
pub struct CharSet {
    chars: [u64; 256],
    is_used: [bool; 256],
    is_new: [bool; 256],
}

impl Default for CharSet {
    fn default() -> Self {
        Self {
            chars: [0xDEADBEEFDEADBEEF; 256],
            is_used: [false; 256],
            is_new: [false; 256],
        }
    }
}

impl CharSet {
    fn reset(&mut self) {
        self.is_used = [false; 256];
        self.is_new = [false; 256];
    }
    pub fn update_char(&mut self, index: u8, char: u64) {
        self.chars[index as usize] = char;
        self.is_new[index as usize] = true;
    }
    pub fn mark_used(&mut self, index: u8) {
        self.is_used[index as usize] = true;
    }
}

#[derive(Debug, Clone)]
pub struct TextScreen {
    pub screen_chars: [u8; 1000],
}
impl Default for TextScreen {
    fn default() -> Self {
        Self {
            screen_chars: [0; 1000],
        }
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
            result = self.text_screen.get_pixel_color(x, y, &self.charset);
        }
        if (320..(320 + 128 + 15 + 15)).contains(&x) && (0..(128 + 15 + 15)).contains(&y) {
            result = self.charset.get_pixel_color(x - 320, y);
        }
        result
    }
}

impl TextScreen {
    fn get_pixel_color(&self, x: usize, y: usize, char_map: &CharSet) -> SRGB {
        let index = (x / 8) + (y / 8) * 40;
        let charcode = self.screen_chars[index];
        let charmap_x = (charcode as usize % 16) * 10 + (x % 8) + 1;
        let charmap_y = (charcode as usize / 16) * 10 + (y % 8) + 1;
        char_map.get_pixel_color(charmap_x, charmap_y)
    }
}

impl Image for CharSet {
    fn width(&self) -> usize {
        128 + 15 + 15
    }

    fn height(&self) -> usize {
        128 + 15 + 15
    }

    fn get_pixel_color(&self, x: usize, y: usize) -> c64_colors::colors::SRGB {
        let char_code = x / 10 + (y / 10) * 16;
        let char_bits = self.chars[char_code];
        let char_x = x % 10;
        let char_y = y % 10;
        if char_x == 0 || char_x == 9 || char_y == 0 || char_y == 9 {
            let mut result = SRGB::from_rgb(0, 0, 0);
            if self.is_used[char_code] {
                result.g = 255;
            }
            if self.is_new[char_code] {
                result.r = 255;
            }
            return result;
        }
        let char_x = char_x - 1;
        let char_y = char_y - 1;
        let bit_mask = 1_u64 << ((7 - char_x) + (7 - char_y) * 8);
        if char_bits & bit_mask != 0 {
            SRGB::from(Color::White)
        } else {
            SRGB::from(Color::Black)
        }
    }
}
