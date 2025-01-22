//! Crate containing definitions of the 6502 instruction set.
//!
//! # Table
//!
//! Table with all the instructions and op-codes per addressing mode.
//!
//! | **Instruction** | **Implied** | **Immediate** | **Accumulator** | **Absolute** | **Absolute,X** | **Absolute,Y** | **Zero Page** | **Zero Page,X** | **Zero Page,Y** | **Relative** | **Indirect** | **Indirect,X** | **Indirect,Y** |
//! | --------------- | ----------- | ------------- | --------------- | ------------ | -------------- | -------------- | ------------- | --------------- | --------------- | ------------ | ------------ | -------------- | -------------- |
//! | ADC             |             | 0x69          |                 | 0x6D         | 0x7D           | 0x79           | 0x65          | 0x75            |                 |              |              | 0x61           | 0x71           |
//! | AND             |             | 0x29          |                 | 0x2D         | 0x3D           | 0x39           | 0x25          | 0x35            |                 |              |              | 0x21           | 0x01           |
//! | ASL             |             |               | 0x0A            | 0x0E         | 0x1E           |                | 0x06          | 0x16            |                 |              |              |                |                |
//! | BCC             |             |               |                 |              |                |                |               |                 |                 | 0x90         |              |                |                |
//! | BCS             |             |               |                 |              |                |                |               |                 |                 | 0xB0         |              |                |                |
//! | BEQ             |             |               |                 |              |                |                |               |                 |                 | 0xF0         |              |                |                |
//! | BIT             |             |               |                 | 0x2C         |                |                | 0x24          |                 |                 |              |              |                |                |
//! | BMI             |             |               |                 |              |                |                |               |                 |                 | 0x30         |              |                |                |
//! | BNE             |             |               |                 |              |                |                |               |                 |                 | 0xD0         |              |                |                |
//! | BPL             |             |               |                 |              |                |                |               |                 |                 | 0x10         |              |                |                |
//! | BRK             | 0x00        |               |                 |              |                |                |               |                 |                 |              |              |                |                |
//! | BVC             |             |               |                 |              |                |                |               |                 |                 | 0x50         |              |                |                |
//! | BVS             |             |               |                 |              |                |                |               |                 |                 | 0x70         |              |                |                |
//! | CLC             | 0x18        |               |                 |              |                |                |               |                 |                 |              |              |                |                |
//! | CLD             | 0xD8        |               |                 |              |                |                |               |                 |                 |              |              |                |                |
//! | CLI             | 0x58        |               |                 |              |                |                |               |                 |                 |              |              |                |                |
//! | CLV             | 0xB8        |               |                 |              |                |                |               |                 |                 |              |              |                |                |
//! | CMP             |             | 0xC9          |                 | 0xCD         | 0xDD           | 0xD9           | 0xC5          | 0xD5            |                 |              |              | 0xC1           | 0xD1           |
//! | CPX             |             | 0xE0          |                 | 0xEC         |                |                | 0xE4          |                 |                 |              |              |                |                |
//! | CPY             |             | 0xC0          |                 | 0xCC         |                |                | 0xC4          |                 |                 |              |              |                |                |
//! | DEC             |             |               |                 | 0xCE         | 0xDE           |                | 0xC6          | 0xD6            |                 |              |              |                |                |
//! | DEX             | 0xCA        |               |                 |              |                |                |               |                 |                 |              |              |                |                |
//! | DEY             | 0x88        |               |                 |              |                |                |               |                 |                 |              |              |                |                |
//! | EOR             |             | 0x49          |                 | 0x4D         | 0x5D           | 0x59           | 0x45          | 0x55            |                 |              |              | 0x41           | 0x51           |
//! | INC             |             |               |                 | 0xEE         | 0xFE           |                | 0xE6          | 0xF6            |                 |              |              |                |                |
//! | INX             | 0xE8        |               |                 |              |                |                |               |                 |                 |              |              |                |                |
//! | INY             | 0xC8        |               |                 |              |                |                |               |                 |                 |              |              |                |                |
//! | JMP             |             |               |                 | 0x4C         |                |                |               |                 |                 |              | 0x6C         |                |                |
//! | JSR             |             |               |                 | 0x20         |                |                |               |                 |                 |              |              |                |                |
//! | LDA             |             | 0xA9          |                 | 0xAD         | 0xBD           | 0xB9           | 0xA5          | 0xB5            |                 |              |              | 0xA1           | 0xB1           |
//! | LDX             |             | 0xA2          |                 | 0xAE         |                | 0xBE           | 0xA6          |                 | 0xB6            |              |              |                |                |
//! | LDY             |             | 0xA0          |                 | 0xAC         | 0xBC           |                | 0xA4          | 0xB4            |                 |              |              |                |                |
//! | LSR             |             |               | 0x4A            | 0x4E         | 0x5E           |                | 0x46          | 0x56            |                 |              |              |                |                |
//! | NOP             | 0xEA        |               |                 |              |                |                |               |                 |                 |              |              |                |                |
//! | ORA             |             | 0x09          |                 | 0x0D         | 0x1D           | 0x19           | 0x05          | 0x15            |                 |              |              | 0x01           | 0x11           |
//! | PHA             | 0x48        |               |                 |              |                |                |               |                 |                 |              |              |                |                |
//! | PHP             | 0x08        |               |                 |              |                |                |               |                 |                 |              |              |                |                |
//! | PLA             | 0x68        |               |                 |              |                |                |               |                 |                 |              |              |                |                |
//! | PLP             | 0x28        |               |                 |              |                |                |               |                 |                 |              |              |                |                |
//! | ROL             |             |               | 0x2A            | 0x2E         | 0x3E           |                | 0x26          | 0x36            |                 |              |              |                |                |
//! | ROR             |             |               | 0x6A            | 0x6E         | 0x7E           |                | 0x66          | 0x76            |                 |              |              |                |                |
//! | RTI             | 0x40        |               |                 |              |                |                |               |                 |                 |              |              |                |                |
//! | RTS             | 0x60        |               |                 |              |                |                |               |                 |                 |              |              |                |                |
//! | SBC             |             | 0xE9          |                 | 0xED         | 0xFD           | 0xF9           | 0xE5          | 0xF5            |                 |              |              | 0xE1           | 0xF1           |
//! | SEC             | 0x38        |               |                 |              |                |                |               |                 |                 |              |              |                |                |
//! | SED             | 0xF8        |               |                 |              |                |                |               |                 |                 |              |              |                |                |
//! | SEI             | 0x78        |               |                 |              |                |                |               |                 |                 |              |              |                |                |
//! | STA             |             |               |                 | 0x8D         | 0x9D           | 0x99           | 0x85          | 0x95            |                 |              |              | 0x81           | 0x91           |
//! | STX             |             |               |                 | 0x8E         |                |                | 0x86          |                 | 0x96            |              |              |                |                |
//! | STY             |             |               |                 | 0x8C         |                |                | 0x84          | 0x94            |                 |              |              |                |                |
//! | TAX             | 0xAA        |               |                 |              |                |                |               |                 |                 |              |              |                |                |
//! | TAY             | 0xA8        |               |                 |              |                |                |               |                 |                 |              |              |                |                |
//! | TSX             | 0xBA        |               |                 |              |                |                |               |                 |                 |              |              |                |                |
//! | TXA             | 0x8A        |               |                 |              |                |                |               |                 |                 |              |              |                |                |
//! | TXS             | 0x9A        |               |                 |              |                |                |               |                 |                 |              |              |                |                |
//! | TYA             | 0x98        |               |                 |              |                |                |               |                 |                 |              |              |                |                |

pub mod instruction;
pub mod opcodes;

use instruction::*;

/// Return all instruction definitions inside the 6502 instruction set.
pub fn isa_6502() -> Vec<InstructionDef> {
    vec![
        OPCODES_ADC,
        OPCODES_AND,
        OPCODES_ASL,
        OPCODES_BCC,
        OPCODES_BCS,
        OPCODES_BEQ,
        OPCODES_BIT,
        OPCODES_BMI,
        OPCODES_BNE,
        OPCODES_BPL,
        OPCODES_BRK,
        OPCODES_BVC,
        OPCODES_BVS,
        OPCODES_CLC,
        OPCODES_CLD,
        OPCODES_CLI,
        OPCODES_CLV,
        OPCODES_CMP,
        OPCODES_CPX,
        OPCODES_CPY,
        OPCODES_DEC,
        OPCODES_DEX,
        OPCODES_DEY,
        OPCODES_EOR,
        OPCODES_INC,
        OPCODES_INX,
        OPCODES_INY,
        OPCODES_JMP,
        OPCODES_JSR,
        OPCODES_LDA,
        OPCODES_LDX,
        OPCODES_LDY,
        OPCODES_LSR,
        OPCODES_NOP,
        OPCODES_ORA,
        OPCODES_PHA,
        OPCODES_PHP,
        OPCODES_PLA,
        OPCODES_PLP,
        OPCODES_ROL,
        OPCODES_ROR,
        OPCODES_RTI,
        OPCODES_RTS,
        OPCODES_SBC,
        OPCODES_SEC,
        OPCODES_SED,
        OPCODES_SEI,
        OPCODES_STA,
        OPCODES_STX,
        OPCODES_STY,
        OPCODES_TAX,
        OPCODES_TAY,
        OPCODES_TSX,
        OPCODES_TXA,
        OPCODES_TXS,
        OPCODES_TYA,
    ]
}
