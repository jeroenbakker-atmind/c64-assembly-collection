use operation::Operation;

use crate::builder::application::Application;
use crate::memory::address_mode::AddressMode;
use crate::memory::label::AddressReference;
use crate::memory::ZeroPage;
pub mod operation;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Instruction {
    pub operation: Operation,
    pub address_mode: AddressMode,
    pub comments: Vec<String>,
}

fn is_zeropage(application: &Application, address_reference: &AddressReference) -> bool {
    if let Some(address) = application.address_lookup.get(&address_reference.name) {
        address.is_zeropage()
    } else {
        false
    }
}

impl Instruction {
    /// Total number of bytes this instruction occupies on a 6502.
    pub fn byte_size(&self, application: &Application) -> u16 {
        if let Operation::Raw(bytes) = &self.operation {
            bytes.len() as u16
        } else if let Operation::Label(_) = &self.operation {
            0
        } else {
            match &self.address_mode {
                AddressMode::Implied | AddressMode::Accumulator => 1,
                AddressMode::Relative(_) | AddressMode::Immediate(_) => 2,
                AddressMode::Absolute(address_reference)
                | AddressMode::AbsoluteX(address_reference)
                | AddressMode::AbsoluteY(address_reference) => {
                    if is_zeropage(application, address_reference) {
                        2
                    } else {
                        3
                    }
                }
                AddressMode::Indirect(_address_reference) => 3,
                AddressMode::IndexedIndirect(_address_reference)
                | AddressMode::IndirectIndexed(_address_reference) => 2,
            }
        }
    }
}
