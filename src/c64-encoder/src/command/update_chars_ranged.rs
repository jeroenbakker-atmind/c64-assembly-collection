use c64_assembler::{
    builder::{FunctionBuilder, InstructionBuilder, ModuleBuilder},
    Module,
};

use crate::{
    charmap::encoding::encode_char,
    encoder::{writer::Writer, Encoder},
};

use super::{modules::CurrentPtrMacros, DecoderModule};

#[derive(Default, Debug, Clone)]
pub struct UpdateCharsRangedU16Encoded {
    pub offset: u8,
    pub chars: Vec<UpdateCharRanged>,
}
#[derive(Default, Debug, Copy, Clone)]
pub struct UpdateCharRanged {
    pub data: u64,
}

impl Encoder for UpdateCharRanged {
    fn byte_size(&self) -> usize {
        2
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        let mut encoded_data = encoded_data;
        let encoded_char = encode_char(self.data);
        encoded_data = encoded_data.add(&encoded_char);

        encoded_data
    }
}

impl Encoder for UpdateCharsRangedU16Encoded {
    fn byte_size(&self) -> usize {
        1 + 1 + self.chars.len() * UpdateCharRanged::default().byte_size()
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        let mut encoded_data = encoded_data;
        let num_chars = self.chars.len() as u8;
        encoded_data = encoded_data.add(&num_chars).add(&self.offset);
        for char in &self.chars {
            encoded_data = char.encode(encoded_data)
        }

        encoded_data
    }
}

impl DecoderModule for UpdateCharsRangedU16Encoded {
    fn module() -> Module {
        ModuleBuilder::default()
            .name("update_chars_ranged_u16")
            .function(
                FunctionBuilder::default()
                    .name("update_chars_ranged_u16__process")
                    .instructions(
                        InstructionBuilder::default()
                            .lda_current_ptr_offs(2, "Load the starting character to update into the accumulator.")
                            .pha()
                            // TODO: ASL & LSR support other addressing modes as well. Need to check how they work.
                            .asl_acc()
                            .comment("Multiply accumulator with 8 by bit shifting 3 times")
                            .asl_acc()
                            .asl_acc()
                            .sta_addr("CHAR_DECODE_DST_PTR")
                            .pla()
                            .lsr_acc()
                            .comment("Shift 5 times for the page.")
                            .lsr_acc()
                            .lsr_acc()
                            .lsr_acc()
                            .lsr_acc()
                            .adc_imm_high("CHARSET_PTR_PAGE0")
                            .sta_addr_offs("CHAR_DECODE_DST_PTR", 1)
                            .lda_current_ptr_offs(1, "Load the number of characters into the accumulator.")
                            .pha()
                            .inc_current_ptr(3)
                            .label("update_chars_ranged_u16__decode_next")
                            .jsr_addr("char__decode_u16")
                            .inc_current_ptr(2)
                            .pla()
                            .tax()
                            .dex()
                            .beq_addr("update_chars_ranged_u16__exit")
                            .txa()
                            .pha()
                            .lda_imm(8)
                            .adc_addr("CHAR_DECODE_DST_PTR")
                            .sta_addr("CHAR_DECODE_DST_PTR")
                            .lda_imm(0)
                            .adc_addr_offs("CHAR_DECODE_DST_PTR", 1)
                            .sta_addr_offs("CHAR_DECODE_DST_PTR", 1)
                            .jmp_addr("update_chars_ranged_u16__decode_next")
                            .label("update_chars_ranged_u16__exit")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .build()
    }
}
