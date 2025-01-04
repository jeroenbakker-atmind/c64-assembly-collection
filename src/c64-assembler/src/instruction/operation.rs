#[derive(Clone)]
pub enum Operation {
    LoadAccumulator,
    StoreAccumulator,
    Jump,
    JumpSubRoutine,

    SetCarryFlag,
    ClearCarryFlag,
    Return,

    /// Store a byte in the instruction stream. Only immediate addressing mode can be used.
    Raw(Vec<u8>),
    /// Label
    Label(String),
}
