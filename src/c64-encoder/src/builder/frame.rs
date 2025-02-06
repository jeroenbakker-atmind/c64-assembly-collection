use c64_colors::colors::Color;

use crate::{
    command::{
        fill_video_memory::FillVideoMemory, set_border_color::SetBorderColor, set_palette4::SetPalette4,
        update_chars::UpdateCharsU16Encoded, update_text_mode_screen::UpdateTextModeScreen, Command,
    },
    encoder::{writer::Writer, Encoder},
};

pub type Commands = Vec<Command>;

#[derive(Default, Clone)]
pub struct FrameBuilder {
    pub commands: Commands,
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

    pub fn update_charmap_u16(&mut self, update_chars: UpdateCharsU16Encoded) -> &mut Self {
        self.commands.push(Command::UpdateCharsU16Encoded(update_chars));
        self
    }
    pub fn update_text_mode_screen(&mut self, update_text_mode_screen: UpdateTextModeScreen) -> &mut Self {
        self.commands
            .push(Command::UpdateTextModeScreen(update_text_mode_screen));
        self
    }

    /// Extend the frame from commands.
    ///
    /// Used when pushing generated commands from a sub-builder to a frame.
    pub fn extend(&mut self, commands: &[Command]) -> &mut Self {
        self.commands.extend_from_slice(commands);
        self
    }

    /// Add the given command at the end of the command list.
    pub fn push(&mut self, command: Command) -> &mut Self {
        self.commands.push(command);
        self
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}

impl Encoder for FrameBuilder {
    fn byte_size(&self) -> usize {
        self.commands.byte_size() + size_of::<u16>()
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        let mut encoded_data = encoded_data.add(&(self.commands.len() as u16));
        encoded_data = self.commands.encode(encoded_data);
        encoded_data
    }
}

impl Encoder for Commands {
    fn byte_size(&self) -> usize {
        self.iter().map(Command::byte_size).sum::<usize>()
    }
    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        let mut encoded_data = encoded_data;
        for command in self {
            encoded_data = command.encode(encoded_data);
        }
        encoded_data
    }
}
