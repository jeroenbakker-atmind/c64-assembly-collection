use super::Address;

pub struct Label {
    pub name: String,
    pub address: Address,
}

#[derive(Clone)]
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
}

