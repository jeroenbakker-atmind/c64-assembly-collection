use crate::{
    charmap::encoding::encode_char,
    encoder::{writer::Writer, Encoder},
};

#[derive(Default, Debug, Clone)]
pub struct UpdateCharsRangedU16Encoded {
    pub offset: u8,
    pub chars: Vec<UpdateCharRanged>,
}
#[derive(Default, Debug, Copy, Clone)]
pub struct UpdateCharRanged {
    pub data: u64,
}

impl Encoder for UpdateCharRanged {
    fn byte_size(&self) -> usize {
        2
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        let mut encoded_data = encoded_data;
        let encoded_char = encode_char(self.data);
        encoded_data = encoded_data.add(&encoded_char);

        encoded_data
    }
}

impl Encoder for UpdateCharsRangedU16Encoded {
    fn byte_size(&self) -> usize {
        1 + 1 + self.chars.len() * UpdateCharRanged::default().byte_size()
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        let mut encoded_data = encoded_data;
        let num_chars = self.chars.len() as u8;
        encoded_data = encoded_data.add(&num_chars).add(&self.offset);
        for char in &self.chars {
            encoded_data = char.encode(encoded_data)
        }

        encoded_data
    }
}
