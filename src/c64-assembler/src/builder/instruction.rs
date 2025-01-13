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

macro_rules! private_add {
    ($function_name:ident, $operation:ident) => {
        fn $function_name(&mut self, address_mode: AddressMode) -> &mut Self {
            self.add_instruction(Operation::$operation, address_mode);
            self
        }
    };
}

macro_rules! implied {
    ($function_name:ident, $operation:ident) => {
        pub fn $function_name(&mut self) -> &mut Self {
            self.add_instruction(Operation::$operation, AddressMode::Implied);
            self
        }
    };
}
macro_rules! acc {
    ($function_name:ident, $add:ident) => {
        pub fn $function_name(&mut self) -> &mut Self {
            self.$add(AddressMode::Accumulator)
        }
    };
}
macro_rules! addr {
    ($function_name:ident, $add:ident) => {
        pub fn $function_name(&mut self, address: &str) -> &mut Self {
            self.$add(AddressMode::Absolute(AddressReference::new(address)))
        }
    };
}

macro_rules! addr_offs {
    ($function_name:ident, $add:ident) => {
        pub fn $function_name(&mut self, address: &str, offset: Address) -> &mut Self {
            self.$add(AddressMode::Absolute(AddressReference::with_offset(
                address, offset,
            )))
        }
    };
}
macro_rules! addr_x {
    ($function_name:ident, $add:ident) => {
        pub fn $function_name(&mut self, address: &str) -> &mut Self {
            self.$add(AddressMode::AbsoluteX(AddressReference::new(address)))
        }
    };
}
macro_rules! addr_y {
    ($function_name:ident, $add:ident) => {
        pub fn $function_name(&mut self, address: &str) -> &mut Self {
            self.$add(AddressMode::AbsoluteY(AddressReference::new(address)))
        }
    };
}

impl InstructionBuilder {
    fn add_instruction(&mut self, operation: Operation, address_mode: AddressMode) {
        self.instructions.instructions.push(Instruction {
            operation,
            address_mode,
            comments: vec![],
        });
    }

    private_add!(asl, ASL);
    acc!(asl_acc, asl);
    addr!(asl_addr, asl);
    addr_offs!(asl_addr_offs, asl);
    addr_x!(asl_addr_x, asl);
    implied!(brk, BRK);
    implied!(cld, CLD);
    implied!(cli, CLI);
    implied!(clv, CLV);
    implied!(dex, DEX);
    implied!(dey, DEY);
    implied!(inx, INX);
    implied!(iny, INY);
    implied!(nop, NOP);
    implied!(pha, PHA);
    implied!(psr, PSR);
    implied!(pla, PLA);
    implied!(plp, PLP);
    implied!(rti, RTI);
    implied!(sed, SED);
    implied!(sei, SEI);
    implied!(tax, TAX);
    implied!(tay, TAY);
    implied!(tsx, TSX);
    implied!(txa, TXA);
    implied!(txs, TXS);
    implied!(tya, TYA);
    implied!(clc, CLC);
    implied!(rts, RTS);

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
