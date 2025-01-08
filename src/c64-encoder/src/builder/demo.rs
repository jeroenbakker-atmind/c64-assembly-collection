use crate::encoder::writer::Writer;
use crate::encoder::Encoder;

use super::frame::FrameBuilder;

#[derive(Default)]
pub struct DemoBuilder {
    pub frames: Vec<FrameBuilder>,
}

impl DemoBuilder {
    pub fn frame(&mut self, frame: FrameBuilder) ->&mut Self{
        self.frames.push(frame);
        self
    }


    pub fn build(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.resize(self.byte_size(), 0);
        self.encode(result.as_mut_slice());
        result
    }
}

impl Encoder for DemoBuilder {
    fn byte_size(&self) -> usize {
        self.frames
            .iter()
            .map(FrameBuilder::byte_size)
            .sum::<usize>()
            + size_of::<u16>()
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        let mut encoded_data = encoded_data;

        encoded_data = encoded_data.add(&(self.frames.len() as u16));
        for frame in &self.frames {
            encoded_data = encoded_data.add(frame);
        }

        encoded_data
    }
}
