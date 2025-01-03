use crate::{
    instruction::{operation::Operation, Instruction},
    memory::address_mode::AddressMode,
};

#[derive(Default)]
pub struct InstructionBuilder {
    pub(crate) instructions: Vec<Instruction>,
}

impl InstructionBuilder {
    fn add_instruction(&mut self, operation: Operation, address_mode: AddressMode) {
        self.instructions.push(Instruction {
            operation,
            address_mode,
            comments: vec![],
        });
    }

    pub fn load_accumulator(&mut self, address_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::LoadAccumulator, address_mode);
        self
    }

    pub fn store_accumulator(&mut self, address_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::StoreAccumulator, address_mode);
        self
    }

    pub fn return_to_caller(&mut self) -> &mut Self {
        self.add_instruction(Operation::Return, AddressMode::Implied);
        self
    }

    pub fn raw(&mut self, data: &[u8]) -> &mut Self {
        self.add_instruction(Operation::Raw(Vec::from(data)), AddressMode::Implied);
        self
    }

    pub fn label(&mut self, label: &str) -> &mut Self {
        self.add_instruction(Operation::Label(label.to_string()), AddressMode::Implied);
        self
    }

    /// Add a comment to the last instruction.
    pub fn comment(&mut self, comment: &str) -> &mut Self {
        self.instructions
            .last_mut()
            .unwrap()
            .comments
            .push(comment.to_string());
        self
    }
}
