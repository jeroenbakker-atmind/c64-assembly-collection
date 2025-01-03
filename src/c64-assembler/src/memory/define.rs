use super::Address;

pub struct Define {
    pub name: String,
    pub value: Value,
}

pub enum Value {
    Address(Address),
}
