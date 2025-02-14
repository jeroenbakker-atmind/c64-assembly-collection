use c64_assembler::builder::{FunctionBuilder, InstructionBuilder, ModuleBuilder};
use c64_colors::colors::Color;

use crate::encoder::{writer::Writer, Encoder};

use super::{modules::CurrentPtrMacros, DecoderModule};

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
                            .lda_current_ptr_offs(1, "Load border color into the accumulator.")
                            .sta_addr("VIC2_BORDER_COLOR")
                            .inc_current_ptr(2)
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .build()
    }
}
