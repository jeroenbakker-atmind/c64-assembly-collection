use c64_assembler::{
    builder::{ApplicationBuilder, FunctionBuilder, InstructionBuilder, ModuleBuilder},
    Instructions, Module,
};
use c64_assembler_macro::function;

use super::{CommandsLeft, CurrentPTR, CurrentPtrMacros, DecodeU16Char, ScreenCharPTR};
use crate::command::{all_decoder_modules, DecoderModule};

pub trait EngineBuilder {
    fn add_engine(&mut self) -> &mut Self;
}

impl EngineBuilder for ApplicationBuilder {
    fn add_engine(&mut self) -> &mut Self {
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
            .define_address("SCRATCH_SPACE_00", 0xFB)
            .module(Engine::module())
            .module(CurrentPTR::module())
            .module(ScreenCharPTR::module())
            .module(CommandsLeft::module())
            .module(DecodeU16Char::module());
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
                        "Initialize the engine.",
                        "",
                        " - assumes engine data is stored at 'engine-data'",
                        " - sets the current pointer to the first frame",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_imm_low("engine_data")
                            .sta_addr("CURRENT_PTR")
                            .lda_imm_high("engine_data")
                            .sta_addr_offs("CURRENT_PTR", 1)
                            .inc_current_ptr(2)
                            .comment("Advance the pointer with 2 bytes.")
                            .comment("Number of frames is only needed when reading directly from disk.")
                            .lda_imm(0)
                            .comment("Attach the VIC-II to bank 3 ($C000-$FFFF)")
                            .sta_addr("C64_BANK_SELECTION")
                            .lda_imm(0b00000011)
                            .comment("Set screen chars to $C000 and charset to $C800.")
                            .sta_addr("VIC2_MEMORY_SETUP")
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
                "Update VIC2 registries"
                "TODO: use shadow registries"
                "TODO: wait for vblank"
                rts

            engine__frame_command__process:
                jsr engine__frame_command__switch
                jsr engine__commands_left__decrease
                jmp engine__frame_commands__next

                )
            ))
            .function(
                FunctionBuilder::default()
                    .name("engine__frame_command__switch")
                    .instructions(command_dispatcher())
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
