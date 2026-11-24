use c64_assembler::{
    builder::{ApplicationBuilder, FunctionBuilder, InstructionBuilder, ModuleBuilder},
    Module,
};

use crate::command::DecoderModule;

/// Number of vertical refreshes per encoded frame: the authored 50/4 = 12.5 fps.
pub const REFRESHES_PER_FRAME: u8 = 4;

pub trait RefreshBuilder {
    fn add_refresh_addresses(&mut self) -> &mut Self;
}

impl RefreshBuilder for ApplicationBuilder {
    fn add_refresh_addresses(&mut self) -> &mut Self {
        self.define_address("C64_IRQ_VECTOR", 0x0314)
            .define_address("C64_IRQ_EPLOGUE", 0xEA81)
            .define_address("CIA1_IRQ_CONTROL", 0xDC0D)
            .define_address("CIA2_IRQ_CONTROL", 0xDD0D)
    }
}

pub struct Refresh {}

impl DecoderModule for Refresh {
    fn module() -> Module {
        ModuleBuilder::default()
            .name("engine__refresh")
            .instructions(
                InstructionBuilder::default()
                    .label("engine__refresh__remaining")
                    .comment("Refreshes still owed to the current frame; 0 means the interval completed.")
                    .raw(&[REFRESHES_PER_FRAME])
                    .label("engine__refresh__irq_vector_save")
                    .comment("Original KERNAL IRQ vector, restored on deinit.")
                    .raw(&[0x00; 2])
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__refresh__on_refresh")
                    .doc(&[
                        "Subtract one refresh from the countdown, saturating at zero.",
                        "",
                        "The ISR calls this as a subroutine so the countdown can be tested",
                        "without IRQ support in the 6502 emulator.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_addr("engine__refresh__remaining")
                            .beq_addr("engine__refresh__on_refresh__done")
                            .comment("Never wrap 0 to 255: that would stall for 255 refreshes.")
                            .dec_addr("engine__refresh__remaining")
                            .label("engine__refresh__on_refresh__done")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__refresh__irq")
                    .doc(&[
                        "VIC raster IRQ handler, installed in $0314/$0315.",
                        "",
                        "Fires once per frame at raster line 0 on both PAL and NTSC.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .jsr_addr("engine__refresh__on_refresh")
                            .comment("Acknowledge the VIC raster IRQ.")
                            .asl_addr("VIC2_IRQ_STATUS")
                            .comment("KERNAL: restore A/X/Y and RTI.")
                            .jmp_addr("C64_IRQ_EPLOGUE")
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__refresh__init")
                    .doc(&[
                        "Install the raster IRQ and arm the refresh countdown.",
                        "",
                        "Call after VIC bank/memory setup and before engine__play.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .sei()
                            .comment("Save the KERNAL IRQ vector for deinit.")
                            .lda_addr("C64_IRQ_VECTOR")
                            .sta_addr("engine__refresh__irq_vector_save")
                            .lda_addr_offs("C64_IRQ_VECTOR", 1)
                            .sta_addr_offs("engine__refresh__irq_vector_save", 1)
                            .comment("Disable CIA-1 IRQ sources and acknowledge pending CIA IRQs.")
                            .lda_imm(0b0111_1111)
                            .sta_addr("CIA1_IRQ_CONTROL")
                            .lda_addr("CIA1_IRQ_CONTROL")
                            .lda_addr("CIA2_IRQ_CONTROL")
                            .comment("Keep raster bit 8 of $D011 clear.")
                            .lda_addr("VIC2_CONTROL_1")
                            .and_imm(0b0111_1111)
                            .sta_addr("VIC2_CONTROL_1")
                            .comment("Clear a pending raster IRQ before enabling the source.")
                            .lda_imm(0b0000_0001)
                            .sta_addr("VIC2_IRQ_STATUS")
                            .comment("Install the handler and compare against raster line 0.")
                            .lda_imm_low("engine__refresh__irq")
                            .sta_addr("C64_IRQ_VECTOR")
                            .lda_imm_high("engine__refresh__irq")
                            .sta_addr_offs("C64_IRQ_VECTOR", 1)
                            .lda_imm(0)
                            .sta_addr("VIC2_RASTER")
                            .lda_imm(0b0000_0001)
                            .sta_addr("VIC2_IRQ_ENABLE")
                            .comment("Four refreshes per frame implement the authored 50/4 framerate.")
                            .lda_imm(REFRESHES_PER_FRAME)
                            .sta_addr("engine__refresh__remaining")
                            .cli()
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__refresh__deinit")
                    .doc(&["Disable the raster IRQ and restore the original KERNAL IRQ vector."])
                    .instructions(
                        InstructionBuilder::default()
                            .sei()
                            .lda_imm(0)
                            .sta_addr("VIC2_IRQ_ENABLE")
                            .lda_imm(0b0000_0001)
                            .sta_addr("VIC2_IRQ_STATUS")
                            .lda_addr("engine__refresh__irq_vector_save")
                            .sta_addr("C64_IRQ_VECTOR")
                            .lda_addr_offs("engine__refresh__irq_vector_save", 1)
                            .sta_addr_offs("C64_IRQ_VECTOR", 1)
                            .cli()
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__refresh__wait")
                    .doc(&[
                        "Wait for the refreshes still owed to the current frame.",
                        "",
                        "Polls the live countdown byte, so refreshes that arrive while decoding are",
                        "already accounted for. When it reaches 0, the countdown is increased by 4",
                        "and the next frame can start immediately.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_addr("engine__refresh__remaining")
                            .bne_addr("engine__refresh__wait")
                            .lda_imm(REFRESHES_PER_FRAME)
                            .sta_addr("engine__refresh__remaining")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .build()
    }
}
