use c64_assembler::{
    builder::{ApplicationBuilder, FunctionBuilder, InstructionBuilder, ModuleBuilder},
    Instructions, Module,
};
use c64_assembler_macro::function;

use super::{
    Bank, BankBuilder, BlockScratchBuilder, CommandsLeft, Condition, ConditionBuilder, CurrentPTR, CurrentPtrMacros,
    DecodeU16Char, FramesLeft, Irq, Loader, LoaderBackend, LoaderBuilder, LoaderRegularBuilder, Refresh,
    RefreshBuilder, RefreshWaitN, RegularLoaderBackend, Resource, ResourceBuilder, ScreenCharPTR,
    UpdateScreenBlocksDecoder,
};
use crate::command::{all_decoder_modules, DecoderModule};

pub trait EngineBuilder {
    fn add_engine(&mut self) -> &mut Self;

    /// Install the engine with an explicitly selected loader backend.
    fn add_engine_with_loader<B: LoaderBackend>(&mut self) -> &mut Self;
}

pub trait FramePlayBuilder {
    fn add_frame_play_addresses(&mut self) -> &mut Self;
}

impl FramePlayBuilder for ApplicationBuilder {
    fn add_frame_play_addresses(&mut self) -> &mut Self {
        self.define_address("FRAME_PLAY_COUNT", 0x8A)
    }
}

impl EngineBuilder for ApplicationBuilder {
    /// Install the regular host-side IEC/U1 DOS-sector backend.
    ///
    /// CPU tests that need a deterministic transfer select
    /// `add_engine_with_loader::<StubLoaderBackend>()` explicitly.
    fn add_engine(&mut self) -> &mut Self {
        self.add_engine_with_loader::<RegularLoaderBackend>()
    }

    fn add_engine_with_loader<B: LoaderBackend>(&mut self) -> &mut Self {
        self.define_address("CURRENT_PTR", 0xFE)
            .define_address("SCREEN_CHAR_PTR", 0xFC)
            .define_address("CHAR_DECODE_SRC_PTR", 0xFE)
            .define_address("CHAR_DECODE_DST_PTR", 0x10)
            .define_address("SCREEN_CHARS_PAGE0", 0xC000)
            .define_address("SCREEN_CHARS_PAGE1", 0xC100)
            .define_address("SCREEN_CHARS_PAGE2", 0xC200)
            .define_address("SCREEN_CHARS_PAGE3", 0xC300)
            .define_address("SCREEN_COLORS_PAGE0", 0xD800)
            .define_address("SCREEN_COLORS_PAGE1", 0xD900)
            .define_address("SCREEN_COLORS_PAGE2", 0xDA00)
            .define_address("SCREEN_COLORS_PAGE3", 0xDB00)
            .define_address("CHARSET_PTR_PAGE0", 0xC800)
            .define_address("CHARSET_PTR_PAGE1", 0xC900)
            .define_address("CHARSET_PTR_PAGE2", 0xCA00)
            .define_address("CHARSET_PTR_PAGE3", 0xCB00)
            .define_address("C64_BANK_SELECTION", 0xDD00)
            .define_address("ENGINE_NMI_A", 0x3E33)
            .define_address("SCRATCH_SPACE_00", 0xFB)
            .add_block_scratch_addresses()
            .add_refresh_addresses()
            .add_frame_play_addresses()
            .add_resource_addresses()
            .add_loader_addresses()
            .add_loader_regular_addresses()
            .add_bank_addresses()
            .add_condition_addresses()
            .module(Engine::module())
            .module(FramePlay::module())
            .module(Irq::module())
            .module(Resource::module())
            .module(Loader::module())
            .module(B::module())
            .module(Bank::module())
            .module(Condition::module())
            .module(CurrentPTR::module())
            .module(ScreenCharPTR::module())
            .module(CommandsLeft::module())
            .module(FramesLeft::module())
            .module(Refresh::module())
            .module(RefreshWaitN::module())
            .module(DecodeU16Char::module())
            .module(UpdateScreenBlocksDecoder::module());
        for (_, module) in all_decoder_modules() {
            self.module(module);
        }
        self
    }
}

pub struct Engine {}

impl DecoderModule for Engine {
    fn module() -> Module {
        ModuleBuilder::default()
            .name("engine")
            .function(
                FunctionBuilder::default()
                    .name("engine__init")
                    .doc(&[
                        "Initialize the VIC setup, ROM bank-out, and shared IRQ dispatcher.",
                        "",
                        "The emitted scripted sequence is responsible for loading resources and",
                        "pointing CURRENT_PTR at them. Legacy applications additionally call",
                        "`engine__stream__init`.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_imm(0)
                            .comment("Attach the VIC-II to bank 3 ($C000-$FFFF)")
                            .sta_addr("C64_BANK_SELECTION")
                            .lda_imm(0b00000011)
                            .comment("D018: screen $C000 and charset $C800 in VIC bank 3.")
                            .sta_addr("VIC2_MEMORY_SETUP")
                            .comment("Make RAM visible at $A000..$BFFF before any music load.")
                            .jsr_addr("engine__bank__init")
                            .comment("Install the shared IRQ dispatcher after the VIC is configured.")
                            .jsr_addr("engine__refresh__init")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__stream__init")
                    .doc(&[
                        "Legacy: point CURRENT_PTR at the monolithic engine_data stream.",
                        "",
                        "Reads the frame count from the start of the stream before advancing",
                        "CURRENT_PTR to the first frame record.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_imm_low("engine_data")
                            .sta_addr("CURRENT_PTR")
                            .lda_imm_high("engine_data")
                            .sta_addr_offs("CURRENT_PTR", 1)
                            .jsr_addr("engine__frames_left__init")
                            .comment("Read the number of frames before advancing the pointer.")
                            .inc_current_ptr(2)
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__deinit")
                    .instructions(
                        InstructionBuilder::default()
                            .jsr_addr("engine__refresh__deinit")
                            .jsr_addr("engine__bank__restore")
                            .lda_imm(3)
                            .comment("Attach the VIC-II to bank 0 ($0000-$7FFF")
                            .sta_addr("C64_BANK_SELECTION")
                            .lda_imm(0b0010101)
                            .sta_addr("VIC2_MEMORY_SETUP")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(function!(
            name="engine__frame__process"
            instructions!(
                jsr engine__commands_left__init
                lda #$2
                jsr engine__current_ptr__advance
            engine__frame_commands__next:
                jsr engine__commands_left__is_zero
                bne engine__frame_command__process
            engine__frame__exit:
                "All commands in frame have been processed."
                "Wait only for the refreshes still owed; the IRQ decremented the countdown"
                "while the frame was being decoded."
                jsr engine__refresh__wait
                "TODO: flush shadow VIC2 registries here once commands use shadows."
                rts

            engine__frame_command__process:
                jsr engine__frame_command__switch
                jsr engine__commands_left__decrease
                jmp engine__frame_commands__next

                )
            ))
            .function(
                FunctionBuilder::default()
                    .name("engine__play")
                    .doc(&[
                        "Play all frames from engine_data, one frame every four vertical refreshes.",
                        "",
                        "Frames that take longer to decode start the next one as soon as the",
                        "remaining refreshes have passed; decode time is counted by the IRQ.",
                        "When all frames have been played the final frame is kept on screen",
                        "by looping forever instead of returning to the caller.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .label("engine__play__next")
                            .jsr_addr("engine__frames_left__is_zero")
                            .beq_addr("engine__play__done")
                            .jsr_addr("engine__frame__process")
                            .jsr_addr("engine__frames_left__decrease")
                            .jmp_addr("engine__play__next")
                            .label("engine__play__done")
                            .jmp_addr("engine__play__done")
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__play_stream")
                    .doc(&["Legacy alias for `engine__play`."])
                    .instructions(InstructionBuilder::default().jmp_addr("engine__play").build())
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__frame_command__switch")
                    .instructions(command_dispatcher())
                    .build(),
            )
            .build()
    }
}

/// Standalone frame-count primitive so CPU tests can stub `engine__frame__process`.
pub struct FramePlay {}

impl DecoderModule for FramePlay {
    fn module() -> Module {
        ModuleBuilder::default()
            .name("engine__frame__play_n")
            .function(
                FunctionBuilder::default()
                    .name("engine__frame__play_n")
                    .doc(&[
                        "Decode A/X frame records from CURRENT_PTR.",
                        "",
                        "A = count low byte, X = count high byte. The counter lives in zero",
                        "page because the frame decoder clobbers all registers.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .sta_addr("FRAME_PLAY_COUNT")
                            .stx_addr_offs("FRAME_PLAY_COUNT", 1)
                            .label("engine__frame__play_n__next")
                            .lda_addr("FRAME_PLAY_COUNT")
                            .bne_addr("engine__frame__play_n__frame")
                            .lda_addr_offs("FRAME_PLAY_COUNT", 1)
                            .beq_addr("engine__frame__play_n__done")
                            .label("engine__frame__play_n__frame")
                            .jsr_addr("engine__frame__process")
                            .lda_addr("FRAME_PLAY_COUNT")
                            .sec()
                            .sbc_imm(1)
                            .sta_addr("FRAME_PLAY_COUNT")
                            .lda_addr_offs("FRAME_PLAY_COUNT", 1)
                            .sbc_imm(0)
                            .sta_addr_offs("FRAME_PLAY_COUNT", 1)
                            .jmp_addr("engine__frame__play_n__next")
                            .label("engine__frame__play_n__done")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .build()
    }
}

fn command_dispatcher() -> Instructions {
    let mut command_switch_builder = InstructionBuilder::default();
    command_switch_builder.lda_current_ptr_offs(0, "Load the current command type in the accumulator");

    let commands = all_decoder_modules();
    for (command_id, command, next_command) in commands.iter().enumerate().map(|(index, module_def)| {
        (
            module_def.0,
            module_def.1.name.as_str(),
            commands
                .get(index + 1)
                .map_or("exit", |next_item| next_item.1.name.as_str()),
        )
    }) {
        command_switch_builder
            .label(format!("engine__frame_command__{command}").as_str())
            .cmp_imm(command_id)
            .bne_addr(format!("engine__frame_command__{next_command}").as_str())
            .jmp_addr(format!("{command}__process").as_str());
    }
    command_switch_builder.label("engine__frame_command__exit").rts();
    command_switch_builder.build()
}
