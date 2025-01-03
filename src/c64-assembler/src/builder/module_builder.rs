use super::instruction_builder::InstructionBuilder;

pub struct ModuleBuilder {
    pub(crate) name: String,
    pub(crate) instructions: InstructionBuilder,
}

impl ModuleBuilder {
    pub fn with_name(name: &str) -> Self {
        ModuleBuilder {
            name: name.to_string(),
            instructions: InstructionBuilder::default(),
        }
    }
    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    pub fn instructions(&mut self) -> &mut InstructionBuilder {
        &mut self.instructions
    }
}
