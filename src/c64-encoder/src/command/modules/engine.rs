use c64_assembler::{
    builder::{ApplicationBuilder, FunctionBuilder, InstructionBuilder, ModuleBuilder},
    Instructions, Module,
};
use c64_assembler_macro::function;

use crate::command::{
    clear_screen_chars::ClearScreenChars, set_border_color::SetBorderColor,
    update_chars_ranged::UpdateCharsRangedU16Encoded, DecoderModule, CLEAR_SCREEN_CHAR, SET_BORDER_COLOR,
    UPDATE_CHARS_RANGED_U16,
};

use super::{CommandsLeft, CurrentPTR, DecodeU16Char};

pub trait EngineBuilder {
    fn add_engine(&mut self) -> &mut Self;
}

impl EngineBuilder for ApplicationBuilder {
    fn add_engine(&mut self) -> &mut Self {
        self.define_address("CURRENT_PTR", 0xFE)
            .define_address("CHAR_DECODE_SRC_PTR", 0xFE)
            .define_address("CHAR_DECODE_DST_PTR", 0x10)
            .define_address("SCREEN_CHARS_PAGE0", 0x0400)
            .define_address("SCREEN_CHARS_PAGE1", 0x0500)
            .define_address("SCREEN_CHARS_PAGE2", 0x0600)
            .define_address("SCREEN_CHARS_PAGE3", 0x0700)
            .define_address("CHARSET_PTR_PAGE0", 0xC000)
            .define_address("CHARSET_PTR_PAGE1", 0xC100)
            .define_address("CHARSET_PTR_PAGE2", 0xC200)
            .define_address("CHARSET_PTR_PAGE3", 0xC300)
            .module(Engine::module())
            .module(CurrentPTR::module())
            .module(CommandsLeft::module())
            .module(ClearScreenChars::module())
            .module(SetBorderColor::module())
            .module(UpdateCharsRangedU16Encoded::module())
            .module(DecodeU16Char::module())
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
                            .lda_imm(2)
                            .comment("Advance the pointer with 2 bytes.")
                            .comment("Number of frames is only needed when reading directly from disk.")
                            .jsr_addr("engine__current_ptr__advance")
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
    command_switch_builder.ldx_imm(0).lda_ind_x("CURRENT_PTR");
    for (command, command_id, next) in [
        ("clear_screen_chars", CLEAR_SCREEN_CHAR, "set_border_color"),
        ("set_border_color", SET_BORDER_COLOR, "update_chars_ranged_u16"),
        ("update_chars_ranged_u16", UPDATE_CHARS_RANGED_U16, "exit"),
    ] {
        command_switch_builder
            .label(format!("engine__frame_command__{command}").as_str())
            .cmp_imm(command_id)
            .bne_addr(format!("engine__frame_command__{next}").as_str())
            .jmp_addr(format!("{command}__process").as_str());
    }
    command_switch_builder.label("engine__frame_command__exit").rts();
    command_switch_builder.build()
}
