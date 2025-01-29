use c64_assembler::{
    builder::{ApplicationBuilder, FunctionBuilder, InstructionBuilder, ModuleBuilder},
    generator::{print_hexdump, DasmGenerator, Generator, ProgramGenerator},
};
use c64_assembler_macro::function;
use c64_colors::colors::Color;
use c64_encoder::builder::{demo::DemoBuilder, frame::FrameBuilder};

pub fn engine_application() -> Vec<u8> {
    let data = DemoBuilder::default()
        .frame(FrameBuilder::default().set_border_color(Color::Black).build())
        .build();

    let engine_data = ModuleBuilder::default()
        .name("engine_data")
        .instructions(InstructionBuilder::default().label("engine_data").raw(&data).build())
        .build();

    let set_border_color = ModuleBuilder::default()
        .name("set_border_color")
        .function(
            FunctionBuilder::default()
                .name("set_border_color__process")
                .instructions(
                    InstructionBuilder::default()
                        .ldy_imm(1)
                        .lda_ind_y("CURRENT_PTR")
                        .sta_addr("set_border_color__data")
                        .lda_imm(2)
                        .jsr_addr("engine__current_ptr__advance")
                        .rts()
                        .build(),
                )
                .build(),
        )
        .function(function!(
            name = "set_border_color__vblank",
            instructions!(
                lda set_border_color__data
                sta VIC20_BORDER_COLOR
                rts
            )
        ))
        .instructions(
            InstructionBuilder::default()
                .label("set_border_color__data")
                .comment("Border color to set at the next vblank")
                .raw(&[0x00])
                .build(),
        )
        .build();

    let application = ApplicationBuilder::default()
        .name("Engine")
        .include_vic20_defines()
        .define_address("CURRENT_PTR", 0xFE)
        .module(
            ModuleBuilder::default()
                .name("main")
                .instructions(
                    InstructionBuilder::default()
                        .add_basic_header()
                        .label("main_entry_point")
                        .jsr_addr("engine__init")
                        .jsr_addr("engine__frame__process")
                        .rts()
                        .build(),
                )
                .build(),
        )
        .module(
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
                    "Update VIC20 registries"
                    "TODO: use shadow registries"
                    "TODO: wait for vblank"
                    jsr set_border_color__vblank
                    rts

                engine__frame_command__process:
                    jsr set_border_color__process
                    jsr engine__commands_left__decrease
                    jmp engine__frame_commands__next

                    )
                ))
                .build(),
        )
        .module(
            ModuleBuilder::default()
                .name("engine__current_ptr")
                .function(
                    FunctionBuilder::default()
                        .name("engine__current_ptr__advance")
                        .doc(&[
                            "Advance current pointer with accumulator",
                            "",
                            "Advance the pointer stored at CURRENT_PTR with the value stored in the accumulator.",
                            "The accumulator is number of bytes to advance.",
                        ])
                        .instructions(
                            InstructionBuilder::default()
                                .clc()
                                .adc_addr("CURRENT_PTR")
                                .sta_addr("CURRENT_PTR")
                                .lda_imm(0x00)
                                .adc_addr_offs("CURRENT_PTR", 1)
                                .sta_addr_offs("CURRENT_PTR", 1)
                                .rts()
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .module(
            ModuleBuilder::default()
                .name("engine__commands_left")
                .instructions(
                    InstructionBuilder::default()
                        .label("engine__commands_left")
                        .comment("Number of commands left to process in the current frame.")
                        .comment("When 0 the frame is finished processing")
                        .raw(&[0x00; 2])
                        .build(),
                )
                .function(
                    FunctionBuilder::default()
                        .name("engine__commands_left__init")
                        .instructions(
                            InstructionBuilder::default()
                                .ldy_imm(0x00)
                                .lda_ind_y("CURRENT_PTR")
                                .sta_addr("engine__commands_left")
                                .iny()
                                .lda_ind_y("CURRENT_PTR")
                                .sta_addr_offs("engine__commands_left", 1)
                                .rts()
                                .build(),
                        )
                        .build(),
                )
                .function(
                    FunctionBuilder::default()
                        .name("engine__commands_left__decrease")
                        .instructions(
                            InstructionBuilder::default()
                                .clc()
                                .lda_addr("engine__commands_left")
                                .sbc_imm(0x00)
                                .sta_addr("engine__commands_left")
                                .lda_addr_offs("engine__commands_left", 1)
                                .sbc_imm(0x00)
                                .sta_addr_offs("engine__commands_left", 1)
                                .rts()
                                .build(),
                        )
                        .build(),
                )
                .function(
                    FunctionBuilder::default()
                        .name("engine__commands_left__is_zero")
                        .instructions(
                            InstructionBuilder::default()
                                .lda_imm(0x00)
                                .cmp_addr("engine__commands_left")
                                .bne_addr("engine__commands_left__is_zero__exit")
                                .cmp_addr_offs("engine__commands_left", 1)
                                .label("engine__commands_left__is_zero__exit")
                                .rts()
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .module(set_border_color)
        .module(engine_data)
        .build();
    println!("{}", DasmGenerator::default().generate(application.clone()));

    let result = ProgramGenerator::default().generate(application);
    print_hexdump(&result);
    result
}
