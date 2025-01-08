use std::ops::{Deref, DerefMut};

use crate::{
    builder::{
        application::{Application, ApplicationBuilder},
        instruction::InstructionBuilder,
        module::ModuleBuilder,
    },
    generator::{dasm::DasmGenerator, program::ProgramGenerator, Generator},
};

fn test_application() -> Application {
    ApplicationBuilder::default()
        .name("test build dasm")
        .add_vic20()
        .add_module(
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
        .finalize()
}

#[test]
fn build_dasm() {
    let application = test_application();
    let dasm_source = DasmGenerator::default().generate(application);
    println!("{dasm_source}");
}

#[test]
fn build_program() {
    let application = test_application();
    let mut address = application.entry_point;
    let program_binary = ProgramGenerator::default().generate(application);

    // print program to console.
    program_binary.chunks(16).for_each(|chunk| {
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
}
