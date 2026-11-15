use c64_assembler::builder::{FunctionBuilder, InstructionBuilder, ModuleBuilder};

use crate::command::DecoderModule;

pub struct FramesLeft {}

impl DecoderModule for FramesLeft {
    fn module() -> c64_assembler::Module {
        ModuleBuilder::default()
            .name("engine__frames_left")
            .instructions(
                InstructionBuilder::default()
                    .label("engine__frames_left")
                    .comment("Number of frames left to play.")
                    .comment("When 0 the animation is finished")
                    .raw(&[0x00; 2])
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__frames_left__init")
                    .instructions(
                        InstructionBuilder::default()
                            .ldy_imm(0x00)
                            .lda_ind_y("CURRENT_PTR")
                            .sta_addr("engine__frames_left")
                            .iny()
                            .lda_ind_y("CURRENT_PTR")
                            .sta_addr_offs("engine__frames_left", 1)
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__frames_left__decrease")
                    .instructions(
                        InstructionBuilder::default()
                            .clc()
                            .lda_addr("engine__frames_left")
                            .sbc_imm(0x00)
                            .sta_addr("engine__frames_left")
                            .lda_addr_offs("engine__frames_left", 1)
                            .sbc_imm(0x00)
                            .sta_addr_offs("engine__frames_left", 1)
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__frames_left__is_zero")
                    .instructions(
                        InstructionBuilder::default()
                            .lda_imm(0x00)
                            .cmp_addr("engine__frames_left")
                            .bne_addr("engine__frames_left__is_zero__exit")
                            .cmp_addr_offs("engine__frames_left", 1)
                            .label("engine__frames_left__is_zero__exit")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .build()
    }
}
