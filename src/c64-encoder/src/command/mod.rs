use fill_video_memory::FillVideoMemory;
use set_border_color::SetBorderColor;
use set_palette4::SetPalette4;

use crate::encoder::Encoder;

pub mod fill_video_memory;
pub mod set_border_color;
pub mod set_palette4;

const FILL_VIDEO_MEMORY: u8 = 1;
const SET_PALETTE4: u8 = 2;
const SET_BORDER_COLOR: u8 = 3;

pub enum Command {
    FillVideoMemory(FillVideoMemory),
    SetPalette4(SetPalette4),
    SetBorderColor(SetBorderColor),
}

impl Encoder for Command {
    fn byte_size(&self) -> usize {
        let command_size = match self {
            Command::FillVideoMemory(fill_video_memory) => fill_video_memory.byte_size(),
            Command::SetPalette4(set_palette4) => set_palette4.byte_size(),
            Command::SetBorderColor(set_border_color) => set_border_color.byte_size(),
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
        }
    }
}
