use c64_assembler::builder::{FunctionBuilder, InstructionBuilder, ModuleBuilder};
use c64_colors::colors::Color;

use crate::encoder::{writer::Writer, Encoder};

use super::{modules::CurrentPtrMacros, DecoderModule};

#[derive(Copy, Clone, Debug)]
pub struct SetPalette4 {
    pub palette: [Color; 4],
}

impl Encoder for SetPalette4 {
    fn byte_size(&self) -> usize {
        size_of::<u8>() * 2
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        let b1 = u8::from(self.palette[0]) + u8::from(self.palette[1]) * 16_u8;
        let b2 = u8::from(self.palette[2]) + u8::from(self.palette[3]) * 16_u8;
        encoded_data.add(&b1).add(&b2)
    }
}

impl DecoderModule for SetPalette4 {
    fn module() -> c64_assembler::Module {
        ModuleBuilder::default()
            .name("set_palette4")
            .function(
                FunctionBuilder::default()
                    .name("set_palette4__process")
                    .instructions(
                        InstructionBuilder::default()
                            .lda_current_ptr_offs(1, "Load back/foreground color into the accumulator.")
                            .sta_addr("VIC2_BACKGROUND_COLOR")
                            .sta_addr("VIC2_BORDER_COLOR")
                            .lsr_acc()
                            .lsr_acc()
                            .lsr_acc()
                            .lsr_acc()
                            .ldx_imm(0x00)
                            .label("set_palette4__next")
                            .sta_addr_x("SCREEN_COLORS_PAGE0")
                            .sta_addr_x("SCREEN_COLORS_PAGE1")
                            .sta_addr_x("SCREEN_COLORS_PAGE2")
                            .sta_addr_x("SCREEN_COLORS_PAGE3")
                            .inx()
                            .bne_addr("set_palette4__next")
                            .inc_current_ptr(3)
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .build()
    }
}
