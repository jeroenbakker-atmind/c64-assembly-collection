use crate::{
    instruction::{operation::Operation, Instruction},
    memory::{
        address_mode::{AddressMode, Immediate},
        label::AddressReference,
        Address,
    },
};

#[derive(Debug, Default, Clone)]
pub struct Instructions {
    pub instructions: Vec<Instruction>,
}

#[derive(Default, Clone)]
pub struct InstructionBuilder {
    instructions: Instructions,
}

impl InstructionBuilder {
    fn add_instruction(&mut self, operation: Operation, address_mode: AddressMode) {
        self.instructions.instructions.push(Instruction {
            operation,
            address_mode,
            comments: vec![],
        });
    }

    fn asl(&mut self, address_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::ASL, address_mode);
        self
    }
    pub fn asl_acc(&mut self) -> &mut Self {
        self.asl(AddressMode::Accumulator)
    }
    pub fn asl_addr(&mut self, address: &str) -> &mut Self {
        self.asl(AddressMode::Absolute(AddressReference::new(address)))
    }
    pub fn asl_addr_offs(&mut self, address: &str, offset: Address) -> &mut Self {
        self.asl(AddressMode::Absolute(AddressReference::with_offset(
            address, offset,
        )))
    }
    pub fn asl_addr_x(&mut self, address: &str) -> &mut Self {
        self.asl(AddressMode::AbsoluteX(AddressReference::new(address)))
    }

    pub fn brk(&mut self) -> &mut Self {
        self.add_instruction(Operation::BRK, AddressMode::Implied);
        self
    }
    pub fn cld(&mut self) -> &mut Self {
        self.add_instruction(Operation::CLD, AddressMode::Implied);
        self
    }
    pub fn cli(&mut self) -> &mut Self {
        self.add_instruction(Operation::CLI, AddressMode::Implied);
        self
    }
    pub fn clv(&mut self) -> &mut Self {
        self.add_instruction(Operation::CLV, AddressMode::Implied);
        self
    }
    pub fn dex(&mut self) -> &mut Self {
        self.add_instruction(Operation::DEX, AddressMode::Implied);
        self
    }
    pub fn dey(&mut self) -> &mut Self {
        self.add_instruction(Operation::DEY, AddressMode::Implied);
        self
    }
    pub fn inx(&mut self) -> &mut Self {
        self.add_instruction(Operation::INX, AddressMode::Implied);
        self
    }
    pub fn iny(&mut self) -> &mut Self {
        self.add_instruction(Operation::INY, AddressMode::Implied);
        self
    }
    pub fn nop(&mut self) -> &mut Self {
        self.add_instruction(Operation::NOP, AddressMode::Implied);
        self
    }
    pub fn pha(&mut self) -> &mut Self {
        self.add_instruction(Operation::PHA, AddressMode::Implied);
        self
    }
    pub fn psr(&mut self) -> &mut Self {
        self.add_instruction(Operation::PSR, AddressMode::Implied);
        self
    }
    pub fn pla(&mut self) -> &mut Self {
        self.add_instruction(Operation::PLA, AddressMode::Implied);
        self
    }
    pub fn plp(&mut self) -> &mut Self {
        self.add_instruction(Operation::PLP, AddressMode::Implied);
        self
    }
    pub fn rti(&mut self) -> &mut Self {
        self.add_instruction(Operation::RTI, AddressMode::Implied);
        self
    }
    pub fn sed(&mut self) -> &mut Self {
        self.add_instruction(Operation::SED, AddressMode::Implied);
        self
    }
    pub fn sei(&mut self) -> &mut Self {
        self.add_instruction(Operation::SEI, AddressMode::Implied);
        self
    }
    pub fn tax(&mut self) -> &mut Self {
        self.add_instruction(Operation::TAX, AddressMode::Implied);
        self
    }
    pub fn tay(&mut self) -> &mut Self {
        self.add_instruction(Operation::TAY, AddressMode::Implied);
        self
    }
    pub fn tsx(&mut self) -> &mut Self {
        self.add_instruction(Operation::TSX, AddressMode::Implied);
        self
    }
    pub fn txa(&mut self) -> &mut Self {
        self.add_instruction(Operation::TXA, AddressMode::Implied);
        self
    }
    pub fn txs(&mut self) -> &mut Self {
        self.add_instruction(Operation::TXS, AddressMode::Implied);
        self
    }
    pub fn tya(&mut self) -> &mut Self {
        self.add_instruction(Operation::TYA, AddressMode::Implied);
        self
    }
    pub fn clc(&mut self) -> &mut Self {
        self.add_instruction(Operation::CLC, AddressMode::Implied);
        self
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
    pub fn lda_addr(&mut self, address: &str) -> &mut Self {
        self.lda(AddressMode::Absolute(AddressReference::new(address)))
    }
    pub fn lda_addr_offs(&mut self, address: &str, offset: Address) -> &mut Self {
        self.lda(AddressMode::Absolute(AddressReference::with_offset(
            address, offset,
        )))
    }
    pub fn lda_addr_x(&mut self, address: &str) -> &mut Self {
        self.lda(AddressMode::AbsoluteX(AddressReference::new(address)))
    }
    pub fn lda_addr_y(&mut self, address: &str) -> &mut Self {
        self.lda(AddressMode::AbsoluteY(AddressReference::new(address)))
    }
    pub fn lda_ind_y(&mut self, address: &str) -> &mut Self {
        self.lda(AddressMode::IndirectIndexed(AddressReference::new(address)))
    }
    pub fn lda_ind_x(&mut self, address: &str) -> &mut Self {
        self.lda(AddressMode::IndexedIndirect(AddressReference::new(address)))
    }

    fn ldy(&mut self, address_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::LDY, address_mode);
        self
    }
    pub fn ldy_imm(&mut self, byte: u8) -> &mut Self {
        self.ldy(AddressMode::Immediate(Immediate::Byte(byte)))
    }
    pub fn ldy_imm_low(&mut self, address: &str) -> &mut Self {
        self.ldy(AddressMode::Immediate(Immediate::Low(
            AddressReference::new(address),
        )));
        self
    }
    pub fn ldy_imm_high(&mut self, address: &str) -> &mut Self {
        self.ldy(AddressMode::Immediate(Immediate::High(
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
    pub fn sta_addr_x(&mut self, name: &str) -> &mut Self {
        self.sta(AddressMode::AbsoluteX(AddressReference::new(name)))
    }
    pub fn sta_addr_y(&mut self, name: &str) -> &mut Self {
        self.sta(AddressMode::AbsoluteY(AddressReference::new(name)))
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
    pub fn adc_imm(&mut self, byte: u8) -> &mut Self {
        self.adc(AddressMode::Immediate(Immediate::Byte(byte)))
    }
    pub fn adc_imm_low(&mut self, address: &str) -> &mut Self {
        self.adc(AddressMode::Immediate(Immediate::Low(
            AddressReference::new(address),
        )));
        self
    }
    pub fn adc_imm_high(&mut self, address: &str) -> &mut Self {
        self.adc(AddressMode::Immediate(Immediate::High(
            AddressReference::new(address),
        )));
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
    pub fn adc_addr_x(&mut self, name: &str) -> &mut Self {
        self.adc(AddressMode::AbsoluteX(AddressReference::new(name)))
    }
    pub fn adc_addr_y(&mut self, name: &str) -> &mut Self {
        self.adc(AddressMode::AbsoluteY(AddressReference::new(name)))
    }

    fn cmp(&mut self, address_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::CMP, address_mode);
        self
    }
    pub fn cmp_addr(&mut self, name: &str) -> &mut Self {
        self.cmp(AddressMode::Absolute(AddressReference::new(name)))
    }
    pub fn cmp_addr_offs(&mut self, name: &str, offset: Address) -> &mut Self {
        self.cmp(AddressMode::Absolute(AddressReference::with_offset(
            name, offset,
        )))
    }
    fn cpx(&mut self, address_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::CPX, address_mode);
        self
    }
    pub fn cmx_addr(&mut self, name: &str) -> &mut Self {
        self.cpx(AddressMode::Absolute(AddressReference::new(name)))
    }
    pub fn cmx_addr_offs(&mut self, name: &str, offset: Address) -> &mut Self {
        self.cpx(AddressMode::Absolute(AddressReference::with_offset(
            name, offset,
        )))
    }
    fn cpy(&mut self, address_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::CPY, address_mode);
        self
    }
    pub fn cpy_addr(&mut self, name: &str) -> &mut Self {
        self.cpy(AddressMode::Absolute(AddressReference::new(name)))
    }
    pub fn cpy_addr_offs(&mut self, name: &str, offset: Address) -> &mut Self {
        self.cpy(AddressMode::Absolute(AddressReference::with_offset(
            name, offset,
        )))
    }

    fn bne(&mut self, address_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::BNE, address_mode);
        self
    }
    pub fn bne_addr(&mut self, name: &str) -> &mut Self {
        self.bne(AddressMode::Relative(AddressReference::new(name)))
    }
    pub fn bne_addr_offs(&mut self, name: &str, offset: Address) -> &mut Self {
        self.bne(AddressMode::Relative(AddressReference::with_offset(
            name, offset,
        )))
    }

    fn sbc(&mut self, address_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::SBC, address_mode);
        self
    }
    pub fn sbc_imm(&mut self, byte: u8) -> &mut Self {
        self.sbc(AddressMode::Immediate(Immediate::Byte(byte)))
    }
    pub fn sbc_addr(&mut self, name: &str) -> &mut Self {
        self.sbc(AddressMode::Absolute(AddressReference::new(name)))
    }
    pub fn sbc_addr_offs(&mut self, name: &str, offset: Address) -> &mut Self {
        self.sbc(AddressMode::Absolute(AddressReference::with_offset(
            name, offset,
        )))
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
            .instructions
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

    pub fn finalize(&self) -> Instructions {
        self.instructions.clone()
    }
}
