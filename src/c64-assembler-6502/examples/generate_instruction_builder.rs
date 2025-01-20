use c64_assembler_6502::isa::{
    isa_6502, NO_ABSOLUTE, NO_ABSOLUTE_X, NO_ABSOLUTE_Y, NO_ACCUMULATOR, NO_IMMEDIATE, NO_IMPLIED, NO_INDEXED_INDIRECT,
    NO_INDIRECT, NO_INDIRECT_INDEXED, NO_RELATIVE, NO_ZEROPAGE, NO_ZEROPAGE_X, NO_ZEROPAGE_Y,
};

fn main() {
    let mut lines = Vec::<String>::default();

    for def in isa_6502() {
        if def.implied != NO_IMPLIED {
            lines.push(format!(
                "
                /// Record a new {0} instruction (addressing mode is implied).
                /// 
                /// # Example
                /// ```
                /// use c64_assembler::builder::instruction::InstructionBuilder;
                /// let instructions = InstructionBuilder::default()
                ///     .{0}()
                ///     .finalize();
                /// ```
                pub fn {0}(&mut self) -> &mut Self {{
                    self.add_instruction(Operation::{1}, AddressMode::Implied);
                    self
                }}",
                def.instruction.to_string(),
                def.instruction.to_uppercase()
            ));
        } else {
            lines.push(format!(
                "
                /// Record a new {0} instruction with the given addressing mode.
                fn {0}(&mut self, addressing_mode: AddressMode) -> &mut Self {{
                    self.add_instruction(Operation::{1}, addressing_mode);
                    self
                }}
                ",
                def.instruction.to_string(),
                def.instruction.to_uppercase()
            ));
        }
        if def.immediate != NO_IMMEDIATE {
            lines.push(format!(
                "
                /// Record a {0} instruction with data (byte).
                /// # Example
                /// ```
                /// use c64_assembler::builder::instruction::InstructionBuilder;
                /// let instructions = InstructionBuilder::default()
                ///     .{0}_imm(0xC0)
                ///     .finalize();
                /// ```
                pub fn {0}_imm(&mut self, byte: u8) -> &mut Self {{
                    self.{0}(AddressMode::Immediate(Immediate::Byte(byte)))
                }}

                /// Record a {0} instruction with lower byte of an address.
                /// 
                /// # Example
                /// ```
                /// use c64_assembler::builder::instruction::InstructionBuilder;
                /// let instructions = InstructionBuilder::default()
                ///     .{0}_imm_low(\"test_data\")
                ///     .label(\"test_data\")
                ///     .finalize();
                /// ```
                pub fn {0}_imm_low(&mut self, address_name: &str) -> &mut Self {{
                    self.{0}(AddressMode::Immediate(Immediate::Low(
                        AddressReference::new(address_name)
                    )))
                }}

                /// Record a {0} instruction with higher byte of an address.
                /// 
                /// # Example
                /// ```
                /// use c64_assembler::builder::instruction::InstructionBuilder;
                /// let instructions = InstructionBuilder::default()
                ///     .{0}_imm_high(\"test_data\")
                ///     .label(\"test_data\")
                ///     .finalize();
                /// ```
                pub fn {0}_imm_high(&mut self, address_name: &str) -> &mut Self {{
                    self.{0}(AddressMode::Immediate(Immediate::High(
                        AddressReference::new(address_name)
                    )))
                }}
                ",
                def.instruction.to_string()
            ));
        }
        if def.accumulator != NO_ACCUMULATOR {
            lines.push(format!(
                "
                /// Record a {0} instruction that uses accumulator as address mode.
                /// 
                /// # Example
                /// ```
                /// use c64_assembler::builder::instruction::InstructionBuilder;
                /// let instructions = InstructionBuilder::default()
                ///     .{0}_acc()
                ///     .finalize();
                /// ```
                pub fn {0}_acc(&mut self) -> &mut Self {{
                    self.{0}(AddressMode::Accumulator)
                }}
                ",
                def.instruction.to_string()
            ));
        }

        if def.absolute != NO_ABSOLUTE || def.zeropage != NO_ZEROPAGE {
            lines.push(format!(
                "
                /// Record a {0} instruction that use an absolute address. 
                /// 
                /// # Example
                /// ```
                /// use c64_assembler::builder::instruction::InstructionBuilder;
                /// let instructions = InstructionBuilder::default()
                ///     .{0}_addr(\"test_label\")
                ///     .label(\"test_label\")
                ///     .finalize();
                /// ```
                pub fn {0}_addr(&mut self, address_name: &str) -> &mut Self {{
                    self.{0}(AddressMode::Absolute(AddressReference::new(address_name)))
                }}
        
                /// Record a {0} instruction that use an absolute address with an offset.
                /// Offset is in bytes.
                /// 
                /// # Example
                /// ```
                /// use c64_assembler::builder::instruction::InstructionBuilder;
                /// let instructions = InstructionBuilder::default()
                ///     .{0}_addr_offs(\"test_label\", 8)
                ///     .label(\"test_label\")
                ///     .finalize();
                /// ```
                pub fn {0}_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {{
                    self.{0}(AddressMode::Absolute(AddressReference::with_offset(
                        address_name, offset
                    )))
                }}
                ",
                def.instruction.to_string()
            ));
        }
        if def.absolute_x != NO_ABSOLUTE_X || def.zeropage_x != NO_ZEROPAGE_X {
            lines.push(format!(
                "
                /// Record a {0} instructon that use an absolute address with x-register as indexer.
                /// 
                /// # Example
                /// ```
                /// use c64_assembler::builder::instruction::InstructionBuilder;
                /// let instructions = InstructionBuilder::default()
                ///     .lda_imm(0x08)
                ///     .{0}_addr_x(\"test_label\")
                ///     .label(\"test_label\")
                ///     .finalize();
                /// ```
                pub fn {0}_addr_x(&mut self, address_name: &str) -> &mut Self {{
                    self.{0}(AddressMode::AbsoluteX(AddressReference::new(address_name)))
                }}
                ",
                def.instruction.to_string()
            ));
        }
        if def.absolute_y != NO_ABSOLUTE_Y || def.zeropage_y != NO_ZEROPAGE_Y {
            lines.push(format!(
                "
                /// Record a {0} instructon that use an absolute address with y-register as indexer.
                /// 
                /// # Example
                /// ```
                /// use c64_assembler::builder::instruction::InstructionBuilder;
                /// let instructions = InstructionBuilder::default()
                ///     .lda_imm(0x08)
                ///     .{0}_addr_y(\"test_label\")
                ///     .label(\"test_label\")
                ///     .finalize();
                /// ```
                pub fn {0}_addr_y(&mut self, address_name: &str) -> &mut Self {{
                    self.{0}(AddressMode::AbsoluteY(AddressReference::new(address_name)))
                }}
                ",
                def.instruction.to_string()
            ));
        }
        if def.relative != NO_RELATIVE {
            lines.push(format!(
                "
                /// Record a {0} instruction that use  relativeeeeeeeee address. 
                /// 
                /// # Example
                /// ```
                /// use c64_assembler::builder::instruction::InstructionBuilder;
                /// let instructions = InstructionBuilder::default()
                ///     .{0}_addr(\"test_label\")
                ///     .label(\"test_label\")
                ///     .finalize();
                /// ```
                pub fn {0}_addr(&mut self, address_name: &str) -> &mut Self {{
                    self.{0}(AddressMode::Relative(AddressReference::new(address_name)))
                }}
        
                /// Record a {0} instruction that use a relative address with an offset.
                /// Offset is in bytes.
                /// 
                /// # Example
                /// ```
                /// use c64_assembler::builder::instruction::InstructionBuilder;
                /// let instructions = InstructionBuilder::default()
                ///     .{0}_addr_offs(\"test_label\", 8)
                ///     .label(\"test_label\")
                ///     .finalize();
                /// ```
                pub fn {0}_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {{
                    self.{0}(AddressMode::Relative(AddressReference::with_offset(
                        address_name, offset
                    )))
                }}
                ",
                def.instruction.to_string()
            ));
        }
        if def.indirect != NO_INDIRECT {
            lines.push(format!(
                "
                pub fn {0}_ind(&mut self, address_name: &str) -> &mut Self {{
                    self.{0}(AddressMode::Indirect(AddressReference::new(address_name)))
                }}
                ",
                def.instruction.to_string()
            ));
        }
        if def.indexed_indirect != NO_INDEXED_INDIRECT {
            lines.push(format!(
                "
                pub fn {0}_ind_x(&mut self, address_name: &str) -> &mut Self {{
                    self.{0}(AddressMode::IndexedIndirect(AddressReference::new(address_name)))
                }}
                ",
                def.instruction.to_string()
            ));
        }
        if def.indirect_indexed != NO_INDIRECT_INDEXED {
            lines.push(format!(
                "
                pub fn {0}_ind_y(&mut self, address_name: &str) -> &mut Self {{
                    self.{0}(AddressMode::IndirectIndexed(AddressReference::new(address_name)))
                }}
                ",
                def.instruction.to_string()
            ));
        }
    }

    print!("{}", lines.join("\n"));
}
