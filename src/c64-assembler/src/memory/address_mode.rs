use super::label::AddressReference;

#[derive(Clone)]
pub enum AddressMode {
    Implied,
    Immediate(Immediate),
    Absolute(AddressReference),
    AbsoluteX(AddressReference),
    AbsoluteY(AddressReference),
    IndexedIndirect(AddressReference),
    IndirectIndexed(AddressReference),
}

#[derive(Clone)]
pub enum Immediate {
    Byte(u8),
    Low(AddressReference),
    High(AddressReference),
}
