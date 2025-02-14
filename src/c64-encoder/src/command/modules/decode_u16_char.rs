use c64_assembler::{
    builder::{FunctionBuilder, InstructionBuilder, ModuleBuilder},
    Module,
};

use crate::command::DecoderModule;

pub struct DecodeU16Char {}

impl DecoderModule for DecodeU16Char {
    fn module() -> Module {
        ModuleBuilder::default()
            .name("Decode U16 encoded char")
            .function(
                FunctionBuilder::default()
                    .name("char__decode_u16")
                    .doc(&[
                        "Decodes a u16 encoded character referenced by 'CHAR_DECODE_SRC_PTR'",
                        "and write the decoded character in the memory address referenced by",
                        "'CHAR_DECODE_DST_PTR'",
                        "",
                        "Uses Accumulator, X and Y indexer",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            // Decode first byte high bits
                            .ldy_imm(1)
                            .lda_ind_y("CHAR_DECODE_SRC_PTR")
                            .pha()
                            .dey()
                            .lda_ind_y("CHAR_DECODE_SRC_PTR")
                            .jsr_addr("char__decode_u16__process_byte")
                            .pla()
                            .jsr_addr("char__decode_u16__process_byte")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .instructions(
                InstructionBuilder::default()
                    .label("char__decode_u16__process_byte")
                    .pha()
                    .lsr_acc()
                    .lsr_acc()
                    .lsr_acc()
                    .lsr_acc()
                    .tax()
                    .jsr_addr("char__decode_u16__load_store")
                    .pla()
                    .and_imm(0x0F)
                    .tax()
                    // Be aware of the pass-through
                    .label("char__decode_u16__load_store")
                    .comment("Decode lower 4 bits of the accumulator to the a byte and store them in the destination")
                    .lda_addr_x("char__decode_u16__table")
                    .sta_ind_y("CHAR_DECODE_DST_PTR")
                    .iny()
                    .sta_ind_y("CHAR_DECODE_DST_PTR")
                    .iny()
                    .rts()
                    .label("char__decode_u16__table")
                    .raw(&[
                        0b00000000, 0b00000011, 0b00001100, 0b00001111, 0b00110000, 0b00110011, 0b00111100, 0b00111111,
                        0b11000000, 0b11000011, 0b11001100, 0b11001111, 0b11110000, 0b11110011, 0b11111100, 0b11111111,
                    ])
                    .build(),
            )
            .build()
    }
}
