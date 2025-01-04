use super::label::AddressReference;

#[derive(Clone)]
pub enum AddressMode {
    Implied,
    Immediate(u8),
    Absolute(AddressReference),
    AbsoluteX(AddressReference),
    AbsoluteY(AddressReference),
    IndexedIndirect(AddressReference),
    IndirectIndexed(AddressReference),
}
