use c64_colors::colors::Color;

use crate::encoder::{writer::Writer, Encoder};

pub struct SetBorderColor {
    pub color: Color,
}

impl Encoder for SetBorderColor {
    fn byte_size(&self) -> usize {
        size_of::<u8>()
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        encoded_data.add(&u8::from(self.color))
    }
}
