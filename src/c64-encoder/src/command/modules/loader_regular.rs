use c64_assembler::{
    builder::{ApplicationBuilder, FunctionBuilder, InstructionBuilder, ModuleBuilder},
    Instructions, Module,
};

use super::{LoaderBackend, LOADER_STATUS_DONE, LOADER_STATUS_ERROR};

pub const LOADER_REGULAR_DEVICE_LISTEN: u8 = 0x20 + 8;
pub const LOADER_REGULAR_DEVICE_TALK: u8 = 0x40 + 8;
pub const LOADER_REGULAR_OPEN_BUFFER: u8 = 0xF0 + 2;
pub const LOADER_REGULAR_OPEN_COMMAND: u8 = 0xF0 + 15;
pub const LOADER_REGULAR_SECOND_BUFFER: u8 = 0x60 + 2;
pub const LOADER_REGULAR_CLOSE_BUFFER: u8 = 0xE0 + 2;
pub const LOADER_REGULAR_UNLISTEN: u8 = 0x3F;
pub const LOADER_REGULAR_UNTALK: u8 = 0x5F;

/// IEC flags stored in `LOADER_IEC_FLAGS`.
const IEC_EOI: u8 = 0b0000_0001;
const IEC_EOI_SEEN: u8 = 0b0000_0010;
const IEC_ATN: u8 = 0b0000_0100;
const IEC_ERROR: u8 = 0b0000_1000;
const IEC_RELEASE_ATN: u8 = 0b0001_0000;
const IEC_RELEASE_ALL: u8 = 0b0010_0000;

/// CIA2 port output bits. The serial drivers invert the line: writing a one
/// asserts (pulls low) the bus line, while writing a zero releases it.
const OUT_ATN: u8 = 0b0000_1000;
const OUT_CLK: u8 = 0b0001_0000;
const OUT_DATA: u8 = 0b0010_0000;

/// CIA2 port input bits read as one while the line is released/high and zero
/// while the line is asserted/low. KERNAL ACPTR's `ASL $DD00` leaves CLK in N
/// (original bit 6) and DATA in C (original bit 7) at `$EE5A`; `$EEA9` returns
/// that same status, so the receive loop must test these exact bits.
const IN_CLK: u8 = 0b0100_0000;
const IN_DATA: u8 = 0b1000_0000;
/// CIA2 port base image used by KERNAL while the VIC stays in bank 0:
/// bits 0-1 select `$0000-$3FFF`, bit 2 is fixed high, and input bits 6-7 are
/// written zero so VICE releases the CLK/DATA line drivers.
const PORT_KERNAL_BASE: u8 = 0b0000_0111;

/// Timer A period. At ~1 MHz this is 100 cycles / 25 us, short enough to keep
/// IEC bit timing valid while still yielding the CPU between phases.
pub const IEC_TIMER_LATCH_LO: u8 = 50;
const IEC_TIMER_LATCH_HI: u8 = 0;

/// KERNAL's ISOURA holds CLK low for about 1 ms before accepting the first
/// listener acknowledgement. Each phase includes the NMI handler, so twenty
/// Timer A periods are roughly one millisecond in practice.
const IEC_SEND_SETTLE_PHASES: u8 = 20;

/// KERNAL waits about 1 ms after UNLISTEN before asserting ATN for the next
/// command. The 1541 is still completing the previous job during that gap, so
/// reasserting ATN too early makes it miss the next LISTEN byte.
const IEC_ATN_RELEASE_SETTLE_PHASES: u8 = 40;

/// Timer B delay between Timer A phases. Starting Timer B only after the owner
/// has finished its loader phase means its NMI arrives after the owner's RTI,
/// so no new phase can begin while an earlier NMI frame is still unwinding.
pub const TIMER_B_REARM_DELAY: u8 = 100;

pub trait LoaderRegularBuilder {
    fn add_loader_regular_addresses(&mut self) -> &mut Self;
}

impl LoaderRegularBuilder for ApplicationBuilder {
    fn add_loader_regular_addresses(&mut self) -> &mut Self {
        self.define_address("LOADER_IEC_STATE", 0x3E20)
            .define_address("LOADER_IEC_PHASE", 0x3E21)
            .define_address("LOADER_IEC_BIT", 0x3E22)
            .define_address("LOADER_IEC_BYTE", 0x3E23)
            .define_address("LOADER_IEC_FLAGS", 0x3E24)
            .define_address("LOADER_IEC_PORT", 0x3E25)
            .define_address("LOADER_IEC_TRACK", 0x3E26)
            .define_address("LOADER_IEC_SECTOR", 0x3E27)
            .define_address("LOADER_IEC_SECTOR_LEFT", 0x3E28)
            .define_address("LOADER_IEC_INDEX", 0x3E29)
            .define_address("LOADER_IEC_TENS", 0x3E2A)
            .define_address("LOADER_IEC_ONES", 0x3E2B)
            .define_address("LOADER_IEC_MODULUS", 0x3E2C)
            .define_address("LOADER_IEC_TIMEOUT_HI", 0x3E2D)
            .define_address("LOADER_IEC_TIMEOUT", 0x3E2E)
            .define_address("LOADER_IEC_BUFFER_OPEN", 0x3E30)
            .define_address("LOADER_IEC_SETTLE", 0x3E32)
            .define_address("LOADER_IEC_POSITION", 0x3E35)
            .define_address("LOADER_IEC_POSITION_SCRATCH", 0x3E36)
            .define_address("LOADER_INTERLEAVE", 7)
            .define_address("CIA2_DATA_PORT", 0xDD00)
            .define_address("CIA2_DATA_DDR", 0xDD02)
            .define_address("CIA2_TIMER_A_LO", 0xDD04)
            .define_address("CIA2_TIMER_A_HI", 0xDD05)
            .define_address("CIA2_TIMER_B_LO", 0xDD06)
            .define_address("CIA2_TIMER_B_HI", 0xDD07)
            .define_address("CIA2_TIMER_A_CONTROL", 0xDD0E)
            .define_address("CIA2_TIMER_B_CONTROL", 0xDD0F)
    }
}

/// Default regular loader backend.
///
/// The C64 drives the standard IEC bus directly from a CIA2 Timer A NMI: no
/// KERNAL serial routine is called from the handler and no 1541 drive code is
/// uploaded. Channel 2 is allocated once with `#2`; each sector is fetched
/// with `U1:2,0,track,sector`, and exactly 256 bytes are read back. Only the
/// payload bytes still owed by the resource are copied to its destination.
///
/// One bounded IEC step runs per timer NMI (~50 us): a handshake phase for
/// commands and a complete byte handshake for receive data. The main thread
/// keeps decoding frames and the music hook keeps running while the transfer
/// is in flight. ATN, EOI, turnaround, and byte handshakes follow the polarity
/// and LSB-first order of the KERNAL serial routines. Real-drive timing and
/// the final interleave remain a hardware/VICE tuning pass.
pub struct RegularLoaderBackend {}

impl LoaderBackend for RegularLoaderBackend {
    fn module() -> Module {
        ModuleBuilder::default()
            .name("engine__loader__backend_regular")
            .instructions(
                InstructionBuilder::default()
                    .label("loader_iec_open_command")
                    .raw(b"#2")
                    .label("loader_iec_u1_command")
                    .raw(&[0; 12])
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__backend__init")
                    .doc(&[
                        "Configure CIA2 for host-side standard serial and release all lines.",
                        "",
                        "Bits 0-5 are outputs (VIC bank and serial); bits 6-7 are the CLK/DATA",
                        "inputs. Timer A is stopped here and started only for an active job.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_imm(0b0011_1111)
                            .sta_addr("CIA2_DATA_DDR")
                            .comment("VIC bank 0 is fixed; release the serial lines from KERNAL's base.")
                            .jsr_addr("engine__loader__regular__release_all")
                            .jsr_addr("engine__loader__regular__disable_timer")
                            .lda_imm(0)
                            .sta_addr("LOADER_IEC_STATE")
                            .sta_addr("LOADER_IEC_PHASE")
                            .sta_addr("LOADER_IEC_FLAGS")
                            .sta_addr("LOADER_IEC_TIMEOUT")
                            .sta_addr("LOADER_IEC_TIMEOUT_HI")
                            .sta_addr("LOADER_IEC_BUFFER_OPEN")
                            .sta_addr("LOADER_IEC_SETTLE")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__backend__start")
                    .doc(&[
                        "Read the physical sector chain and start the first IEC command.",
                        "",
                        "The DOS buffer is allocated lazily by the first state-machine pass;",
                        "it stays allocated until deinit so later sectors need no OPEN.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_addr("LOADER_CURRENT_RESOURCE")
                            .jsr_addr("engine__resource__track")
                            .sta_addr("LOADER_IEC_TRACK")
                            .lda_addr("LOADER_CURRENT_RESOURCE")
                            .jsr_addr("engine__resource__sector")
                            .sta_addr("LOADER_IEC_SECTOR")
                            .lda_addr("LOADER_CURRENT_RESOURCE")
                            .jsr_addr("engine__resource__sector_count")
                            .sta_addr("LOADER_IEC_SECTOR_LEFT")
                            .comment("Recover the planner's logical slot for the starting physical sector.")
                            .lda_addr("LOADER_IEC_TRACK")
                            .jsr_addr("engine__loader__regular__track_modulus")
                            .jsr_addr("engine__loader__regular__init_position")
                            .jsr_addr("engine__loader__regular__build_u1_command")
                            .jsr_addr("engine__loader__regular__release_all")
                            .lda_imm(0)
                            .sta_addr("LOADER_IEC_INDEX")
                            .sta_addr("LOADER_IEC_FLAGS")
                            .comment("Prepare the first byte before arming the timer; otherwise an NMI can interrupt prepare_send.")
                            .lda_addr("LOADER_IEC_BUFFER_OPEN")
                            .bne_addr("engine__loader__backend__start__reuse")
                            .jsr_addr("engine__loader__regular__prepare_listen_buffer")
                            .jmp_addr("engine__loader__backend__start__armed")
                            .label("engine__loader__backend__start__reuse")
                            .comment("The previous resource ended with UNLISTEN; wait before ATN falls again.")
                            .lda_imm(IEC_ATN_RELEASE_SETTLE_PHASES)
                            .sta_addr("LOADER_IEC_SETTLE")
                            .jsr_addr("engine__loader__regular__prepare_listen_command")
                            .label("engine__loader__backend__start__armed")
                            .jsr_addr("engine__loader__regular__enable_timer")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__backend__on_irq")
                    .doc(&["Run one bounded IEC phase of the regular U1 state machine."])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_addr("LOADER_IEC_STATE")
                            .beq_addr("engine__loader__backend__regular__idle")
                            .cmp_imm(12)
                            .bne_addr("engine__loader__backend__regular__not_turnaround")
                            .jsr_addr("engine__loader__regular__turnaround_phase")
                            .rts()
                            .label("engine__loader__backend__regular__not_turnaround")
                            .cmp_imm(13)
                            .bne_addr("engine__loader__backend__regular__send_phase")
                            .jsr_addr("engine__loader__regular__recv_phase")
                            .rts()
                            .label("engine__loader__backend__regular__send_phase")
                            .jsr_addr("engine__loader__regular__send_phase")
                            .label("engine__loader__backend__regular__idle")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__backend__deinit")
                    .doc(&["Stop the timer, release the bus, and leave the DOS buffer allocated."])
                    .instructions(
                        InstructionBuilder::default()
                            .jsr_addr("engine__loader__regular__disable_timer")
                            .jsr_addr("engine__loader__regular__release_all")
                            .lda_imm(0)
                            .sta_addr("LOADER_IEC_STATE")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__port_write")
                    .doc(&["Write the current CIA2 port image."])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_addr("LOADER_IEC_PORT")
                            .sta_addr("CIA2_DATA_PORT")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__release_all")
                    .doc(&[
                        "Release ATN, CLK and DATA while preserving VIC bank 0.",
                        "",
                        "Writing zero to the CIA input bits keeps their serial drivers released.",
                        "The drive floats DATA after acknowledging, so it must not be held by",
                        "a stale output bit.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_imm(PORT_KERNAL_BASE)
                            .sta_addr("LOADER_IEC_PORT")
                            .jsr_addr("engine__loader__regular__port_write")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__release_atn")
                    .doc(&["Release ATN only, as KERNAL SCATN does after a secondary address."])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_addr("LOADER_IEC_PORT")
                            .and_imm(!OUT_ATN)
                            .sta_addr("LOADER_IEC_PORT")
                            .jsr_addr("engine__loader__regular__port_write")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__release_clk")
                    .doc(&["Release CLK only during the read turnaround."])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_addr("LOADER_IEC_PORT")
                            .and_imm(!OUT_CLK)
                            .sta_addr("LOADER_IEC_PORT")
                            .jsr_addr("engine__loader__regular__port_write")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__line")
                    .doc(&["Write A's asserted-line mask from bits 3-5, preserving VIC bank 0."])
                    .instructions(
                        InstructionBuilder::default()
                            .sta_addr("LOADER_IEC_TENS")
                            .lda_imm(PORT_KERNAL_BASE)
                            .ora_addr("LOADER_IEC_TENS")
                            .sta_addr("LOADER_IEC_PORT")
                            .jsr_addr("engine__loader__regular__port_write")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__line_preserve_clk")
                    .doc(&[
                        "Write A's asserted-line mask while retaining the current ATN and CLK outputs.",
                        "",
                        "KERNAL sets each DATA bit before releasing CLK. ATN must stay asserted",
                        "for the whole command byte, and CLK must stay asserted between bits.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .sta_addr("LOADER_IEC_TENS")
                            .lda_addr("LOADER_IEC_PORT")
                            .and_imm(OUT_ATN | OUT_CLK)
                            .ora_addr("LOADER_IEC_TENS")
                            .ora_imm(PORT_KERNAL_BASE)
                            .sta_addr("LOADER_IEC_PORT")
                            .jsr_addr("engine__loader__regular__port_write")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__atn_mask")
                    .doc(&["Return the ATN output mask when the current byte is under ATN."])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_addr("LOADER_IEC_FLAGS")
                            .and_imm(IEC_ATN)
                            .beq_addr("engine__loader__regular__atn_mask__data")
                            .lda_imm(OUT_ATN)
                            .rts()
                            .label("engine__loader__regular__atn_mask__data")
                            .lda_imm(0)
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__disable_timer")
                    .doc(&["Stop both CIA2 timers and mask their IRQs without disturbing the port."])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_imm(0)
                            .sta_addr("CIA2_TIMER_A_CONTROL")
                            .sta_addr("CIA2_TIMER_B_CONTROL")
                            .lda_imm(0b0000_0011)
                            .sta_addr("CIA2_IRQ_CONTROL")
                            .lda_addr("CIA2_IRQ_CONTROL")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__enable_timer")
                    .doc(&[
                        "Start CIA2 Timer A as a ~50 us periodic source for IEC steps.",
                        "",
                        "CRA bit 3 is one-shot, so it stays clear; bit 0 starts the reload timer.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_imm(0)
                            .sta_addr("CIA2_TIMER_A_CONTROL")
                            .lda_imm(IEC_TIMER_LATCH_LO)
                            .sta_addr("CIA2_TIMER_A_LO")
                            .lda_imm(IEC_TIMER_LATCH_HI)
                            .sta_addr("CIA2_TIMER_A_HI")
                            .lda_addr("CIA2_IRQ_CONTROL")
                            .lda_imm(0b1000_0001)
                            .sta_addr("CIA2_IRQ_CONTROL")
                            .lda_imm(0b0000_0001)
                            .sta_addr("CIA2_TIMER_A_CONTROL")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__timeout_tick")
                    .doc(&[
                        "Decrement the two-byte phase timeout; return Z=1 while it is still valid.",
                        "",
                        "A low byte of 0 extends the window by 256 more decrements of that byte.",
                        "Receive edge polling uses an inline X countdown instead, because the JSR",
                        "here is too slow to sample short CLK edges.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .dec_addr("LOADER_IEC_TIMEOUT")
                            .bne_addr("engine__loader__regular__timeout_tick__valid")
                            .dec_addr("LOADER_IEC_TIMEOUT_HI")
                            .bne_addr("engine__loader__regular__timeout_tick__valid")
                            .lda_imm(1)
                            .rts()
                            .label("engine__loader__regular__timeout_tick__valid")
                            .lda_imm(0)
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__fail")
                    .doc(&[
                        "Record a sticky transfer error, stop the timer, and release the bus.",
                        "",
                        "Returns A=0/Z=1 so callers can distinguish an error through",
                        "LOADER_IEC_FLAGS without inverting the success flag.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .jsr_addr("engine__loader__regular__disable_timer")
                            .jsr_addr("engine__loader__regular__release_all")
                            .lda_addr("LOADER_IEC_FLAGS")
                            .ora_imm(IEC_ERROR)
                            .sta_addr("LOADER_IEC_FLAGS")
                            .lda_imm(LOADER_STATUS_DONE | LOADER_STATUS_ERROR)
                            .sta_addr("LOADER_STATUS")
                            .lda_addr("LOADER_IEC_STATE")
                            .sta_addr("LOADER_ERROR")
                            .lda_imm(0)
                            .sta_addr("LOADER_IEC_STATE")
                            .lda_imm(0)
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__prepare_send")
                    .doc(&["Start a new send byte; A holds the byte and FLAGS holds ATN/EOI bits."])
                    .instructions(
                        InstructionBuilder::default()
                            .sta_addr("LOADER_IEC_BYTE")
                            .lda_imm(0)
                            .sta_addr("LOADER_IEC_PHASE")
                            .lda_imm(8)
                            .sta_addr("LOADER_IEC_BIT")
                            .lda_imm(255)
                            .sta_addr("LOADER_IEC_TIMEOUT")
                            .lda_imm(255)
                            .sta_addr("LOADER_IEC_TIMEOUT_HI")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__prepare_recv")
                    .doc(&["Start a new receive byte with EOI detection enabled."])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_imm(13)
                            .sta_addr("LOADER_IEC_STATE")
                            .lda_imm(0)
                            .sta_addr("LOADER_IEC_PHASE")
                            .sta_addr("LOADER_IEC_BYTE")
                            .lda_imm(8)
                            .sta_addr("LOADER_IEC_BIT")
                            .lda_imm(IEC_EOI)
                            .sta_addr("LOADER_IEC_FLAGS")
                            .lda_imm(10)
                            .sta_addr("LOADER_IEC_TIMEOUT")
                            .lda_imm(1)
                            .sta_addr("LOADER_IEC_TIMEOUT_HI")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__send_phase")
                    .doc(&[
                        "Advance one IEC send phase with bounded handshakes.",
                        "",
                        "Bits are sent LSB first. DATA is released for one and asserted for zero;",
                        "CLK is released while a bit is valid and asserted between bits.",
                    ])
                    .instructions(send_phase())
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__recv_phase")
                    .doc(&[
                        "Receive one complete sector byte in a single bounded NMI phase.",
                        "",
                        "IEC bit edges are much shorter than one Timer A period, so the host polls",
                        "CLK and latches DATA while CLK is high, matching KERNAL ACPTR at $EE13.",
                        "A long CLK-high wait is EOI, acknowledged with a DATA pulse before the",
                        "final byte.",
                    ])
                    .instructions(recv_phase())
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__turnaround_phase")
                    .doc(&["Reverse CLK/DATA roles after SECOND so the C64 receives sector bytes."])
                    .instructions(turnaround_phase())
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__on_send_done")
                    .doc(&["Advance the command/file-name sequence after one byte is acknowledged."])
                    .instructions(on_send_done())
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__on_recv_done")
                    .doc(&["Store one sector byte and advance to the next receive/untalk state."])
                    .instructions(on_recv_done())
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__sector_done")
                    .doc(&["Advance to the next physical sector or finish the single job."])
                    .instructions(
                        InstructionBuilder::default()
                            .dec_addr("LOADER_IEC_SECTOR_LEFT")
                            .bne_addr("engine__loader__regular__sector_done__next")
                            .jsr_addr("engine__loader__regular__release_all")
                            .jsr_addr("engine__loader__regular__disable_timer")
                            .lda_imm(LOADER_STATUS_DONE)
                            .sta_addr("LOADER_STATUS")
                            .lda_imm(0)
                            .sta_addr("LOADER_IEC_STATE")
                            .rts()
                            .label("engine__loader__regular__sector_done__next")
                            .jsr_addr("engine__loader__regular__advance_sector")
                            .jsr_addr("engine__loader__regular__build_u1_command")
                            .jsr_addr("engine__loader__regular__prepare_listen_command")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__prepare_listen_buffer")
                    .doc(&["LISTEN 8 under ATN for the buffer OPEN command."])
                    .instructions(prepare_atn_byte(
                        "engine__loader__regular__prepare_listen_buffer",
                        1,
                        LOADER_REGULAR_DEVICE_LISTEN,
                        0,
                    ))
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__prepare_open_buffer")
                    .doc(&["OPEN 2 under ATN, releasing ATN after this byte."])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_imm(2)
                            .sta_addr("LOADER_IEC_STATE")
                            .lda_imm(IEC_ATN | IEC_RELEASE_ATN)
                            .sta_addr("LOADER_IEC_FLAGS")
                            .lda_imm(LOADER_REGULAR_OPEN_BUFFER)
                            .jsr_addr("engine__loader__regular__prepare_send")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__prepare_name_hash")
                    .doc(&["Send the first buffer-name byte without EOI."])
                    .instructions(prepare_data_byte(
                        "engine__loader__regular__prepare_name_hash",
                        3,
                        b'#',
                        false,
                    ))
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__prepare_name_two")
                    .doc(&["Send the final buffer-name byte with EOI."])
                    .instructions(prepare_data_byte(
                        "engine__loader__regular__prepare_name_two",
                        4,
                        b'2',
                        true,
                    ))
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__prepare_unlisten_buffer")
                    .doc(&["UNLISTEN under ATN after the buffer name."])
                    .instructions(prepare_atn_byte(
                        "engine__loader__regular__prepare_unlisten_buffer",
                        5,
                        LOADER_REGULAR_UNLISTEN,
                        IEC_RELEASE_ALL,
                    ))
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__prepare_listen_command")
                    .doc(&["LISTEN 8 under ATN for the U1 command channel."])
                    .instructions(prepare_atn_byte(
                        "engine__loader__regular__prepare_listen_command",
                        6,
                        LOADER_REGULAR_DEVICE_LISTEN,
                        0,
                    ))
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__prepare_open_command")
                    .doc(&["OPEN 15 under ATN for a one-line DOS command."])
                    .instructions(prepare_atn_byte(
                        "engine__loader__regular__prepare_open_command",
                        7,
                        LOADER_REGULAR_OPEN_COMMAND,
                        IEC_RELEASE_ATN,
                    ))
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__prepare_command_byte")
                    .doc(&["Send `U1:2,0,tt,ss` from LOADER_IEC_INDEX, with EOI on the last byte."])
                    .instructions(prepare_command_byte())
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__prepare_unlisten_command")
                    .doc(&["UNLISTEN under ATN after the U1 command string."])
                    .instructions(prepare_atn_byte(
                        "engine__loader__regular__prepare_unlisten_command",
                        9,
                        LOADER_REGULAR_UNLISTEN,
                        IEC_RELEASE_ALL,
                    ))
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__prepare_talk")
                    .doc(&["TALK 8 under ATN before the channel-2 sector read."])
                    .instructions(prepare_atn_byte(
                        "engine__loader__regular__prepare_talk",
                        10,
                        LOADER_REGULAR_DEVICE_TALK,
                        0,
                    ))
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__prepare_talk_second")
                    .doc(&["TALK secondary 2 under ATN with EOI; turnaround releases ATN after it."])
                    .instructions(prepare_atn_eoi_byte(
                        "engine__loader__regular__prepare_talk_second",
                        11,
                        LOADER_REGULAR_SECOND_BUFFER,
                    ))
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__prepare_turnaround")
                    .doc(&["Begin the CLK/DATA role reversal for sector reads."])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_imm(12)
                            .sta_addr("LOADER_IEC_STATE")
                            .lda_imm(0)
                            .sta_addr("LOADER_IEC_PHASE")
                            .lda_imm(0)
                            .sta_addr("LOADER_IEC_INDEX")
                            .lda_imm(255)
                            .sta_addr("LOADER_IEC_TIMEOUT")
                            .lda_imm(40)
                            .sta_addr("LOADER_IEC_TIMEOUT_HI")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__prepare_untalk")
                    .doc(&["UNTALK under ATN after all 256 sector bytes."])
                    .instructions(prepare_atn_byte(
                        "engine__loader__regular__prepare_untalk",
                        14,
                        LOADER_REGULAR_UNTALK,
                        IEC_RELEASE_ALL,
                    ))
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__build_u1_command")
                    .doc(&["Build `U1:2,0,tt,ss` from the current sector cursor."])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_imm(b'U')
                            .sta_addr("loader_iec_u1_command")
                            .lda_imm(b'1')
                            .sta_addr_offs("loader_iec_u1_command", 1)
                            .lda_imm(b':')
                            .sta_addr_offs("loader_iec_u1_command", 2)
                            .lda_imm(b'2')
                            .sta_addr_offs("loader_iec_u1_command", 3)
                            .lda_imm(b',')
                            .sta_addr_offs("loader_iec_u1_command", 4)
                            .lda_imm(b'0')
                            .sta_addr_offs("loader_iec_u1_command", 5)
                            .lda_imm(b',')
                            .sta_addr_offs("loader_iec_u1_command", 6)
                            .lda_addr("LOADER_IEC_TRACK")
                            .jsr_addr("engine__loader__regular__ascii_digits")
                            .lda_addr("LOADER_IEC_TENS")
                            .sta_addr_offs("loader_iec_u1_command", 7)
                            .lda_addr("LOADER_IEC_ONES")
                            .sta_addr_offs("loader_iec_u1_command", 8)
                            .lda_imm(b',')
                            .sta_addr_offs("loader_iec_u1_command", 9)
                            .lda_addr("LOADER_IEC_SECTOR")
                            .jsr_addr("engine__loader__regular__ascii_digits")
                            .lda_addr("LOADER_IEC_TENS")
                            .sta_addr_offs("loader_iec_u1_command", 10)
                            .lda_addr("LOADER_IEC_ONES")
                            .sta_addr_offs("loader_iec_u1_command", 11)
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__ascii_digits")
                    .doc(&["Split A into two ASCII decimal digits in zero page."])
                    .instructions(
                        InstructionBuilder::default()
                            .ldx_imm(0)
                            .label("engine__loader__regular__ascii_digits__next")
                            .cmp_imm(10)
                            .bcc_addr("engine__loader__regular__ascii_digits__done")
                            .sec()
                            .sbc_imm(10)
                            .inx()
                            .jmp_addr("engine__loader__regular__ascii_digits__next")
                            .label("engine__loader__regular__ascii_digits__done")
                            .stx_addr("LOADER_IEC_TENS")
                            .clc()
                            .adc_imm(0x30)
                            .sta_addr("LOADER_IEC_ONES")
                            .lda_addr("LOADER_IEC_TENS")
                            .clc()
                            .adc_imm(0x30)
                            .sta_addr("LOADER_IEC_TENS")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__sector_from_position")
                    .doc(&[
                        "Map a logical interleave slot in A to its physical sector.",
                        "",
                        "The master planner uses `1 + (position * interleave) % modulus`. Adding",
                        "`position` `interleave` times keeps every intermediate value below two",
                        "times the modulus, so the whole calculation stays in one byte.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .sta_addr("LOADER_IEC_POSITION_SCRATCH")
                            .lda_imm(0)
                            .sta_addr("LOADER_IEC_ONES")
                            .lda_imm_low("LOADER_INTERLEAVE")
                            .tax()
                            .label("engine__loader__regular__sector_from_position__add")
                            .clc()
                            .lda_addr("LOADER_IEC_ONES")
                            .adc_addr("LOADER_IEC_POSITION_SCRATCH")
                            .cmp_addr("LOADER_IEC_MODULUS")
                            .bcc_addr("engine__loader__regular__sector_from_position__store")
                            .sbc_addr("LOADER_IEC_MODULUS")
                            .label("engine__loader__regular__sector_from_position__store")
                            .sta_addr("LOADER_IEC_ONES")
                            .dex()
                            .bne_addr("engine__loader__regular__sector_from_position__add")
                            .lda_addr("LOADER_IEC_ONES")
                            .clc()
                            .adc_imm(1)
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__init_position")
                    .doc(&["Recover the logical interleave slot for the current physical sector."])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_imm(0)
                            .sta_addr("LOADER_IEC_POSITION")
                            .label("engine__loader__regular__init_position__next")
                            .lda_addr("LOADER_IEC_POSITION")
                            .cmp_addr("LOADER_IEC_MODULUS")
                            .bcs_addr("engine__loader__regular__init_position__done")
                            .jsr_addr("engine__loader__regular__sector_from_position")
                            .cmp_addr("LOADER_IEC_SECTOR")
                            .beq_addr("engine__loader__regular__init_position__done")
                            .inc_addr("LOADER_IEC_POSITION")
                            .jmp_addr("engine__loader__regular__init_position__next")
                            .label("engine__loader__regular__init_position__done")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__advance_sector")
                    .doc(&[
                        "Move to the next physical sector in the mastered interleave.",
                        "",
                        "The logical slot advances independently of the physical sector so a",
                        "modulo wrap does not change tracks early. A track changes only after the",
                        "last usable slot, matching `MasterPlanner`'s sector cursor.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .inc_addr("LOADER_IEC_POSITION")
                            .lda_addr("LOADER_IEC_POSITION")
                            .cmp_addr("LOADER_IEC_MODULUS")
                            .bcc_addr("engine__loader__regular__advance_sector__map")
                            .lda_imm(0)
                            .sta_addr("LOADER_IEC_POSITION")
                            .jsr_addr("engine__loader__regular__next_track")
                            .lda_addr("LOADER_IEC_TRACK")
                            .jsr_addr("engine__loader__regular__track_modulus")
                            .label("engine__loader__regular__advance_sector__map")
                            .lda_addr("LOADER_IEC_POSITION")
                            .jsr_addr("engine__loader__regular__sector_from_position")
                            .sta_addr("LOADER_IEC_SECTOR")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__next_track")
                    .doc(&["Advance to the next mastered track, skipping the DOS directory track."])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_addr("LOADER_IEC_TRACK")
                            .cmp_imm(17)
                            .bne_addr("engine__loader__regular__next_track__increment")
                            .lda_imm(19)
                            .sta_addr("LOADER_IEC_TRACK")
                            .rts()
                            .label("engine__loader__regular__next_track__increment")
                            .inc_addr("LOADER_IEC_TRACK")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__track_modulus")
                    .doc(&[
                        "Set the usable-sector modulus for the current track.",
                        "",
                        "A carries the caller's computed sector offset. Restore it before returning:",
                        "`advance_sector` needs that value to build the next interleaved sector.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .pha()
                            .lda_addr("LOADER_IEC_TRACK")
                            .cmp_imm(18)
                            .bcc_addr("engine__loader__regular__track_modulus__20")
                            .cmp_imm(25)
                            .bcc_addr("engine__loader__regular__track_modulus__18")
                            .cmp_imm(31)
                            .bcc_addr("engine__loader__regular__track_modulus__17")
                            .lda_imm(16)
                            .sta_addr("LOADER_IEC_MODULUS")
                            .jmp_addr("engine__loader__regular__track_modulus__done")
                            .label("engine__loader__regular__track_modulus__20")
                            .lda_imm(20)
                            .sta_addr("LOADER_IEC_MODULUS")
                            .jmp_addr("engine__loader__regular__track_modulus__done")
                            .label("engine__loader__regular__track_modulus__18")
                            .lda_imm(18)
                            .sta_addr("LOADER_IEC_MODULUS")
                            .jmp_addr("engine__loader__regular__track_modulus__done")
                            .label("engine__loader__regular__track_modulus__17")
                            .lda_imm(17)
                            .sta_addr("LOADER_IEC_MODULUS")
                            .label("engine__loader__regular__track_modulus__done")
                            .pla()
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__store_sector_byte")
                    .doc(&["Store one received byte while payload bytes remain."])
                    .instructions(store_sector_byte())
                    .build(),
            )
            .build()
    }
}

fn prepare_atn_byte(label: &str, state: u8, byte: u8, release: u8) -> Instructions {
    let flags = IEC_ATN | release;
    InstructionBuilder::default()
        .label(label)
        .lda_imm(state)
        .sta_addr("LOADER_IEC_STATE")
        .lda_imm(flags)
        .sta_addr("LOADER_IEC_FLAGS")
        .lda_imm(byte)
        .jsr_addr("engine__loader__regular__prepare_send")
        .rts()
        .build()
}

fn prepare_atn_eoi_byte(label: &str, state: u8, byte: u8) -> Instructions {
    let flags = IEC_ATN | IEC_EOI;
    InstructionBuilder::default()
        .label(label)
        .lda_imm(state)
        .sta_addr("LOADER_IEC_STATE")
        .lda_imm(flags)
        .sta_addr("LOADER_IEC_FLAGS")
        .lda_imm(byte)
        .jsr_addr("engine__loader__regular__prepare_send")
        .rts()
        .build()
}

fn prepare_data_byte(label: &str, state: u8, byte: u8, eoi: bool) -> Instructions {
    InstructionBuilder::default()
        .label(label)
        .lda_imm(state)
        .sta_addr("LOADER_IEC_STATE")
        .lda_imm(if eoi { IEC_EOI } else { 0 })
        .sta_addr("LOADER_IEC_FLAGS")
        .lda_imm(byte)
        .jsr_addr("engine__loader__regular__prepare_send")
        .rts()
        .build()
}

fn prepare_command_byte() -> Instructions {
    let label = "engine__loader__regular__prepare_command_byte";
    InstructionBuilder::default()
        .label(label)
        .lda_imm(8)
        .sta_addr("LOADER_IEC_STATE")
        .lda_addr("LOADER_IEC_INDEX")
        .cmp_imm(11)
        .bne_addr(&format!("{label}__not_last"))
        .lda_imm(IEC_EOI)
        .sta_addr("LOADER_IEC_FLAGS")
        .jmp_addr(&format!("{label}__load"))
        .label(&format!("{label}__not_last"))
        .lda_imm(0)
        .sta_addr("LOADER_IEC_FLAGS")
        .label(&format!("{label}__load"))
        .ldx_addr("LOADER_IEC_INDEX")
        .lda_addr_x("loader_iec_u1_command")
        .jsr_addr("engine__loader__regular__prepare_send")
        .rts()
        .build()
}

fn send_phase() -> Instructions {
    let label = "engine__loader__regular__send_phase";
    let phase = |number: u8| format!("{label}__phase{number}");
    let wait = |number: u8| format!("{label}__phase{number}_wait");
    let dispatch = |number: u8| format!("{label}__dispatch{number}");
    let mut b = InstructionBuilder::default();
    b.label(label)
        .lda_addr("LOADER_IEC_PHASE")
        .bne_addr(&format!("{label}__dispatch"))
        .lda_addr("LOADER_IEC_SETTLE")
        .bne_addr(&format!("{label}__phase0_settle"))
        .comment("Assert ATN and CLK together, then let the listener pull DATA low.")
        .jsr_addr("engine__loader__regular__atn_mask")
        .ora_imm(OUT_CLK)
        .jsr_addr("engine__loader__regular__line")
        .lda_imm(IEC_SEND_SETTLE_PHASES)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .lda_imm(1)
        .sta_addr("LOADER_IEC_PHASE")
        .rts();
    b.label(&format!("{label}__phase0_settle"))
        .comment("Let the drive finish UNLISTEN before ATN falls for the next command.")
        .dec_addr("LOADER_IEC_SETTLE")
        .rts();
    b.label(&format!("{label}__dispatch"))
        .cmp_imm(1)
        .bne_addr(&dispatch(2))
        .jmp_addr(&phase(1));
    b.label(&dispatch(2))
        .cmp_imm(2)
        .bne_addr(&dispatch(3))
        .jmp_addr(&phase(2));
    b.label(&dispatch(3))
        .cmp_imm(3)
        .bne_addr(&dispatch(4))
        .jmp_addr(&phase(3));
    b.label(&dispatch(4))
        .cmp_imm(4)
        .bne_addr(&dispatch(5))
        .jmp_addr(&phase(4));
    b.label(&dispatch(5))
        .cmp_imm(5)
        .bne_addr(&dispatch(6))
        .jmp_addr(&phase(5));
    b.label(&dispatch(6))
        .cmp_imm(6)
        .bne_addr(&dispatch(7))
        .jmp_addr(&phase(6));
    b.label(&dispatch(7))
        .cmp_imm(7)
        .bne_addr(&dispatch(8))
        .jmp_addr(&phase(7));
    b.label(&dispatch(8))
        .cmp_imm(8)
        .bne_addr(&dispatch(9))
        .jmp_addr(&phase(8));
    b.label(&dispatch(9))
        .cmp_imm(9)
        .bne_addr(&dispatch(10))
        .jmp_addr(&phase(9));
    b.label(&dispatch(10)).jmp_addr(&phase(10));
    b.label(&phase(1))
        .dec_addr("LOADER_IEC_TIMEOUT")
        .bne_addr(&wait(1))
        .jsr_addr("engine__loader__regular__atn_mask")
        .ora_imm(OUT_CLK)
        .jsr_addr("engine__loader__regular__line")
        .lda_imm(2)
        .sta_addr("LOADER_IEC_PHASE")
        .lda_imm(255)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .rts()
        .label(&wait(1))
        .rts();
    b.label(&phase(2))
        .comment("The listener pulls DATA low only after CLK has been held low.")
        .lda_addr("CIA2_DATA_PORT")
        .and_imm(IN_DATA)
        .beq_addr(&format!("{label}__phase2_acked"))
        .dec_addr("LOADER_IEC_TIMEOUT")
        .bne_addr(&wait(2))
        .jsr_addr("engine__loader__regular__fail")
        .rts()
        .label(&wait(2))
        .rts()
        .label(&format!("{label}__phase2_acked"))
        .jsr_addr("engine__loader__regular__atn_mask")
        .jsr_addr("engine__loader__regular__line")
        .lda_addr("LOADER_IEC_FLAGS")
        .and_imm(IEC_EOI)
        .beq_addr(&format!("{label}__phase2_data"))
        .comment("EOI ack is a short DATA-low pulse; complete the whole handshake in this NMI.")
        .lda_imm(255)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .label(&format!("{label}__eoi_high1_poll"))
        .lda_addr("CIA2_DATA_PORT")
        .and_imm(IN_DATA)
        .bne_addr(&format!("{label}__eoi_low_poll"))
        .dec_addr("LOADER_IEC_TIMEOUT")
        .bne_addr(&format!("{label}__eoi_high1_poll"))
        .jsr_addr("engine__loader__regular__fail")
        .rts()
        .label(&format!("{label}__eoi_low_poll"))
        .lda_imm(255)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .label(&format!("{label}__eoi_low_wait"))
        .lda_addr("CIA2_DATA_PORT")
        .and_imm(IN_DATA)
        .beq_addr(&format!("{label}__eoi_high2_poll"))
        .dec_addr("LOADER_IEC_TIMEOUT")
        .bne_addr(&format!("{label}__eoi_low_wait"))
        .jsr_addr("engine__loader__regular__fail")
        .rts()
        .label(&format!("{label}__eoi_high2_poll"))
        .lda_imm(255)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .label(&format!("{label}__eoi_high2_wait"))
        .lda_addr("CIA2_DATA_PORT")
        .and_imm(IN_DATA)
        .bne_addr(&format!("{label}__eoi_done"))
        .dec_addr("LOADER_IEC_TIMEOUT")
        .bne_addr(&format!("{label}__eoi_high2_wait"))
        .jsr_addr("engine__loader__regular__fail")
        .rts()
        .label(&format!("{label}__eoi_done"))
        .jsr_addr("engine__loader__regular__atn_mask")
        .ora_imm(OUT_CLK)
        .jsr_addr("engine__loader__regular__line")
        .lda_imm(6)
        .sta_addr("LOADER_IEC_PHASE")
        .lda_imm(255)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .rts()
        .label(&format!("{label}__phase2_data"))
        .comment("KERNAL ED5A/ED5F releases CLK, waits for DATA-high, then reasserts CLK.")
        .jsr_addr("engine__loader__regular__atn_mask")
        .jsr_addr("engine__loader__regular__line")
        .lda_imm(5)
        .sta_addr("LOADER_IEC_PHASE")
        .lda_imm(255)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .lda_imm(0)
        .sta_addr("LOADER_IEC_TIMEOUT_HI")
        .rts()
        .label(&phase(5))
        .lda_addr("CIA2_DATA_PORT")
        .and_imm(IN_DATA)
        .beq_addr(&format!("{label}__phase5_wait"))
        .jsr_addr("engine__loader__regular__atn_mask")
        .ora_imm(OUT_CLK)
        .jsr_addr("engine__loader__regular__line")
        .lda_imm(6)
        .sta_addr("LOADER_IEC_PHASE")
        .lda_imm(255)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .lda_imm(0)
        .sta_addr("LOADER_IEC_TIMEOUT_HI")
        .rts()
        .label(&format!("{label}__phase5_wait"))
        .jsr_addr("engine__loader__regular__timeout_tick")
        .beq_addr(&format!("{label}__phase5_wait_done"))
        .jsr_addr("engine__loader__regular__fail")
        .rts()
        .label(&format!("{label}__phase5_wait_done"))
        .rts()
        .label(&phase(3))
        .lda_addr("CIA2_DATA_PORT")
        .and_imm(IN_DATA)
        .beq_addr(&wait(3))
        .lda_imm(4)
        .sta_addr("LOADER_IEC_PHASE")
        .comment("The drive's EOI DATA-low pulse is short; allow a full bounded window.")
        .lda_imm(255)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .sta_addr("LOADER_IEC_TIMEOUT_HI")
        .rts()
        .label(&wait(3))
        .jsr_addr("engine__loader__regular__timeout_tick")
        .beq_addr(&format!("{}__done", wait(3)))
        .jsr_addr("engine__loader__regular__fail")
        .rts()
        .label(&format!("{}__done", wait(3)))
        .rts()
        .label(&phase(4))
        .lda_addr("CIA2_DATA_PORT")
        .and_imm(IN_DATA)
        .bne_addr(&wait(4))
        .lda_imm(5)
        .sta_addr("LOADER_IEC_PHASE")
        .lda_imm(255)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .sta_addr("LOADER_IEC_TIMEOUT_HI")
        .rts()
        .label(&wait(4))
        .jsr_addr("engine__loader__regular__timeout_tick")
        .beq_addr(&format!("{}__done", wait(4)))
        .jsr_addr("engine__loader__regular__fail")
        .rts()
        .label(&format!("{}__done", wait(4)))
        .rts()
        .label(&phase(6))
        .comment("KERNAL DATAHI/DATALO are inverted: a one bit releases DATA, a zero asserts it.")
        .comment("Wait for the listener to release DATA before setting the next bit, as ISR01 does.")
        .label(&format!("{label}__phase6_wait"))
        .lda_addr("CIA2_DATA_PORT")
        .and_imm(IN_DATA)
        .bne_addr(&format!("{label}__phase6_ready"))
        .dec_addr("LOADER_IEC_TIMEOUT")
        .bne_addr(&format!("{label}__phase6_wait"))
        .jsr_addr("engine__loader__regular__fail")
        .rts()
        .label(&format!("{label}__phase6_ready"))
        .comment("Set DATA while preserving CLK; phase 7 releases CLK to validate the bit.")
        .jsr_addr("engine__loader__regular__atn_mask")
        .sta_addr("LOADER_IEC_TENS")
        .lda_addr("LOADER_IEC_BYTE")
        .ror_addr("LOADER_IEC_BYTE")
        .lda_addr("LOADER_IEC_TENS")
        .bcs_addr(&format!("{label}__phase6_one"))
        .ora_imm(OUT_DATA)
        .label(&format!("{label}__phase6_one"))
        .sta_addr("LOADER_IEC_ONES")
        .jsr_addr("engine__loader__regular__line_preserve_clk")
        .lda_imm(7)
        .sta_addr("LOADER_IEC_PHASE")
        .rts()
        .label(&phase(7))
        .comment("Hold DATA for the whole CLK-high interval, as KERNAL ISR01 does.")
        .lda_addr("LOADER_IEC_ONES")
        .jsr_addr("engine__loader__regular__line")
        .lda_imm(8)
        .sta_addr("LOADER_IEC_PHASE")
        .rts()
        .label(&phase(8))
        .comment("Release DATA and assert CLK as KERNAL does after the final bit.")
        .jsr_addr("engine__loader__regular__atn_mask")
        .ora_imm(OUT_CLK)
        .jsr_addr("engine__loader__regular__line")
        .dec_addr("LOADER_IEC_BIT")
        .bne_addr(&format!("{label}__phase8_more"))
        .comment("The end-of-byte DATA ack can be shorter than one timer phase, so poll it here.")
        .lda_imm(255)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .label(&format!("{label}__phase9_poll"))
        .lda_addr("CIA2_DATA_PORT")
        .and_imm(IN_DATA)
        .beq_addr(&format!("{label}__phase9_acked"))
        .dec_addr("LOADER_IEC_TIMEOUT")
        .bne_addr(&format!("{label}__phase9_poll"))
        .jsr_addr("engine__loader__regular__fail")
        .rts()
        .label(&format!("{label}__phase9_acked"))
        .comment("KERNAL leaves CLK asserted after a byte ACK; the EDCC release/wait is only for bus end.")
        .jsr_addr("engine__loader__regular__atn_mask")
        .ora_imm(OUT_CLK)
        .jsr_addr("engine__loader__regular__line")
        .jsr_addr("engine__loader__regular__on_send_done")
        .rts()
        .label(&format!("{label}__phase8_more"))
        .lda_imm(6)
        .sta_addr("LOADER_IEC_PHASE")
        .lda_imm(255)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .rts()
        .label(&phase(9))
        .comment("Retained for state-machine completeness; phase 8 completes the byte.")
        .rts()
        .label(&phase(10))
        .comment("Wait across phases for the drive to float DATA before the next ATN edge.")
        .lda_addr("CIA2_DATA_PORT")
        .and_imm(IN_DATA)
        .bne_addr(&format!("{label}__phase10_ready"))
        .jsr_addr("engine__loader__regular__timeout_tick")
        .beq_addr(&format!("{label}__phase10_wait_done"))
        .jsr_addr("engine__loader__regular__fail")
        .rts()
        .label(&format!("{label}__phase10_wait_done"))
        .rts()
        .label(&format!("{label}__phase10_ready"))
        .lda_addr("LOADER_IEC_FLAGS")
        .and_imm(!(IEC_RELEASE_ATN | IEC_RELEASE_ALL))
        .sta_addr("LOADER_IEC_FLAGS")
        .lda_imm(IEC_ATN_RELEASE_SETTLE_PHASES)
        .sta_addr("LOADER_IEC_SETTLE")
        .jsr_addr("engine__loader__regular__on_send_done")
        .rts();
    b.build()
}

fn recv_phase() -> Instructions {
    let label = "engine__loader__regular__recv_phase";
    let complete = format!("{label}__complete");
    let fail = format!("{label}__fail");
    let unexpected = format!("{label}__unexpected_phase");
    let start = format!("{label}__start");
    let high_wait = format!("{label}__high_wait");
    let high_tick = format!("{label}__high_tick");
    let high_ready = format!("{label}__high_ready");
    let first_low_wait = format!("{label}__first_low_wait");
    let first_low_tick = format!("{label}__first_low_tick");
    let first_low = format!("{label}__first_low");
    let eoi_start = format!("{label}__eoi_start");
    let eoi_ack_delay = format!("{label}__eoi_ack_delay");
    let eoi_low_wait = format!("{label}__eoi_low_wait");
    let eoi_low_tick = format!("{label}__eoi_low_tick");
    let bit_loop = format!("{label}__bit_loop");
    let bit_high_wait = format!("{label}__bit_high_wait");
    let bit_high_tick = format!("{label}__bit_high_tick");
    let bit_low_wait = format!("{label}__bit_low_wait");
    let bit_low_tick = format!("{label}__bit_low_tick");
    let byte_ack = format!("{label}__byte_ack");
    let eoi_end_delay = format!("{label}__eoi_end_delay");
    let normal_end = format!("{label}__normal_end");

    let mut b = InstructionBuilder::default();
    b.label(label)
        .lda_addr("LOADER_IEC_PHASE")
        .beq_addr(&start)
        .jmp_addr(&unexpected)
        .label(&start)
        .comment("One NMI receives one complete byte; X is the only loop counter so polling stays tight.")
        .comment("Release CLK while keeping the previous byte's DATA ACK asserted.")
        .jsr_addr("engine__loader__regular__release_clk")
        .ldx_imm(255);
    b.label(&high_wait)
        .comment("ASL leaves CLK in N and DATA in C, exactly like KERNAL ACPTR at $EE5A.")
        .lda_addr("CIA2_DATA_PORT")
        .asl_acc()
        .bpl_addr(&high_tick)
        .jmp_addr(&high_ready);
    b.label(&high_tick).dex().bne_addr(&high_wait).jmp_addr(&fail);
    b.label(&high_ready)
        .comment("Release DATA so the drive may start clocking the next byte.")
        .lda_imm(0)
        .jsr_addr("engine__loader__regular__line")
        .ldx_imm(255);
    b.label(&first_low_wait)
        .lda_addr("CIA2_DATA_PORT")
        .asl_acc()
        .bmi_addr(&first_low_tick)
        .jmp_addr(&first_low);
    b.label(&first_low_tick)
        .dex()
        .bne_addr(&first_low_wait)
        .jmp_addr(&eoi_start);
    b.label(&first_low).jmp_addr(&bit_loop);
    b.label(&eoi_start)
        .comment("A long CLK-high wait is EOI: pulse DATA, then wait for the final byte's bits.")
        .lda_addr("LOADER_IEC_FLAGS")
        .ora_imm(IEC_EOI_SEEN)
        .sta_addr("LOADER_IEC_FLAGS")
        .lda_imm(OUT_DATA)
        .jsr_addr("engine__loader__regular__line")
        .ldx_imm(64);
    b.label(&eoi_ack_delay)
        .dex()
        .bne_addr(&eoi_ack_delay)
        .lda_imm(0)
        .jsr_addr("engine__loader__regular__line")
        .ldx_imm(255);
    b.label(&eoi_low_wait)
        .lda_addr("CIA2_DATA_PORT")
        .asl_acc()
        .bmi_addr(&eoi_low_tick)
        .jmp_addr(&bit_loop);
    b.label(&eoi_low_tick).dex().bne_addr(&eoi_low_wait).jmp_addr(&fail);

    b.label(&bit_loop).lda_imm(8).sta_addr("LOADER_IEC_BIT").ldx_imm(64);
    b.label(&bit_high_wait)
        .comment("Latch DATA from the same read that observed CLK high, as KERNAL ACPTR does.")
        .lda_addr("CIA2_DATA_PORT")
        .asl_acc()
        .bpl_addr(&bit_high_tick)
        .ror_addr("LOADER_IEC_BYTE")
        .ldx_imm(64)
        .jmp_addr(&bit_low_wait);
    b.label(&bit_high_tick).dex().bne_addr(&bit_high_wait).jmp_addr(&fail);
    b.label(&bit_low_wait)
        .lda_addr("CIA2_DATA_PORT")
        .asl_acc()
        .bmi_addr(&bit_low_tick)
        .dec_addr("LOADER_IEC_BIT")
        .bne_addr(&bit_high_wait)
        .jmp_addr(&byte_ack);
    b.label(&bit_low_tick).dex().bne_addr(&bit_low_wait).jmp_addr(&fail);
    b.label(&byte_ack)
        .comment("After eight bits the receiver holds DATA low as the byte ACK.")
        .lda_imm(OUT_DATA)
        .jsr_addr("engine__loader__regular__line")
        .lda_addr("LOADER_IEC_FLAGS")
        .and_imm(IEC_EOI_SEEN)
        .beq_addr(&normal_end)
        .ldx_imm(64);
    b.label(&eoi_end_delay)
        .dex()
        .bne_addr(&eoi_end_delay)
        .lda_imm(0)
        .jsr_addr("engine__loader__regular__line")
        .jmp_addr(&complete);
    b.label(&normal_end)
        .comment("Keep DATA asserted until the next NMI releases CLK and waits for CLK-high.")
        .jmp_addr(&complete);
    b.label(&unexpected).rts();
    b.label(&fail).jsr_addr("engine__loader__regular__fail").rts();
    b.label(&complete)
        .jsr_addr("engine__loader__regular__on_recv_done")
        .rts();
    b.build()
}

fn turnaround_phase() -> Instructions {
    let label = "engine__loader__regular__turnaround_phase";
    let wait = format!("{label}__wait");
    InstructionBuilder::default()
        .label(label)
        .lda_addr("LOADER_IEC_PHASE")
        .bne_addr(&wait)
        .comment("KERNAL TKATN: assert DATA, release ATN, release CLK, then wait for CLK.")
        .jsr_addr("engine__loader__regular__atn_mask")
        .ora_imm(OUT_CLK)
        .ora_imm(OUT_DATA)
        .jsr_addr("engine__loader__regular__line")
        .comment("KERNAL TKATN releases ATN and CLK back-to-back; the drive acknowledges quickly.")
        .jsr_addr("engine__loader__regular__release_atn")
        .jsr_addr("engine__loader__regular__release_clk")
        .lda_imm(1)
        .sta_addr("LOADER_IEC_PHASE")
        .lda_imm(255)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .comment("The drive's turnaround CLK pulse lasts ~68 cycles; poll inside this NMI.")
        .label(&wait)
        .lda_addr("CIA2_DATA_PORT")
        .and_imm(IN_CLK)
        .beq_addr(&format!("{label}__ready"))
        .dec_addr("LOADER_IEC_TIMEOUT")
        .bne_addr(&wait)
        .jsr_addr("engine__loader__regular__fail")
        .rts()
        .label(&format!("{label}__ready"))
        .jsr_addr("engine__loader__regular__prepare_recv")
        .rts()
        .build()
}

fn on_send_done() -> Instructions {
    let label = "engine__loader__regular__on_send_done";
    let mut b = InstructionBuilder::default();
    b.label(label)
        .lda_addr("LOADER_IEC_FLAGS")
        .and_imm(IEC_RELEASE_ATN | IEC_RELEASE_ALL)
        .beq_addr(&format!("{label}__no_release"))
        .lda_addr("LOADER_IEC_FLAGS")
        .and_imm(IEC_RELEASE_ALL)
        .bne_addr(&format!("{label}__release_all"))
        .jsr_addr("engine__loader__regular__release_atn")
        .jmp_addr(&format!("{label}__no_release"))
        .label(&format!("{label}__release_all"))
        .jsr_addr("engine__loader__regular__release_all")
        .comment("After UNLISTEN/UNTALK the drive must float DATA high before ATN is reasserted.")
        .lda_imm(10)
        .sta_addr("LOADER_IEC_PHASE")
        .lda_imm(255)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .lda_imm(0)
        .sta_addr("LOADER_IEC_TIMEOUT_HI")
        .rts()
        .label(&format!("{label}__no_release"))
        .lda_addr("LOADER_IEC_STATE")
        .cmp_imm(1)
        .bne_addr(&format!("{label}__state2"))
        .jsr_addr("engine__loader__regular__prepare_open_buffer")
        .rts()
        .label(&format!("{label}__state2"))
        .cmp_imm(2)
        .bne_addr(&format!("{label}__state3"))
        .jsr_addr("engine__loader__regular__prepare_name_hash")
        .rts()
        .label(&format!("{label}__state3"))
        .cmp_imm(3)
        .bne_addr(&format!("{label}__state4"))
        .jsr_addr("engine__loader__regular__prepare_name_two")
        .rts()
        .label(&format!("{label}__state4"))
        .cmp_imm(4)
        .bne_addr(&format!("{label}__state5"))
        .jsr_addr("engine__loader__regular__prepare_unlisten_buffer")
        .rts()
        .label(&format!("{label}__state5"))
        .cmp_imm(5)
        .bne_addr(&format!("{label}__state6"))
        .lda_imm(1)
        .sta_addr("LOADER_IEC_BUFFER_OPEN")
        .jsr_addr("engine__loader__regular__prepare_listen_command")
        .rts()
        .label(&format!("{label}__state6"))
        .cmp_imm(6)
        .bne_addr(&format!("{label}__state7"))
        .jsr_addr("engine__loader__regular__prepare_open_command")
        .rts()
        .label(&format!("{label}__state7"))
        .cmp_imm(7)
        .bne_addr(&format!("{label}__state8"))
        .lda_imm(0)
        .sta_addr("LOADER_IEC_INDEX")
        .jsr_addr("engine__loader__regular__prepare_command_byte")
        .rts()
        .label(&format!("{label}__state8"))
        .cmp_imm(8)
        .bne_addr(&format!("{label}__state9"))
        .inc_addr("LOADER_IEC_INDEX")
        .lda_addr("LOADER_IEC_INDEX")
        .cmp_imm(12)
        .bne_addr(&format!("{label}__command_more"))
        .jsr_addr("engine__loader__regular__prepare_unlisten_command")
        .rts()
        .label(&format!("{label}__command_more"))
        .jsr_addr("engine__loader__regular__prepare_command_byte")
        .rts()
        .label(&format!("{label}__state9"))
        .cmp_imm(9)
        .bne_addr(&format!("{label}__state10"))
        .jsr_addr("engine__loader__regular__prepare_talk")
        .rts()
        .label(&format!("{label}__state10"))
        .cmp_imm(10)
        .bne_addr(&format!("{label}__state11"))
        .jsr_addr("engine__loader__regular__prepare_talk_second")
        .rts()
        .label(&format!("{label}__state11"))
        .cmp_imm(11)
        .bne_addr(&format!("{label}__state14"))
        .jsr_addr("engine__loader__regular__prepare_turnaround")
        .rts()
        .label(&format!("{label}__state14"))
        .cmp_imm(14)
        .bne_addr(&format!("{label}__done"))
        .jsr_addr("engine__loader__regular__sector_done")
        .label(&format!("{label}__done"))
        .rts();
    b.build()
}

fn on_recv_done() -> Instructions {
    let label = "engine__loader__regular__on_recv_done";
    InstructionBuilder::default()
        .label(label)
        .lda_addr("LOADER_IEC_INDEX")
        .bne_addr(&format!("{label}__stored"))
        .jsr_addr("engine__condition__mark_disk_turned")
        .label(&format!("{label}__stored"))
        .jsr_addr("engine__loader__regular__store_sector_byte")
        .inc_addr("LOADER_IEC_INDEX")
        .bne_addr(&format!("{label}__more"))
        .jsr_addr("engine__loader__regular__prepare_untalk")
        .rts()
        .label(&format!("{label}__more"))
        .jsr_addr("engine__loader__regular__prepare_recv")
        .rts()
        .build()
}

fn store_sector_byte() -> Instructions {
    let label = "engine__loader__regular__store_sector_byte";
    InstructionBuilder::default()
        .label(label)
        .lda_addr("LOADER_REMAINING")
        .bne_addr(&format!("{label}__store"))
        .lda_addr_offs("LOADER_REMAINING", 1)
        .beq_addr(&format!("{label}__done"))
        .label(&format!("{label}__store"))
        .lda_addr("LOADER_IEC_BYTE")
        .ldy_imm(0)
        .sta_ind_y("LOADER_DST_PTR")
        .inc_addr("LOADER_DST_PTR")
        .bne_addr(&format!("{label}__count"))
        .inc_addr_offs("LOADER_DST_PTR", 1)
        .label(&format!("{label}__count"))
        .lda_addr("LOADER_REMAINING")
        .sec()
        .sbc_imm(1)
        .sta_addr("LOADER_REMAINING")
        .lda_addr_offs("LOADER_REMAINING", 1)
        .sbc_imm(0)
        .sta_addr_offs("LOADER_REMAINING", 1)
        .label(&format!("{label}__done"))
        .rts()
        .build()
}
