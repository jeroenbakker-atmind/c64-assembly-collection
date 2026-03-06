use c64_assembler::generator::{DasmGenerator, Generator, ProgramGenerator};
use c64_assembler_macro::application;

pub fn senimul_application() -> Vec<u8> {
    let application = application!(
        name="Senimul"
        include_vic2_defines
        define_address C64_BANK_SELECTION=$DD00
        define_address SCREEN_CHARS_PAGE0=$C000
        define_address SCREEN_CHARS_PAGE1=$C100
        define_address SCREEN_CHARS_PAGE2=$C200
        define_address SCREEN_CHARS_PAGE3=$C300
        define_address SCREEN_COLORS_PAGE0=$D800
        define_address SCREEN_COLORS_PAGE1=$D900
        define_address SCREEN_COLORS_PAGE2=$DA00
        define_address SCREEN_COLORS_PAGE3=$DB00
        define_address CHARSET_PTR_PAGE0=$C800
        define_address CHARSET_PTR_PAGE1=$C900
        define_address CHARSET_PTR_PAGE2=$CA00
        define_address CHARSET_PTR_PAGE3=$CB00

        module!(
            name="main"
            instructions!(
            include_basic_header
            main_entry_point:
                "Load black color into accumulator"
                jsr screen_init
                rts
            )
        )
        module!(
            name="screen"
            function!(
                name="screen_clear"
                instructions!(
                    rts
                )
            )
            function!(
                name="screen_init"
                instructions!(
                    lda #$00
                    sta VIC2_BORDER_COLOR
                    sta VIC2_BACKGROUND_COLOR

                    // Attach VIC-II to back 3 ($c000-$ffff)
                    lda #$00
                    sta C64_BANK_SELECTION
                    // Set screen chars to $c000 and charset to $c800
                    lda #$03
                    sta VIC2_MEMORY_SETUP

                    rts
                )
            )
        )
    )
    .unwrap();

    println!("{}", DasmGenerator::default().generate(application.clone()).unwrap());

    ProgramGenerator::default().generate(application).unwrap()
}

