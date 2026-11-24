use c64_assembler::{builder::FunctionBuilder, builder::InstructionBuilder, builder::ModuleBuilder, Module};

use crate::command::DecoderModule;

/// Shared raster IRQ owner.
///
/// Refresh pacing and loader servicing are both hooks of this dispatcher. The
/// compatibility handler in `Refresh` remains available for refresh-only tests.
pub struct Irq {}

impl DecoderModule for Irq {
    fn module() -> Module {
        ModuleBuilder::default()
            .name("engine__irq")
            .function(
                FunctionBuilder::default()
                    .name("engine__irq__handler")
                    .doc(&[
                        "Service refresh pacing and the single active loader job.",
                        "",
                        "Only raster IRQs are enabled, so `ASL VIC2_IRQ_STATUS` acknowledges",
                        "the pending source before returning through the KERNAL epilogue.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .jsr_addr("engine__refresh__on_refresh")
                            .jsr_addr("engine__loader__on_irq")
                            .asl_addr("VIC2_IRQ_STATUS")
                            .jmp_addr("C64_IRQ_EPLOGUE")
                            .build(),
                    )
                    .build(),
            )
            .build()
    }
}
