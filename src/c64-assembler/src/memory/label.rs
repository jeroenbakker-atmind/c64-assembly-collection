use super::Address;

pub struct Label {
    pub name: String,
    pub address: Address,
}

#[derive(Clone, Debug)]
pub struct AddressReference {
    pub name: String,
    pub offset: Address,
}
impl AddressReference {
    pub fn new(name: &str) -> AddressReference {
        AddressReference {
            name: name.to_string(),
            offset: 0,
        }
    }
    pub fn with_offset(name: &str, offset: Address) -> AddressReference {
        AddressReference {
            name: name.to_string(),
            offset: offset,
        }
    }
}
