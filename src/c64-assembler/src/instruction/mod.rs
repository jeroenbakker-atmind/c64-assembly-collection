use operation::Operation;

use crate::memory::address_mode::AddressMode;

pub mod operation;

#[derive(Clone)]
pub struct Instruction {
    pub operation: Operation,
    pub address_mode: AddressMode,
    pub comments: Vec<String>,
}
