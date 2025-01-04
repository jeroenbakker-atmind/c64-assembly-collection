use c64_assembler::{
    builder::application_builder::ApplicationBuilder,
    generator::{program::ProgramGenerator, Generator},
    memory::{address_mode::AddressMode, label::AddressReference},
};

pub fn create_generated_application() -> Vec<u8> {
    let mut application = ApplicationBuilder::default();
    let main_module = application
        .name("test build dasm")
        .add_vic20()
        .basic_header()
        .main_module();

    main_module
        .instructions()
        .label("main_entry_point")
        .load_accumulator(AddressMode::Immediate(0))
        .comment("Load black color")
        .store_accumulator(AddressMode::Absolute(AddressReference::new(
            "VIC20_BORDER_COLOR",
        )))
        .return_to_caller();

    application.finalize();

    ProgramGenerator::default().generate(application)
}
