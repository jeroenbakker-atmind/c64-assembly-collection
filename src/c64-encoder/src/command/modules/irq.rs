use c64_assembler::{builder::FunctionBuilder, builder::InstructionBuilder, builder::ModuleBuilder, Module};

use crate::command::DecoderModule;

/// Shared IRQ owner.
///
/// Raster refresh pacing and CIA2-timer loader servicing are both hooks of this
/// dispatcher. The compatibility handler in `Refresh` remains available for
/// refresh-only tests.
pub struct Irq {}

impl DecoderModule for Irq {
    fn module() -> Module {
        ModuleBuilder::default()
            .name("engine__irq")
            .function(
                FunctionBuilder::default()
                    .name("engine__irq__handler")
                    .doc(&[
                        "Service raster refresh pacing and CIA2-timer IEC phases.",
                        "",
                        "The VIC source is acknowledged with `ASL VIC2_IRQ_STATUS`; reading",
                        "CIA2 ICR acknowledges the Timer A source. The loader hook is only",
                        "called for the timer so frame pacing stays once per frame.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_addr("VIC2_IRQ_STATUS")
                            .and_imm(0b1000_0000)
                            .beq_addr("engine__irq__handler__cia")
                            .jsr_addr("engine__refresh__on_refresh")
                            .asl_addr("VIC2_IRQ_STATUS")
                            .label("engine__irq__handler__cia")
                            .lda_addr("CIA2_IRQ_CONTROL")
                            .and_imm(0b0000_0001)
                            .beq_addr("engine__irq__handler__done")
                            .jsr_addr("engine__loader__on_irq")
                            .label("engine__irq__handler__done")
                            .jmp_addr("C64_IRQ_EPLOGUE")
                            .build(),
                    )
                    .build(),
            )
            .build()
    }
}
