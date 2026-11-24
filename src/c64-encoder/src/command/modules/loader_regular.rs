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

/// CIA2 port output bits. The serial drivers invert the line: writing a one
/// asserts (pulls low) the bus line, while writing a zero releases it.
const OUT_ATN: u8 = 0b0000_1000;
const OUT_CLK: u8 = 0b0001_0000;
const OUT_DATA: u8 = 0b0010_0000;

/// CIA2 port input bits read as one while the line is released/high and zero
/// while the line is asserted/low.
const IN_CLK: u8 = 0b0100_0000;
const IN_DATA: u8 = 0b1000_0000;
const PORT_PRESERVE: u8 = 0b1100_0011;

/// Timer A period. At ~1 MHz this is 100 cycles / 25 us, short enough to keep
/// IEC bit timing valid while still yielding the CPU between phases.
const IEC_TIMER_LATCH_LO: u8 = 100;
const IEC_TIMER_LATCH_HI: u8 = 0;

pub trait LoaderRegularBuilder {
    fn add_loader_regular_addresses(&mut self) -> &mut Self;
}

impl LoaderRegularBuilder for ApplicationBuilder {
    fn add_loader_regular_addresses(&mut self) -> &mut Self {
        self.define_address("LOADER_IEC_STATE", 0x91)
            .define_address("LOADER_IEC_PHASE", 0x92)
            .define_address("LOADER_IEC_BIT", 0x93)
            .define_address("LOADER_IEC_BYTE", 0x94)
            .define_address("LOADER_IEC_FLAGS", 0x95)
            .define_address("LOADER_IEC_PORT", 0x96)
            .define_address("LOADER_IEC_TRACK", 0x97)
            .define_address("LOADER_IEC_SECTOR", 0x98)
            .define_address("LOADER_IEC_SECTOR_LEFT", 0x99)
            .define_address("LOADER_IEC_INDEX", 0x9A)
            .define_address("LOADER_IEC_TENS", 0x9B)
            .define_address("LOADER_IEC_ONES", 0x9C)
            .define_address("LOADER_IEC_MODULUS", 0x9D)
            .define_address("LOADER_IEC_TIMEOUT", 0x9F)
            .define_address("LOADER_IEC_TIMEOUT_HI", 0x9E)
            .define_address("LOADER_INTERLEAVE", 7)
            .define_address("CIA2_DATA_PORT", 0xDD00)
            .define_address("CIA2_DATA_DDR", 0xDD03)
            .define_address("CIA2_TIMER_A_LO", 0xDD04)
            .define_address("CIA2_TIMER_A_HI", 0xDD05)
            .define_address("CIA2_TIMER_A_CONTROL", 0xDD0E)
    }
}

/// Default regular loader backend.
///
/// The C64 drives the standard IEC bus directly from a CIA2 Timer A IRQ: no
/// KERNAL serial routine is called from the handler and no 1541 drive code is
/// uploaded. Channel 2 is allocated once with `#2`; each sector is fetched
/// with `U1:2,0,track,sector`, and exactly 256 bytes are read back. Only the
/// payload bytes still owed by the resource are copied to its destination.
///
/// One IEC phase runs per timer IRQ (~25 us), so the main thread keeps
/// decoding frames and the music hook keeps running while the transfer is in
/// flight. ATN, EOI, turnaround, and byte handshakes follow the polarity and
/// LSB-first order of the KERNAL serial routines. Real-drive timing and the
/// final interleave remain a hardware/VICE tuning pass.
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
                            .jsr_addr("engine__loader__regular__release_all")
                            .jsr_addr("engine__loader__regular__disable_timer")
                            .lda_imm(0)
                            .sta_addr("LOADER_IEC_STATE")
                            .sta_addr("LOADER_IEC_PHASE")
                            .sta_addr("LOADER_IEC_FLAGS")
                            .sta_addr("LOADER_IEC_TIMEOUT")
                            .sta_addr("LOADER_IEC_TIMEOUT_HI")
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
                            .jsr_addr("engine__loader__regular__build_u1_command")
                            .jsr_addr("engine__loader__regular__release_all")
                            .lda_imm(0)
                            .sta_addr("LOADER_IEC_INDEX")
                            .sta_addr("LOADER_IEC_FLAGS")
                            .jsr_addr("engine__loader__regular__enable_timer")
                            .jsr_addr("engine__loader__regular__prepare_listen_buffer")
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
                            .cmp_imm(10)
                            .bne_addr("engine__loader__backend__regular__not_turnaround")
                            .jsr_addr("engine__loader__regular__turnaround_phase")
                            .rts()
                            .label("engine__loader__backend__regular__not_turnaround")
                            .cmp_imm(11)
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
                    .doc(&["Write the cached CIA2 port image."])
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
                    .doc(&["Release ATN, CLK and DATA while preserving the VIC bank bits."])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_addr("CIA2_DATA_PORT")
                            .and_imm(PORT_PRESERVE)
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
                    .doc(&["Write A's asserted-line mask from bits 3-5, preserving the VIC bank."])
                    .instructions(
                        InstructionBuilder::default()
                            .sta_addr("LOADER_IEC_TENS")
                            .lda_addr("LOADER_IEC_PORT")
                            .and_imm(PORT_PRESERVE)
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
                    .doc(&["Stop CIA2 Timer A and mask its IRQ without disturbing the port."])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_imm(0)
                            .sta_addr("CIA2_TIMER_A_CONTROL")
                            .lda_imm(0b1000_0001)
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
                    .doc(&["Start CIA2 Timer A as a ~25 us periodic source for IEC phases."])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_imm(0)
                            .sta_addr("CIA2_TIMER_A_CONTROL")
                            .lda_imm(IEC_TIMER_LATCH_LO)
                            .sta_addr("CIA2_TIMER_A_LO")
                            .lda_imm(IEC_TIMER_LATCH_HI)
                            .sta_addr("CIA2_TIMER_A_HI")
                            .lda_addr("CIA2_IRQ_CONTROL")
                            .lda_imm(0b0000_0001)
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
                    .doc(&["Decrement the two-byte phase timeout; return Z=0 while it is still valid."])
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
                            .lda_imm(1)
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
                            .lda_imm(11)
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
                        "Advance one IEC receive phase with bounded handshakes.",
                        "",
                        "A long CLK release is treated as EOI, acknowledged with a DATA pulse",
                        "before the final byte.",
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
                        false,
                    ))
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__prepare_open_buffer")
                    .doc(&["OPEN 2 under ATN, releasing ATN after this byte."])
                    .instructions(prepare_atn_byte(
                        "engine__loader__regular__prepare_open_buffer",
                        2,
                        LOADER_REGULAR_OPEN_BUFFER,
                        true,
                    ))
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
                        true,
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
                        false,
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
                        true,
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
                        true,
                    ))
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__loader__regular__prepare_turnaround")
                    .doc(&["Begin the CLK/DATA role reversal for sector reads."])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_imm(10)
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
                        12,
                        LOADER_REGULAR_UNTALK,
                        true,
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
                    .name("engine__loader__regular__advance_sector")
                    .doc(&["Move to the next physical sector in the mastered interleave."])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_addr("LOADER_IEC_SECTOR")
                            .sec()
                            .sbc_imm(1)
                            .clc()
                            .adc_imm_low("LOADER_INTERLEAVE")
                            .jsr_addr("engine__loader__regular__track_modulus")
                            .label("engine__loader__regular__advance_sector__compare")
                            .cmp_addr("LOADER_IEC_MODULUS")
                            .bcc_addr("engine__loader__regular__advance_sector__store")
                            .sbc_addr("LOADER_IEC_MODULUS")
                            .sta_addr("LOADER_IEC_TENS")
                            .jsr_addr("engine__loader__regular__next_track")
                            .jsr_addr("engine__loader__regular__track_modulus")
                            .lda_addr("LOADER_IEC_TENS")
                            .jmp_addr("engine__loader__regular__advance_sector__compare")
                            .label("engine__loader__regular__advance_sector__store")
                            .clc()
                            .adc_imm(1)
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
                    .doc(&["Set the usable-sector modulus for the current track."])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_addr("LOADER_IEC_TRACK")
                            .cmp_imm(18)
                            .bcc_addr("engine__loader__regular__track_modulus__20")
                            .cmp_imm(25)
                            .bcc_addr("engine__loader__regular__track_modulus__18")
                            .cmp_imm(31)
                            .bcc_addr("engine__loader__regular__track_modulus__17")
                            .lda_imm(16)
                            .sta_addr("LOADER_IEC_MODULUS")
                            .rts()
                            .label("engine__loader__regular__track_modulus__20")
                            .lda_imm(20)
                            .sta_addr("LOADER_IEC_MODULUS")
                            .rts()
                            .label("engine__loader__regular__track_modulus__18")
                            .lda_imm(18)
                            .sta_addr("LOADER_IEC_MODULUS")
                            .rts()
                            .label("engine__loader__regular__track_modulus__17")
                            .lda_imm(17)
                            .sta_addr("LOADER_IEC_MODULUS")
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

fn prepare_atn_byte(label: &str, state: u8, byte: u8, release: bool) -> Instructions {
    let flags = if release { IEC_ATN | IEC_RELEASE_ATN } else { IEC_ATN };
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
    let run = |number: u8| format!("{label}__run_phase{number}");
    let mut b = InstructionBuilder::default();
    b.label(label)
        .lda_addr("LOADER_IEC_PHASE")
        .beq_addr(&format!("{label}__init"))
        .cmp_imm(1)
        .bne_addr(&run(2))
        .label(&phase(1))
        .lda_addr("CIA2_DATA_PORT")
        .and_imm(IN_DATA)
        .bne_addr(&wait(1))
        .jsr_addr("engine__loader__regular__atn_mask")
        .jsr_addr("engine__loader__regular__line")
        .lda_imm(2)
        .sta_addr("LOADER_IEC_PHASE")
        .lda_imm(255)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .rts()
        .label(&wait(1))
        .jsr_addr("engine__loader__regular__timeout_tick")
        .bne_addr("engine__loader__regular__fail")
        .rts()
        .label(&run(2))
        .cmp_imm(2)
        .bne_addr(&run(3))
        .lda_addr("LOADER_IEC_FLAGS")
        .and_imm(IEC_EOI)
        .beq_addr(&format!("{label}__phase2_data"))
        .lda_imm(3)
        .sta_addr("LOADER_IEC_PHASE")
        .lda_imm(10)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .rts()
        .label(&format!("{label}__phase2_data"))
        .lda_imm(5)
        .sta_addr("LOADER_IEC_PHASE")
        .lda_imm(255)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .rts()
        .label(&run(3))
        .cmp_imm(3)
        .bne_addr(&run(4))
        .lda_addr("CIA2_DATA_PORT")
        .and_imm(IN_DATA)
        .beq_addr(&wait(3))
        .lda_imm(4)
        .sta_addr("LOADER_IEC_PHASE")
        .lda_imm(10)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .rts()
        .label(&wait(3))
        .dec_addr("LOADER_IEC_TIMEOUT")
        .bne_addr(&format!("{}__done", wait(3)))
        .jmp_addr("engine__loader__regular__fail")
        .label(&format!("{}__done", wait(3)))
        .rts()
        .label(&run(4))
        .cmp_imm(4)
        .bne_addr(&run(5))
        .lda_addr("CIA2_DATA_PORT")
        .and_imm(IN_DATA)
        .bne_addr(&wait(4))
        .lda_imm(5)
        .sta_addr("LOADER_IEC_PHASE")
        .lda_imm(255)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .rts()
        .label(&wait(4))
        .dec_addr("LOADER_IEC_TIMEOUT")
        .bne_addr(&format!("{}__done", wait(4)))
        .jmp_addr("engine__loader__regular__fail")
        .label(&format!("{}__done", wait(4)))
        .rts()
        .label(&run(5))
        .cmp_imm(5)
        .bne_addr(&run(6))
        .lda_addr("CIA2_DATA_PORT")
        .and_imm(IN_DATA)
        .beq_addr(&wait(5))
        .jmp_addr(&phase(6))
        .label(&wait(5))
        .dec_addr("LOADER_IEC_TIMEOUT")
        .bne_addr(&format!("{}__done", wait(5)))
        .jmp_addr("engine__loader__regular__fail")
        .label(&format!("{}__done", wait(5)))
        .rts()
        .label(&run(6))
        .cmp_imm(6)
        .bne_addr(&run(7))
        .label(&phase(6))
        .jsr_addr("engine__loader__regular__atn_mask")
        .sta_addr("LOADER_IEC_TENS")
        .lda_addr("LOADER_IEC_BYTE")
        .ror_addr("LOADER_IEC_BYTE")
        .bcc_addr(&format!("{label}__phase6_zero"))
        .lda_addr("LOADER_IEC_TENS")
        .ora_imm(OUT_DATA)
        .jmp_addr(&format!("{label}__phase6_line"))
        .label(&format!("{label}__phase6_zero"))
        .lda_addr("LOADER_IEC_TENS")
        .label(&format!("{label}__phase6_line"))
        .jsr_addr("engine__loader__regular__line")
        .lda_imm(7)
        .sta_addr("LOADER_IEC_PHASE")
        .rts()
        .label(&run(7))
        .cmp_imm(7)
        .bne_addr(&run(8))
        .jsr_addr("engine__loader__regular__atn_mask")
        .sta_addr("LOADER_IEC_TENS")
        .lda_addr("LOADER_IEC_BYTE")
        .and_imm(0x80)
        .beq_addr(&format!("{label}__phase7_line"))
        .lda_addr("LOADER_IEC_TENS")
        .ora_imm(OUT_DATA)
        .sta_addr("LOADER_IEC_TENS")
        .label(&format!("{label}__phase7_line"))
        .lda_addr("LOADER_IEC_TENS")
        .jsr_addr("engine__loader__regular__line")
        .lda_imm(8)
        .sta_addr("LOADER_IEC_PHASE")
        .rts()
        .label(&run(8))
        .cmp_imm(8)
        .bne_addr(&run(9))
        .jsr_addr("engine__loader__regular__atn_mask")
        .ora_imm(OUT_CLK)
        .jsr_addr("engine__loader__regular__line")
        .dec_addr("LOADER_IEC_BIT")
        .bne_addr(&format!("{label}__phase8_more"))
        .lda_imm(9)
        .sta_addr("LOADER_IEC_PHASE")
        .lda_imm(255)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .rts()
        .label(&format!("{label}__phase8_more"))
        .lda_imm(6)
        .sta_addr("LOADER_IEC_PHASE")
        .lda_imm(255)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .rts()
        .label(&run(9))
        .lda_addr("CIA2_DATA_PORT")
        .and_imm(IN_DATA)
        .bne_addr(&wait(9))
        .jsr_addr("engine__loader__regular__atn_mask")
        .jsr_addr("engine__loader__regular__line")
        .jsr_addr("engine__loader__regular__on_send_done")
        .rts()
        .label(&wait(9))
        .jsr_addr("engine__loader__regular__timeout_tick")
        .bne_addr("engine__loader__regular__fail")
        .rts()
        .label(&format!("{label}__init"))
        .jsr_addr("engine__loader__regular__atn_mask")
        .jsr_addr("engine__loader__regular__line")
        .lda_imm(1)
        .sta_addr("LOADER_IEC_PHASE")
        .lda_imm(255)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .rts();
    b.build()
}

fn recv_phase() -> Instructions {
    let label = "engine__loader__regular__recv_phase";
    let complete = format!("{label}__complete");
    let enter_eoi_ack = format!("{label}__enter_eoi_ack");
    let enter_eoi_hold = format!("{label}__enter_eoi_hold");
    let mut b = InstructionBuilder::default();
    b.label(label)
        .lda_addr("LOADER_IEC_PHASE")
        .beq_addr(&format!("{label}__init"))
        .cmp_imm(1)
        .bne_addr(&format!("{label}__run_phase2"))
        .label(&format!("{label}__phase1"))
        .lda_addr("CIA2_DATA_PORT")
        .and_imm(IN_CLK)
        .bne_addr(&format!("{label}__phase1_high"))
        .dec_addr("LOADER_IEC_TIMEOUT")
        .bne_addr(&format!("{label}__phase1_wait"))
        .lda_addr("LOADER_IEC_FLAGS")
        .and_imm(IEC_EOI_SEEN)
        .bne_addr("engine__loader__regular__fail")
        .lda_addr("LOADER_IEC_FLAGS")
        .ora_imm(IEC_EOI_SEEN)
        .sta_addr("LOADER_IEC_FLAGS")
        .jmp_addr(&enter_eoi_ack)
        .label(&format!("{label}__phase1_high"))
        .lda_imm(2)
        .sta_addr("LOADER_IEC_PHASE")
        .rts()
        .label(&format!("{label}__phase1_wait"))
        .rts()
        .label(&format!("{label}__run_phase2"))
        .cmp_imm(2)
        .bne_addr(&format!("{label}__run_phase3"))
        .lda_addr("CIA2_DATA_PORT")
        .and_imm(IN_DATA)
        .beq_addr(&format!("{label}__phase2_zero"))
        .sec()
        .jmp_addr(&format!("{label}__phase2_shift"))
        .label(&format!("{label}__phase2_zero"))
        .clc()
        .label(&format!("{label}__phase2_shift"))
        .ror_addr("LOADER_IEC_BYTE")
        .lda_imm(0)
        .jsr_addr("engine__loader__regular__line")
        .lda_imm(3)
        .sta_addr("LOADER_IEC_PHASE")
        .lda_imm(255)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .rts()
        .label(&format!("{label}__run_phase3"))
        .cmp_imm(3)
        .bne_addr(&format!("{label}__run_phase5"))
        .lda_addr("CIA2_DATA_PORT")
        .and_imm(IN_CLK)
        .bne_addr(&format!("{label}__phase3_wait"))
        .dec_addr("LOADER_IEC_BIT")
        .bne_addr(&format!("{label}__phase3_more"))
        .lda_imm(OUT_DATA)
        .jsr_addr("engine__loader__regular__line")
        .lda_addr("LOADER_IEC_FLAGS")
        .and_imm(IEC_EOI_SEEN)
        .bne_addr(&enter_eoi_hold)
        .jmp_addr(&complete)
        .label(&format!("{label}__phase3_more"))
        .lda_imm(1)
        .sta_addr("LOADER_IEC_PHASE")
        .lda_imm(10)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .rts()
        .label(&format!("{label}__phase3_wait"))
        .dec_addr("LOADER_IEC_TIMEOUT")
        .bne_addr(&format!("{label}__phase3_wait_done"))
        .jmp_addr("engine__loader__regular__fail")
        .label(&format!("{label}__phase3_wait_done"))
        .rts()
        .label(&format!("{label}__run_phase5"))
        .cmp_imm(5)
        .bne_addr(&format!("{label}__run_phase8"))
        .dec_addr("LOADER_IEC_TIMEOUT")
        .bne_addr(&format!("{label}__phase5_wait"))
        .jmp_addr(&complete)
        .label(&format!("{label}__phase5_wait"))
        .rts()
        .label(&format!("{label}__run_phase8"))
        .cmp_imm(8)
        .bne_addr(&format!("{label}__run_phase9"))
        .jmp_addr(&enter_eoi_ack)
        .label(&format!("{label}__run_phase9"))
        .dec_addr("LOADER_IEC_TIMEOUT")
        .bne_addr(&format!("{label}__phase9_wait"))
        .lda_imm(0)
        .jsr_addr("engine__loader__regular__line")
        .lda_imm(10)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .lda_imm(1)
        .sta_addr("LOADER_IEC_PHASE")
        .rts()
        .label(&format!("{label}__phase9_wait"))
        .rts()
        .label(&enter_eoi_ack)
        .lda_imm(OUT_DATA)
        .jsr_addr("engine__loader__regular__line")
        .lda_imm(9)
        .sta_addr("LOADER_IEC_PHASE")
        .lda_imm(3)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .rts()
        .label(&enter_eoi_hold)
        .lda_imm(5)
        .sta_addr("LOADER_IEC_PHASE")
        .lda_imm(3)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .rts()
        .label(&format!("{label}__init"))
        .lda_imm(0)
        .jsr_addr("engine__loader__regular__line")
        .lda_imm(0)
        .sta_addr("LOADER_IEC_BYTE")
        .lda_imm(IEC_EOI)
        .sta_addr("LOADER_IEC_FLAGS")
        .lda_imm(8)
        .sta_addr("LOADER_IEC_BIT")
        .lda_imm(10)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .lda_imm(1)
        .sta_addr("LOADER_IEC_PHASE")
        .rts()
        .label(&complete)
        .lda_imm(0)
        .jsr_addr("engine__loader__regular__line")
        .jsr_addr("engine__loader__regular__on_recv_done")
        .rts();
    b.build()
}

fn turnaround_phase() -> Instructions {
    let label = "engine__loader__regular__turnaround_phase";
    InstructionBuilder::default()
        .label(label)
        .lda_addr("LOADER_IEC_PHASE")
        .bne_addr(&format!("{label}__wait"))
        .lda_imm(OUT_DATA)
        .jsr_addr("engine__loader__regular__line")
        .lda_imm(1)
        .sta_addr("LOADER_IEC_PHASE")
        .lda_imm(255)
        .sta_addr("LOADER_IEC_TIMEOUT")
        .rts()
        .label(&format!("{label}__wait"))
        .lda_addr("CIA2_DATA_PORT")
        .and_imm(IN_CLK)
        .bne_addr(&format!("{label}__still"))
        .jsr_addr("engine__loader__regular__prepare_recv")
        .rts()
        .label(&format!("{label}__still"))
        .jsr_addr("engine__loader__regular__timeout_tick")
        .bne_addr("engine__loader__regular__fail")
        .rts()
        .build()
}

fn on_send_done() -> Instructions {
    let label = "engine__loader__regular__on_send_done";
    let mut b = InstructionBuilder::default();
    b.label(label)
        .lda_addr("LOADER_IEC_FLAGS")
        .and_imm(IEC_RELEASE_ATN)
        .beq_addr(&format!("{label}__no_release"))
        .lda_imm(0)
        .jsr_addr("engine__loader__regular__line")
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
        .bne_addr(&format!("{label}__state12"))
        .jsr_addr("engine__loader__regular__prepare_turnaround")
        .rts()
        .label(&format!("{label}__state12"))
        .cmp_imm(12)
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
