use c64_assembler::{
    builder::{
        application::ApplicationBuilder, instruction::InstructionBuilder, module::ModuleBuilder,
    },
    generator::{program::ProgramGenerator, Generator},
};

pub fn set_black_border_application() -> Vec<u8> {
    let application = ApplicationBuilder::default()
        .name("Set black border")
        .add_vic20()
        .module(
            ModuleBuilder::default()
                .name("main")
                .instructions(
                    InstructionBuilder::default()
                        .add_basic_header()
                        .label("main_entry_point")
                        .lda_imm(0x00)
                        .comment("Load black color")
                        .sta_addr("VIC20_BORDER_COLOR")
                        .rts()
                        .finalize(),
                )
                .finalize(),
        )
        .finalize();

    ProgramGenerator::default().generate(application)
}
