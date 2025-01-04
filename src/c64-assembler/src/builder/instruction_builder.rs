use crate::{
    instruction::{operation::Operation, Instruction},
    memory::address_mode::AddressMode,
};

#[derive(Default, Clone)]
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

    pub fn add_basic_header(&mut self) -> &mut Self {
        /* Basic line header */
        self.raw(&[0x00, 0x0c, 0x08])
            .comment("New basic line")
            /* 10 SYS 2062 */
            .raw(&[0x0a, 0x00, 0x9e, 0x20, 0x32, 0x30, 0x36, 0x32])
            .comment("10 SYS 2062")
            /* Basic line heaer */
            .raw(&[0x00, 0x00, 0x00])
            .comment("End basic program")
    }

    pub fn finalize(&self) -> InstructionBuilder {
        self.clone()
    }
}
