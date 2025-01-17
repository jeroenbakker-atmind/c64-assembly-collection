//!  | **Instructie** | **Implied** | **Immediate** | **Accumulator** | **Absolute** | **Absolute,X** | **Absolute,Y** | **Zero Page** | **Zero Page,X** | **Indirect** | **Indirect,X** | **Indirect,Y** |
//! |----------------|-------------|---------------|-----------------|--------------|----------------|----------------|---------------|-----------------|--------------|----------------|----------------|
//! | ADC           |             | 69            |                 | 6D           | 7D             | 79             | 65            | 75             |              | 61             | 71             |
//! | AND           |             | 29            |                 | 2D           | 3D             | 39             | 25            | 35             |              | 21             | 31             |
//! | ASL           |             |               | 0A              | 0E           | 1E             |                | 06            | 16             |              |                |                |
//! | BCC           | 90          |               |                 |              |                |                |               |                 |              |                |                |
//! | BCS           | B0          |               |                 |              |                |                |               |                 |              |                |                |
//! | BEQ           | F0          |               |                 |              |                |                |               |                 |              |                |                |
//! | BIT           |             |               |                 | 2C           |                |                | 24            |                 |              |                |                |
//! | BMI           | 30          |               |                 |              |                |                |               |                 |              |                |                |
//! | BNE           | D0          |               |                 |              |                |                |               |                 |              |                |                |
//! | BPL           | 10          |               |                 |              |                |                |               |                 |              |                |                |
//! | BRK           | 00          |               |                 |              |                |                |               |                 |              |                |                |
//! | BVC           | 50          |               |                 |              |                |                |               |                 |              |                |                |
//! | BVS           | 70          |               |                 |              |                |                |               |                 |              |                |                |
//! | CLC           | 18          |               |                 |              |                |                |               |                 |              |                |                |
//! | CLD           | D8          |               |                 |              |                |                |               |                 |              |                |                |
//! | CLI           | 58          |               |                 |              |                |                |               |                 |              |                |                |
//! | CLV           | B8          |               |                 |              |                |                |               |                 |              |                |                |
//! | CMP           |             | C9            |                 | CD           | DD             | D9             | C5            | D5             |              | C1             | D1             |
//! | CPX           |             | E0            |                 | EC           |                |                | E4            |                 |              |                |                |
//! | CPY           |             | C0            |                 | CC           |                |                | C4            |                 |              |                |                |
//! | DEC           |             |               |                 | CE           | DE             |                | C6            | D6             |              |                |                |
//! | DEX           | CA          |               |                 |              |                |                |               |                 |              |                |                |
//! | DEY           | 88          |               |                 |              |                |                |               |                 |              |                |                |
//! | EOR           |             | 49            |                 | 4D           | 5D             | 59             | 45            | 55             |              | 41             | 51             |
//! | INC           |             |               |                 | EE           | FE             |                | E6            | F6             |              |                |                |
//! | INX           | E8          |               |                 |              |                |                |               |                 |              |                |                |
//! | INY           | C8          |               |                 |              |                |                |               |                 |              |                |                |
//! | JMP           |             |               |                 | 4C           |                |                |               |                 | 6C           |                |                |
//! | JSR           |             |               |                 | 20           |                |                |               |                 |              |                |                |
//! | LDA           |             | A9            |                 | AD           | BD             | B9             | A5            | B5             |              | A1             | B1             |
//! | LDX           |             | A2            |                 | AE           |                | BE             | A6            |                 |              |                |                |
//! | LDY           |             | A0            |                 | AC           | BC             |                | A4            | B4             |              |                |                |
//! | LSR           |             |               | 4A              | 4E           | 5E             |                | 46            | 56             |              |                |                |
//! | NOP           | EA          |               |                 |              |                |                |               |                 |              |                |                |
//! | ORA           |             | 09            |                 | 0D           | 1D             | 19             | 05            | 15             |              | 01             | 11             |
//! | PHA           |             |               | 48              |              |                |                |               |                 |              |                |                |
//! | PHP           | 08          |               |                 |              |                |                |               |                 |              |                |                |
//! | PLA           |             |               | 68              |              |                |                |               |                 |              |                |                |
//! | PLP           | 28          |               |                 |              |                |                |               |                 |              |                |                |
//! | ROL           |             |               | 2A              | 2E           | 3E             |                | 26            | 36             |              |                |                |
//! | ROR           |             |               | 6A              | 6E           | 7E             |                | 66            | 76             |              |                |                |
//! | RTI           | 40          |               |                 |              |                |                |               |                 |              |                |                |
//! | RTS           | 60          |               |                 |              |                |                |               |                 |              |                |                |
//! | SBC           |             | E9            |                 | ED           | FD             | F9             | E5            | F5             |              | E1             | F1             |
//! | SEC           | 38          |               |                 |              |                |                |               |                 |              |                |                |
//! | SED           | F8          |               |                 |              |                |                |               |                 |              |                |                |
//! | SEI           | 78          |               |                 |              |                |                |               |                 |              |                |                |
//! | STA           |             |               |                 | 8D           | 9D             | 99             | 85            | 95             |              | 81             | 91             |

pub type OpCode = u8;
const UNUSED: OpCode = 0xFF;
pub const IMPLIED: OpCode = UNUSED;
pub const IMM: OpCode = UNUSED;
pub const ACC: OpCode = UNUSED;
pub const ADDR: OpCode = UNUSED;
pub const ADDR_X: OpCode = UNUSED;
pub const ADDR_Y: OpCode = UNUSED;
pub const ZERO: OpCode = UNUSED;
pub const ZERO_X: OpCode = UNUSED;
pub const ZERO_Y: OpCode = UNUSED;
pub const IND: OpCode = UNUSED;
pub const IND_X: OpCode = UNUSED;
pub const IND_Y: OpCode = UNUSED;
pub const RELATIVE: OpCode = UNUSED;

// TODO add relative?
pub struct InstructionDef {
    pub instruction: &'static str,
    pub implied: OpCode,
    pub immediate: OpCode,
    pub accumulator: OpCode,
    pub absolute: OpCode,
    pub absolute_x: OpCode,
    pub absolute_y: OpCode,
    pub zeropage: OpCode,
    pub zeropage_x: OpCode,
    pub zeropage_y: OpCode,
    pub relative: OpCode,
    pub indirect: OpCode,
    pub indexed_indirect: OpCode,
    pub indirect_indexed: OpCode,
}
impl InstructionDef {
    fn new(
        instruction: &'static str,
        implied: OpCode,
        immediate: OpCode,
        accumulator: OpCode,
        absolute: OpCode,
        absolute_x: OpCode,
        absolute_y: OpCode,
        zeropage: OpCode,
        zeropage_x: OpCode,
        zeropage_y: OpCode,
        relative: OpCode,
        indirect: OpCode,
        indexed_indirect: OpCode,
        indirect_indexed: OpCode,
    ) -> InstructionDef {
        InstructionDef {
            instruction,
            implied,
            immediate,
            accumulator,
            absolute,
            absolute_x,
            absolute_y,
            zeropage,
            zeropage_x,
            zeropage_y,
            relative,
            indirect,
            indexed_indirect,
            indirect_indexed,
        }
    }
}

pub fn isa_6502() -> Vec<InstructionDef> {
    vec![
        InstructionDef::new(
            "adc", IMPLIED, 0x69, ACC, 0x6D, 0x7D, 0x79, 0x65, 0x75, ZERO_Y, RELATIVE, IND, 0x61,
            0x71,
        ),
        InstructionDef::new(
            "and", IMPLIED, 0x29, ACC, 0x2D, 0x3D, 0x39, 0x25, 0x35, ZERO_Y, RELATIVE, IND, 0x21,
            0x1,
        ),
        InstructionDef::new(
            "asl", IMPLIED, IMM, 0x0A, 0x0E, 0x1E, ADDR_Y, 0x06, 0x16, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "bcc", IMPLIED, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, 0x90, IND, IND_X,
            IND_Y,
        ),
        InstructionDef::new(
            "bcs", IMPLIED, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, 0xB0, IND, IND_X,
            IND_Y,
        ),
        InstructionDef::new(
            "beq", IMPLIED, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, 0xF0, IND, IND_X,
            IND_Y,
        ),
        InstructionDef::new(
            "bit", IMPLIED, IMM, ACC, 0x2C, ADDR_X, ADDR_Y, 0x24, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "bmi", IMPLIED, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, 0x30, IND, IND_X,
            IND_Y,
        ),
        InstructionDef::new(
            "bne", IMPLIED, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, 0xD0, IND, IND_X,
            IND_Y,
        ),
        InstructionDef::new(
            "bpl", IMPLIED, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, 0x10, IND, IND_X,
            IND_Y,
        ),
        InstructionDef::new(
            "brk", 0x00, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, IND, RELATIVE,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "bvc", IMPLIED, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, 0x50, IND, IND_X,
            IND_Y,
        ),
        InstructionDef::new(
            "bvs", IMPLIED, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, 0x70, IND, IND_X,
            IND_Y,
        ),
        InstructionDef::new(
            "clc", 0x18, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "cld", 0xD8, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "cli", 0x58, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "clv", 0xB8, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, RELATIVE, IND,
            UNUSED, IND_Y,
        ),
        InstructionDef::new(
            "cmp", IMPLIED, 0xC9, ACC, 0xCD, 0xDD, 0xD9, 0xC5, 0xD5, ZERO_Y, RELATIVE, IND, 0xC1,
            0xD1,
        ),
        InstructionDef::new(
            "cpx", IMPLIED, 0xE0, ACC, 0xEC, ADDR_X, ADDR_Y, 0xE4, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "cpy", IMPLIED, 0xC0, ACC, 0xCC, ADDR_X, ADDR_Y, 0xC4, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "dec", IMPLIED, IMM, ACC, 0xCE, 0xDE, ADDR_Y, 0xC6, 0xD6, ZERO_Y, RELATIVE, IND, IND_X,
            IND_Y,
        ),
        InstructionDef::new(
            "dex", 0xCA, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "dey", 0x88, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "eor", IMPLIED, 0x49, ACC, 0x4D, 0x5D, 0x59, 0x45, 0x55, ZERO_Y, RELATIVE, IND, 0x41,
            0x51,
        ),
        InstructionDef::new(
            "inc", IMPLIED, IMM, ACC, 0xEE, 0xFE, ADDR_Y, 0xE6, 0xF6, ZERO_Y, RELATIVE, IND, IND_X,
            IND_Y,
        ),
        InstructionDef::new(
            "inx", 0xE8, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "iny", 0xC8, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "jmp", IMPLIED, IMM, ACC, 0x4C, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, RELATIVE, 0x6C,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "jsr", IMPLIED, IMM, ACC, 0x20, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "lda", IMPLIED, 0xA9, ACC, 0xAD, 0xBD, 0xB9, 0xA5, 0xB5, ZERO_Y, RELATIVE, IND, 0xA1,
            0xB1,
        ),
        InstructionDef::new(
            "ldx", IMPLIED, 0xA2, ACC, 0xAE, ADDR_X, 0xBE, 0xA6, ZERO_X, 0xB6, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "ldy", IMPLIED, 0xA0, ACC, 0xAC, 0xBC, ADDR_Y, 0xA4, 0xB4, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "lsr", IMPLIED, IMM, 0x4A, 0x4E, 0x5E, ADDR_Y, 0x46, 0x56, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "nop", 0xEA, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "ora", IMPLIED, 0x09, ACC, 0x0D, 0x1D, 0x19, 0x05, 0x15, ZERO_Y, RELATIVE, IND, 0x01,
            0x11,
        ),
        InstructionDef::new(
            "pha", 0x48, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "php", 0x08, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "pla", 0x68, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "plp", 0x28, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "rol", IMPLIED, IMM, 0x2A, 0x2E, 0x3E, ADDR_Y, 0x26, 0x36, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "ror", IMPLIED, IMM, 0x6A, 0x6E, 0x7E, ADDR_Y, 0x66, 0x76, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "rti", 0x40, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "rts", 0x60, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "sbc", IMPLIED, 0xE9, ACC, 0xED, 0xFD, 0xF9, 0xE5, 0xF5, ZERO_Y, RELATIVE, IND, 0xE1,
            0xF1,
        ),
        InstructionDef::new(
            "sec", 0x38, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "sed", 0xF8, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "sei", 0x78, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "sta", IMPLIED, IMM, ACC, 0x8D, 0x9D, 0x99, 0x85, 0x95, ZERO_Y, RELATIVE, IND, 0x81,
            0x91,
        ),
        InstructionDef::new(
            "stx", IMPLIED, IMM, ACC, 0x8E, ADDR_X, ADDR_Y, 0x86, ZERO_X, 0x96, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "sty", IMPLIED, IMM, ACC, 0x8C, ADDR_X, ADDR_Y, 0x84, 0x94, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "tax", 0xAA, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "tay", 0xA8, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "tsx", 0xBA, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "txa", 0x8A, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "txs", 0x9A, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
        InstructionDef::new(
            "tya", 0x98, IMM, ACC, ADDR, ADDR_X, ADDR_Y, ZERO, ZERO_X, ZERO_Y, RELATIVE, IND,
            IND_X, IND_Y,
        ),
    ]
}
