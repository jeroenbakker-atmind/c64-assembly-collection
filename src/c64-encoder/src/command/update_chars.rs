use crate::{
    charmap::encoding::encode_char,
    encoder::{writer::Writer, Encoder},
};

#[derive(Default, Debug, Clone)]
pub struct UpdateCharsU16Encoded {
    pub chars: Vec<UpdateChar>,
}
#[derive(Default, Debug, Copy, Clone)]
pub struct UpdateChar {
    pub char: u8,
    pub data: u64,
}

impl Encoder for UpdateChar {
    fn byte_size(&self) -> usize {
        1 + 2
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        let mut encoded_data = encoded_data;
        let encoded_char = encode_char(self.data);
        encoded_data = encoded_data.add(&self.char).add(&encoded_char);

        encoded_data
    }
}

impl Encoder for UpdateCharsU16Encoded {
    fn byte_size(&self) -> usize {
        1 + self.chars.len() * UpdateChar::default().byte_size()
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        let mut encoded_data = encoded_data;
        let num_chars = self.chars.len() as u8;
        encoded_data = encoded_data.add(&num_chars);
        for char in &self.chars {
            encoded_data = char.encode(encoded_data)
        }

        encoded_data
    }
}
