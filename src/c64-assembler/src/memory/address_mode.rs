use super::label::AddressReference;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum AddressMode {
    #[default]
    Implied,
    Accumulator,
    Immediate(Immediate),
    Absolute(AddressReference),
    AbsoluteX(AddressReference),
    AbsoluteY(AddressReference),
    Relative(AddressReference),
    Indirect(AddressReference),
    IndexedIndirect(AddressReference),
    IndirectIndexed(AddressReference),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Immediate {
    Byte(u8),
    Low(AddressReference),
    High(AddressReference),
}
