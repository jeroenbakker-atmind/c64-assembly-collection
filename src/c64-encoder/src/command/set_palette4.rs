use c64_colors::colors::Color;

use crate::encoder::{writer::Writer, Encoder};

#[derive(Copy, Clone, Debug)]
pub struct SetPalette4 {
    pub palette: [Color; 4],
}

impl Encoder for SetPalette4 {
    fn byte_size(&self) -> usize {
        size_of::<u8>() * 2
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        let b1 = u8::from(self.palette[0]) + u8::from(self.palette[1]) * 16_u8;
        let b2 = u8::from(self.palette[2]) + u8::from(self.palette[3]) * 16_u8;
        encoded_data.add(&b1).add(&b2)
    }
}
