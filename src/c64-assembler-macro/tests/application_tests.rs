use c64_assembler::generator::{program::ProgramGenerator, Generator};
use c64_assembler_macro::*;

#[test]
fn application_name() {
    let application = application!(name = "HelloWorld!");
    assert_eq!("HelloWorld!", application.name);
    let byte_code = ProgramGenerator::default().generate(application);
    assert_eq!(vec![0x00, 0x08,], byte_code);
}

#[test]
fn application_entry_point() {
    let application = application!();
    assert_eq!(0x0800, application.entry_point);
    let byte_code = ProgramGenerator::default().generate(application);
    assert_eq!(vec![0x00, 0x08,], byte_code);
}

#[test]
fn application_entry_point_0x1000() {
    let application = application!(entry_point = 0x1000);
    assert_eq!(0x1000, application.entry_point);
    let byte_code = ProgramGenerator::default().generate(application);
    assert_eq!(vec![0x00, 0x10,], byte_code);
}

#[test]
fn application_basic_header() {
    let application = application!(
        name = "My demo"
        entry_point = 0x0800
        module!(
            name = "main"
            instructions!(
                include_basic_header

            main_entry_point:
                lda #$00
                rts
            )
        )
    );
    let byte_code = ProgramGenerator::default().generate(application);
    assert_eq!(
        vec![0x00, 0x08, 0, 12, 8, 10, 0, 158, 32, 50, 48, 54, 50, 0, 0, 0, 169, 0, 96],
        byte_code
    );
}
