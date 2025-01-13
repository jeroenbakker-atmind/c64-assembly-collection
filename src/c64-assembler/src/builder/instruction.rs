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

macro_rules! add {
    ($function_name:ident, $operation:ident) => {
        fn $function_name(&mut self, address_mode: AddressMode) -> &mut Self {
            self.add_instruction(Operation::$operation, address_mode);
            self
        }
    };
}

macro_rules! implied {
    ($function_name:ident, $operation:ident) => {
        /// Record a new operation to the instruction list.
        #[doc = "Records a new operation to the instruction list."]
        pub fn $function_name(&mut self) -> &mut Self {
            self.add_instruction(Operation::$operation, AddressMode::Implied);
            self
        }
    };
}

macro_rules! imm {
    ($function_name:ident, $function_name_low:ident, $function_name_high:ident, $add:ident) => {
        pub fn $function_name(&mut self, byte: u8) -> &mut Self {
            self.$add(AddressMode::Immediate(Immediate::Byte(byte)))
        }
        pub fn $function_name_low(&mut self, address_name: &str) -> &mut Self {
            self.$add(AddressMode::Immediate(Immediate::Low(
                AddressReference::new(address_name),
            )))
        }
        pub fn $function_name_high(&mut self, address_name: &str) -> &mut Self {
            self.$add(AddressMode::Immediate(Immediate::High(
                AddressReference::new(address_name),
            )))
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
    ($function_name:ident, $function_name_offset:ident, $add:ident) => {
        pub fn $function_name(&mut self, address: &str) -> &mut Self {
            self.$add(AddressMode::Absolute(AddressReference::new(address)))
        }
        pub fn $function_name_offset(&mut self, address: &str, offset: Address) -> &mut Self {
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

macro_rules! ind_x {
    ($function_name:ident, $add:ident) => {
        pub fn $function_name(&mut self, address: &str) -> &mut Self {
            self.$add(AddressMode::IndexedIndirect(AddressReference::new(address)))
        }
    };
}

macro_rules! ind_y {
    ($function_name:ident, $add:ident) => {
        pub fn $function_name(&mut self, address: &str) -> &mut Self {
            self.$add(AddressMode::IndirectIndexed(AddressReference::new(address)))
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
    add!(adc, ADC);
    imm!(adc_imm, adc_imm_low, adc_imm_high, adc);
    addr!(adc_addr, adc_addr_offs, adc);
    addr_x!(adc_addr_x, adc);
    addr_y!(adc_addr_y, adc);
    ind_x!(adc_ind_x, adc);
    ind_y!(adc_ind_y, adc);

    add!(and, AND);
    imm!(and_imm, and_imm_low, and_imm_high, adc);
    addr!(and_addr, and_addr_offs, and);
    addr_x!(and_addr_x, and);
    addr_y!(and_addy_y, and);
    ind_x!(and_ind_x, and);
    ind_y!(and_ind_y, and);

    add!(asl, ASL);
    acc!(asl_acc, asl);
    addr!(asl_addr, asl_addr_offs, asl);
    addr_x!(asl_addr_x, asl);

    add!(bcc, BCC);
    addr!(bcc_addr, bcc_addr_offs, bcc);

    add!(bcs, BCS);
    addr!(bcs_addr, bcs_addr_offs, bcs);

    add!(beq, BEQ);
    addr!(beq_addr, beq_addr_offs, beq);

    add!(bit, BIT);
    addr!(bit_addr, bit_addr_offs, bit);

    add!(bmi, BMI);
    addr!(bmi_addr, bmi_addr_offs, bmi);

    add!(bne, BNE);
    addr!(bne_addr, bne_addr_offs, bne);

    add!(bpl, BPL);
    addr!(bpl_addr, bpl_addr_offs, bpl);

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
    add!(lda, LDA);
    imm!(lda_imm, lda_imm_low, lda_imm_high, lda);
    addr!(lda_addr, lda_addr_offs, lda);
    addr_x!(lda_addr_x, lda);
    addr_y!(lda_addr_y, lda);
    ind_y!(lda_ind_y, lda);
    ind_x!(lda_ind_x, lda);
    add!(ldy, LDY);
    imm!(ldy_imm, ldy_imm_low, ldy_imm_high, ldy);
    add!(sta, STA);
    addr!(sta_addr, sta_addr_offs, sta);
    addr_x!(sta_addr_x, sta);
    addr_y!(sta_addr_y, sta);
    add!(jsr, JSR);
    addr!(jsr_addr, jsr_addr_offs, jsr);
    add!(cmp, CMP);
    addr!(cmp_addr, cmp_addr_offs, cmp);
    add!(cpy, CPY);
    addr!(cpy_addr, cpy_addr_offs, cpy);
    add!(cpx, CPX);
    addr!(cpx_addr, cpx_addr_offs, cpx);
    add!(sbc, SBC);
    imm!(sbc_imm, sbc_imm_low, sbc_imm_high, sbc);
    addr!(sbc_addr, sbc_addr_offs, sbc);

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
