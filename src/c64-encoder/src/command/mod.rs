use fill_video_memory::FillVideoMemory;
use partial_update_text_mode::PartialUpdateTextModeScreen;
use set_border_color::SetBorderColor;
use set_palette4::SetPalette4;
use update_chars::UpdateCharsU16Encoded;
use update_text_mode_screen::UpdateTextModeScreen;

use crate::encoder::Encoder;

pub mod fill_video_memory;
pub mod partial_update_text_mode;
pub mod set_border_color;
pub mod set_palette4;
pub mod update_chars;
pub mod update_text_mode_screen;

pub const FILL_VIDEO_MEMORY: u8 = 1;
pub const SET_PALETTE4: u8 = 2;
pub const SET_BORDER_COLOR: u8 = 3;
pub const UPDATE_CHARS_U16: u8 = 16;
pub const UPDATE_TEXT_MODE_SCREEN: u8 = 32;
pub const PARTIAL_UPDATE_TEXT_MODE_SCREEN: u8 = 33;

#[derive(Debug, Clone)]
pub enum Command {
    FillVideoMemory(FillVideoMemory),
    SetPalette4(SetPalette4),
    SetBorderColor(SetBorderColor),
    UpdateCharsU16Encoded(UpdateCharsU16Encoded),
    UpdateTextModeScreen(UpdateTextModeScreen),
    PartialUpdateTextModeScreen(PartialUpdateTextModeScreen),
}

impl Encoder for Command {
    fn byte_size(&self) -> usize {
        let command_size = match self {
            Command::FillVideoMemory(fill_video_memory) => fill_video_memory.byte_size(),
            Command::SetPalette4(set_palette4) => set_palette4.byte_size(),
            Command::SetBorderColor(set_border_color) => set_border_color.byte_size(),
            Command::UpdateCharsU16Encoded(update_chars) => update_chars.byte_size(),
            Command::UpdateTextModeScreen(update_text_mode_screen) => update_text_mode_screen.byte_size(),
            Command::PartialUpdateTextModeScreen(partial_update_text_mode) => partial_update_text_mode.byte_size(),
        };
        command_size + size_of::<u8>()
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        match self {
            Command::FillVideoMemory(fill_video_memory) => {
                let mut encoded_data = FILL_VIDEO_MEMORY.encode(encoded_data);
                encoded_data = fill_video_memory.encode(encoded_data);
                encoded_data
            }
            Command::SetPalette4(set_palette4) => {
                let mut encoded_data = SET_PALETTE4.encode(encoded_data);
                encoded_data = set_palette4.encode(encoded_data);
                encoded_data
            }
            Command::SetBorderColor(set_border_color) => {
                let mut encoded_data = SET_BORDER_COLOR.encode(encoded_data);
                encoded_data = set_border_color.encode(encoded_data);
                encoded_data
            }
            Command::UpdateCharsU16Encoded(update_chars) => {
                let mut encoded_data = UPDATE_CHARS_U16.encode(encoded_data);
                encoded_data = update_chars.encode(encoded_data);
                encoded_data
            }
            Command::UpdateTextModeScreen(update_text_mode_screen) => {
                let mut encoded_data = UPDATE_TEXT_MODE_SCREEN.encode(encoded_data);
                encoded_data = update_text_mode_screen.encode(encoded_data);
                encoded_data
            }
            Command::PartialUpdateTextModeScreen(partial_update_text_mode) => {
                let mut encoded_data = PARTIAL_UPDATE_TEXT_MODE_SCREEN.encode(encoded_data);
                encoded_data = partial_update_text_mode.encode(encoded_data);
                encoded_data
            }
        }
    }
}
