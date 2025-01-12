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
    PSR,
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
