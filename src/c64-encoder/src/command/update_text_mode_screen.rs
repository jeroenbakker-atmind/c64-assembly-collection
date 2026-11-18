use c64_assembler::{
    builder::{FunctionBuilder, InstructionBuilder, ModuleBuilder},
    Module,
};

use crate::{
    command::{modules::CurrentPtrMacros, DecoderModule},
    encoder::{writer::Writer, Encoder},
};

#[derive(Debug, Clone)]
pub struct UpdateTextModeScreen {
    pub chars: [u8; 1000],
}

impl Default for UpdateTextModeScreen {
    fn default() -> Self {
        Self { chars: [0; 1000] }
    }
}

impl UpdateTextModeScreen {
    pub fn filled(char: u8) -> UpdateTextModeScreen {
        UpdateTextModeScreen { chars: [char; 1000] }
    }
}

impl Encoder for UpdateTextModeScreen {
    fn byte_size(&self) -> usize {
        1000
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        let mut encoded_data = encoded_data;
        for ch in &self.chars {
            encoded_data = encoded_data.add(ch);
        }
        encoded_data
    }
}

impl DecoderModule for UpdateTextModeScreen {
    fn module() -> Module {
        ModuleBuilder::default()
            .name("update_text_mode_screen")
            .function(
                FunctionBuilder::default()
                    .name("update_text_mode_screen__process")
                    .doc(&["Decode 1000 consecutive screen characters from the command stream."])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_imm_low("SCREEN_CHARS_PAGE0")
                            .sta_addr("SCREEN_CHAR_PTR")
                            .lda_imm_high("SCREEN_CHARS_PAGE0")
                            .sta_addr_offs("SCREEN_CHAR_PTR", 1)
                            .lda_imm(1)
                            .jsr_addr("engine__current_ptr__advance")
                            .comment("Skip the opcode byte.")
                            .label("update_text_mode_screen__next")
                            .ldy_imm(0)
                            .lda_current_ptr_offs(0, "Load the next screen character.")
                            .sta_ind_y("SCREEN_CHAR_PTR")
                            .inc_current_ptr(1)
                            .inc_addr("SCREEN_CHAR_PTR")
                            .bne_addr("update_text_mode_screen__check_done")
                            .inc_addr_offs("SCREEN_CHAR_PTR", 1)
                            .label("update_text_mode_screen__check_done")
                            .lda_addr("SCREEN_CHAR_PTR")
                            .cmp_imm(0xE8)
                            .bne_addr("update_text_mode_screen__next")
                            .lda_addr_offs("SCREEN_CHAR_PTR", 1)
                            .cmp_imm(0xC3)
                            .bne_addr("update_text_mode_screen__next")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .build()
    }
}
