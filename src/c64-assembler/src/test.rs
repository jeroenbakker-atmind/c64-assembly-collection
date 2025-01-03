use crate::{
    builder::application_builder::ApplicationBuilder,
    generator::{dasm::DasmGenerator, Generator},
    memory::{address_mode::AddressMode, label::AddressReference},
};

#[test]
fn build_dasm() {
    let mut application = ApplicationBuilder::default();
    application
        .name("test build dasm")
        .define_address("VIC20_BORDER_COLOR", 0xD020)
        .basic_header()
        .main_module()
        .instructions()
        .label("main_entry_point")
        .load_accumulator(AddressMode::Immediate(0))
        .store_accumulator(AddressMode::Absolute(AddressReference::new(
            "VIC_BORDER_COLOR",
        )))
        .return_to_caller();

    let dasm_source = DasmGenerator::default().generate(application);
    println!("{dasm_source}");
}
