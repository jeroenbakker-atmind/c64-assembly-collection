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
    use crate::instruction::InstructionDef;

    use super::{
        OpCode, NO_ABSOLUTE, NO_ABSOLUTE_X, NO_ABSOLUTE_Y, NO_ACCUMULATOR, NO_IMMEDIATE, NO_IMPLIED,
        NO_INDEXED_INDIRECT, NO_INDIRECT, NO_INDIRECT_INDEXED, NO_RELATIVE, NO_ZEROPAGE, NO_ZEROPAGE_X, NO_ZEROPAGE_Y,
    };

    const ADC_IMMEDIATE: OpCode = 0x69;
    const ADC_ABSOLUTE: OpCode = 0x6d;
    const ADC_ABSOLUTE_X: OpCode = 0x7d;
    const ADC_ABSOLUTE_Y: OpCode = 0x79;
    const ADC_ZEROPAGE: OpCode = 0x65;
    const ADC_ZEROPAGE_X: OpCode = 0x75;
    const ADC_INDEXED_INDIRECT: OpCode = 0x61;
    const ADC_INDIRECT_INDEXED: OpCode = 0x71;
    const AND_IMMEDIATE: OpCode = 0x29;
    const AND_ABSOLUTE: OpCode = 0x2d;
    const AND_ABSOLUTE_X: OpCode = 0x3d;
    const AND_ABSOLUTE_Y: OpCode = 0x39;
    const AND_ZEROPAGE: OpCode = 0x25;
    const AND_ZEROPAGE_X: OpCode = 0x35;
    const AND_INDEXED_INDIRECT: OpCode = 0x21;
    const AND_INDIRECT_INDEXED: OpCode = 0x01;
    const ASL_ACCUMULATOR: OpCode = 0x0a;
    const ASL_ABSOLUTE: OpCode = 0x0e;
    const ASL_ABSOLUTE_X: OpCode = 0x1e;
    const ASL_ZEROPAGE: OpCode = 0x06;
    const ASL_ZEROPAGE_X: OpCode = 0x16;
    const BCC_RELATIVE: OpCode = 0x90;
    const BCS_RELATIVE: OpCode = 0xb0;
    const BEQ_RELATIVE: OpCode = 0xf0;
    const BIT_ABSOLUTE: OpCode = 0x2c;
    const BIT_ZEROPAGE: OpCode = 0x24;
    const BMI_RELATIVE: OpCode = 0x30;
    const BNE_RELATIVE: OpCode = 0xd0;
    const BPL_RELATIVE: OpCode = 0x10;
    const BRK_IMPLIED: OpCode = 0x00;
    const BVC_RELATIVE: OpCode = 0x50;
    const BVS_RELATIVE: OpCode = 0x70;
    const CLC_IMPLIED: OpCode = 0x18;
    const CLD_IMPLIED: OpCode = 0xd8;
    const CLI_IMPLIED: OpCode = 0x58;
    const CLV_IMPLIED: OpCode = 0xb8;
    const CMP_IMMEDIATE: OpCode = 0xc9;
    const CMP_ABSOLUTE: OpCode = 0xcd;
    const CMP_ABSOLUTE_X: OpCode = 0xdd;
    const CMP_ABSOLUTE_Y: OpCode = 0xd9;
    const CMP_ZEROPAGE: OpCode = 0xc5;
    const CMP_ZEROPAGE_X: OpCode = 0xd5;
    const CMP_INDEXED_INDIRECT: OpCode = 0xc1;
    const CMP_INDIRECT_INDEXED: OpCode = 0xd1;
    const CPX_IMMEDIATE: OpCode = 0xe0;
    const CPX_ABSOLUTE: OpCode = 0xec;
    const CPX_ZEROPAGE: OpCode = 0xe4;
    const CPY_IMMEDIATE: OpCode = 0xc0;
    const CPY_ABSOLUTE: OpCode = 0xcc;
    const CPY_ZEROPAGE: OpCode = 0xc4;
    const DEC_ABSOLUTE: OpCode = 0xce;
    const DEC_ABSOLUTE_X: OpCode = 0xde;
    const DEC_ZEROPAGE: OpCode = 0xc6;
    const DEC_ZEROPAGE_X: OpCode = 0xd6;
    const DEX_IMPLIED: OpCode = 0xca;
    const DEY_IMPLIED: OpCode = 0x88;
    const EOR_IMMEDIATE: OpCode = 0x49;
    const EOR_ABSOLUTE: OpCode = 0x4d;
    const EOR_ABSOLUTE_X: OpCode = 0x5d;
    const EOR_ABSOLUTE_Y: OpCode = 0x59;
    const EOR_ZEROPAGE: OpCode = 0x45;
    const EOR_ZEROPAGE_X: OpCode = 0x55;
    const EOR_INDEXED_INDIRECT: OpCode = 0x41;
    const EOR_INDIRECT_INDEXED: OpCode = 0x51;
    const INC_ABSOLUTE: OpCode = 0xee;
    const INC_ABSOLUTE_X: OpCode = 0xfe;
    const INC_ZEROPAGE: OpCode = 0xe6;
    const INC_ZEROPAGE_X: OpCode = 0xf6;
    const INX_IMPLIED: OpCode = 0xe8;
    const INY_IMPLIED: OpCode = 0xc8;
    const JMP_ABSOLUTE: OpCode = 0x4c;
    const JMP_INDIRECT: OpCode = 0x6c;
    const JSR_ABSOLUTE: OpCode = 0x20;
    const LDA_IMMEDIATE: OpCode = 0xa9;
    const LDA_ABSOLUTE: OpCode = 0xad;
    const LDA_ABSOLUTE_X: OpCode = 0xbd;
    const LDA_ABSOLUTE_Y: OpCode = 0xb9;
    const LDA_ZEROPAGE: OpCode = 0xa5;
    const LDA_ZEROPAGE_X: OpCode = 0xb5;
    const LDA_INDEXED_INDIRECT: OpCode = 0xa1;
    const LDA_INDIRECT_INDEXED: OpCode = 0xb1;
    const LDX_IMMEDIATE: OpCode = 0xa2;
    const LDX_ABSOLUTE: OpCode = 0xae;
    const LDX_ABSOLUTE_Y: OpCode = 0xbe;
    const LDX_ZEROPAGE: OpCode = 0xa6;
    const LDX_ZEROPAGE_Y: OpCode = 0xb6;
    const LDY_IMMEDIATE: OpCode = 0xa0;
    const LDY_ABSOLUTE: OpCode = 0xac;
    const LDY_ABSOLUTE_X: OpCode = 0xbc;
    const LDY_ZEROPAGE: OpCode = 0xa4;
    const LDY_ZEROPAGE_X: OpCode = 0xb4;
    const LSR_ACCUMULATOR: OpCode = 0x4a;
    const LSR_ABSOLUTE: OpCode = 0x4e;
    const LSR_ABSOLUTE_X: OpCode = 0x5e;
    const LSR_ZEROPAGE: OpCode = 0x46;
    const LSR_ZEROPAGE_X: OpCode = 0x56;
    const NOP_IMPLIED: OpCode = 0xea;
    const ORA_IMMEDIATE: OpCode = 0x09;
    const ORA_ABSOLUTE: OpCode = 0x0d;
    const ORA_ABSOLUTE_X: OpCode = 0x1d;
    const ORA_ABSOLUTE_Y: OpCode = 0x19;
    const ORA_ZEROPAGE: OpCode = 0x05;
    const ORA_ZEROPAGE_X: OpCode = 0x15;
    const ORA_INDEXED_INDIRECT: OpCode = 0x01;
    const ORA_INDIRECT_INDEXED: OpCode = 0x11;
    const PHA_IMPLIED: OpCode = 0x48;
    const PHP_IMPLIED: OpCode = 0x08;
    const PLA_IMPLIED: OpCode = 0x68;
    const PLP_IMPLIED: OpCode = 0x28;
    const ROL_ACCUMULATOR: OpCode = 0x2a;
    const ROL_ABSOLUTE: OpCode = 0x2e;
    const ROL_ABSOLUTE_X: OpCode = 0x3e;
    const ROL_ZEROPAGE: OpCode = 0x26;
    const ROL_ZEROPAGE_X: OpCode = 0x36;
    const ROR_ACCUMULATOR: OpCode = 0x6a;
    const ROR_ABSOLUTE: OpCode = 0x6e;
    const ROR_ABSOLUTE_X: OpCode = 0x7e;
    const ROR_ZEROPAGE: OpCode = 0x66;
    const ROR_ZEROPAGE_X: OpCode = 0x76;
    const RTI_IMPLIED: OpCode = 0x40;
    const RTS_IMPLIED: OpCode = 0x60;
    const SBC_IMMEDIATE: OpCode = 0xe9;
    const SBC_ABSOLUTE: OpCode = 0xed;
    const SBC_ABSOLUTE_X: OpCode = 0xfd;
    const SBC_ABSOLUTE_Y: OpCode = 0xf9;
    const SBC_ZEROPAGE: OpCode = 0xe5;
    const SBC_ZEROPAGE_X: OpCode = 0xf5;
    const SBC_INDEXED_INDIRECT: OpCode = 0xe1;
    const SBC_INDIRECT_INDEXED: OpCode = 0xf1;
    const SEC_IMPLIED: OpCode = 0x38;
    const SED_IMPLIED: OpCode = 0xf8;
    const SEI_IMPLIED: OpCode = 0x78;
    const STA_ABSOLUTE: OpCode = 0x8d;
    const STA_ABSOLUTE_X: OpCode = 0x9d;
    const STA_ABSOLUTE_Y: OpCode = 0x99;
    const STA_ZEROPAGE: OpCode = 0x85;
    const STA_ZEROPAGE_X: OpCode = 0x95;
    const STA_INDEXED_INDIRECT: OpCode = 0x81;
    const STA_INDIRECT_INDEXED: OpCode = 0x91;
    const STX_ABSOLUTE: OpCode = 0x8e;
    const STX_ZEROPAGE: OpCode = 0x86;
    const STX_ZEROPAGE_Y: OpCode = 0x96;
    const STY_ABSOLUTE: OpCode = 0x8c;
    const STY_ZEROPAGE: OpCode = 0x84;
    const STY_ZEROPAGE_X: OpCode = 0x94;
    const TAX_IMPLIED: OpCode = 0xaa;
    const TAY_IMPLIED: OpCode = 0xa8;
    const TSX_IMPLIED: OpCode = 0xba;
    const TXA_IMPLIED: OpCode = 0x8a;
    const TXS_IMPLIED: OpCode = 0x9a;
    const TYA_IMPLIED: OpCode = 0x98;
    pub const OPCODES_ADC: InstructionDef = InstructionDef {
        instruction: "adc",
        implied: NO_IMPLIED,
        immediate: ADC_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: ADC_ABSOLUTE,
        absolute_x: ADC_ABSOLUTE_X,
        absolute_y: ADC_ABSOLUTE_Y,
        zeropage: ADC_ZEROPAGE,
        zeropage_x: ADC_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: ADC_INDEXED_INDIRECT,
        indirect_indexed: ADC_INDIRECT_INDEXED,
    };
    pub const OPCODES_AND: InstructionDef = InstructionDef {
        instruction: "and",
        implied: NO_IMPLIED,
        immediate: AND_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: AND_ABSOLUTE,
        absolute_x: AND_ABSOLUTE_X,
        absolute_y: AND_ABSOLUTE_Y,
        zeropage: AND_ZEROPAGE,
        zeropage_x: AND_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: AND_INDEXED_INDIRECT,
        indirect_indexed: AND_INDIRECT_INDEXED,
    };
    pub const OPCODES_ASL: InstructionDef = InstructionDef {
        instruction: "asl",
        implied: NO_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: ASL_ACCUMULATOR,
        absolute: ASL_ABSOLUTE,
        absolute_x: ASL_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: ASL_ZEROPAGE,
        zeropage_x: ASL_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_BCC: InstructionDef = InstructionDef {
        instruction: "bcc",
        implied: NO_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: BCC_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_BCS: InstructionDef = InstructionDef {
        instruction: "bcs",
        implied: NO_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: BCS_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_BEQ: InstructionDef = InstructionDef {
        instruction: "beq",
        implied: NO_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: BEQ_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_BIT: InstructionDef = InstructionDef {
        instruction: "bit",
        implied: NO_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: BIT_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: BIT_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_BMI: InstructionDef = InstructionDef {
        instruction: "bmi",
        implied: NO_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: BMI_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_BNE: InstructionDef = InstructionDef {
        instruction: "bne",
        implied: NO_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: BNE_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_BPL: InstructionDef = InstructionDef {
        instruction: "bpl",
        implied: NO_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: BPL_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_BRK: InstructionDef = InstructionDef {
        instruction: "brk",
        implied: BRK_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_BVC: InstructionDef = InstructionDef {
        instruction: "bvc",
        implied: NO_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: BVC_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_BVS: InstructionDef = InstructionDef {
        instruction: "bvs",
        implied: NO_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: BVS_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_CLC: InstructionDef = InstructionDef {
        instruction: "clc",
        implied: CLC_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_CLD: InstructionDef = InstructionDef {
        instruction: "cld",
        implied: CLD_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_CLI: InstructionDef = InstructionDef {
        instruction: "cli",
        implied: CLI_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_CLV: InstructionDef = InstructionDef {
        instruction: "clv",
        implied: CLV_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_CMP: InstructionDef = InstructionDef {
        instruction: "cmp",
        implied: NO_IMPLIED,
        immediate: CMP_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: CMP_ABSOLUTE,
        absolute_x: CMP_ABSOLUTE_X,
        absolute_y: CMP_ABSOLUTE_Y,
        zeropage: CMP_ZEROPAGE,
        zeropage_x: CMP_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: CMP_INDEXED_INDIRECT,
        indirect_indexed: CMP_INDIRECT_INDEXED,
    };
    pub const OPCODES_CPX: InstructionDef = InstructionDef {
        instruction: "cpx",
        implied: NO_IMPLIED,
        immediate: CPX_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: CPX_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: CPX_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_CPY: InstructionDef = InstructionDef {
        instruction: "cpy",
        implied: NO_IMPLIED,
        immediate: CPY_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: CPY_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: CPY_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_DEC: InstructionDef = InstructionDef {
        instruction: "dec",
        implied: NO_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: DEC_ABSOLUTE,
        absolute_x: DEC_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: DEC_ZEROPAGE,
        zeropage_x: DEC_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_DEX: InstructionDef = InstructionDef {
        instruction: "dex",
        implied: DEX_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_DEY: InstructionDef = InstructionDef {
        instruction: "dey",
        implied: DEY_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_EOR: InstructionDef = InstructionDef {
        instruction: "eor",
        implied: NO_IMPLIED,
        immediate: EOR_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: EOR_ABSOLUTE,
        absolute_x: EOR_ABSOLUTE_X,
        absolute_y: EOR_ABSOLUTE_Y,
        zeropage: EOR_ZEROPAGE,
        zeropage_x: EOR_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: EOR_INDEXED_INDIRECT,
        indirect_indexed: EOR_INDIRECT_INDEXED,
    };
    pub const OPCODES_INC: InstructionDef = InstructionDef {
        instruction: "inc",
        implied: NO_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: INC_ABSOLUTE,
        absolute_x: INC_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: INC_ZEROPAGE,
        zeropage_x: INC_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_INX: InstructionDef = InstructionDef {
        instruction: "inx",
        implied: INX_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_INY: InstructionDef = InstructionDef {
        instruction: "iny",
        implied: INY_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_JMP: InstructionDef = InstructionDef {
        instruction: "jmp",
        implied: NO_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: JMP_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: JMP_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_JSR: InstructionDef = InstructionDef {
        instruction: "jsr",
        implied: NO_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: JSR_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_LDA: InstructionDef = InstructionDef {
        instruction: "lda",
        implied: NO_IMPLIED,
        immediate: LDA_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: LDA_ABSOLUTE,
        absolute_x: LDA_ABSOLUTE_X,
        absolute_y: LDA_ABSOLUTE_Y,
        zeropage: LDA_ZEROPAGE,
        zeropage_x: LDA_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: LDA_INDEXED_INDIRECT,
        indirect_indexed: LDA_INDIRECT_INDEXED,
    };
    pub const OPCODES_LDX: InstructionDef = InstructionDef {
        instruction: "ldx",
        implied: NO_IMPLIED,
        immediate: LDX_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: LDX_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: LDX_ABSOLUTE_Y,
        zeropage: LDX_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: LDX_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_LDY: InstructionDef = InstructionDef {
        instruction: "ldy",
        implied: NO_IMPLIED,
        immediate: LDY_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: LDY_ABSOLUTE,
        absolute_x: LDY_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: LDY_ZEROPAGE,
        zeropage_x: LDY_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_LSR: InstructionDef = InstructionDef {
        instruction: "lsr",
        implied: NO_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: LSR_ACCUMULATOR,
        absolute: LSR_ABSOLUTE,
        absolute_x: LSR_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: LSR_ZEROPAGE,
        zeropage_x: LSR_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_NOP: InstructionDef = InstructionDef {
        instruction: "nop",
        implied: NOP_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_ORA: InstructionDef = InstructionDef {
        instruction: "ora",
        implied: NO_IMPLIED,
        immediate: ORA_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: ORA_ABSOLUTE,
        absolute_x: ORA_ABSOLUTE_X,
        absolute_y: ORA_ABSOLUTE_Y,
        zeropage: ORA_ZEROPAGE,
        zeropage_x: ORA_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: ORA_INDEXED_INDIRECT,
        indirect_indexed: ORA_INDIRECT_INDEXED,
    };
    pub const OPCODES_PHA: InstructionDef = InstructionDef {
        instruction: "pha",
        implied: PHA_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_PHP: InstructionDef = InstructionDef {
        instruction: "php",
        implied: PHP_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_PLA: InstructionDef = InstructionDef {
        instruction: "pla",
        implied: PLA_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_PLP: InstructionDef = InstructionDef {
        instruction: "plp",
        implied: PLP_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_ROL: InstructionDef = InstructionDef {
        instruction: "rol",
        implied: NO_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: ROL_ACCUMULATOR,
        absolute: ROL_ABSOLUTE,
        absolute_x: ROL_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: ROL_ZEROPAGE,
        zeropage_x: ROL_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_ROR: InstructionDef = InstructionDef {
        instruction: "ror",
        implied: NO_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: ROR_ACCUMULATOR,
        absolute: ROR_ABSOLUTE,
        absolute_x: ROR_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: ROR_ZEROPAGE,
        zeropage_x: ROR_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_RTI: InstructionDef = InstructionDef {
        instruction: "rti",
        implied: RTI_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_RTS: InstructionDef = InstructionDef {
        instruction: "rts",
        implied: RTS_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_SBC: InstructionDef = InstructionDef {
        instruction: "sbc",
        implied: NO_IMPLIED,
        immediate: SBC_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: SBC_ABSOLUTE,
        absolute_x: SBC_ABSOLUTE_X,
        absolute_y: SBC_ABSOLUTE_Y,
        zeropage: SBC_ZEROPAGE,
        zeropage_x: SBC_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: SBC_INDEXED_INDIRECT,
        indirect_indexed: SBC_INDIRECT_INDEXED,
    };
    pub const OPCODES_SEC: InstructionDef = InstructionDef {
        instruction: "sec",
        implied: SEC_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_SED: InstructionDef = InstructionDef {
        instruction: "sed",
        implied: SED_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_SEI: InstructionDef = InstructionDef {
        instruction: "sei",
        implied: SEI_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_STA: InstructionDef = InstructionDef {
        instruction: "sta",
        implied: NO_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: STA_ABSOLUTE,
        absolute_x: STA_ABSOLUTE_X,
        absolute_y: STA_ABSOLUTE_Y,
        zeropage: STA_ZEROPAGE,
        zeropage_x: STA_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: STA_INDEXED_INDIRECT,
        indirect_indexed: STA_INDIRECT_INDEXED,
    };
    pub const OPCODES_STX: InstructionDef = InstructionDef {
        instruction: "stx",
        implied: NO_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: STX_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: STX_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: STX_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_STY: InstructionDef = InstructionDef {
        instruction: "sty",
        implied: NO_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: STY_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: STY_ZEROPAGE,
        zeropage_x: STY_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_TAX: InstructionDef = InstructionDef {
        instruction: "tax",
        implied: TAX_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_TAY: InstructionDef = InstructionDef {
        instruction: "tay",
        implied: TAY_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_TSX: InstructionDef = InstructionDef {
        instruction: "tsx",
        implied: TSX_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_TXA: InstructionDef = InstructionDef {
        instruction: "txa",
        implied: TXA_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_TXS: InstructionDef = InstructionDef {
        instruction: "txs",
        implied: TXS_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
    pub const OPCODES_TYA: InstructionDef = InstructionDef {
        instruction: "tya",
        implied: TYA_IMPLIED,
        immediate: NO_IMMEDIATE,
        accumulator: NO_ACCUMULATOR,
        absolute: NO_ABSOLUTE,
        absolute_x: NO_ABSOLUTE_X,
        absolute_y: NO_ABSOLUTE_Y,
        zeropage: NO_ZEROPAGE,
        zeropage_x: NO_ZEROPAGE_X,
        zeropage_y: NO_ZEROPAGE_Y,
        relative: NO_RELATIVE,
        indirect: NO_INDIRECT,
        indexed_indirect: NO_INDEXED_INDIRECT,
        indirect_indexed: NO_INDIRECT_INDEXED,
    };
}
