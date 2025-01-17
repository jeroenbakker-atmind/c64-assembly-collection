use c64_colors::colors::Color;

use crate::{
    command::{
        fill_video_memory::FillVideoMemory, set_border_color::SetBorderColor, set_palette4::SetPalette4, Command,
    },
    encoder::{writer::Writer, Encoder},
};

#[derive(Default, Clone)]
pub struct FrameBuilder {
    pub commands: Vec<Command>,
}

impl FrameBuilder {
    pub fn set_palette4(&mut self, palette: [Color; 4]) -> &mut Self {
        self.commands.push(Command::SetPalette4(SetPalette4 { palette }));
        self
    }

    pub fn fill_video_memory(&mut self, pixel_data: u8) -> &mut Self {
        self.commands
            .push(Command::FillVideoMemory(FillVideoMemory { pixel_data }));
        self
    }

    pub fn set_border_color(&mut self, color: Color) -> &mut Self {
        self.commands.push(Command::SetBorderColor(SetBorderColor { color }));
        self
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}

impl Encoder for FrameBuilder {
    fn byte_size(&self) -> usize {
        self.commands.iter().map(Command::byte_size).sum::<usize>() + size_of::<u16>()
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        let mut encoded_data = encoded_data.add(&(self.commands.len() as u16));

        for frame in &self.commands {
            encoded_data = frame.encode(encoded_data);
        }

        encoded_data
    }
}
