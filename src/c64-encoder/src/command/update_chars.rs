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
pub struct UpdateCharsU16Encoded {
    pub chars: Vec<UpdateChar>,
}
#[derive(Default, Debug, Copy, Clone)]
pub struct UpdateChar {
    pub char: u8,
    pub data: u64,
}

impl Encoder for UpdateChar {
    fn byte_size(&self) -> usize {
        1 + 2
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        let mut encoded_data = encoded_data;
        let encoded_char = encode_char(self.data);
        encoded_data = encoded_data.add(&self.char).add(&encoded_char);

        encoded_data
    }
}

impl Encoder for UpdateCharsU16Encoded {
    fn byte_size(&self) -> usize {
        1 + self.chars.len() * UpdateChar::default().byte_size()
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        let mut encoded_data = encoded_data;
        let num_chars = self.chars.len() as u8;
        encoded_data = encoded_data.add(&num_chars);
        for char in &self.chars {
            encoded_data = char.encode(encoded_data)
        }

        encoded_data
    }
}

impl DecoderModule for UpdateCharsU16Encoded {
    fn module() -> Module {
        ModuleBuilder::default()
            .name("update_chars_u16")
            .function(
                FunctionBuilder::default()
                    .name("update_chars_u16__process")
                    .instructions(
                        InstructionBuilder::default()
                            .lda_current_ptr_offs(1, "Load the number of characters to update.")
                            .pha()
                            .inc_current_ptr(2)
                            .label("update_chars_u16__decode_next")
                            .lda_current_ptr_offs(0, "Load the character index to update.")
                            .comment("Calculate the destination address: CHARSET_PTR_PAGE0 + character * 8.")
                            .pha()
                            .asl_acc()
                            .asl_acc()
                            .asl_acc()
                            .sta_addr("CHAR_DECODE_DST_PTR")
                            .pla()
                            .comment("Shift 5 times to calculate the page of the character.")
                            .lsr_acc()
                            .lsr_acc()
                            .lsr_acc()
                            .lsr_acc()
                            .lsr_acc()
                            .clc()
                            .adc_imm_high("CHARSET_PTR_PAGE0")
                            .sta_addr_offs("CHAR_DECODE_DST_PTR", 1)
                            .inc_current_ptr(1)
                            .jsr_addr("char__decode_u16")
                            .inc_current_ptr(2)
                            .pla()
                            .tax()
                            .dex()
                            .beq_addr("update_chars_u16__exit")
                            .txa()
                            .pha()
                            .jmp_addr("update_chars_u16__decode_next")
                            .label("update_chars_u16__exit")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .build()
    }
}
