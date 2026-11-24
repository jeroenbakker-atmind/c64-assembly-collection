use c64_assembler::{
    builder::{ApplicationBuilder, FunctionBuilder, InstructionBuilder, ModuleBuilder},
    Module,
};

use crate::command::DecoderModule;

pub const LOADER_STATUS_DONE: u8 = 0b0000_0001;
pub const LOADER_STATUS_BUSY: u8 = 0b0000_0010;
pub const LOADER_STATUS_ERROR: u8 = 0b0000_0100;

/// A backend provides the implementation of the backend hooks called by the
/// shared `engine__loader__*` entry points.
pub trait LoaderBackend {
    fn module() -> Module;
}

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

/// Shared single-job loader contract.
///
/// Build-time sequence validation guarantees that `start` is never called
/// while a job is active, so there is deliberately no runtime
/// wait-required latch here. Backend selection happens in Rust when the
/// application is built; both backends expose the same entry points to the
/// emitted sequence.
pub struct Loader {}

impl DecoderModule for Loader {
    fn module() -> Module {
        ModuleBuilder::default()
            .name("engine__loader")
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__init")
                    .doc(&[
                        "Reset the shared loader state and initialize the selected backend.",
                        "",
                        "The regular backend is installed by `add_engine()`; tests can select",
                        "the deterministic stub backend with `add_engine_with_loader()`.",
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
                            .jsr_addr("engine__loader__backend__init")
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
                        "payload size. This routine returns immediately; the selected backend",
                        "completes the transfer through `engine__loader__on_irq`.",
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
                            .jsr_addr("engine__loader__backend__start")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__on_irq")
                    .doc(&[
                        "Advance the one active load through the selected backend.",
                        "",
                        "The regular backend performs a bounded slice of its IEC/U1 state",
                        "machine here; the stub backend copies a preloaded image.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_addr("LOADER_STATUS")
                            .and_imm(LOADER_STATUS_BUSY)
                            .beq_addr("engine__loader__on_irq__idle")
                            .jsr_addr("engine__loader__backend__on_irq")
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
                    .doc(&["Stop the selected backend and unblock any waiter."])
                    .instructions(
                        InstructionBuilder::default()
                            .jsr_addr("engine__loader__backend__deinit")
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
