use c64_assembler::{
    builder::{FunctionBuilder, InstructionBuilder, ModuleBuilder},
    Module,
};

use crate::encoder::{writer::Writer, Encoder};

use super::DecoderModule;

#[derive(Copy, Clone, Debug, Default)]
pub struct ClearScreenChars {
    pub screen_char: u8,
}

impl Encoder for ClearScreenChars {
    fn byte_size(&self) -> usize {
        size_of::<u8>()
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        encoded_data.add(&self.screen_char)
    }
}

impl DecoderModule for ClearScreenChars {
    fn module() -> Module {
        ModuleBuilder::default()
            .name("clear_screen_chars")
            .function(
                FunctionBuilder::default()
                    .name("clear_screen_chars__process")
                    .instructions(
                        InstructionBuilder::default()
                            .ldy_imm(1)
                            .lda_ind_y("CURRENT_PTR")
                            .ldx_imm(0x00)
                            .label("clear_screen_char__next")
                            .sta_addr_x("SCREEN_CHARS_PAGE0")
                            .sta_addr_x("SCREEN_CHARS_PAGE1")
                            .sta_addr_x("SCREEN_CHARS_PAGE2")
                            .sta_addr_x("SCREEN_CHARS_PAGE3")
                            .inx()
                            .bne_addr("clear_screen_char__next")
                            .lda_imm(2)
                            .jsr_addr("engine__current_ptr__advance")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .build()
    }
}
