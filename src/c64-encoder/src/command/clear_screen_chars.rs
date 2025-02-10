use crate::encoder::{writer::Writer, Encoder};

#[derive(Copy, Clone, Debug, Default)]
pub struct ClearScreenChars {
    pub screen_char: u8,
}

impl Encoder for ClearScreenChars {
    fn byte_size(&self) -> usize {
        size_of::<u8>()
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        encoded_data.add(&self.screen_char)
    }
}
