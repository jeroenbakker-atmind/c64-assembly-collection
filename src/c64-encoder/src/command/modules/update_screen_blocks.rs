use c64_assembler::{
    builder::{ApplicationBuilder, FunctionBuilder, InstructionBuilder, ModuleBuilder},
    Module,
};

use crate::command::{update_screen_blocks::BlockKind, DecoderModule};

/// Zero-page scratch addresses used by the shared block decoder.
pub trait BlockScratchBuilder {
    fn add_block_scratch_addresses(&mut self) -> &mut Self;
}

impl BlockScratchBuilder for ApplicationBuilder {
    fn add_block_scratch_addresses(&mut self) -> &mut Self {
        self.define_address("BLOCK_OUTER_BITMAP_0", 0xA0)
            .define_address("BLOCK_OUTER_BITMAP_1", 0xA1)
            .define_address("BLOCK_OUTER_BITMAP_2", 0xA2)
            .define_address("BLOCK_INNER_BITMAP_0", 0xA3)
            .define_address("BLOCK_INNER_BITMAP_1", 0xA4)
            .define_address("BLOCK_INNER_BITMAP_2", 0xA5)
            .define_address("BLOCK_MASK_0", 0xA6)
            .define_address("BLOCK_MASK_1", 0xA7)
            .define_address("BLOCK_MASK_2", 0xA8)
            .define_address("BLOCK_BASE_PTR", 0xA9)
            .define_address("BLOCK_CELL_PTR", 0xAB)
            .define_address("BLOCK_OUTER_BITS", 0xBE)
            .define_address("BLOCK_INNER_BITS", 0xBF)
            .define_address("BLOCK_MASK_BITS", 0xC0)
            .define_address("BLOCK_OUTER_BIT_COUNT", 0xC1)
            .define_address("BLOCK_INNER_BIT_COUNT", 0xC2)
            .define_address("BLOCK_MASK_BIT_COUNT", 0xC3)
            .define_address("BLOCK_OUTER_INDEX", 0xAF)
            .define_address("BLOCK_INNER_INDEX", 0xB0)
            .define_address("BLOCK_CELL_INDEX", 0xB1)
            .define_address("BLOCK_OUTER_BYTES", 0xB2)
            .define_address("BLOCK_INNER_BYTES", 0xB3)
            .define_address("BLOCK_OUTER_BANDS", 0xB4)
            .define_address("BLOCK_INNER_BANDS", 0xB5)
            .define_address("BLOCK_MASK_BYTES", 0xB6)
            .define_address("BLOCK_CELL_COUNT", 0xB7)
            .define_address("BLOCK_CELL_MASK", 0xB8)
            .define_address("BLOCK_ROW_WRAP", 0xB9)
            .define_address("BLOCK_OUTER_STEP_LO", 0xBA)
            .define_address("BLOCK_OUTER_STEP_HI", 0xBB)
            .define_address("BLOCK_INNER_STEP_LO", 0xBC)
            .define_address("BLOCK_INNER_STEP_HI", 0xBD)
    }
}

/// Shared 6502 decoder for all ten block opcodes.
///
/// The per-opcode wrappers skip the opcode byte, load the geometry described by
/// [`BlockKind`] into the scratch variables defined by [`BlockScratchBuilder`],
/// and jump here.
pub struct UpdateScreenBlocksDecoder {}

impl DecoderModule for UpdateScreenBlocksDecoder {
    fn module() -> Module {
        ModuleBuilder::default()
            .name("update_screen_blocks")
            .function(
                FunctionBuilder::default()
                    .name("update_screen_blocks__read_byte")
                    .doc(&[
                        "Read one payload byte through CURRENT_PTR and advance the pointer.",
                        "Accumulator contains the byte on return; X is preserved.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .ldy_imm(0)
                            .lda_ind_y("CURRENT_PTR")
                            .pha()
                            .lda_imm(1)
                            .jsr_addr("engine__current_ptr__advance")
                            .pla()
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("update_screen_blocks__advance_base_ptr")
                    .instructions(
                        InstructionBuilder::default()
                            .clc()
                            .lda_addr("BLOCK_OUTER_STEP_LO")
                            .adc_addr("BLOCK_BASE_PTR")
                            .sta_addr("BLOCK_BASE_PTR")
                            .lda_addr("BLOCK_OUTER_STEP_HI")
                            .adc_addr_offs("BLOCK_BASE_PTR", 1)
                            .sta_addr_offs("BLOCK_BASE_PTR", 1)
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("update_screen_blocks__advance_inner_ptr")
                    .instructions(
                        InstructionBuilder::default()
                            .clc()
                            .lda_addr("BLOCK_INNER_STEP_LO")
                            .adc_addr("SCREEN_CHAR_PTR")
                            .sta_addr("SCREEN_CHAR_PTR")
                            .lda_addr("BLOCK_INNER_STEP_HI")
                            .adc_addr_offs("SCREEN_CHAR_PTR", 1)
                            .sta_addr_offs("SCREEN_CHAR_PTR", 1)
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("update_screen_blocks__advance_cell_ptr")
                    .instructions(
                        InstructionBuilder::default()
                            .lda_addr("BLOCK_CELL_PTR")
                            .and_addr("BLOCK_CELL_MASK")
                            .cmp_addr("BLOCK_CELL_MASK")
                            .bne_addr("update_screen_blocks__advance_cell_same_row")
                            .lda_addr("BLOCK_ROW_WRAP")
                            .jmp_addr("update_screen_blocks__advance_cell_add")
                            .label("update_screen_blocks__advance_cell_same_row")
                            .lda_imm(1)
                            .label("update_screen_blocks__advance_cell_add")
                            .clc()
                            .adc_addr("BLOCK_CELL_PTR")
                            .sta_addr("BLOCK_CELL_PTR")
                            .lda_imm(0)
                            .adc_addr_offs("BLOCK_CELL_PTR", 1)
                            .sta_addr_offs("BLOCK_CELL_PTR", 1)
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("update_screen_blocks__process_block")
                    .instructions(
                        InstructionBuilder::default()
                            .txa()
                            .pha()
                            .ldx_imm(0)
                            .label("update_screen_blocks__mask_read_next")
                            .cpx_addr("BLOCK_MASK_BYTES")
                            .beq_addr("update_screen_blocks__mask_scan_init")
                            .jsr_addr("update_screen_blocks__read_byte")
                            .sta_addr_x("BLOCK_MASK_0")
                            .inx()
                            .jmp_addr("update_screen_blocks__mask_read_next")
                            .label("update_screen_blocks__mask_scan_init")
                            .lda_addr("SCREEN_CHAR_PTR")
                            .sta_addr("BLOCK_CELL_PTR")
                            .lda_addr_offs("SCREEN_CHAR_PTR", 1)
                            .sta_addr_offs("BLOCK_CELL_PTR", 1)
                            .lda_imm(0)
                            .sta_addr("BLOCK_CELL_INDEX")
                            .ldx_imm(0)
                            .label("update_screen_blocks__mask_byte_next")
                            .cpx_addr("BLOCK_MASK_BYTES")
                            .beq_addr("update_screen_blocks__block_done")
                            .lda_addr_x("BLOCK_MASK_0")
                            .sta_addr("BLOCK_MASK_BITS")
                            .lda_imm(8)
                            .sta_addr("BLOCK_MASK_BIT_COUNT")
                            .label("update_screen_blocks__mask_bit_next")
                            .lsr_addr("BLOCK_MASK_BITS")
                            .bcc_addr("update_screen_blocks__cell_inactive")
                            .jsr_addr("update_screen_blocks__read_byte")
                            .ldy_imm(0)
                            .sta_ind_y("BLOCK_CELL_PTR")
                            .label("update_screen_blocks__cell_inactive")
                            .jsr_addr("update_screen_blocks__advance_cell_ptr")
                            .inc_addr("BLOCK_CELL_INDEX")
                            .lda_addr("BLOCK_CELL_INDEX")
                            .cmp_addr("BLOCK_CELL_COUNT")
                            .beq_addr("update_screen_blocks__block_done")
                            .dec_addr("BLOCK_MASK_BIT_COUNT")
                            .bne_addr("update_screen_blocks__mask_bit_next")
                            .inx()
                            .jmp_addr("update_screen_blocks__mask_byte_next")
                            .label("update_screen_blocks__block_done")
                            .pla()
                            .tax()
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("update_screen_blocks__process_outer_band")
                    .instructions(
                        InstructionBuilder::default()
                            .txa()
                            .pha()
                            .lda_addr("BLOCK_BASE_PTR")
                            .sta_addr("SCREEN_CHAR_PTR")
                            .lda_addr_offs("BLOCK_BASE_PTR", 1)
                            .sta_addr_offs("SCREEN_CHAR_PTR", 1)
                            .ldx_imm(0)
                            .label("update_screen_blocks__inner_read_next")
                            .cpx_addr("BLOCK_INNER_BYTES")
                            .beq_addr("update_screen_blocks__inner_scan_init")
                            .jsr_addr("update_screen_blocks__read_byte")
                            .sta_addr_x("BLOCK_INNER_BITMAP_0")
                            .inx()
                            .jmp_addr("update_screen_blocks__inner_read_next")
                            .label("update_screen_blocks__inner_scan_init")
                            .lda_imm(0)
                            .sta_addr("BLOCK_INNER_INDEX")
                            .ldx_imm(0)
                            .label("update_screen_blocks__inner_byte_next")
                            .cpx_addr("BLOCK_INNER_BYTES")
                            .beq_addr("update_screen_blocks__inner_done")
                            .lda_addr_x("BLOCK_INNER_BITMAP_0")
                            .sta_addr("BLOCK_INNER_BITS")
                            .lda_imm(8)
                            .sta_addr("BLOCK_INNER_BIT_COUNT")
                            .label("update_screen_blocks__inner_bit_next")
                            .lsr_addr("BLOCK_INNER_BITS")
                            .bcc_addr("update_screen_blocks__inner_bit_inactive")
                            .jsr_addr("update_screen_blocks__process_block")
                            .label("update_screen_blocks__inner_bit_inactive")
                            .jsr_addr("update_screen_blocks__advance_inner_ptr")
                            .inc_addr("BLOCK_INNER_INDEX")
                            .lda_addr("BLOCK_INNER_INDEX")
                            .cmp_addr("BLOCK_INNER_BANDS")
                            .beq_addr("update_screen_blocks__inner_done")
                            .dec_addr("BLOCK_INNER_BIT_COUNT")
                            .bne_addr("update_screen_blocks__inner_bit_next")
                            .inx()
                            .jmp_addr("update_screen_blocks__inner_byte_next")
                            .label("update_screen_blocks__inner_done")
                            .pla()
                            .tax()
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("update_screen_blocks__process")
                    .doc(&[
                        "Decode the current block payload.",
                        "The opcode byte must already have been skipped by the per-opcode wrapper.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_imm_low("SCREEN_CHARS_PAGE0")
                            .sta_addr("BLOCK_BASE_PTR")
                            .lda_imm_high("SCREEN_CHARS_PAGE0")
                            .sta_addr_offs("BLOCK_BASE_PTR", 1)
                            .ldx_imm(0)
                            .label("update_screen_blocks__outer_read_next")
                            .cpx_addr("BLOCK_OUTER_BYTES")
                            .beq_addr("update_screen_blocks__outer_scan_init")
                            .jsr_addr("update_screen_blocks__read_byte")
                            .sta_addr_x("BLOCK_OUTER_BITMAP_0")
                            .inx()
                            .jmp_addr("update_screen_blocks__outer_read_next")
                            .label("update_screen_blocks__outer_scan_init")
                            .lda_imm(0)
                            .sta_addr("BLOCK_OUTER_INDEX")
                            .ldx_imm(0)
                            .label("update_screen_blocks__outer_byte_next")
                            .cpx_addr("BLOCK_OUTER_BYTES")
                            .beq_addr("update_screen_blocks__done")
                            .lda_addr_x("BLOCK_OUTER_BITMAP_0")
                            .sta_addr("BLOCK_OUTER_BITS")
                            .lda_imm(8)
                            .sta_addr("BLOCK_OUTER_BIT_COUNT")
                            .label("update_screen_blocks__outer_bit_next")
                            .lsr_addr("BLOCK_OUTER_BITS")
                            .bcc_addr("update_screen_blocks__outer_bit_inactive")
                            .jsr_addr("update_screen_blocks__process_outer_band")
                            .label("update_screen_blocks__outer_bit_inactive")
                            .jsr_addr("update_screen_blocks__advance_base_ptr")
                            .inc_addr("BLOCK_OUTER_INDEX")
                            .lda_addr("BLOCK_OUTER_INDEX")
                            .cmp_addr("BLOCK_OUTER_BANDS")
                            .beq_addr("update_screen_blocks__done")
                            .dec_addr("BLOCK_OUTER_BIT_COUNT")
                            .bne_addr("update_screen_blocks__outer_bit_next")
                            .inx()
                            .jmp_addr("update_screen_blocks__outer_byte_next")
                            .label("update_screen_blocks__done")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .build()
    }
}

/// Build the thin per-opcode wrapper that configures the shared decoder for `kind`.
pub fn decoder_module(kind: BlockKind) -> Module {
    let name = format!("update_screen_blocks_{}", kind.name());
    let process_name = format!("{name}__process");
    let width = kind.width() as u8;
    let outer_step = if kind.column_first() {
        width as u16
    } else {
        (kind.height() * 40) as u16
    };
    let inner_step = if kind.column_first() {
        (kind.height() * 40) as u16
    } else {
        width as u16
    };

    ModuleBuilder::default()
        .name(name.as_str())
        .function(
            FunctionBuilder::default()
                .name(process_name.as_str())
                .doc(&["Configure the shared block decoder for this opcode and decode the payload."])
                .instructions(
                    InstructionBuilder::default()
                        .lda_imm(1)
                        .jsr_addr("engine__current_ptr__advance")
                        .comment("Skip the opcode byte.")
                        .lda_imm(kind.outer_bytes() as u8)
                        .sta_addr("BLOCK_OUTER_BYTES")
                        .lda_imm(kind.inner_bytes() as u8)
                        .sta_addr("BLOCK_INNER_BYTES")
                        .lda_imm(kind.outer_bands() as u8)
                        .sta_addr("BLOCK_OUTER_BANDS")
                        .lda_imm(kind.inner_bands() as u8)
                        .sta_addr("BLOCK_INNER_BANDS")
                        .lda_imm(kind.mask_bytes() as u8)
                        .sta_addr("BLOCK_MASK_BYTES")
                        .lda_imm(kind.cell_count() as u8)
                        .sta_addr("BLOCK_CELL_COUNT")
                        .lda_imm(width - 1)
                        .sta_addr("BLOCK_CELL_MASK")
                        .lda_imm(41 - width)
                        .sta_addr("BLOCK_ROW_WRAP")
                        .lda_imm(outer_step as u8)
                        .sta_addr("BLOCK_OUTER_STEP_LO")
                        .lda_imm((outer_step >> 8) as u8)
                        .sta_addr("BLOCK_OUTER_STEP_HI")
                        .lda_imm(inner_step as u8)
                        .sta_addr("BLOCK_INNER_STEP_LO")
                        .lda_imm((inner_step >> 8) as u8)
                        .sta_addr("BLOCK_INNER_STEP_HI")
                        .jmp_addr("update_screen_blocks__process")
                        .build(),
                )
                .build(),
        )
        .build()
}
