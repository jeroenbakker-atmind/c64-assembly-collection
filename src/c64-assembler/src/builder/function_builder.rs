use crate::memory::user_count::UserCount;

use super::instruction_builder::InstructionBuilder;

#[derive(Default, Clone)]
pub struct FunctionBuilder {
    pub(crate) name: String,
    pub(crate) instructions: InstructionBuilder,
    user_count: usize,
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

impl UserCount for FunctionBuilder {
    fn user_increase(&mut self) {
        self.user_count += 1;
    }

    fn user_count(&self) -> usize {
        self.user_count
    }
}
