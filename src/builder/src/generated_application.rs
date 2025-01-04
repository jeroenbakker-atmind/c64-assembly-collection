use c64_assembler::{
    builder::{
        application_builder::ApplicationBuilder, instruction_builder::InstructionBuilder,
        module_builder::ModuleBuilder,
    },
    generator::{program::ProgramGenerator, Generator},
    memory::{address_mode::AddressMode, label::AddressReference},
};

pub fn create_generated_application() -> Vec<u8> {
    let application = ApplicationBuilder::default()
        .name("test build dasm")
        .add_vic20()
        .add_module(
            ModuleBuilder::default()
                .name("main")
                .instructions(
                    InstructionBuilder::default()
                        .add_basic_header()
                        .label("main_entry_point")
                        .load_accumulator(AddressMode::Immediate(0))
                        .comment("Load black color")
                        .store_accumulator(AddressMode::Absolute(AddressReference::new(
                            "VIC20_BORDER_COLOR",
                        )))
                        .return_to_caller()
                        .finalize(),
                )
                .finalize(),
        )
        .finalize();

    ProgramGenerator::default().generate(application)
}
