use std::collections::HashMap;

use crate::memory::{
    define::{Define, Value},
    Address,
};

use super::module_builder::ModuleBuilder;

pub struct ApplicationBuilder {
    pub(crate) name: String,
    pub(crate) entry_point: Address,
    pub(crate) modules: Vec<ModuleBuilder>,
    pub(crate) defines: Vec<Define>,
    pub(crate) address_lookup: HashMap<String, Address>,
}

impl Default for ApplicationBuilder {
    fn default() -> Self {
        Self {
            name: String::default(),
            entry_point: Default::default(),
            modules: vec![ModuleBuilder::with_name("main")],
            defines: vec![],
            address_lookup: HashMap::default(),
        }
    }
}

impl ApplicationBuilder {
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }
    pub fn entry_point(&mut self, entry_point: Address) -> &mut Self {
        assert!(self.entry_point == 0);
        self.entry_point = entry_point;
        self
    }

    pub fn basic_header(&mut self) -> &mut Self {
        self.entry_point(0x0800)
            .main_module()
            .instructions()
            /* Basic line header */
            .raw(&[0x00, 0x0c, 0x08])
            .comment("New basic line")
            /* 10 SYS 2062 */
            .raw(&[0x0a, 0x00, 0x9e, 0x20, 0x32, 0x30, 0x36, 0x32])
            .comment("10 SYS 2062")
            /* Basic line heaer */
            .raw(&[0x00, 0x00, 0x00])
            .comment("End basic program");
        self
    }

    pub fn define_address(&mut self, name: &str, address: Address) -> &mut Self {
        self.address_lookup.insert(name.to_string(), address);
        self.defines.push(Define {
            name: name.to_string(),
            value: Value::Address(address),
        });
        self
    }

    pub fn main_module(&mut self) -> &mut ModuleBuilder {
        self.module("main")
    }

    pub fn module(&mut self, name: &str) -> &mut ModuleBuilder {
        self.modules.get_mut(0).unwrap()
    }
}
