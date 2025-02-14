use c64_assembler::builder::{FunctionBuilder, InstructionBuilder, ModuleBuilder};

use crate::command::DecoderModule;

pub struct CurrentPTR {}

impl DecoderModule for CurrentPTR {
    fn module() -> c64_assembler::Module {
        ModuleBuilder::default()
            .name("engine__current_ptr")
            .function(
                FunctionBuilder::default()
                    .name("engine__current_ptr__advance")
                    .doc(&[
                        "Advance current pointer with accumulator",
                        "",
                        "Advance the pointer stored at CURRENT_PTR with the value stored in the accumulator.",
                        "The accumulator is number of bytes to advance.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .clc()
                            .adc_addr("CURRENT_PTR")
                            .sta_addr("CURRENT_PTR")
                            .lda_imm(0x00)
                            .adc_addr_offs("CURRENT_PTR", 1)
                            .sta_addr_offs("CURRENT_PTR", 1)
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .build()
    }
}

pub trait CurrentPtrMacros {
    fn lda_current_ptr_offs(&mut self, offset: u8, comment: &str) -> &mut Self;
    fn inc_current_ptr(&mut self, increment: u8) -> &mut Self;
}

impl CurrentPtrMacros for InstructionBuilder {
    fn lda_current_ptr_offs(&mut self, offset: u8, comment: &str) -> &mut Self {
        self.ldy_imm(offset).comment(comment).lda_ind_y("CURRENT_PTR")
    }

    fn inc_current_ptr(&mut self, increment: u8) -> &mut Self {
        self.lda_imm(increment).jsr_addr("engine__current_ptr__advance")
    }
}
