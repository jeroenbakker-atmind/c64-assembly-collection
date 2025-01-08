use std::collections::HashMap;

use crate::memory::{
    define::{Define, Value},
    label::AddressReference,
    Address, ZeroPage,
};

use super::{finalize::finalize, module::Module};

#[derive(Clone)]
pub struct Application {
    pub name: String,
    pub entry_point: Address,
    pub modules: Vec<Module>,
    pub defines: Vec<Define>,
    pub address_lookup: HashMap<String, Address>,
}

#[derive(Clone)]
pub struct ApplicationBuilder {
    application: Application,
}

impl Default for ApplicationBuilder {
    fn default() -> Self {
        Self {
            application: Application {
                name: String::default(),
                entry_point: 0x0800,
                modules: vec![],
                defines: vec![],
                address_lookup: HashMap::default(),
            },
        }
    }
}

impl ApplicationBuilder {
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.application.name = name.to_string();
        self
    }
    pub fn entry_point(&mut self, entry_point: Address) -> &mut Self {
        self.application.entry_point = entry_point;
        self
    }

    pub fn define_address(&mut self, name: &str, address: Address) -> &mut Self {
        self.application
            .address_lookup
            .insert(name.to_string(), address);
        self.application
            .defines
            .push(Define::new(name, Value::Address(address)));
        self
    }

    pub fn define_zeropage(&mut self, name: &str, address: Address) -> &mut Self {
        assert!(address.is_zeropage());
        self.application
            .address_lookup
            .insert(name.to_string(), address);
        self.application
            .defines
            .push(Define::new(name, Value::Address(address)));
        self
    }

    pub fn add_vic20(&mut self) -> &mut Self {
        self.define_address("VIC20_BORDER_COLOR", 0xD020)
            .define_address("VIC20_BACKGROUND_COLOR", 0xD021)
    }

    pub fn add_module(&mut self, module: Module) -> &mut Self {
        self.application.modules.push(module);
        self
    }

    pub fn finalize(&mut self) -> Application {
        finalize(&mut self.application);
        self.application.clone()
    }
}

impl Application {
    pub(crate) fn define_mut(&mut self, define_name: &String) -> &mut Define {
        self.defines
            .iter_mut()
            .find(|define| &define.name == define_name)
            .unwrap()
    }

    pub fn address(&self, address_reference: &AddressReference) -> Address {
        self.address_lookup.get(&address_reference.name).unwrap() + address_reference.offset
    }
}
