use c64_assembler::{
    builder::{
        application_builder::ApplicationBuilder, function_builder::FunctionBuilder,
        instruction_builder::InstructionBuilder, module_builder::ModuleBuilder,
    },
    generator::{dasm::DasmGenerator, program::ProgramGenerator, Generator},
};

pub fn engine_application() -> Vec<u8> {
    let application = ApplicationBuilder::default()
        .name("Engine")
        .add_vic20()
        .define_zeropage("CURRENT_PTR", 0xFE)
        .add_module(
            ModuleBuilder::default()
                .name("main")
                .instructions(
                    InstructionBuilder::default()
                        .add_basic_header()
                        .label("main_entry_point")
                        .rts()
                        .finalize(),
                )
                .finalize(),
        )
        .add_module(
            ModuleBuilder::default()
                .name("engine")
                .function(
                    FunctionBuilder::default()
                        .name("engine_init")
                        .instructions(
                            InstructionBuilder::default()
                                .lda_imm_low("engine_data")
                                .sta_addr("CURRENT_PTR")
                                .lda_imm_high("engine_data")
                                .sta_addr_offs("CURRENT_PTR", 1)
                                .rts()
                                .finalize(),
                        )
                        .finalize(),
                )
                .finalize(),
        )
        .add_module(
            ModuleBuilder::default()
                .name("engine_data")
                .instructions(
                    InstructionBuilder::default()
                        .label("engine_data")
                        .raw(&[0x01, 0x00])
                        .comment("Data contains one frame")
                        .label("engine_data__frame_1")
                        .raw(&[0x01, 0x00])
                        .comment("Frame contains one command")
                        .label("engine_data__frame_1__command_1")
                        .raw(&[0x03, 0x00])
                        .comment("Set border color to black")
                        .finalize(),
                )
                .finalize(),
        )
        .finalize();
    println!("{}", DasmGenerator::default().generate(application.clone()));
    ProgramGenerator::default().generate(application)
}
