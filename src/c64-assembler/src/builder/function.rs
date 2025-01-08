use crate::memory::user_count::UserCount;

use super::instruction::Instructions;

#[derive(Default, Clone)]
pub struct Function {
    pub(crate) name: String,
    pub(crate) documentation: Vec<String>,
    pub(crate) instructions: Instructions,
    user_count: usize,
}

#[derive(Default, Clone)]
pub struct FunctionBuilder {
    function: Function,
}

impl FunctionBuilder {
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.function.name = name.to_string();
        self
    }

    pub fn doc(&mut self, documentation: &[&str]) -> &mut Self {
        for d in documentation {
            self.function.documentation.push(d.to_string());
        }
        self
    }

    pub fn instructions(&mut self, instructions: Instructions) -> &mut Self {
        self.function.instructions = instructions;
        self
    }

    pub fn finalize(&self) -> Function {
        self.function.clone()
    }
}

impl UserCount for Function {
    fn user_increase(&mut self) {
        self.user_count += 1;
    }

    fn user_count(&self) -> usize {
        self.user_count
    }
}
