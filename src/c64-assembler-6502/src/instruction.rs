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
