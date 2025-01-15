use c64_assembler::generator::{program::ProgramGenerator, Generator};
use c64_assembler_macro::application;

pub fn set_black_border_application() -> Vec<u8> {
    let application = application!(
        name="Set back border"
        include_vic20_defines
        module!(
            name="main"
            instructions!(
            include_basic_header
            main_entry_point:
                "Load black color into accumulator"
                lda #$00
                sta VIC20_BORDER_COLOR
                rts
            )
        )
    );

    ProgramGenerator::default().generate(application)
}
