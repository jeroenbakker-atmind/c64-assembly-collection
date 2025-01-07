use operation::Operation;

use crate::builder::application_builder::ApplicationBuilder;
use crate::memory::address_mode::AddressMode;
use crate::memory::label::AddressReference;
use crate::memory::ZeroPage;
pub mod operation;

#[derive(Clone)]
pub struct Instruction {
    pub operation: Operation,
    pub address_mode: AddressMode,
    pub comments: Vec<String>,
}

fn is_zeropage(application: &ApplicationBuilder, address_reference: &AddressReference) -> bool {
    if let Some(address) = application.address_lookup.get(&address_reference.name) {
        address.is_zeropage()
    } else {
        false
    }
}

impl Instruction {
    /// Total number of bytes this instruction occupies on a 6502.
    pub fn byte_size(&self, application: &ApplicationBuilder) -> u16 {
        match &self.address_mode {
            AddressMode::Implied => 1,
            AddressMode::Immediate(_immediate) => 2,
            AddressMode::Absolute(address_reference)
            | AddressMode::AbsoluteX(address_reference)
            | AddressMode::AbsoluteY(address_reference) => {
                if is_zeropage(application, address_reference) {
                    2
                } else {
                    3
                }
            }
            AddressMode::IndexedIndirect(_address_reference)
            | AddressMode::IndirectIndexed(_address_reference) => 2,
        }
    }
}
