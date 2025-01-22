use crate::opcodes::OpCode;

/// Instruction definition.
///
/// Contains the instruction (as str) and the op-codes per addressing mode.
#[derive(Debug)]
pub struct InstructionDef {
    /// Instruction as lowercase str (lda, sta, ...)
    pub instruction: &'static str,
    /// OpCode for implied addressing mode.
    ///
    /// Contains [crate::opcodes::NO_IMPLIED] when no op-code exist.
    pub implied: OpCode,
    /// OpCode for immediate addressing mode.
    ///
    /// Contains [crate::opcodes::NO_IMMEDIATE] when no op-code exists.
    pub immediate: OpCode,
    /// OpCode for accumualtor addressing mode.
    ///
    /// Contains [crate::opcodes::NO_ACCUMULATOR] when no op-code exists.
    pub accumulator: OpCode,
    /// OpCode for absolute addressing mode.
    ///
    /// Contains [crate::opcodes::NO_ABSOLUTE] when no op-code exists.
    pub absolute: OpCode,
    /// OpCode for absolute-x addressing mode.
    ///
    /// Contains [crate::opcodes::NO_ABSOLUTE_X] when no op-code exists.
    pub absolute_x: OpCode,
    /// OpCode for absolute-y addressing mode.
    ///
    /// Contains [crate::opcodes::NO_ABSOLUTE_Y] when no op-code exists.
    pub absolute_y: OpCode,
    /// OpCode for zeropage addressing mode.
    ///
    /// Contains [crate::opcodes::NO_ZEROPAGE] when no op-code exists.
    pub zeropage: OpCode,
    /// OpCode for zeropage-x addressing mode.
    ///
    /// Contains [crate::opcodes::NO_ZEROPAGE_X] when no op-code exists.
    pub zeropage_x: OpCode,
    /// OpCode for zeropage-y addressing mode.
    ///
    /// Contains [crate::opcodes::NO_ZEROPAGE_Y] when no op-code exists.
    pub zeropage_y: OpCode,
    /// OpCode for relative addressing mode.
    ///
    /// Contains [crate::opcodes::NO_RELATIVE] when no op-code exists.
    pub relative: OpCode,
    /// OpCode for indirect addressing mode.
    ///
    /// Contains [crate::opcodes::NO_INDIRECT] when no op-code exists.
    pub indirect: OpCode,
    /// OpCode for indexed indirect addressing mode.
    ///
    /// Contains [crate::opcodes::NO_INDEXED_INDIRECT] when no op-code exists.
    pub indexed_indirect: OpCode,
    /// OpCode for indirect indexed addressing mode.
    ///
    /// Contains [crate::opcodes::NO_INDIRECT_INDEXED] when no op-code exists.
    pub indirect_indexed: OpCode,
}

pub use gen::*;

mod gen {
    use crate::opcodes::*;

    use super::InstructionDef;

    /// Instruction definition for adc
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for and
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for asl
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for bcc
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for bcs
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for beq
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for bit
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for bmi
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for bne
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for bpl
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for brk
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for bvc
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for bvs
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for cld
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for cli
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for clv
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for cmp
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for cpx
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for cpy
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for dec
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for dex
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for dey
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for eor
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for inc
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for inx
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for iny
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for ldx
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for lsr
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for nop
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for ora
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for pha
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for php
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for pla
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for plp
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for rol
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for ror
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for rti
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for sbc
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for sed
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for sei
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for stx
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for sty
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for tax
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for tay
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for tsx
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for txa
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for txs
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for tya
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for lda
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for ldy
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for sta
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for jmp
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for jsr
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for sec
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for clc
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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

    /// Instruction definition for rts
    ///
    /// Includes the instruction name and its [OpCode] for a address mode.
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
}
