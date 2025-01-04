use super::{user_count::UserCount, Address};

pub struct Define {
    pub name: String,
    pub value: Value,
    user_count: usize,
}
impl Define {
    pub fn new(name: &str, value: Value) -> Define {
        Define {
            name: name.to_string(),
            value,
            user_count: 0,
        }
    }
}

pub enum Value {
    Address(Address),
}

impl UserCount for Define {
    fn user_increase(&mut self) {
        self.user_count += 1;
    }

    fn user_count(&self) -> usize {
        self.user_count
    }
}
