use c64_assembler::{
    builder::{ApplicationBuilder, InstructionBuilder, ModuleBuilder},
    generator::{print_hexdump, DasmGenerator, Generator, ProgramGenerator},
    validator::{AssemblerResult, Validator},
};
use c64_colors::colors::Color;
use c64_encoder::{
    builder::{demo::DemoBuilder, frame::FrameBuilder},
    command::modules::EngineBuilder,
};

pub fn engine_application() -> AssemblerResult<Vec<u8>> {
    let data = DemoBuilder::default()
        .frame(
            FrameBuilder::default()
                .clear_screen_chars(0x00)
                .set_border_color(Color::Black)
                .build(),
        )
        .build();

    let engine_data = ModuleBuilder::default()
        .name("engine_data")
        .instructions(InstructionBuilder::default().label("engine_data").raw(&data).build())
        .build();

    let application = ApplicationBuilder::default()
        .name("Engine")
        .include_vic20_defines()
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
        .add_engine()
        .module(engine_data)
        .build()
        .unwrap();
    
    application.validate()?;
    
    println!("{}", DasmGenerator::default().generate(application.clone()).unwrap());
    let result = ProgramGenerator::default().generate(application).unwrap();
    print_hexdump(&result);
    Ok(result)
}
