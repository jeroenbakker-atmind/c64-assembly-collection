#[cfg(test)]
mod encoder_test;
pub mod utils;
pub mod writer;

pub trait Encoder {
    fn byte_size(&self) -> usize;
    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8];
}

impl Encoder for u16 {
    fn byte_size(&self) -> usize {
        size_of::<u16>()
    }

    // NOTE: stores in 6502 endian (LOW,HIGH)
    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        let low = (self % 0x100) as u8;
        let high = (self / 0x100) as u8;
        encoded_data[0] = low;
        encoded_data[1] = high;
        &mut encoded_data[size_of::<u16>()..]
    }
}

impl Encoder for u8 {
    fn byte_size(&self) -> usize {
        size_of::<u8>()
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        encoded_data[0] = *self;
        &mut encoded_data[size_of::<u8>()..]
    }
}
