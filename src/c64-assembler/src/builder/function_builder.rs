use super::instruction_builder::InstructionBuilder;

#[derive(Default, Clone)]
pub struct FunctionBuilder {
    pub(crate) name: String,
    pub(crate) instructions: InstructionBuilder,
}

impl FunctionBuilder {
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }

    pub fn instructions(&mut self, instructions: InstructionBuilder) -> &mut Self {
        self.instructions = instructions;
        self
    }

    pub fn finalize(&self) -> Self {
        self.clone()
    }
}
