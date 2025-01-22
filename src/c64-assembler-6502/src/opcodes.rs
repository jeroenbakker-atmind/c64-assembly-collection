pub use gen::*;

/// An OpCode is the first byte that encodes the instruction and type of addressing mode.
pub type OpCode = u8;

const UNUSED: OpCode = 0xFF;
/// Special OpCode for instructions that don't have an op-code for implied addressing mode.
pub const NO_IMPLIED: OpCode = UNUSED;
/// Special OpCode for instructions that don't have an op-code for immediate addressing mode.
pub const NO_IMMEDIATE: OpCode = UNUSED;
/// Special OpCode for instructions that don't have an op-code for accumulator addressing mode.
pub const NO_ACCUMULATOR: OpCode = UNUSED;
/// Special OpCode for instructions that don't have an op-code for absolute addressing mode.
pub const NO_ABSOLUTE: OpCode = UNUSED;
/// Special OpCode for instructions that don't have an op-code for absolute-x addressing mode.
pub const NO_ABSOLUTE_X: OpCode = UNUSED;
/// Special OpCode for instructions that don't have an op-code for absolute-y addressing mode.
pub const NO_ABSOLUTE_Y: OpCode = UNUSED;
/// Special OpCode for instructions that don't have an op-code for zeropage addressing mode.
pub const NO_ZEROPAGE: OpCode = UNUSED;
/// Special OpCode for instructions that don't have an op-code for zeropage-x addressing mode.
pub const NO_ZEROPAGE_X: OpCode = UNUSED;
/// Special OpCode for instructions that don't have an op-code for zeropage-y addressing mode.
pub const NO_ZEROPAGE_Y: OpCode = UNUSED;
/// Special OpCode for instructions that don't have an op-code for indirect addressing mode.
pub const NO_INDIRECT: OpCode = UNUSED;
/// Special OpCode for instructions that don't have an op-code for indexed indirect addressing mode.
pub const NO_INDEXED_INDIRECT: OpCode = UNUSED;
/// Special OpCode for instructions that don't have an op-code for indirect indexed addressing mode.
pub const NO_INDIRECT_INDEXED: OpCode = UNUSED;
/// Special OpCode for instructions that don't have an op-code for relative addressing mode.
pub const NO_RELATIVE: OpCode = UNUSED;

mod gen {
    use super::OpCode;

    /// OpCode for the adc instruction in addressing mode immediate
    pub const ADC_IMMEDIATE: OpCode = 0x69;
    /// OpCode for the adc instruction in addressing mode absolute
    pub const ADC_ABSOLUTE: OpCode = 0x6d;
    /// OpCode for the adc instruction in addressing mode absolute_x
    pub const ADC_ABSOLUTE_X: OpCode = 0x7d;
    /// OpCode for the adc instruction in addressing mode absolute_y
    pub const ADC_ABSOLUTE_Y: OpCode = 0x79;
    /// OpCode for the adc instruction in addressing mode zeropage
    pub const ADC_ZEROPAGE: OpCode = 0x65;
    /// OpCode for the adc instruction in addressing mode zeropage_x
    pub const ADC_ZEROPAGE_X: OpCode = 0x75;
    /// OpCode for the adc instruction in addressing mode indexed_indirect
    pub const ADC_INDEXED_INDIRECT: OpCode = 0x61;
    /// OpCode for the adc instruction in addressing mode indirect_indexed
    pub const ADC_INDIRECT_INDEXED: OpCode = 0x71;
    /// OpCode for the and instruction in addressing mode immediate
    pub const AND_IMMEDIATE: OpCode = 0x29;
    /// OpCode for the and instruction in addressing mode absolute
    pub const AND_ABSOLUTE: OpCode = 0x2d;
    /// OpCode for the and instruction in addressing mode absolute_x
    pub const AND_ABSOLUTE_X: OpCode = 0x3d;
    /// OpCode for the and instruction in addressing mode absolute_y
    pub const AND_ABSOLUTE_Y: OpCode = 0x39;
    /// OpCode for the and instruction in addressing mode zeropage
    pub const AND_ZEROPAGE: OpCode = 0x25;
    /// OpCode for the and instruction in addressing mode zeropage_x
    pub const AND_ZEROPAGE_X: OpCode = 0x35;
    /// OpCode for the and instruction in addressing mode indexed_indirect
    pub const AND_INDEXED_INDIRECT: OpCode = 0x21;
    /// OpCode for the and instruction in addressing mode indirect_indexed
    pub const AND_INDIRECT_INDEXED: OpCode = 0x01;
    /// OpCode for the asl instruction in addressing mode accumulator
    pub const ASL_ACCUMULATOR: OpCode = 0x0a;
    /// OpCode for the asl instruction in addressing mode absolute
    pub const ASL_ABSOLUTE: OpCode = 0x0e;
    /// OpCode for the asl instruction in addressing mode absolute_x
    pub const ASL_ABSOLUTE_X: OpCode = 0x1e;
    /// OpCode for the asl instruction in addressing mode zeropage
    pub const ASL_ZEROPAGE: OpCode = 0x06;
    /// OpCode for the asl instruction in addressing mode zeropage_x
    pub const ASL_ZEROPAGE_X: OpCode = 0x16;
    /// OpCode for the bcc instruction in addressing mode relative
    pub const BCC_RELATIVE: OpCode = 0x90;
    /// OpCode for the bcs instruction in addressing mode relative
    pub const BCS_RELATIVE: OpCode = 0xb0;
    /// OpCode for the beq instruction in addressing mode relative
    pub const BEQ_RELATIVE: OpCode = 0xf0;
    /// OpCode for the bit instruction in addressing mode absolute
    pub const BIT_ABSOLUTE: OpCode = 0x2c;
    /// OpCode for the bit instruction in addressing mode zeropage
    pub const BIT_ZEROPAGE: OpCode = 0x24;
    /// OpCode for the bmi instruction in addressing mode relative
    pub const BMI_RELATIVE: OpCode = 0x30;
    /// OpCode for the bne instruction in addressing mode relative
    pub const BNE_RELATIVE: OpCode = 0xd0;
    /// OpCode for the bpl instruction in addressing mode relative
    pub const BPL_RELATIVE: OpCode = 0x10;
    /// OpCode for the brk instruction in addressing mode implied
    pub const BRK_IMPLIED: OpCode = 0x00;
    /// OpCode for the bvc instruction in addressing mode relative
    pub const BVC_RELATIVE: OpCode = 0x50;
    /// OpCode for the bvs instruction in addressing mode relative
    pub const BVS_RELATIVE: OpCode = 0x70;
    /// OpCode for the cld instruction in addressing mode implied
    pub const CLD_IMPLIED: OpCode = 0xd8;
    /// OpCode for the cli instruction in addressing mode implied
    pub const CLI_IMPLIED: OpCode = 0x58;
    /// OpCode for the clv instruction in addressing mode implied
    pub const CLV_IMPLIED: OpCode = 0xb8;
    /// OpCode for the cmp instruction in addressing mode immediate
    pub const CMP_IMMEDIATE: OpCode = 0xc9;
    /// OpCode for the cmp instruction in addressing mode absolute
    pub const CMP_ABSOLUTE: OpCode = 0xcd;
    /// OpCode for the cmp instruction in addressing mode absolute_x
    pub const CMP_ABSOLUTE_X: OpCode = 0xdd;
    /// OpCode for the cmp instruction in addressing mode absolute_y
    pub const CMP_ABSOLUTE_Y: OpCode = 0xd9;
    /// OpCode for the cmp instruction in addressing mode zeropage
    pub const CMP_ZEROPAGE: OpCode = 0xc5;
    /// OpCode for the cmp instruction in addressing mode zeropage_x
    pub const CMP_ZEROPAGE_X: OpCode = 0xd5;
    /// OpCode for the cmp instruction in addressing mode indexed_indirect
    pub const CMP_INDEXED_INDIRECT: OpCode = 0xc1;
    /// OpCode for the cmp instruction in addressing mode indirect_indexed
    pub const CMP_INDIRECT_INDEXED: OpCode = 0xd1;
    /// OpCode for the cpx instruction in addressing mode immediate
    pub const CPX_IMMEDIATE: OpCode = 0xe0;
    /// OpCode for the cpx instruction in addressing mode absolute
    pub const CPX_ABSOLUTE: OpCode = 0xec;
    /// OpCode for the cpx instruction in addressing mode zeropage
    pub const CPX_ZEROPAGE: OpCode = 0xe4;
    /// OpCode for the cpy instruction in addressing mode immediate
    pub const CPY_IMMEDIATE: OpCode = 0xc0;
    /// OpCode for the cpy instruction in addressing mode absolute
    pub const CPY_ABSOLUTE: OpCode = 0xcc;
    /// OpCode for the cpy instruction in addressing mode zeropage
    pub const CPY_ZEROPAGE: OpCode = 0xc4;
    /// OpCode for the dec instruction in addressing mode absolute
    pub const DEC_ABSOLUTE: OpCode = 0xce;
    /// OpCode for the dec instruction in addressing mode absolute_x
    pub const DEC_ABSOLUTE_X: OpCode = 0xde;
    /// OpCode for the dec instruction in addressing mode zeropage
    pub const DEC_ZEROPAGE: OpCode = 0xc6;
    /// OpCode for the dec instruction in addressing mode zeropage_x
    pub const DEC_ZEROPAGE_X: OpCode = 0xd6;
    /// OpCode for the dex instruction in addressing mode implied
    pub const DEX_IMPLIED: OpCode = 0xca;
    /// OpCode for the dey instruction in addressing mode implied
    pub const DEY_IMPLIED: OpCode = 0x88;
    /// OpCode for the eor instruction in addressing mode immediate
    pub const EOR_IMMEDIATE: OpCode = 0x49;
    /// OpCode for the eor instruction in addressing mode absolute
    pub const EOR_ABSOLUTE: OpCode = 0x4d;
    /// OpCode for the eor instruction in addressing mode absolute_x
    pub const EOR_ABSOLUTE_X: OpCode = 0x5d;
    /// OpCode for the eor instruction in addressing mode absolute_y
    pub const EOR_ABSOLUTE_Y: OpCode = 0x59;
    /// OpCode for the eor instruction in addressing mode zeropage
    pub const EOR_ZEROPAGE: OpCode = 0x45;
    /// OpCode for the eor instruction in addressing mode zeropage_x
    pub const EOR_ZEROPAGE_X: OpCode = 0x55;
    /// OpCode for the eor instruction in addressing mode indexed_indirect
    pub const EOR_INDEXED_INDIRECT: OpCode = 0x41;
    /// OpCode for the eor instruction in addressing mode indirect_indexed
    pub const EOR_INDIRECT_INDEXED: OpCode = 0x51;
    /// OpCode for the inc instruction in addressing mode absolute
    pub const INC_ABSOLUTE: OpCode = 0xee;
    /// OpCode for the inc instruction in addressing mode absolute_x
    pub const INC_ABSOLUTE_X: OpCode = 0xfe;
    /// OpCode for the inc instruction in addressing mode zeropage
    pub const INC_ZEROPAGE: OpCode = 0xe6;
    /// OpCode for the inc instruction in addressing mode zeropage_x
    pub const INC_ZEROPAGE_X: OpCode = 0xf6;
    /// OpCode for the inx instruction in addressing mode implied
    pub const INX_IMPLIED: OpCode = 0xe8;
    /// OpCode for the iny instruction in addressing mode implied
    pub const INY_IMPLIED: OpCode = 0xc8;
    /// OpCode for the ldx instruction in addressing mode immediate
    pub const LDX_IMMEDIATE: OpCode = 0xa2;
    /// OpCode for the ldx instruction in addressing mode absolute
    pub const LDX_ABSOLUTE: OpCode = 0xae;
    /// OpCode for the ldx instruction in addressing mode absolute_y
    pub const LDX_ABSOLUTE_Y: OpCode = 0xbe;
    /// OpCode for the ldx instruction in addressing mode zeropage
    pub const LDX_ZEROPAGE: OpCode = 0xa6;
    /// OpCode for the ldx instruction in addressing mode zeropage_y
    pub const LDX_ZEROPAGE_Y: OpCode = 0xb6;
    /// OpCode for the lsr instruction in addressing mode accumulator
    pub const LSR_ACCUMULATOR: OpCode = 0x4a;
    /// OpCode for the lsr instruction in addressing mode absolute
    pub const LSR_ABSOLUTE: OpCode = 0x4e;
    /// OpCode for the lsr instruction in addressing mode absolute_x
    pub const LSR_ABSOLUTE_X: OpCode = 0x5e;
    /// OpCode for the lsr instruction in addressing mode zeropage
    pub const LSR_ZEROPAGE: OpCode = 0x46;
    /// OpCode for the lsr instruction in addressing mode zeropage_x
    pub const LSR_ZEROPAGE_X: OpCode = 0x56;
    /// OpCode for the nop instruction in addressing mode implied
    pub const NOP_IMPLIED: OpCode = 0xea;
    /// OpCode for the ora instruction in addressing mode immediate
    pub const ORA_IMMEDIATE: OpCode = 0x09;
    /// OpCode for the ora instruction in addressing mode absolute
    pub const ORA_ABSOLUTE: OpCode = 0x0d;
    /// OpCode for the ora instruction in addressing mode absolute_x
    pub const ORA_ABSOLUTE_X: OpCode = 0x1d;
    /// OpCode for the ora instruction in addressing mode absolute_y
    pub const ORA_ABSOLUTE_Y: OpCode = 0x19;
    /// OpCode for the ora instruction in addressing mode zeropage
    pub const ORA_ZEROPAGE: OpCode = 0x05;
    /// OpCode for the ora instruction in addressing mode zeropage_x
    pub const ORA_ZEROPAGE_X: OpCode = 0x15;
    /// OpCode for the ora instruction in addressing mode indexed_indirect
    pub const ORA_INDEXED_INDIRECT: OpCode = 0x01;
    /// OpCode for the ora instruction in addressing mode indirect_indexed
    pub const ORA_INDIRECT_INDEXED: OpCode = 0x11;
    /// OpCode for the pha instruction in addressing mode implied
    pub const PHA_IMPLIED: OpCode = 0x48;
    /// OpCode for the php instruction in addressing mode implied
    pub const PHP_IMPLIED: OpCode = 0x08;
    /// OpCode for the pla instruction in addressing mode implied
    pub const PLA_IMPLIED: OpCode = 0x68;
    /// OpCode for the plp instruction in addressing mode implied
    pub const PLP_IMPLIED: OpCode = 0x28;
    /// OpCode for the rol instruction in addressing mode accumulator
    pub const ROL_ACCUMULATOR: OpCode = 0x2a;
    /// OpCode for the rol instruction in addressing mode absolute
    pub const ROL_ABSOLUTE: OpCode = 0x2e;
    /// OpCode for the rol instruction in addressing mode absolute_x
    pub const ROL_ABSOLUTE_X: OpCode = 0x3e;
    /// OpCode for the rol instruction in addressing mode zeropage
    pub const ROL_ZEROPAGE: OpCode = 0x26;
    /// OpCode for the rol instruction in addressing mode zeropage_x
    pub const ROL_ZEROPAGE_X: OpCode = 0x36;
    /// OpCode for the ror instruction in addressing mode accumulator
    pub const ROR_ACCUMULATOR: OpCode = 0x6a;
    /// OpCode for the ror instruction in addressing mode absolute
    pub const ROR_ABSOLUTE: OpCode = 0x6e;
    /// OpCode for the ror instruction in addressing mode absolute_x
    pub const ROR_ABSOLUTE_X: OpCode = 0x7e;
    /// OpCode for the ror instruction in addressing mode zeropage
    pub const ROR_ZEROPAGE: OpCode = 0x66;
    /// OpCode for the ror instruction in addressing mode zeropage_x
    pub const ROR_ZEROPAGE_X: OpCode = 0x76;
    /// OpCode for the rti instruction in addressing mode implied
    pub const RTI_IMPLIED: OpCode = 0x40;
    /// OpCode for the sbc instruction in addressing mode immediate
    pub const SBC_IMMEDIATE: OpCode = 0xe9;
    /// OpCode for the sbc instruction in addressing mode absolute
    pub const SBC_ABSOLUTE: OpCode = 0xed;
    /// OpCode for the sbc instruction in addressing mode absolute_x
    pub const SBC_ABSOLUTE_X: OpCode = 0xfd;
    /// OpCode for the sbc instruction in addressing mode absolute_y
    pub const SBC_ABSOLUTE_Y: OpCode = 0xf9;
    /// OpCode for the sbc instruction in addressing mode zeropage
    pub const SBC_ZEROPAGE: OpCode = 0xe5;
    /// OpCode for the sbc instruction in addressing mode zeropage_x
    pub const SBC_ZEROPAGE_X: OpCode = 0xf5;
    /// OpCode for the sbc instruction in addressing mode indexed_indirect
    pub const SBC_INDEXED_INDIRECT: OpCode = 0xe1;
    /// OpCode for the sbc instruction in addressing mode indirect_indexed
    pub const SBC_INDIRECT_INDEXED: OpCode = 0xf1;
    /// OpCode for the sed instruction in addressing mode implied
    pub const SED_IMPLIED: OpCode = 0xf8;
    /// OpCode for the sei instruction in addressing mode implied
    pub const SEI_IMPLIED: OpCode = 0x78;
    /// OpCode for the stx instruction in addressing mode absolute
    pub const STX_ABSOLUTE: OpCode = 0x8e;
    /// OpCode for the stx instruction in addressing mode zeropage
    pub const STX_ZEROPAGE: OpCode = 0x86;
    /// OpCode for the stx instruction in addressing mode zeropage_y
    pub const STX_ZEROPAGE_Y: OpCode = 0x96;
    /// OpCode for the sty instruction in addressing mode absolute
    pub const STY_ABSOLUTE: OpCode = 0x8c;
    /// OpCode for the sty instruction in addressing mode zeropage
    pub const STY_ZEROPAGE: OpCode = 0x84;
    /// OpCode for the sty instruction in addressing mode zeropage_x
    pub const STY_ZEROPAGE_X: OpCode = 0x94;
    /// OpCode for the tax instruction in addressing mode implied
    pub const TAX_IMPLIED: OpCode = 0xaa;
    /// OpCode for the tay instruction in addressing mode implied
    pub const TAY_IMPLIED: OpCode = 0xa8;
    /// OpCode for the tsx instruction in addressing mode implied
    pub const TSX_IMPLIED: OpCode = 0xba;
    /// OpCode for the txa instruction in addressing mode implied
    pub const TXA_IMPLIED: OpCode = 0x8a;
    /// OpCode for the txs instruction in addressing mode implied
    pub const TXS_IMPLIED: OpCode = 0x9a;
    /// OpCode for the tya instruction in addressing mode implied
    pub const TYA_IMPLIED: OpCode = 0x98;
    /// OpCode for the lda instruction in addressing mode immediate
    pub const LDA_IMMEDIATE: OpCode = 0xa9;
    /// OpCode for the lda instruction in addressing mode absolute
    pub const LDA_ABSOLUTE: OpCode = 0xad;
    /// OpCode for the lda instruction in addressing mode absolute_x
    pub const LDA_ABSOLUTE_X: OpCode = 0xbd;
    /// OpCode for the lda instruction in addressing mode absolute_y
    pub const LDA_ABSOLUTE_Y: OpCode = 0xb9;
    /// OpCode for the lda instruction in addressing mode zeropage
    pub const LDA_ZEROPAGE: OpCode = 0xa5;
    /// OpCode for the lda instruction in addressing mode zeropage_x
    pub const LDA_ZEROPAGE_X: OpCode = 0xb5;
    /// OpCode for the lda instruction in addressing mode indexed_indirect
    pub const LDA_INDEXED_INDIRECT: OpCode = 0xa1;
    /// OpCode for the lda instruction in addressing mode indirect_indexed
    pub const LDA_INDIRECT_INDEXED: OpCode = 0xb1;
    /// OpCode for the ldy instruction in addressing mode immediate
    pub const LDY_IMMEDIATE: OpCode = 0xa0;
    /// OpCode for the ldy instruction in addressing mode absolute
    pub const LDY_ABSOLUTE: OpCode = 0xac;
    /// OpCode for the ldy instruction in addressing mode absolute_x
    pub const LDY_ABSOLUTE_X: OpCode = 0xbc;
    /// OpCode for the ldy instruction in addressing mode zeropage
    pub const LDY_ZEROPAGE: OpCode = 0xa4;
    /// OpCode for the ldy instruction in addressing mode zeropage_x
    pub const LDY_ZEROPAGE_X: OpCode = 0xb4;
    /// OpCode for the sta instruction in addressing mode absolute
    pub const STA_ABSOLUTE: OpCode = 0x8d;
    /// OpCode for the sta instruction in addressing mode absolute_x
    pub const STA_ABSOLUTE_X: OpCode = 0x9d;
    /// OpCode for the sta instruction in addressing mode absolute_y
    pub const STA_ABSOLUTE_Y: OpCode = 0x99;
    /// OpCode for the sta instruction in addressing mode zeropage
    pub const STA_ZEROPAGE: OpCode = 0x85;
    /// OpCode for the sta instruction in addressing mode zeropage_x
    pub const STA_ZEROPAGE_X: OpCode = 0x95;
    /// OpCode for the sta instruction in addressing mode indexed_indirect
    pub const STA_INDEXED_INDIRECT: OpCode = 0x81;
    /// OpCode for the sta instruction in addressing mode indirect_indexed
    pub const STA_INDIRECT_INDEXED: OpCode = 0x91;
    /// OpCode for the jmp instruction in addressing mode absolute
    pub const JMP_ABSOLUTE: OpCode = 0x4c;
    /// OpCode for the jmp instruction in addressing mode indirect
    pub const JMP_INDIRECT: OpCode = 0x6c;
    /// OpCode for the jsr instruction in addressing mode absolute
    pub const JSR_ABSOLUTE: OpCode = 0x20;
    /// OpCode for the sec instruction in addressing mode implied
    pub const SEC_IMPLIED: OpCode = 0x38;
    /// OpCode for the clc instruction in addressing mode implied
    pub const CLC_IMPLIED: OpCode = 0x18;
    /// OpCode for the rts instruction in addressing mode implied
    pub const RTS_IMPLIED: OpCode = 0x60;
}
