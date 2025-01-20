use crate::{
    instruction::{operation::Operation, Instruction},
    memory::{
        address_mode::{AddressMode, Immediate},
        label::AddressReference,
        Address,
    },
};

/// Stream of instructions.
#[derive(Debug, Default, Clone)]
pub struct Instructions {
    pub instructions: Vec<Instruction>,
}

/// Utility to build instructions.
///
/// # Example
///
/// ```
/// use c64_assembler::builder::instruction::InstructionBuilder;
///
/// let instructions = InstructionBuilder::default()
///     .label("main_entry_point")
///     .lda_imm(0x00)
///     .sta_addr("VIC20_BORDER_COLOR")
///     .rts()
///     .finalize();
/// ```
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

    /// Record a new adc instruction with the given addressing mode.
    fn adc(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::ADC, addressing_mode);
        self
    }

    /// Record a adc instruction with data (byte).
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .adc_imm(0xC0)
    ///     .finalize();
    /// ```
    pub fn adc_imm(&mut self, byte: u8) -> &mut Self {
        self.adc(AddressMode::Immediate(Immediate::Byte(byte)))
    }

    /// Record a adc instruction with lower byte of an address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .adc_imm_low("test_data")
    ///     .label("test_data")
    ///     .finalize();
    /// ```
    pub fn adc_imm_low(&mut self, address_name: &str) -> &mut Self {
        self.adc(AddressMode::Immediate(Immediate::Low(AddressReference::new(
            address_name,
        ))))
    }

    /// Record a adc instruction with higher byte of an address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .adc_imm_high("test_data")
    ///     .label("test_data")
    ///     .finalize();
    /// ```
    pub fn adc_imm_high(&mut self, address_name: &str) -> &mut Self {
        self.adc(AddressMode::Immediate(Immediate::High(AddressReference::new(
            address_name,
        ))))
    }

    /// Record a adc instruction that use an absolute address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .adc_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn adc_addr(&mut self, address_name: &str) -> &mut Self {
        self.adc(AddressMode::Absolute(AddressReference::new(address_name)))
    }

    /// Record a adc instruction that use an absolute address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .adc_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn adc_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.adc(AddressMode::Absolute(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a adc instructon that use an absolute address with x-register as indexer.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0x08)
    ///     .adc_addr_x("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn adc_addr_x(&mut self, address_name: &str) -> &mut Self {
        self.adc(AddressMode::AbsoluteX(AddressReference::new(address_name)))
    }

    /// Record a adc instructon that use an absolute address with y-register as indexer.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0x08)
    ///     .adc_addr_y("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn adc_addr_y(&mut self, address_name: &str) -> &mut Self {
        self.adc(AddressMode::AbsoluteY(AddressReference::new(address_name)))
    }

    pub fn adc_ind_x(&mut self, address_name: &str) -> &mut Self {
        self.adc(AddressMode::IndexedIndirect(AddressReference::new(address_name)))
    }

    pub fn adc_ind_y(&mut self, address_name: &str) -> &mut Self {
        self.adc(AddressMode::IndirectIndexed(AddressReference::new(address_name)))
    }

    /// Record a new and instruction with the given addressing mode.
    fn and(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::AND, addressing_mode);
        self
    }

    /// Record a and instruction with data (byte).
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .and_imm(0xC0)
    ///     .finalize();
    /// ```
    pub fn and_imm(&mut self, byte: u8) -> &mut Self {
        self.and(AddressMode::Immediate(Immediate::Byte(byte)))
    }

    /// Record a and instruction with lower byte of an address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .and_imm_low("test_data")
    ///     .label("test_data")
    ///     .finalize();
    /// ```
    pub fn and_imm_low(&mut self, address_name: &str) -> &mut Self {
        self.and(AddressMode::Immediate(Immediate::Low(AddressReference::new(
            address_name,
        ))))
    }

    /// Record a and instruction with higher byte of an address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .and_imm_high("test_data")
    ///     .label("test_data")
    ///     .finalize();
    /// ```
    pub fn and_imm_high(&mut self, address_name: &str) -> &mut Self {
        self.and(AddressMode::Immediate(Immediate::High(AddressReference::new(
            address_name,
        ))))
    }

    /// Record a and instruction that use an absolute address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .and_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn and_addr(&mut self, address_name: &str) -> &mut Self {
        self.and(AddressMode::Absolute(AddressReference::new(address_name)))
    }

    /// Record a and instruction that use an absolute address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .and_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn and_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.and(AddressMode::Absolute(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a and instructon that use an absolute address with x-register as indexer.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0x08)
    ///     .and_addr_x("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn and_addr_x(&mut self, address_name: &str) -> &mut Self {
        self.and(AddressMode::AbsoluteX(AddressReference::new(address_name)))
    }

    /// Record a and instructon that use an absolute address with y-register as indexer.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0x08)
    ///     .and_addr_y("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn and_addr_y(&mut self, address_name: &str) -> &mut Self {
        self.and(AddressMode::AbsoluteY(AddressReference::new(address_name)))
    }

    pub fn and_ind_x(&mut self, address_name: &str) -> &mut Self {
        self.and(AddressMode::IndexedIndirect(AddressReference::new(address_name)))
    }

    pub fn and_ind_y(&mut self, address_name: &str) -> &mut Self {
        self.and(AddressMode::IndirectIndexed(AddressReference::new(address_name)))
    }

    /// Record a new asl instruction with the given addressing mode.
    fn asl(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::ASL, addressing_mode);
        self
    }

    /// Record a asl instruction that uses accumulator as address mode.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .asl_acc()
    ///     .finalize();
    /// ```
    pub fn asl_acc(&mut self) -> &mut Self {
        self.asl(AddressMode::Accumulator)
    }

    /// Record a asl instruction that use an absolute address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .asl_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn asl_addr(&mut self, address_name: &str) -> &mut Self {
        self.asl(AddressMode::Absolute(AddressReference::new(address_name)))
    }

    /// Record a asl instruction that use an absolute address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .asl_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn asl_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.asl(AddressMode::Absolute(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a asl instructon that use an absolute address with x-register as indexer.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0x08)
    ///     .asl_addr_x("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn asl_addr_x(&mut self, address_name: &str) -> &mut Self {
        self.asl(AddressMode::AbsoluteX(AddressReference::new(address_name)))
    }

    /// Record a new bcc instruction with the given addressing mode.
    fn bcc(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::BCC, addressing_mode);
        self
    }

    /// Record a bcc instruction that use  relativeeeeeeeee address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .bcc_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn bcc_addr(&mut self, address_name: &str) -> &mut Self {
        self.bcc(AddressMode::Relative(AddressReference::new(address_name)))
    }

    /// Record a bcc instruction that use a relative address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .bcc_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn bcc_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.bcc(AddressMode::Relative(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a new bcs instruction with the given addressing mode.
    fn bcs(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::BCS, addressing_mode);
        self
    }

    /// Record a bcs instruction that use  relativeeeeeeeee address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .bcs_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn bcs_addr(&mut self, address_name: &str) -> &mut Self {
        self.bcs(AddressMode::Relative(AddressReference::new(address_name)))
    }

    /// Record a bcs instruction that use a relative address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .bcs_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn bcs_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.bcs(AddressMode::Relative(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a new beq instruction with the given addressing mode.
    fn beq(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::BEQ, addressing_mode);
        self
    }

    /// Record a beq instruction that use  relativeeeeeeeee address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .beq_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn beq_addr(&mut self, address_name: &str) -> &mut Self {
        self.beq(AddressMode::Relative(AddressReference::new(address_name)))
    }

    /// Record a beq instruction that use a relative address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .beq_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn beq_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.beq(AddressMode::Relative(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a new bit instruction with the given addressing mode.
    fn bit(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::BIT, addressing_mode);
        self
    }

    /// Record a bit instruction that use an absolute address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .bit_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn bit_addr(&mut self, address_name: &str) -> &mut Self {
        self.bit(AddressMode::Absolute(AddressReference::new(address_name)))
    }

    /// Record a bit instruction that use an absolute address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .bit_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn bit_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.bit(AddressMode::Absolute(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a new bmi instruction with the given addressing mode.
    fn bmi(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::BMI, addressing_mode);
        self
    }

    /// Record a bmi instruction that use  relativeeeeeeeee address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .bmi_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn bmi_addr(&mut self, address_name: &str) -> &mut Self {
        self.bmi(AddressMode::Relative(AddressReference::new(address_name)))
    }

    /// Record a bmi instruction that use a relative address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .bmi_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn bmi_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.bmi(AddressMode::Relative(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a new bne instruction with the given addressing mode.
    fn bne(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::BNE, addressing_mode);
        self
    }

    /// Record a bne instruction that use  relativeeeeeeeee address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .bne_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn bne_addr(&mut self, address_name: &str) -> &mut Self {
        self.bne(AddressMode::Relative(AddressReference::new(address_name)))
    }

    /// Record a bne instruction that use a relative address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .bne_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn bne_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.bne(AddressMode::Relative(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a new bpl instruction with the given addressing mode.
    fn bpl(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::BPL, addressing_mode);
        self
    }

    /// Record a bpl instruction that use  relativeeeeeeeee address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .bpl_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn bpl_addr(&mut self, address_name: &str) -> &mut Self {
        self.bpl(AddressMode::Relative(AddressReference::new(address_name)))
    }

    /// Record a bpl instruction that use a relative address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .bpl_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn bpl_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.bpl(AddressMode::Relative(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a new brk instruction (addressing mode is implied).
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .brk()
    ///     .finalize();
    /// ```
    pub fn brk(&mut self) -> &mut Self {
        self.add_instruction(Operation::BRK, AddressMode::Implied);
        self
    }

    /// Record a new bvc instruction with the given addressing mode.
    fn bvc(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::BVC, addressing_mode);
        self
    }

    /// Record a bvc instruction that use  relativeeeeeeeee address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .bvc_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn bvc_addr(&mut self, address_name: &str) -> &mut Self {
        self.bvc(AddressMode::Relative(AddressReference::new(address_name)))
    }

    /// Record a bvc instruction that use a relative address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .bvc_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn bvc_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.bvc(AddressMode::Relative(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a new bvs instruction with the given addressing mode.
    fn bvs(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::BVS, addressing_mode);
        self
    }

    /// Record a bvs instruction that use  relativeeeeeeeee address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .bvs_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn bvs_addr(&mut self, address_name: &str) -> &mut Self {
        self.bvs(AddressMode::Relative(AddressReference::new(address_name)))
    }

    /// Record a bvs instruction that use a relative address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .bvs_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn bvs_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.bvs(AddressMode::Relative(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a new clc instruction (addressing mode is implied).
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .clc()
    ///     .finalize();
    /// ```
    pub fn clc(&mut self) -> &mut Self {
        self.add_instruction(Operation::CLC, AddressMode::Implied);
        self
    }

    /// Record a new cld instruction (addressing mode is implied).
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .cld()
    ///     .finalize();
    /// ```
    pub fn cld(&mut self) -> &mut Self {
        self.add_instruction(Operation::CLD, AddressMode::Implied);
        self
    }

    /// Record a new cli instruction (addressing mode is implied).
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .cli()
    ///     .finalize();
    /// ```
    pub fn cli(&mut self) -> &mut Self {
        self.add_instruction(Operation::CLI, AddressMode::Implied);
        self
    }

    /// Record a new clv instruction (addressing mode is implied).
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .clv()
    ///     .finalize();
    /// ```
    pub fn clv(&mut self) -> &mut Self {
        self.add_instruction(Operation::CLV, AddressMode::Implied);
        self
    }

    /// Record a new cmp instruction with the given addressing mode.
    fn cmp(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::CMP, addressing_mode);
        self
    }

    /// Record a cmp instruction with data (byte).
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .cmp_imm(0xC0)
    ///     .finalize();
    /// ```
    pub fn cmp_imm(&mut self, byte: u8) -> &mut Self {
        self.cmp(AddressMode::Immediate(Immediate::Byte(byte)))
    }

    /// Record a cmp instruction with lower byte of an address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .cmp_imm_low("test_data")
    ///     .label("test_data")
    ///     .finalize();
    /// ```
    pub fn cmp_imm_low(&mut self, address_name: &str) -> &mut Self {
        self.cmp(AddressMode::Immediate(Immediate::Low(AddressReference::new(
            address_name,
        ))))
    }

    /// Record a cmp instruction with higher byte of an address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .cmp_imm_high("test_data")
    ///     .label("test_data")
    ///     .finalize();
    /// ```
    pub fn cmp_imm_high(&mut self, address_name: &str) -> &mut Self {
        self.cmp(AddressMode::Immediate(Immediate::High(AddressReference::new(
            address_name,
        ))))
    }

    /// Record a cmp instruction that use an absolute address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .cmp_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn cmp_addr(&mut self, address_name: &str) -> &mut Self {
        self.cmp(AddressMode::Absolute(AddressReference::new(address_name)))
    }

    /// Record a cmp instruction that use an absolute address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .cmp_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn cmp_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.cmp(AddressMode::Absolute(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a cmp instructon that use an absolute address with x-register as indexer.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0x08)
    ///     .cmp_addr_x("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn cmp_addr_x(&mut self, address_name: &str) -> &mut Self {
        self.cmp(AddressMode::AbsoluteX(AddressReference::new(address_name)))
    }

    /// Record a cmp instructon that use an absolute address with y-register as indexer.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0x08)
    ///     .cmp_addr_y("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn cmp_addr_y(&mut self, address_name: &str) -> &mut Self {
        self.cmp(AddressMode::AbsoluteY(AddressReference::new(address_name)))
    }

    pub fn cmp_ind_x(&mut self, address_name: &str) -> &mut Self {
        self.cmp(AddressMode::IndexedIndirect(AddressReference::new(address_name)))
    }

    pub fn cmp_ind_y(&mut self, address_name: &str) -> &mut Self {
        self.cmp(AddressMode::IndirectIndexed(AddressReference::new(address_name)))
    }

    /// Record a new cpx instruction with the given addressing mode.
    fn cpx(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::CPX, addressing_mode);
        self
    }

    /// Record a cpx instruction with data (byte).
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .cpx_imm(0xC0)
    ///     .finalize();
    /// ```
    pub fn cpx_imm(&mut self, byte: u8) -> &mut Self {
        self.cpx(AddressMode::Immediate(Immediate::Byte(byte)))
    }

    /// Record a cpx instruction with lower byte of an address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .cpx_imm_low("test_data")
    ///     .label("test_data")
    ///     .finalize();
    /// ```
    pub fn cpx_imm_low(&mut self, address_name: &str) -> &mut Self {
        self.cpx(AddressMode::Immediate(Immediate::Low(AddressReference::new(
            address_name,
        ))))
    }

    /// Record a cpx instruction with higher byte of an address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .cpx_imm_high("test_data")
    ///     .label("test_data")
    ///     .finalize();
    /// ```
    pub fn cpx_imm_high(&mut self, address_name: &str) -> &mut Self {
        self.cpx(AddressMode::Immediate(Immediate::High(AddressReference::new(
            address_name,
        ))))
    }

    /// Record a cpx instruction that use an absolute address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .cpx_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn cpx_addr(&mut self, address_name: &str) -> &mut Self {
        self.cpx(AddressMode::Absolute(AddressReference::new(address_name)))
    }

    /// Record a cpx instruction that use an absolute address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .cpx_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn cpx_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.cpx(AddressMode::Absolute(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a new cpy instruction with the given addressing mode.
    fn cpy(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::CPY, addressing_mode);
        self
    }

    /// Record a cpy instruction with data (byte).
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .cpy_imm(0xC0)
    ///     .finalize();
    /// ```
    pub fn cpy_imm(&mut self, byte: u8) -> &mut Self {
        self.cpy(AddressMode::Immediate(Immediate::Byte(byte)))
    }

    /// Record a cpy instruction with lower byte of an address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .cpy_imm_low("test_data")
    ///     .label("test_data")
    ///     .finalize();
    /// ```
    pub fn cpy_imm_low(&mut self, address_name: &str) -> &mut Self {
        self.cpy(AddressMode::Immediate(Immediate::Low(AddressReference::new(
            address_name,
        ))))
    }

    /// Record a cpy instruction with higher byte of an address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .cpy_imm_high("test_data")
    ///     .label("test_data")
    ///     .finalize();
    /// ```
    pub fn cpy_imm_high(&mut self, address_name: &str) -> &mut Self {
        self.cpy(AddressMode::Immediate(Immediate::High(AddressReference::new(
            address_name,
        ))))
    }

    /// Record a cpy instruction that use an absolute address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .cpy_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn cpy_addr(&mut self, address_name: &str) -> &mut Self {
        self.cpy(AddressMode::Absolute(AddressReference::new(address_name)))
    }

    /// Record a cpy instruction that use an absolute address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .cpy_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn cpy_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.cpy(AddressMode::Absolute(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a new dec instruction with the given addressing mode.
    fn dec(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::DEC, addressing_mode);
        self
    }

    /// Record a dec instruction that use an absolute address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .dec_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn dec_addr(&mut self, address_name: &str) -> &mut Self {
        self.dec(AddressMode::Absolute(AddressReference::new(address_name)))
    }

    /// Record a dec instruction that use an absolute address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .dec_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn dec_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.dec(AddressMode::Absolute(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a dec instructon that use an absolute address with x-register as indexer.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0x08)
    ///     .dec_addr_x("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn dec_addr_x(&mut self, address_name: &str) -> &mut Self {
        self.dec(AddressMode::AbsoluteX(AddressReference::new(address_name)))
    }

    /// Record a new dex instruction (addressing mode is implied).
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .dex()
    ///     .finalize();
    /// ```
    pub fn dex(&mut self) -> &mut Self {
        self.add_instruction(Operation::DEX, AddressMode::Implied);
        self
    }

    /// Record a new dey instruction (addressing mode is implied).
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .dey()
    ///     .finalize();
    /// ```
    pub fn dey(&mut self) -> &mut Self {
        self.add_instruction(Operation::DEY, AddressMode::Implied);
        self
    }

    /// Record a new eor instruction with the given addressing mode.
    fn eor(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::EOR, addressing_mode);
        self
    }

    /// Record a eor instruction with data (byte).
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .eor_imm(0xC0)
    ///     .finalize();
    /// ```
    pub fn eor_imm(&mut self, byte: u8) -> &mut Self {
        self.eor(AddressMode::Immediate(Immediate::Byte(byte)))
    }

    /// Record a eor instruction with lower byte of an address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .eor_imm_low("test_data")
    ///     .label("test_data")
    ///     .finalize();
    /// ```
    pub fn eor_imm_low(&mut self, address_name: &str) -> &mut Self {
        self.eor(AddressMode::Immediate(Immediate::Low(AddressReference::new(
            address_name,
        ))))
    }

    /// Record a eor instruction with higher byte of an address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .eor_imm_high("test_data")
    ///     .label("test_data")
    ///     .finalize();
    /// ```
    pub fn eor_imm_high(&mut self, address_name: &str) -> &mut Self {
        self.eor(AddressMode::Immediate(Immediate::High(AddressReference::new(
            address_name,
        ))))
    }

    /// Record a eor instruction that use an absolute address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .eor_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn eor_addr(&mut self, address_name: &str) -> &mut Self {
        self.eor(AddressMode::Absolute(AddressReference::new(address_name)))
    }

    /// Record a eor instruction that use an absolute address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .eor_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn eor_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.eor(AddressMode::Absolute(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a eor instructon that use an absolute address with x-register as indexer.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0x08)
    ///     .eor_addr_x("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn eor_addr_x(&mut self, address_name: &str) -> &mut Self {
        self.eor(AddressMode::AbsoluteX(AddressReference::new(address_name)))
    }

    /// Record a eor instructon that use an absolute address with y-register as indexer.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0x08)
    ///     .eor_addr_y("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn eor_addr_y(&mut self, address_name: &str) -> &mut Self {
        self.eor(AddressMode::AbsoluteY(AddressReference::new(address_name)))
    }

    pub fn eor_ind_x(&mut self, address_name: &str) -> &mut Self {
        self.eor(AddressMode::IndexedIndirect(AddressReference::new(address_name)))
    }

    pub fn eor_ind_y(&mut self, address_name: &str) -> &mut Self {
        self.eor(AddressMode::IndirectIndexed(AddressReference::new(address_name)))
    }

    /// Record a new inc instruction with the given addressing mode.
    fn inc(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::INC, addressing_mode);
        self
    }

    /// Record a inc instruction that use an absolute address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .inc_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn inc_addr(&mut self, address_name: &str) -> &mut Self {
        self.inc(AddressMode::Absolute(AddressReference::new(address_name)))
    }

    /// Record a inc instruction that use an absolute address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .inc_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn inc_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.inc(AddressMode::Absolute(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a inc instructon that use an absolute address with x-register as indexer.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0x08)
    ///     .inc_addr_x("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn inc_addr_x(&mut self, address_name: &str) -> &mut Self {
        self.inc(AddressMode::AbsoluteX(AddressReference::new(address_name)))
    }

    /// Record a new inx instruction (addressing mode is implied).
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .inx()
    ///     .finalize();
    /// ```
    pub fn inx(&mut self) -> &mut Self {
        self.add_instruction(Operation::INX, AddressMode::Implied);
        self
    }

    /// Record a new iny instruction (addressing mode is implied).
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .iny()
    ///     .finalize();
    /// ```
    pub fn iny(&mut self) -> &mut Self {
        self.add_instruction(Operation::INY, AddressMode::Implied);
        self
    }

    /// Record a new jmp instruction with the given addressing mode.
    fn jmp(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::JMP, addressing_mode);
        self
    }

    /// Record a jmp instruction that use an absolute address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .jmp_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn jmp_addr(&mut self, address_name: &str) -> &mut Self {
        self.jmp(AddressMode::Absolute(AddressReference::new(address_name)))
    }

    /// Record a jmp instruction that use an absolute address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .jmp_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn jmp_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.jmp(AddressMode::Absolute(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    pub fn jmp_ind(&mut self, address_name: &str) -> &mut Self {
        self.jmp(AddressMode::Indirect(AddressReference::new(address_name)))
    }

    /// Record a new jsr instruction with the given addressing mode.
    fn jsr(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::JSR, addressing_mode);
        self
    }

    /// Record a jsr instruction that use an absolute address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .jsr_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn jsr_addr(&mut self, address_name: &str) -> &mut Self {
        self.jsr(AddressMode::Absolute(AddressReference::new(address_name)))
    }

    /// Record a jsr instruction that use an absolute address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .jsr_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn jsr_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.jsr(AddressMode::Absolute(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a new lda instruction with the given addressing mode.
    fn lda(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::LDA, addressing_mode);
        self
    }

    /// Record a lda instruction with data (byte).
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0xC0)
    ///     .finalize();
    /// ```
    pub fn lda_imm(&mut self, byte: u8) -> &mut Self {
        self.lda(AddressMode::Immediate(Immediate::Byte(byte)))
    }

    /// Record a lda instruction with lower byte of an address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm_low("test_data")
    ///     .label("test_data")
    ///     .finalize();
    /// ```
    pub fn lda_imm_low(&mut self, address_name: &str) -> &mut Self {
        self.lda(AddressMode::Immediate(Immediate::Low(AddressReference::new(
            address_name,
        ))))
    }

    /// Record a lda instruction with higher byte of an address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm_high("test_data")
    ///     .label("test_data")
    ///     .finalize();
    /// ```
    pub fn lda_imm_high(&mut self, address_name: &str) -> &mut Self {
        self.lda(AddressMode::Immediate(Immediate::High(AddressReference::new(
            address_name,
        ))))
    }

    /// Record a lda instruction that use an absolute address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn lda_addr(&mut self, address_name: &str) -> &mut Self {
        self.lda(AddressMode::Absolute(AddressReference::new(address_name)))
    }

    /// Record a lda instruction that use an absolute address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn lda_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.lda(AddressMode::Absolute(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a lda instructon that use an absolute address with x-register as indexer.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0x08)
    ///     .lda_addr_x("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn lda_addr_x(&mut self, address_name: &str) -> &mut Self {
        self.lda(AddressMode::AbsoluteX(AddressReference::new(address_name)))
    }

    /// Record a lda instructon that use an absolute address with y-register as indexer.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0x08)
    ///     .lda_addr_y("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn lda_addr_y(&mut self, address_name: &str) -> &mut Self {
        self.lda(AddressMode::AbsoluteY(AddressReference::new(address_name)))
    }

    pub fn lda_ind_x(&mut self, address_name: &str) -> &mut Self {
        self.lda(AddressMode::IndexedIndirect(AddressReference::new(address_name)))
    }

    pub fn lda_ind_y(&mut self, address_name: &str) -> &mut Self {
        self.lda(AddressMode::IndirectIndexed(AddressReference::new(address_name)))
    }

    /// Record a new ldx instruction with the given addressing mode.
    fn ldx(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::LDX, addressing_mode);
        self
    }

    /// Record a ldx instruction with data (byte).
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .ldx_imm(0xC0)
    ///     .finalize();
    /// ```
    pub fn ldx_imm(&mut self, byte: u8) -> &mut Self {
        self.ldx(AddressMode::Immediate(Immediate::Byte(byte)))
    }

    /// Record a ldx instruction with lower byte of an address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .ldx_imm_low("test_data")
    ///     .label("test_data")
    ///     .finalize();
    /// ```
    pub fn ldx_imm_low(&mut self, address_name: &str) -> &mut Self {
        self.ldx(AddressMode::Immediate(Immediate::Low(AddressReference::new(
            address_name,
        ))))
    }

    /// Record a ldx instruction with higher byte of an address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .ldx_imm_high("test_data")
    ///     .label("test_data")
    ///     .finalize();
    /// ```
    pub fn ldx_imm_high(&mut self, address_name: &str) -> &mut Self {
        self.ldx(AddressMode::Immediate(Immediate::High(AddressReference::new(
            address_name,
        ))))
    }

    /// Record a ldx instruction that use an absolute address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .ldx_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn ldx_addr(&mut self, address_name: &str) -> &mut Self {
        self.ldx(AddressMode::Absolute(AddressReference::new(address_name)))
    }

    /// Record a ldx instruction that use an absolute address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .ldx_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn ldx_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.ldx(AddressMode::Absolute(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a ldx instructon that use an absolute address with y-register as indexer.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0x08)
    ///     .ldx_addr_y("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn ldx_addr_y(&mut self, address_name: &str) -> &mut Self {
        self.ldx(AddressMode::AbsoluteY(AddressReference::new(address_name)))
    }

    /// Record a new ldy instruction with the given addressing mode.
    fn ldy(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::LDY, addressing_mode);
        self
    }

    /// Record a ldy instruction with data (byte).
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .ldy_imm(0xC0)
    ///     .finalize();
    /// ```
    pub fn ldy_imm(&mut self, byte: u8) -> &mut Self {
        self.ldy(AddressMode::Immediate(Immediate::Byte(byte)))
    }

    /// Record a ldy instruction with lower byte of an address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .ldy_imm_low("test_data")
    ///     .label("test_data")
    ///     .finalize();
    /// ```
    pub fn ldy_imm_low(&mut self, address_name: &str) -> &mut Self {
        self.ldy(AddressMode::Immediate(Immediate::Low(AddressReference::new(
            address_name,
        ))))
    }

    /// Record a ldy instruction with higher byte of an address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .ldy_imm_high("test_data")
    ///     .label("test_data")
    ///     .finalize();
    /// ```
    pub fn ldy_imm_high(&mut self, address_name: &str) -> &mut Self {
        self.ldy(AddressMode::Immediate(Immediate::High(AddressReference::new(
            address_name,
        ))))
    }

    /// Record a ldy instruction that use an absolute address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .ldy_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn ldy_addr(&mut self, address_name: &str) -> &mut Self {
        self.ldy(AddressMode::Absolute(AddressReference::new(address_name)))
    }

    /// Record a ldy instruction that use an absolute address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .ldy_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn ldy_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.ldy(AddressMode::Absolute(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a ldy instructon that use an absolute address with x-register as indexer.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0x08)
    ///     .ldy_addr_x("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn ldy_addr_x(&mut self, address_name: &str) -> &mut Self {
        self.ldy(AddressMode::AbsoluteX(AddressReference::new(address_name)))
    }

    /// Record a new lsr instruction with the given addressing mode.
    fn lsr(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::LSR, addressing_mode);
        self
    }

    /// Record a lsr instruction that uses accumulator as address mode.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lsr_acc()
    ///     .finalize();
    /// ```
    pub fn lsr_acc(&mut self) -> &mut Self {
        self.lsr(AddressMode::Accumulator)
    }

    /// Record a lsr instruction that use an absolute address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lsr_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn lsr_addr(&mut self, address_name: &str) -> &mut Self {
        self.lsr(AddressMode::Absolute(AddressReference::new(address_name)))
    }

    /// Record a lsr instruction that use an absolute address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lsr_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn lsr_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.lsr(AddressMode::Absolute(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a lsr instructon that use an absolute address with x-register as indexer.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0x08)
    ///     .lsr_addr_x("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn lsr_addr_x(&mut self, address_name: &str) -> &mut Self {
        self.lsr(AddressMode::AbsoluteX(AddressReference::new(address_name)))
    }

    /// Record a new nop instruction (addressing mode is implied).
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .nop()
    ///     .finalize();
    /// ```
    pub fn nop(&mut self) -> &mut Self {
        self.add_instruction(Operation::NOP, AddressMode::Implied);
        self
    }

    /// Record a new ora instruction with the given addressing mode.
    fn ora(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::ORA, addressing_mode);
        self
    }

    /// Record a ora instruction with data (byte).
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .ora_imm(0xC0)
    ///     .finalize();
    /// ```
    pub fn ora_imm(&mut self, byte: u8) -> &mut Self {
        self.ora(AddressMode::Immediate(Immediate::Byte(byte)))
    }

    /// Record a ora instruction with lower byte of an address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .ora_imm_low("test_data")
    ///     .label("test_data")
    ///     .finalize();
    /// ```
    pub fn ora_imm_low(&mut self, address_name: &str) -> &mut Self {
        self.ora(AddressMode::Immediate(Immediate::Low(AddressReference::new(
            address_name,
        ))))
    }

    /// Record a ora instruction with higher byte of an address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .ora_imm_high("test_data")
    ///     .label("test_data")
    ///     .finalize();
    /// ```
    pub fn ora_imm_high(&mut self, address_name: &str) -> &mut Self {
        self.ora(AddressMode::Immediate(Immediate::High(AddressReference::new(
            address_name,
        ))))
    }

    /// Record a ora instruction that use an absolute address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .ora_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn ora_addr(&mut self, address_name: &str) -> &mut Self {
        self.ora(AddressMode::Absolute(AddressReference::new(address_name)))
    }

    /// Record a ora instruction that use an absolute address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .ora_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn ora_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.ora(AddressMode::Absolute(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a ora instructon that use an absolute address with x-register as indexer.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0x08)
    ///     .ora_addr_x("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn ora_addr_x(&mut self, address_name: &str) -> &mut Self {
        self.ora(AddressMode::AbsoluteX(AddressReference::new(address_name)))
    }

    /// Record a ora instructon that use an absolute address with y-register as indexer.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0x08)
    ///     .ora_addr_y("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn ora_addr_y(&mut self, address_name: &str) -> &mut Self {
        self.ora(AddressMode::AbsoluteY(AddressReference::new(address_name)))
    }

    pub fn ora_ind_x(&mut self, address_name: &str) -> &mut Self {
        self.ora(AddressMode::IndexedIndirect(AddressReference::new(address_name)))
    }

    pub fn ora_ind_y(&mut self, address_name: &str) -> &mut Self {
        self.ora(AddressMode::IndirectIndexed(AddressReference::new(address_name)))
    }

    /// Record a new pha instruction (addressing mode is implied).
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .pha()
    ///     .finalize();
    /// ```
    pub fn pha(&mut self) -> &mut Self {
        self.add_instruction(Operation::PHA, AddressMode::Implied);
        self
    }

    /// Record a new php instruction (addressing mode is implied).
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .php()
    ///     .finalize();
    /// ```
    pub fn php(&mut self) -> &mut Self {
        self.add_instruction(Operation::PHP, AddressMode::Implied);
        self
    }

    /// Record a new pla instruction (addressing mode is implied).
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .pla()
    ///     .finalize();
    /// ```
    pub fn pla(&mut self) -> &mut Self {
        self.add_instruction(Operation::PLA, AddressMode::Implied);
        self
    }

    /// Record a new plp instruction (addressing mode is implied).
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .plp()
    ///     .finalize();
    /// ```
    pub fn plp(&mut self) -> &mut Self {
        self.add_instruction(Operation::PLP, AddressMode::Implied);
        self
    }

    /// Record a new rol instruction with the given addressing mode.
    fn rol(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::ROL, addressing_mode);
        self
    }

    /// Record a rol instruction that uses accumulator as address mode.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .rol_acc()
    ///     .finalize();
    /// ```
    pub fn rol_acc(&mut self) -> &mut Self {
        self.rol(AddressMode::Accumulator)
    }

    /// Record a rol instruction that use an absolute address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .rol_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn rol_addr(&mut self, address_name: &str) -> &mut Self {
        self.rol(AddressMode::Absolute(AddressReference::new(address_name)))
    }

    /// Record a rol instruction that use an absolute address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .rol_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn rol_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.rol(AddressMode::Absolute(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a rol instructon that use an absolute address with x-register as indexer.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0x08)
    ///     .rol_addr_x("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn rol_addr_x(&mut self, address_name: &str) -> &mut Self {
        self.rol(AddressMode::AbsoluteX(AddressReference::new(address_name)))
    }

    /// Record a new ror instruction with the given addressing mode.
    fn ror(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::ROR, addressing_mode);
        self
    }

    /// Record a ror instruction that uses accumulator as address mode.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .ror_acc()
    ///     .finalize();
    /// ```
    pub fn ror_acc(&mut self) -> &mut Self {
        self.ror(AddressMode::Accumulator)
    }

    /// Record a ror instruction that use an absolute address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .ror_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn ror_addr(&mut self, address_name: &str) -> &mut Self {
        self.ror(AddressMode::Absolute(AddressReference::new(address_name)))
    }

    /// Record a ror instruction that use an absolute address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .ror_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn ror_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.ror(AddressMode::Absolute(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a ror instructon that use an absolute address with x-register as indexer.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0x08)
    ///     .ror_addr_x("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn ror_addr_x(&mut self, address_name: &str) -> &mut Self {
        self.ror(AddressMode::AbsoluteX(AddressReference::new(address_name)))
    }

    /// Record a new rti instruction (addressing mode is implied).
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .rti()
    ///     .finalize();
    /// ```
    pub fn rti(&mut self) -> &mut Self {
        self.add_instruction(Operation::RTI, AddressMode::Implied);
        self
    }

    /// Record a new rts instruction (addressing mode is implied).
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .rts()
    ///     .finalize();
    /// ```
    pub fn rts(&mut self) -> &mut Self {
        self.add_instruction(Operation::RTS, AddressMode::Implied);
        self
    }

    /// Record a new sbc instruction with the given addressing mode.
    fn sbc(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::SBC, addressing_mode);
        self
    }

    /// Record a sbc instruction with data (byte).
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .sbc_imm(0xC0)
    ///     .finalize();
    /// ```
    pub fn sbc_imm(&mut self, byte: u8) -> &mut Self {
        self.sbc(AddressMode::Immediate(Immediate::Byte(byte)))
    }

    /// Record a sbc instruction with lower byte of an address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .sbc_imm_low("test_data")
    ///     .label("test_data")
    ///     .finalize();
    /// ```
    pub fn sbc_imm_low(&mut self, address_name: &str) -> &mut Self {
        self.sbc(AddressMode::Immediate(Immediate::Low(AddressReference::new(
            address_name,
        ))))
    }

    /// Record a sbc instruction with higher byte of an address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .sbc_imm_high("test_data")
    ///     .label("test_data")
    ///     .finalize();
    /// ```
    pub fn sbc_imm_high(&mut self, address_name: &str) -> &mut Self {
        self.sbc(AddressMode::Immediate(Immediate::High(AddressReference::new(
            address_name,
        ))))
    }

    /// Record a sbc instruction that use an absolute address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .sbc_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn sbc_addr(&mut self, address_name: &str) -> &mut Self {
        self.sbc(AddressMode::Absolute(AddressReference::new(address_name)))
    }

    /// Record a sbc instruction that use an absolute address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .sbc_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn sbc_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.sbc(AddressMode::Absolute(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a sbc instructon that use an absolute address with x-register as indexer.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0x08)
    ///     .sbc_addr_x("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn sbc_addr_x(&mut self, address_name: &str) -> &mut Self {
        self.sbc(AddressMode::AbsoluteX(AddressReference::new(address_name)))
    }

    /// Record a sbc instructon that use an absolute address with y-register as indexer.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0x08)
    ///     .sbc_addr_y("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn sbc_addr_y(&mut self, address_name: &str) -> &mut Self {
        self.sbc(AddressMode::AbsoluteY(AddressReference::new(address_name)))
    }

    pub fn sbc_ind_x(&mut self, address_name: &str) -> &mut Self {
        self.sbc(AddressMode::IndexedIndirect(AddressReference::new(address_name)))
    }

    pub fn sbc_ind_y(&mut self, address_name: &str) -> &mut Self {
        self.sbc(AddressMode::IndirectIndexed(AddressReference::new(address_name)))
    }

    /// Record a new sec instruction (addressing mode is implied).
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .sec()
    ///     .finalize();
    /// ```
    pub fn sec(&mut self) -> &mut Self {
        self.add_instruction(Operation::SEC, AddressMode::Implied);
        self
    }

    /// Record a new sed instruction (addressing mode is implied).
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .sed()
    ///     .finalize();
    /// ```
    pub fn sed(&mut self) -> &mut Self {
        self.add_instruction(Operation::SED, AddressMode::Implied);
        self
    }

    /// Record a new sei instruction (addressing mode is implied).
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .sei()
    ///     .finalize();
    /// ```
    pub fn sei(&mut self) -> &mut Self {
        self.add_instruction(Operation::SEI, AddressMode::Implied);
        self
    }

    /// Record a new sta instruction with the given addressing mode.
    fn sta(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::STA, addressing_mode);
        self
    }

    /// Record a sta instruction that use an absolute address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .sta_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn sta_addr(&mut self, address_name: &str) -> &mut Self {
        self.sta(AddressMode::Absolute(AddressReference::new(address_name)))
    }

    /// Record a sta instruction that use an absolute address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .sta_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn sta_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.sta(AddressMode::Absolute(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a sta instructon that use an absolute address with x-register as indexer.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0x08)
    ///     .sta_addr_x("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn sta_addr_x(&mut self, address_name: &str) -> &mut Self {
        self.sta(AddressMode::AbsoluteX(AddressReference::new(address_name)))
    }

    /// Record a sta instructon that use an absolute address with y-register as indexer.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0x08)
    ///     .sta_addr_y("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn sta_addr_y(&mut self, address_name: &str) -> &mut Self {
        self.sta(AddressMode::AbsoluteY(AddressReference::new(address_name)))
    }

    pub fn sta_ind_x(&mut self, address_name: &str) -> &mut Self {
        self.sta(AddressMode::IndexedIndirect(AddressReference::new(address_name)))
    }

    pub fn sta_ind_y(&mut self, address_name: &str) -> &mut Self {
        self.sta(AddressMode::IndirectIndexed(AddressReference::new(address_name)))
    }

    /// Record a new stx instruction with the given addressing mode.
    fn stx(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::STX, addressing_mode);
        self
    }

    /// Record a stx instruction that use an absolute address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .stx_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn stx_addr(&mut self, address_name: &str) -> &mut Self {
        self.stx(AddressMode::Absolute(AddressReference::new(address_name)))
    }

    /// Record a stx instruction that use an absolute address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .stx_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn stx_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.stx(AddressMode::Absolute(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a stx instructon that use an absolute address with y-register as indexer.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0x08)
    ///     .stx_addr_y("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn stx_addr_y(&mut self, address_name: &str) -> &mut Self {
        self.stx(AddressMode::AbsoluteY(AddressReference::new(address_name)))
    }

    /// Record a new sty instruction with the given addressing mode.
    fn sty(&mut self, addressing_mode: AddressMode) -> &mut Self {
        self.add_instruction(Operation::STY, addressing_mode);
        self
    }

    /// Record a sty instruction that use an absolute address.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .sty_addr("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn sty_addr(&mut self, address_name: &str) -> &mut Self {
        self.sty(AddressMode::Absolute(AddressReference::new(address_name)))
    }

    /// Record a sty instruction that use an absolute address with an offset.
    /// Offset is in bytes.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .sty_addr_offs("test_label", 8)
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn sty_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {
        self.sty(AddressMode::Absolute(AddressReference::with_offset(
            address_name,
            offset,
        )))
    }

    /// Record a sty instructon that use an absolute address with x-register as indexer.
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .lda_imm(0x08)
    ///     .sty_addr_x("test_label")
    ///     .label("test_label")
    ///     .finalize();
    /// ```
    pub fn sty_addr_x(&mut self, address_name: &str) -> &mut Self {
        self.sty(AddressMode::AbsoluteX(AddressReference::new(address_name)))
    }

    /// Record a new tax instruction (addressing mode is implied).
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .tax()
    ///     .finalize();
    /// ```
    pub fn tax(&mut self) -> &mut Self {
        self.add_instruction(Operation::TAX, AddressMode::Implied);
        self
    }

    /// Record a new tay instruction (addressing mode is implied).
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .tay()
    ///     .finalize();
    /// ```
    pub fn tay(&mut self) -> &mut Self {
        self.add_instruction(Operation::TAY, AddressMode::Implied);
        self
    }

    /// Record a new tsx instruction (addressing mode is implied).
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .tsx()
    ///     .finalize();
    /// ```
    pub fn tsx(&mut self) -> &mut Self {
        self.add_instruction(Operation::TSX, AddressMode::Implied);
        self
    }

    /// Record a new txa instruction (addressing mode is implied).
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .txa()
    ///     .finalize();
    /// ```
    pub fn txa(&mut self) -> &mut Self {
        self.add_instruction(Operation::TXA, AddressMode::Implied);
        self
    }

    /// Record a new txs instruction (addressing mode is implied).
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .txs()
    ///     .finalize();
    /// ```
    pub fn txs(&mut self) -> &mut Self {
        self.add_instruction(Operation::TXS, AddressMode::Implied);
        self
    }

    /// Record a new tya instruction (addressing mode is implied).
    ///
    /// # Example
    /// ```
    /// use c64_assembler::builder::instruction::InstructionBuilder;
    /// let instructions = InstructionBuilder::default()
    ///     .tya()
    ///     .finalize();
    /// ```
    pub fn tya(&mut self) -> &mut Self {
        self.add_instruction(Operation::TYA, AddressMode::Implied);
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
