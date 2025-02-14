use c64_assembler::{
    builder::{FunctionBuilder, InstructionBuilder, ModuleBuilder},
    Module,
};

use crate::encoder::{writer::Writer, Encoder};

use super::{modules::CurrentPtrMacros, DecoderModule};

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
                            .lda_current_ptr_offs(1, "Load character to fill the screen with into the accumulator")
                            .ldx_imm(0x00)
                            .label("clear_screen_char__next")
                            .sta_addr_x("SCREEN_CHARS_PAGE0")
                            .sta_addr_x("SCREEN_CHARS_PAGE1")
                            .sta_addr_x("SCREEN_CHARS_PAGE2")
                            .sta_addr_x("SCREEN_CHARS_PAGE3")
                            .inx()
                            .bne_addr("clear_screen_char__next")
                            .inc_current_ptr(2)
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .build()
    }
}
