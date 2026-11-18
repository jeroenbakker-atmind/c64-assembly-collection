use c64_assembler::Module;
use clear_screen_chars::ClearScreenChars;
use partial_update_text_mode::PartialUpdateTextModeScreen;
use set_border_color::SetBorderColor;
use set_palette4::SetPalette4;
use update_chars::UpdateCharsU16Encoded;
use update_chars_ranged::UpdateCharsRangedU16Encoded;
use update_screen_blocks::{BlockKind, UpdateScreenBlocks};
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
pub mod update_screen_blocks;
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
pub const UPDATE_SCREEN_BLOCKS_ROW_2X3: u8 = 35;
pub const UPDATE_SCREEN_BLOCKS_ROW_4X3: u8 = 36;
pub const UPDATE_SCREEN_BLOCKS_ROW_2X5: u8 = 37;
pub const UPDATE_SCREEN_BLOCKS_ROW_4X5: u8 = 38;
pub const UPDATE_SCREEN_BLOCKS_COL_2X3: u8 = 39;
pub const UPDATE_SCREEN_BLOCKS_COL_4X3: u8 = 40;
pub const UPDATE_SCREEN_BLOCKS_COL_2X5: u8 = 41;
pub const UPDATE_SCREEN_BLOCKS_COL_4X5: u8 = 42;
pub const UPDATE_SCREEN_BLOCKS_ROW_4X4: u8 = 43;
pub const UPDATE_SCREEN_BLOCKS_COL_4X4: u8 = 44;

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
    UpdateScreenBlocks(UpdateScreenBlocks),
}

impl Command {
    /// Opcode written as the first byte of this command.
    pub fn opcode(&self) -> u8 {
        match self {
            Command::ClearScreenChars(_) => CLEAR_SCREEN_CHAR,
            Command::SetPalette4(_) => SET_PALETTE4,
            Command::SetBorderColor(_) => SET_BORDER_COLOR,
            Command::UpdateCharsU16Encoded(_) => UPDATE_CHARS_U16,
            Command::UpdateCharsRangedU16Encoded(_) => UPDATE_CHARS_RANGED_U16,
            Command::UpdateTextModeScreen(_) => UPDATE_TEXT_MODE_SCREEN,
            Command::PartialUpdateTextModeScreen(_) => PARTIAL_UPDATE_TEXT_MODE_SCREEN,
            Command::UpdateScreenCharsRLE(_) => UPDATE_SCREEN_CHARS_RLE,
            Command::UpdateScreenBlocks(block) => block.kind.opcode(),
        }
    }
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
            Command::UpdateScreenBlocks(update_screen_blocks) => update_screen_blocks.byte_size(),
        };
        command_size + size_of::<u8>()
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        let encoded_data = self.opcode().encode(encoded_data);
        match self {
            Command::ClearScreenChars(fill_video_memory) => fill_video_memory.encode(encoded_data),
            Command::SetPalette4(set_palette4) => set_palette4.encode(encoded_data),
            Command::SetBorderColor(set_border_color) => set_border_color.encode(encoded_data),
            Command::UpdateCharsU16Encoded(update_chars) => update_chars.encode(encoded_data),
            Command::UpdateCharsRangedU16Encoded(update_chars) => update_chars.encode(encoded_data),
            Command::UpdateTextModeScreen(update_text_mode_screen) => update_text_mode_screen.encode(encoded_data),
            Command::PartialUpdateTextModeScreen(partial_update_text_mode) => {
                partial_update_text_mode.encode(encoded_data)
            }
            Command::UpdateScreenCharsRLE(update_screen_chars_rle) => update_screen_chars_rle.encode(encoded_data),
            Command::UpdateScreenBlocks(update_screen_blocks) => update_screen_blocks.encode(encoded_data),
        }
    }
}

pub trait DecoderModule {
    fn module() -> Module;
}
pub fn all_decoder_modules() -> Vec<(u8, Module)> {
    let mut modules = vec![
        (CLEAR_SCREEN_CHAR, ClearScreenChars::module()),
        (SET_BORDER_COLOR, SetBorderColor::module()),
        (SET_PALETTE4, SetPalette4::module()),
        (UPDATE_CHARS_U16, UpdateCharsU16Encoded::module()),
        (UPDATE_CHARS_RANGED_U16, UpdateCharsRangedU16Encoded::module()),
        (UPDATE_TEXT_MODE_SCREEN, UpdateTextModeScreen::module()),
        (PARTIAL_UPDATE_TEXT_MODE_SCREEN, PartialUpdateTextModeScreen::module()),
        (UPDATE_SCREEN_CHARS_RLE, UpdateScreenCharsRLE::module()),
    ];
    modules.extend(
        BlockKind::ALL
            .iter()
            .map(|kind| (kind.opcode(), modules::decoder_module(*kind))),
    );
    modules
}
