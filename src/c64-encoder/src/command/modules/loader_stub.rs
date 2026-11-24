use c64_assembler::{
    builder::{FunctionBuilder, InstructionBuilder, ModuleBuilder},
    Module,
};

use super::{LoaderBackend, LOADER_STATUS_DONE};

/// Deterministic backend used by CPU tests.
///
/// It reads a preloaded image from `LOADER_STUB_SRC` and copies the exact
/// payload size in one `on_irq` call. It exposes the same backend hooks as the
/// regular host-side IEC backend, so the shared loader contract is identical.
pub struct StubLoaderBackend {}

impl LoaderBackend for StubLoaderBackend {
    fn module() -> Module {
        ModuleBuilder::default()
            .name("engine__loader__backend_stub")
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__backend__init")
                    .doc(&["Stub backend initialization is a no-op."])
                    .instructions(InstructionBuilder::default().rts().build())
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__backend__start")
                    .doc(&["The stub uses the source pointer already stored in zero page."])
                    .instructions(InstructionBuilder::default().rts().build())
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__backend__on_irq")
                    .doc(&["Copy the preloaded image to the resource destination."])
                    .instructions(
                        InstructionBuilder::default()
                            .ldy_imm(0)
                            .label("engine__loader__backend__stub__next")
                            .lda_addr("LOADER_REMAINING")
                            .bne_addr("engine__loader__backend__stub__copy")
                            .lda_addr_offs("LOADER_REMAINING", 1)
                            .beq_addr("engine__loader__backend__stub__complete")
                            .label("engine__loader__backend__stub__copy")
                            .lda_ind_y("LOADER_STUB_SRC")
                            .sta_ind_y("LOADER_DST_PTR")
                            .inc_addr("LOADER_DST_PTR")
                            .bne_addr("engine__loader__backend__stub__src")
                            .inc_addr_offs("LOADER_DST_PTR", 1)
                            .label("engine__loader__backend__stub__src")
                            .inc_addr("LOADER_STUB_SRC")
                            .bne_addr("engine__loader__backend__stub__decrement")
                            .inc_addr_offs("LOADER_STUB_SRC", 1)
                            .label("engine__loader__backend__stub__decrement")
                            .dec_addr("LOADER_REMAINING")
                            .bne_addr("engine__loader__backend__stub__next")
                            .lda_addr_offs("LOADER_REMAINING", 1)
                            .beq_addr("engine__loader__backend__stub__complete")
                            .dec_addr_offs("LOADER_REMAINING", 1)
                            .jmp_addr("engine__loader__backend__stub__next")
                            .label("engine__loader__backend__stub__complete")
                            .lda_imm(LOADER_STATUS_DONE)
                            .sta_addr("LOADER_STATUS")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__backend__deinit")
                    .doc(&["Stub backend shutdown is a no-op."])
                    .instructions(InstructionBuilder::default().rts().build())
                    .build(),
            )
            .build()
    }
}
