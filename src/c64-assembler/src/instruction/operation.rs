use c64_assembler_6502::instruction::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub enum Operation {
    /// Add Memory to Accumulator with Carry
    ADC,
    /// AND Memory with Accumulator
    AND,
    /// Shift Left One Bit (Memory or Accumulator)
    ASL,
    /// Branch on carry clear
    BCC,
    /// Branch on Carry Set
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    LDX,
    LSR,
    #[default]
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    SBC,
    SED,
    SEI,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,

    /// Load accumulator
    LDA,
    LDY,
    /// Store accumulator
    STA,
    /// Jump
    JMP,
    /// Jump sub-routine
    JSR,

    /// Set carrier flag
    SEC,
    /// Clear carrier flag
    CLC,
    /// Return (from stack)
    RTS,

    /// Store a byte in the instruction stream. Only immediate addressing mode can be used.
    Raw(Vec<u8>),
    /// Label
    Label(String),
}

impl Operation {
    /// Get the instruction definition of the operation.
    pub fn definition(&self) -> Option<&'static InstructionDef> {
        match self {
            Operation::ADC => Some(&OPCODES_ADC),
            Operation::AND => Some(&OPCODES_AND),
            Operation::ASL => Some(&OPCODES_ASL),
            Operation::BCC => Some(&OPCODES_BCC),
            Operation::BCS => Some(&OPCODES_BCS),
            Operation::BEQ => Some(&OPCODES_BEQ),
            Operation::BIT => Some(&OPCODES_BIT),
            Operation::BMI => Some(&OPCODES_BMI),
            Operation::BNE => Some(&OPCODES_BNE),
            Operation::BPL => Some(&OPCODES_BPL),
            Operation::BRK => Some(&OPCODES_BRK),
            Operation::BVC => Some(&OPCODES_BVC),
            Operation::BVS => Some(&OPCODES_BVS),
            Operation::CLD => Some(&OPCODES_CLD),
            Operation::CLI => Some(&OPCODES_CLI),
            Operation::CLV => Some(&OPCODES_CLV),
            Operation::CMP => Some(&OPCODES_CMP),
            Operation::CPX => Some(&OPCODES_CPX),
            Operation::CPY => Some(&OPCODES_CPY),
            Operation::DEC => Some(&OPCODES_DEC),
            Operation::DEX => Some(&OPCODES_DEX),
            Operation::DEY => Some(&OPCODES_DEY),
            Operation::EOR => Some(&OPCODES_EOR),
            Operation::INC => Some(&OPCODES_INC),
            Operation::INX => Some(&OPCODES_INX),
            Operation::INY => Some(&OPCODES_INY),
            Operation::LDX => Some(&OPCODES_LDX),
            Operation::LSR => Some(&OPCODES_LSR),
            Operation::NOP => Some(&OPCODES_NOP),
            Operation::ORA => Some(&OPCODES_ORA),
            Operation::PHA => Some(&OPCODES_PHA),
            Operation::PHP => Some(&OPCODES_PHP),
            Operation::PLA => Some(&OPCODES_PLA),
            Operation::PLP => Some(&OPCODES_PLP),
            Operation::ROL => Some(&OPCODES_ROL),
            Operation::ROR => Some(&OPCODES_ROR),
            Operation::RTI => Some(&OPCODES_RTI),
            Operation::SBC => Some(&OPCODES_SBC),
            Operation::SED => Some(&OPCODES_SED),
            Operation::SEI => Some(&OPCODES_SEI),
            Operation::STX => Some(&OPCODES_STX),
            Operation::STY => Some(&OPCODES_STY),
            Operation::TAX => Some(&OPCODES_TAX),
            Operation::TAY => Some(&OPCODES_TAY),
            Operation::TSX => Some(&OPCODES_TSX),
            Operation::TXA => Some(&OPCODES_TXA),
            Operation::TXS => Some(&OPCODES_TXS),
            Operation::TYA => Some(&OPCODES_TYA),
            Operation::LDA => Some(&OPCODES_LDA),
            Operation::LDY => Some(&OPCODES_LDY),
            Operation::STA => Some(&OPCODES_STA),
            Operation::JMP => Some(&OPCODES_JMP),
            Operation::JSR => Some(&OPCODES_JSR),
            Operation::SEC => Some(&OPCODES_SEC),
            Operation::CLC => Some(&OPCODES_CLC),
            Operation::RTS => Some(&OPCODES_RTS),
            Operation::Raw(_vec) => None,
            Operation::Label(_) => None,
        }
    }
}
