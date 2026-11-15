use c64_assembler::{
    builder::{FunctionBuilder, InstructionBuilder, ModuleBuilder},
    Module,
};

use crate::encoder::{writer::Writer, Encoder};

use super::{modules::CurrentPtrMacros, DecoderModule};

#[derive(Debug, Clone)]
pub struct PartialUpdateTextModeScreen {
    pub changes: Vec<UpdateSingleChar>,
}

impl PartialUpdateTextModeScreen {
    pub fn transition(from_bytes: &[u8], to_bytes: &[u8]) -> PartialUpdateTextModeScreen {
        let changes = from_bytes
            .iter()
            .zip(to_bytes.iter())
            .enumerate()
            .filter(|(_index, (from, to))| from != to)
            .map(|(index, (_from, to))| UpdateSingleChar {
                offset: index as u16,
                char: *to,
            })
            .collect::<Vec<UpdateSingleChar>>();
        PartialUpdateTextModeScreen { changes }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct UpdateSingleChar {
    pub offset: u16,
    pub char: u8,
}

impl Encoder for PartialUpdateTextModeScreen {
    fn byte_size(&self) -> usize {
        self.changes.len() * (2 + 1) + 2
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        let mut encoded_data = encoded_data;
        let num_changes = self.changes.len() as u16;
        encoded_data = encoded_data.add(&num_changes);
        for change in &self.changes {
            encoded_data = encoded_data.add(change);
        }
        encoded_data
    }
}

impl Encoder for UpdateSingleChar {
    fn byte_size(&self) -> usize {
        2 + 1
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        encoded_data.add(&self.offset).add(&self.char)
    }
}

impl DecoderModule for PartialUpdateTextModeScreen {
    fn module() -> Module {
        ModuleBuilder::default()
            .name("partial_update_text_mode")
            .function(
                FunctionBuilder::default()
                    .name("partial_update_text_mode__process")
                    .instructions(
                        InstructionBuilder::default()
                            .lda_current_ptr_offs(1, "Load the low byte of the number of changed characters.")
                            .tax()
                            .lda_current_ptr_offs(2, "Load the high byte of the number of changed characters.")
                            .sta_addr("SCRATCH_SPACE_00")
                            .inc_current_ptr(3)
                            .comment("Nothing to update when the number of changes is zero.")
                            .cpx_imm(0)
                            .bne_addr("partial_update_text_mode__next")
                            .lda_addr("SCRATCH_SPACE_00")
                            .beq_addr("partial_update_text_mode__exit")
                            .label("partial_update_text_mode__next")
                            .lda_current_ptr_offs(0, "Load the low byte of the screen offset.")
                            .clc()
                            .adc_imm_low("SCREEN_CHARS_PAGE0")
                            .sta_addr("SCREEN_CHAR_PTR")
                            .lda_current_ptr_offs(1, "Load the high byte of the screen offset.")
                            .adc_imm_high("SCREEN_CHARS_PAGE0")
                            .sta_addr_offs("SCREEN_CHAR_PTR", 1)
                            .lda_current_ptr_offs(2, "Load the character to write.")
                            .ldy_imm(0)
                            .sta_ind_y("SCREEN_CHAR_PTR")
                            .inc_current_ptr(3)
                            .comment(
                                "Decrease the 16-bit number of changes; low byte in X, high byte in scratch space.",
                            )
                            .dex()
                            .cpx_imm(0xFF)
                            .bne_addr("partial_update_text_mode__check_done")
                            .dec_addr("SCRATCH_SPACE_00")
                            .label("partial_update_text_mode__check_done")
                            .cpx_imm(0)
                            .bne_addr("partial_update_text_mode__next")
                            .lda_addr("SCRATCH_SPACE_00")
                            .bne_addr("partial_update_text_mode__next")
                            .label("partial_update_text_mode__exit")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .build()
    }
}
