use c64_assembler::generator::{Generator, ProgramGenerator};
use c64_assembler_macro::application;

pub fn set_black_border_application() -> Vec<u8> {
    let application = application!(
        name="Set back border"
        include_vic2_defines
        module!(
            name="main"
            instructions!(
            include_basic_header
            main_entry_point:
                "Load black color into accumulator"
                lda #$00
                sta VIC2_BORDER_COLOR
                rts
            )
        )
    )
    .unwrap();

    ProgramGenerator::default().generate(application).unwrap()
}
