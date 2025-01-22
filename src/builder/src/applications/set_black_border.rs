use c64_assembler::generator::{
    dasm::DasmGenerator,
    program::{print_hexdump, ProgramGenerator},
    Generator,
};
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

    println!("{}", DasmGenerator::default().generate(application.clone()));

    let result = ProgramGenerator::default().generate(application);
    print_hexdump(&result);
    result
}
