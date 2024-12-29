use crate::colors::{Color, SRGB};

pub type PaletteIndex = u8;

#[derive(Debug)]
pub struct Palette4 {
    pub colors: [Color; 4],
}

impl Palette4 {
    pub fn get_nearest_color_index(&self, srgb: SRGB) -> PaletteIndex {
        self.colors
            .iter()
            .enumerate()
            .map(|(index, color)| (index as u8, color))
            .min_by_key(|(_index, color)| SRGB::from(**color).distance(srgb))
            .unwrap()
            .0
    }

    pub fn get_color(&self, color_index: PaletteIndex) -> Color {
        self.colors[color_index as usize]
    }
    pub fn get_srgb_color(&self, color_index: PaletteIndex) -> SRGB {
        SRGB::from(self.get_color(color_index))
    }
}
