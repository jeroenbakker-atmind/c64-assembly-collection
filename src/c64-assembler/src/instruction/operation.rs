#[derive(Clone, Debug)]
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

    /// Load accumulator
    LDA,
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
