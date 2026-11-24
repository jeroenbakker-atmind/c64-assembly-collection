use c64_assembler::{
    builder::{ApplicationBuilder, FunctionBuilder, InstructionBuilder, ModuleBuilder},
    Module,
};

use crate::command::DecoderModule;

pub const MUSIC_POSITION_ADDR: u16 = 0x3E00;
pub const DISK_TURNED_ADDR: u16 = 0x3E01;

pub trait ConditionBuilder {
    fn add_condition_addresses(&mut self) -> &mut Self;
}

impl ConditionBuilder for ApplicationBuilder {
    fn add_condition_addresses(&mut self) -> &mut Self {
        self.define_address("MUSIC_POSITION_ADDR", MUSIC_POSITION_ADDR)
            .define_address("DISK_TURNED_LATCH", DISK_TURNED_ADDR)
            .define_address("CONDITION_VALUE", 0x8E)
    }
}

pub struct Condition {}

impl DecoderModule for Condition {
    fn module() -> Module {
        ModuleBuilder::default()
            .name("engine__condition")
            .function(
                FunctionBuilder::default()
                    .name("engine__condition__init")
                    .doc(&[
                        "Reset condition inputs.",
                        "",
                        "The disk-turned latch is sticky until this routine is called again.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_imm(0)
                            .sta_addr("MUSIC_POSITION_ADDR")
                            .sta_addr("DISK_TURNED_LATCH")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__condition__music_before")
                    .doc(&[
                        "Return Z=1 when the current music position is before A.",
                        "",
                        "The music driver writes MUSIC_POSITION_ADDR; comparison has no side",
                        "effects on the loader.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .sta_addr("CONDITION_VALUE")
                            .lda_addr("MUSIC_POSITION_ADDR")
                            .cmp_addr("CONDITION_VALUE")
                            .bcs_addr("engine__condition__music_before__not_before")
                            .lda_imm(0)
                            .rts()
                            .label("engine__condition__music_before__not_before")
                            .lda_imm(1)
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__condition__disk_turned")
                    .doc(&["Return Z=1 when the sticky disk-turned latch is set."])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_addr("DISK_TURNED_LATCH")
                            .beq_addr("engine__condition__disk_turned__not_turned")
                            .lda_imm(0)
                            .rts()
                            .label("engine__condition__disk_turned__not_turned")
                            .lda_imm(1)
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__condition__mark_disk_turned")
                    .doc(&["Set the sticky disk-turned latch from a test or loader backend."])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_imm(1)
                            .sta_addr("DISK_TURNED_LATCH")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .build()
    }
}
