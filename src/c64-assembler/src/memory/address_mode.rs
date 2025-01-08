use super::label::AddressReference;

#[derive(Clone, Debug)]
pub enum AddressMode {
    Implied,
    Accumulator,
    Immediate(Immediate),
    Absolute(AddressReference),
    AbsoluteX(AddressReference),
    AbsoluteY(AddressReference),
    Relative(AddressReference),
    IndexedIndirect(AddressReference),
    IndirectIndexed(AddressReference),
}

#[derive(Clone, Debug)]
pub enum Immediate {
    Byte(u8),
    Low(AddressReference),
    High(AddressReference),
}
