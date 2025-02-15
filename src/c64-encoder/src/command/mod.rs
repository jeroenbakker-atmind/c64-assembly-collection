use c64_assembler::Module;
use clear_screen_chars::ClearScreenChars;
use partial_update_text_mode::PartialUpdateTextModeScreen;
use set_border_color::SetBorderColor;
use set_palette4::SetPalette4;
use update_chars::UpdateCharsU16Encoded;
use update_chars_ranged::UpdateCharsRangedU16Encoded;
use update_screen_chars_rle::UpdateScreenCharsRLE;
use update_text_mode_screen::UpdateTextModeScreen;

use crate::encoder::Encoder;

pub mod clear_screen_chars;
pub mod modules;
pub mod partial_update_text_mode;
pub mod set_border_color;
pub mod set_palette4;
pub mod update_chars;
pub mod update_chars_ranged;
pub mod update_screen_chars_rle;
pub mod update_text_mode_screen;

pub const CLEAR_SCREEN_CHAR: u8 = 1;
pub const SET_PALETTE4: u8 = 2;
pub const SET_BORDER_COLOR: u8 = 3;
pub const UPDATE_CHARS_U16: u8 = 16;
pub const UPDATE_CHARS_RANGED_U16: u8 = 17;
pub const UPDATE_TEXT_MODE_SCREEN: u8 = 32;
pub const PARTIAL_UPDATE_TEXT_MODE_SCREEN: u8 = 33;
pub const UPDATE_SCREEN_CHARS_RLE: u8 = 34;

#[derive(Debug, Clone)]
pub enum Command {
    ClearScreenChars(ClearScreenChars),
    SetPalette4(SetPalette4),
    SetBorderColor(SetBorderColor),
    UpdateCharsU16Encoded(UpdateCharsU16Encoded),
    UpdateCharsRangedU16Encoded(UpdateCharsRangedU16Encoded),
    UpdateTextModeScreen(UpdateTextModeScreen),
    PartialUpdateTextModeScreen(PartialUpdateTextModeScreen),
    UpdateScreenCharsRLE(UpdateScreenCharsRLE),
}

impl Encoder for Command {
    fn byte_size(&self) -> usize {
        let command_size = match self {
            Command::ClearScreenChars(fill_video_memory) => fill_video_memory.byte_size(),
            Command::SetPalette4(set_palette4) => set_palette4.byte_size(),
            Command::SetBorderColor(set_border_color) => set_border_color.byte_size(),
            Command::UpdateCharsU16Encoded(update_chars) => update_chars.byte_size(),
            Command::UpdateCharsRangedU16Encoded(update_chars_ranged) => update_chars_ranged.byte_size(),
            Command::UpdateTextModeScreen(update_text_mode_screen) => update_text_mode_screen.byte_size(),
            Command::PartialUpdateTextModeScreen(partial_update_text_mode) => partial_update_text_mode.byte_size(),
            Command::UpdateScreenCharsRLE(update_screen_chars_rle) => update_screen_chars_rle.byte_size(),
        };
        command_size + size_of::<u8>()
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        match self {
            Command::ClearScreenChars(fill_video_memory) => {
                let mut encoded_data = CLEAR_SCREEN_CHAR.encode(encoded_data);
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
            Command::UpdateCharsRangedU16Encoded(update_chars) => {
                let mut encoded_data = UPDATE_CHARS_RANGED_U16.encode(encoded_data);
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
            Command::UpdateScreenCharsRLE(update_screen_chars_rle) => {
                let mut encoded_data = UPDATE_SCREEN_CHARS_RLE.encode(encoded_data);
                encoded_data = update_screen_chars_rle.encode(encoded_data);
                encoded_data
            }
        }
    }
}

pub trait DecoderModule {
    fn module() -> Module;
}
pub fn all_decoder_modules() -> Vec<(u8, Module)> {
    vec![
        (CLEAR_SCREEN_CHAR, ClearScreenChars::module()),
        (SET_BORDER_COLOR, SetBorderColor::module()),
        (UPDATE_CHARS_RANGED_U16, UpdateCharsRangedU16Encoded::module()),
        (UPDATE_SCREEN_CHARS_RLE, UpdateScreenCharsRLE::module()),
    ]
}
