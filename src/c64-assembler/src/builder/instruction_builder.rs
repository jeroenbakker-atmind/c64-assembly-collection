use crate::{
    instruction::{operation::Operation, Instruction},
    memory::{
        address_mode::{AddressMode, Immediate},
        label::AddressReference,
        Address,
    },
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

    fn lda(&mut self, address_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::LDA, address_mode);
        self
    }
    pub fn lda_imm(&mut self, byte: u8) -> &mut Self {
        self.lda(AddressMode::Immediate(Immediate::Byte(byte)))
    }
    pub fn lda_imm_low(&mut self, address: &str) -> &mut Self {
        self.lda(AddressMode::Immediate(Immediate::Low(
            AddressReference::new(address),
        )));
        self
    }
    pub fn lda_imm_high(&mut self, address: &str) -> &mut Self {
        self.lda(AddressMode::Immediate(Immediate::High(
            AddressReference::new(address),
        )));
        self
    }

    fn sta(&mut self, address_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::STA, address_mode);
        self
    }

    pub fn sta_addr(&mut self, name: &str) -> &mut Self {
        self.sta(AddressMode::Absolute(AddressReference::new(name)))
    }
    pub fn sta_addr_offs(&mut self, name: &str, offset: Address) -> &mut Self {
        self.sta(AddressMode::Absolute(AddressReference::with_offset(
            name, offset,
        )))
    }
    fn jsr(&mut self, address_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::JSR, address_mode);
        self
    }
    pub fn jsr_addr(&mut self, name: &str) -> &mut Self {
        self.jsr(AddressMode::Absolute(AddressReference::new(name)))
    }
    pub fn rts(&mut self) -> &mut Self {
        self.add_instruction(Operation::RTS, AddressMode::Implied);
        self
    }

    fn adc(&mut self, address_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::ADC, address_mode);
        self
    }
    pub fn adc_addr(&mut self, name: &str) -> &mut Self {
        self.adc(AddressMode::Absolute(AddressReference::new(name)))
    }
    pub fn adc_addr_offs(&mut self, name: &str, offset: Address) -> &mut Self {
        self.adc(AddressMode::Absolute(AddressReference::with_offset(
            name, offset,
        )))
    }
    pub fn clc(&mut self) -> &mut Self {
        self.add_instruction(Operation::CLC, AddressMode::Implied);
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
