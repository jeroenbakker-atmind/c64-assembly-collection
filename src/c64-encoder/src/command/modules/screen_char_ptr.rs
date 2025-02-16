use c64_assembler::builder::{FunctionBuilder, InstructionBuilder, ModuleBuilder};

use crate::command::DecoderModule;

pub struct ScreenCharPTR {}

impl DecoderModule for ScreenCharPTR {
    fn module() -> c64_assembler::Module {
        ModuleBuilder::default()
            .name("engine__current_ptr")
            .function(
                FunctionBuilder::default()
                    .name("engine__screen_char_ptr__reset")
                    .instructions(
                        InstructionBuilder::default()
                            .lda_imm_low("SCREEN_CHARS_PAGE0")
                            .sta_addr("SCREEN_CHAR_PTR")
                            .lda_imm_high("SCREEN_CHARS_PAGE0")
                            .sta_addr_offs("SCREEN_CHAR_PTR", 1)
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__screen_char_ptr__advance")
                    .doc(&["Advance the screen char pointer with the contents of the accumulator"])
                    .instructions(
                        InstructionBuilder::default()
                            .clc()
                            .adc_addr("SCREEN_CHAR_PTR")
                            .sta_addr("SCREEN_CHAR_PTR")
                            .lda_imm(0x00)
                            .adc_addr_offs("SCREEN_CHAR_PTR", 1)
                            .sta_addr_offs("SCREEN_CHAR_PTR", 1)
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .build()
    }
}

pub trait ScreenCharPtrMacros {
    fn sta_screen_char_ptr_offs(&mut self, offset: u8) -> &mut Self;
    fn inc_screen_char_ptr(&mut self, increment: u8) -> &mut Self;
}

impl ScreenCharPtrMacros for InstructionBuilder {
    fn sta_screen_char_ptr_offs(&mut self, offset: u8) -> &mut Self {
        self.ldy_imm(offset).sta_ind_y("SCREEN_CHAR_PTR")
    }

    fn inc_screen_char_ptr(&mut self, increment: u8) -> &mut Self {
        self.lda_imm(increment).jsr_addr("engine__screen_char_ptr__advance")
    }
}
