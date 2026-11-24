use c64_assembler::{
    builder::{ApplicationBuilder, FunctionBuilder, InstructionBuilder, ModuleBuilder},
    Module,
};

use crate::command::DecoderModule;

pub const LOADER_STATUS_DONE: u8 = 0b0000_0001;
pub const LOADER_STATUS_BUSY: u8 = 0b0000_0010;
pub const LOADER_STATUS_ERROR: u8 = 0b0000_0100;

pub trait LoaderBuilder {
    fn add_loader_addresses(&mut self) -> &mut Self;
}

impl LoaderBuilder for ApplicationBuilder {
    fn add_loader_addresses(&mut self) -> &mut Self {
        self.define_address("LOADER_STATUS", 0x80)
            .define_address("LOADER_CURRENT_RESOURCE", 0x81)
            .define_address("LOADER_DST_PTR", 0x82)
            .define_address("LOADER_REMAINING", 0x84)
            .define_address("LOADER_ERROR", 0x86)
            .define_address("LOADER_STUB_SRC", 0x8C)
    }
}

/// Single-job loader with a deterministic copy stub backend.
///
/// Build-time sequence validation guarantees that `start` is never called
/// while a job is active, so there is deliberately no runtime
/// wait-required latch here.
pub struct Loader {}

impl DecoderModule for Loader {
    fn module() -> Module {
        ModuleBuilder::default()
            .name("engine__loader")
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__init")
                    .doc(&[
                        "Reset the loader state.",
                        "",
                        "The stub backend reads from LOADER_STUB_SRC; no CIA source is enabled here.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_imm(0)
                            .sta_addr("LOADER_STATUS")
                            .sta_addr("LOADER_CURRENT_RESOURCE")
                            .sta_addr("LOADER_DST_PTR")
                            .sta_addr_offs("LOADER_DST_PTR", 1)
                            .sta_addr("LOADER_REMAINING")
                            .sta_addr_offs("LOADER_REMAINING", 1)
                            .sta_addr("LOADER_ERROR")
                            .sta_addr("LOADER_STUB_SRC")
                            .sta_addr_offs("LOADER_STUB_SRC", 1)
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__start")
                    .doc(&[
                        "Publish a single asynchronous load request.",
                        "",
                        "A = resource id. The resource table supplies the destination and exact",
                        "payload size. This routine returns immediately; `engine__loader__on_irq`",
                        "performs the stub transfer.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .sta_addr("LOADER_CURRENT_RESOURCE")
                            .jsr_addr("engine__resource__dest")
                            .stx_addr("LOADER_DST_PTR")
                            .sta_addr_offs("LOADER_DST_PTR", 1)
                            .lda_addr("LOADER_CURRENT_RESOURCE")
                            .jsr_addr("engine__resource__payload_bytes")
                            .stx_addr("LOADER_REMAINING")
                            .sta_addr_offs("LOADER_REMAINING", 1)
                            .lda_imm(0)
                            .sta_addr("LOADER_ERROR")
                            .lda_imm(LOADER_STATUS_BUSY)
                            .sta_addr("LOADER_STATUS")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__on_irq")
                    .doc(&[
                        "Advance the one active stub load.",
                        "",
                        "The production backend replaces this routine with a drive protocol;",
                        "the sequence and resource table stay unchanged.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_addr("LOADER_STATUS")
                            .and_imm(LOADER_STATUS_BUSY)
                            .beq_addr("engine__loader__on_irq__idle")
                            .ldy_imm(0)
                            .label("engine__loader__on_irq__next")
                            .lda_addr("LOADER_REMAINING")
                            .bne_addr("engine__loader__on_irq__copy")
                            .lda_addr_offs("LOADER_REMAINING", 1)
                            .beq_addr("engine__loader__on_irq__complete")
                            .label("engine__loader__on_irq__copy")
                            .lda_ind_y("LOADER_STUB_SRC")
                            .sta_ind_y("LOADER_DST_PTR")
                            .inc_addr("LOADER_DST_PTR")
                            .bne_addr("engine__loader__on_irq__src")
                            .inc_addr_offs("LOADER_DST_PTR", 1)
                            .label("engine__loader__on_irq__src")
                            .inc_addr("LOADER_STUB_SRC")
                            .bne_addr("engine__loader__on_irq__decrement")
                            .inc_addr_offs("LOADER_STUB_SRC", 1)
                            .label("engine__loader__on_irq__decrement")
                            .dec_addr("LOADER_REMAINING")
                            .bne_addr("engine__loader__on_irq__next")
                            .lda_addr_offs("LOADER_REMAINING", 1)
                            .beq_addr("engine__loader__on_irq__complete")
                            .dec_addr_offs("LOADER_REMAINING", 1)
                            .jmp_addr("engine__loader__on_irq__next")
                            .label("engine__loader__on_irq__complete")
                            .lda_imm(LOADER_STATUS_DONE)
                            .sta_addr("LOADER_STATUS")
                            .label("engine__loader__on_irq__idle")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__is_done")
                    .doc(&["Return Z=1 when the active load has completed."])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_addr("LOADER_STATUS")
                            .and_imm(LOADER_STATUS_DONE)
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__wait_loaded")
                    .doc(&[
                        "Spin until the in-flight load reports completion.",
                        "",
                        "This only polls the status byte; it never clears a latch or re-arms a load.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .label("engine__loader__wait_loaded__next")
                            .lda_addr("LOADER_STATUS")
                            .and_imm(LOADER_STATUS_DONE)
                            .beq_addr("engine__loader__wait_loaded__next")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__deinit")
                    .doc(&["Stop the stub transfer and unblock any waiter."])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_imm(0)
                            .sta_addr("LOADER_STATUS")
                            .sta_addr("LOADER_REMAINING")
                            .sta_addr_offs("LOADER_REMAINING", 1)
                            .lda_imm(LOADER_STATUS_DONE)
                            .sta_addr("LOADER_STATUS")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .build()
    }
}
