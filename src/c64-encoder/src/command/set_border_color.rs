use c64_assembler::builder::{FunctionBuilder, InstructionBuilder, ModuleBuilder};
use c64_colors::colors::Color;

use crate::encoder::{writer::Writer, Encoder};

use super::DecoderModule;

#[derive(Copy, Clone, Debug)]
pub struct SetBorderColor {
    pub color: Color,
}

impl Encoder for SetBorderColor {
    fn byte_size(&self) -> usize {
        size_of::<u8>()
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        encoded_data.add(&u8::from(self.color))
    }
}

impl DecoderModule for SetBorderColor {
    fn module() -> c64_assembler::Module {
        ModuleBuilder::default()
            .name("set_border_color")
            .function(
                FunctionBuilder::default()
                    .name("set_border_color__process")
                    .instructions(
                        InstructionBuilder::default()
                            .ldy_imm(1)
                            .lda_ind_y("CURRENT_PTR")
                            .sta_addr("VIC20_BORDER_COLOR")
                            .lda_imm(2)
                            .jsr_addr("engine__current_ptr__advance")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .build()
    }
}
