use c64_assembler::{builder::FunctionBuilder, builder::InstructionBuilder, builder::ModuleBuilder, Module};

use super::loader_regular::{IEC_TIMER_LATCH_LO, TIMER_B_REARM_DELAY};
use super::LOADER_STATUS_BUSY;
use crate::command::DecoderModule;

/// Owns the two engine interrupt hooks.
///
/// Raster refresh pacing uses the maskable IRQ vector at `$0314`; the KERNAL
/// IRQ entry saves A/X/Y before calling `engine__irq__handler`, and
/// `engine__irq__epilogue` restores them. The KERNAL NMI entry at `$FE43` is
/// only `SEI; JMP ($0318)`: it does not save registers and does not acknowledge
/// CIA flags. `engine__irq__nmi_handler` therefore saves A/X/Y itself, reads
/// CIA2 ICR to acknowledge the timer flag, and returns with `RTI`.
pub struct Irq {}

impl DecoderModule for Irq {
    fn module() -> Module {
        ModuleBuilder::default()
            .name("engine__irq")
            .function(
                FunctionBuilder::default()
                    .name("engine__irq__epilogue")
                    .doc(&["Restore the registers saved by the KERNAL IRQ entry point and return."])
                    .instructions(
                        InstructionBuilder::default()
                            .pla()
                            .tay()
                            .pla()
                            .tax()
                            .pla()
                            .rti()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__irq__nmi_handler")
                    .doc(&[
                        "CIA2 Timer A NMI handler, installed in $0318.",
                        "",
                        "The KERNAL NMI entry only executes SEI and JMP ($0318), so this handler",
                        "owns the register frame. A is pushed before the re-entrancy guard is",
                        "incremented; X/Y are pushed only for the owner that runs a loader phase.",
                        "Reading CIA2 ICR acknowledges the timer flag. The owner records the Timer",
                        "B handoff in ENGINE_NMI_TIMER_B_PENDING so the next entry can distinguish",
                        "the one-shot restart from a Timer A phase.",
                        "",
                        "Both timers are stopped at entry. The owner runs exactly one loader phase,",
                        "then arms one-shot Timer B instead of restarting Timer A before its own",
                        "return. Timer B's NMI arrives after the owner has returned and restarts",
                        "Timer A for the next phase.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .pha()
                            .inc_addr("ENGINE_NMI_ACTIVE")
                            .lda_imm(0)
                            .sta_addr("CIA2_TIMER_A_CONTROL")
                            .sta_addr("CIA2_TIMER_B_CONTROL")
                            .lda_addr("CIA2_IRQ_CONTROL")
                            .lda_addr("ENGINE_NMI_ACTIVE")
                            .cmp_imm(1)
                            .bne_addr("engine__irq__nmi_handler__nested")
                            .lda_addr("ENGINE_NMI_TIMER_B_PENDING")
                            .bne_addr("engine__irq__nmi_handler__timer_b")
                            .txa()
                            .pha()
                            .tya()
                            .pha()
                            .jsr_addr("engine__loader__on_irq")
                            .jmp_addr("engine__irq__nmi_handler__owner_done")
                            .label("engine__irq__nmi_handler__nested")
                            .dec_addr("ENGINE_NMI_ACTIVE")
                            .pla()
                            .rti()
                            .label("engine__irq__nmi_handler__timer_b")
                            .lda_imm(0)
                            .sta_addr("ENGINE_NMI_TIMER_B_PENDING")
                            .lda_imm(0b0000_0010)
                            .sta_addr("CIA2_IRQ_CONTROL")
                            .lda_imm(0)
                            .sta_addr("CIA2_TIMER_B_CONTROL")
                            .lda_imm(IEC_TIMER_LATCH_LO)
                            .sta_addr("CIA2_TIMER_A_LO")
                            .lda_imm(0)
                            .sta_addr("CIA2_TIMER_A_HI")
                            .lda_imm(0b1000_0001)
                            .sta_addr("CIA2_IRQ_CONTROL")
                            .lda_imm(0b0000_0001)
                            .sta_addr("CIA2_TIMER_A_CONTROL")
                            .dec_addr("ENGINE_NMI_ACTIVE")
                            .pla()
                            .rti()
                            .label("engine__irq__nmi_handler__owner_done")
                            .lda_addr("LOADER_STATUS")
                            .and_imm(LOADER_STATUS_BUSY)
                            .beq_addr("engine__irq__nmi_handler__clear_guard")
                            .lda_imm(0)
                            .sta_addr("CIA2_TIMER_A_CONTROL")
                            .lda_imm(TIMER_B_REARM_DELAY)
                            .sta_addr("CIA2_TIMER_B_LO")
                            .lda_imm(0)
                            .sta_addr("CIA2_TIMER_B_HI")
                            .lda_imm(0b0000_0010)
                            .sta_addr("CIA2_IRQ_CONTROL")
                            .lda_imm(0b1000_0010)
                            .sta_addr("CIA2_IRQ_CONTROL")
                            .lda_imm(1)
                            .sta_addr("ENGINE_NMI_TIMER_B_PENDING")
                            .lda_imm(0b0000_1001)
                            .sta_addr("CIA2_TIMER_B_CONTROL")
                            .label("engine__irq__nmi_handler__clear_guard")
                            .lda_imm(0)
                            .sta_addr("ENGINE_NMI_ACTIVE")
                            .pla()
                            .tay()
                            .pla()
                            .tax()
                            .pla()
                            .rti()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__irq__handler")
                    .doc(&[
                        "Service raster refresh pacing from the VIC raster IRQ.",
                        "",
                        "The VIC raster flag is bit 7, but sprite latches can also be pending.",
                        "Writing `#$FF` to `VIC2_IRQ_STATUS` acknowledges every VIC source;",
                        "writing only `#$80` can leave a sprite IRQ asserted and starve the main",
                        "loop. CIA2 Timer A",
                        "is an NMI source and is serviced by `engine__irq__nmi_handler`.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_addr("VIC2_IRQ_STATUS")
                            .and_imm(0b1000_0000)
                            .beq_addr("engine__irq__handler__done")
                            .jsr_addr("engine__refresh__on_refresh")
                            .lda_imm(0b1111_1111)
                            .sta_addr("VIC2_IRQ_STATUS")
                            .label("engine__irq__handler__done")
                            .jmp_addr("engine__irq__epilogue")
                            .build(),
                    )
                    .build(),
            )
            .build()
    }
}
