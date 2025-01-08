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
                        .jsr_addr("engine__init")
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
                        .name("engine__init")
                        .instructions(
                            InstructionBuilder::default()
                                .lda_imm_low("engine_data")
                                .sta_addr("CURRENT_PTR")
                                .lda_imm_high("engine_data")
                                .sta_addr_offs("CURRENT_PTR", 1)
                                .lda_imm(2).comment("Advance the pointer with 2 bytes.")
                                .comment("Number of frames is only needed when reading directly from disk.")
                                .jsr_addr("engine__current_ptr__advance")
                                .rts()
                                .finalize(),
                        )
                        .finalize(),
                )
                .finalize(),
        )
        .add_module(ModuleBuilder::default().name("engine__current_ptr").function(
            FunctionBuilder::default().name("engine__current_ptr__advance").instructions(
                InstructionBuilder::default()
                .clc().adc_addr("CURRENT_PTR").sta_addr("CURRENT_PTR").lda_imm(0x00).adc_addr_offs("CURRENT_PTR", 1).sta_addr_offs("CURRENT_PTR", 1).rts()
                .finalize()).finalize()).finalize())
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

    let result = ProgramGenerator::default().generate(application);
    let mut address = 0;
    result.chunks(16).for_each(|chunk| {
        let mut line = Vec::new();

        line.push(format!("{:04X}: ", address));
        address += 16;

        chunk.chunks(4).for_each(|chunk| {
            chunk.iter().for_each(|byte| {
                line.push(format!("{:02X}", byte));
            });
            line.push("".to_string());
        });
        println!("{}", line.join(" ").trim_end());
    });

    result
}
