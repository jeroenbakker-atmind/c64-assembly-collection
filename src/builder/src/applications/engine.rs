use c64_assembler::{
    builder::{
        application::ApplicationBuilder, function::FunctionBuilder, instruction::InstructionBuilder,
        module::ModuleBuilder,
    },
    generator::{
        dasm::DasmGenerator,
        program::{print_hexdump, ProgramGenerator},
        Generator,
    },
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
        .instructions(InstructionBuilder::default().label("engine_data").raw(&data).finalize())
        .finalize();

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
                        .finalize(),
                )
                .finalize(),
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
                .finalize(),
        )
        .finalize();

    let application = ApplicationBuilder::default()
        .name("Engine")
        .add_vic20()
        .define_zeropage("CURRENT_PTR", 0xFE)
        .module(
            ModuleBuilder::default()
                .name("main")
                .instructions(
                    InstructionBuilder::default()
                        .add_basic_header()
                        .label("main_entry_point")
                        .jsr_addr("engine__init")
                        .rts()
                        .finalize(),
                )
                .finalize(),
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
                                .finalize(),
                        )
                        .finalize(),
                )
                .finalize(),
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
                                .finalize(),
                        )
                        .finalize(),
                )
                .finalize(),
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
                        .finalize(),
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
                                .finalize(),
                        )
                        .finalize(),
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
                                .finalize(),
                        )
                        .finalize(),
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
                                .finalize(),
                        )
                        .finalize(),
                )
                .finalize(),
        )
        .module(set_border_color)
        .module(engine_data)
        .finalize();
    println!("{}", DasmGenerator::default().generate(application.clone()));

    let result = ProgramGenerator::default().generate(application);
    print_hexdump(&result);
    result
}
