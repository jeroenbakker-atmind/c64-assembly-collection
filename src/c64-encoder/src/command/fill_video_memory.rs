use crate::encoder::{writer::Writer, Encoder};

pub struct FillVideoMemory {
    pub pixel_data: u8,
}

impl Encoder for FillVideoMemory {
    fn byte_size(&self) -> usize {
        size_of::<u8>()
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        encoded_data.add(&self.pixel_data)
    }
}
