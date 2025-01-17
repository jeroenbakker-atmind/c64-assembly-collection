mod isa;

use isa::{
    isa_6502, OpCode, ACC, ADDR, ADDR_X, ADDR_Y, IMM, IMPLIED, IND, IND_X, IND_Y, RELATIVE, ZERO,
    ZERO_X, ZERO_Y,
};
use proc_macro::TokenStream;

#[proc_macro]
pub fn codegen_opcodes(_input: TokenStream) -> TokenStream {
    fn format_opcode(result: &mut Vec<String>, instruction: &str, opcode: OpCode, post: &str) {
        if opcode != IMM {
            result.push(format!(
                "const {}{}:u8 = 0x{:02x};",
                instruction.to_string().to_uppercase(),
                post.to_string(),
                opcode
            ));
        }
    }
    let mut lines = Vec::<String>::default();
    for def in isa_6502() {
        format_opcode(&mut lines, &def.instruction, def.implied, &"");
        format_opcode(&mut lines, &def.instruction, def.immediate, &"_IMMEDIATE");
        format_opcode(
            &mut lines,
            &def.instruction,
            def.accumulator,
            &"_ACCUMULATOR",
        );
        format_opcode(&mut lines, &def.instruction, def.absolute, &"_ABSOLUTE");
        format_opcode(&mut lines, &def.instruction, def.absolute_x, &"_ABSOLUTE_X");
        format_opcode(&mut lines, &def.instruction, def.absolute_y, &"_ABSOLUTE_Y");
        format_opcode(&mut lines, &def.instruction, def.zeropage, &"_ZEROPAGE");
        format_opcode(&mut lines, &def.instruction, def.zeropage_x, &"_ZEROPAGE_X");
        format_opcode(&mut lines, &def.instruction, def.zeropage_y, &"_ZEROPAGE_Y");
        format_opcode(&mut lines, &def.instruction, def.relative, &"_RELATIVE");
        format_opcode(&mut lines, &def.instruction, def.indirect, &"_INDIRECT");
        format_opcode(
            &mut lines,
            &def.instruction,
            def.indexed_indirect,
            &"_INDEXED_INDIRECT",
        );
        format_opcode(
            &mut lines,
            &def.instruction,
            def.indirect_indexed,
            &"_INDIRECT_INDEXED",
        );
    }
    lines.join("\n").parse().unwrap()
}

#[proc_macro]
pub fn codegen_instruction_builder(_input: TokenStream) -> TokenStream {
    let mut lines = Vec::<String>::default();

    for def in isa_6502() {
        if def.implied != IMPLIED {
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
        if def.immediate != IMM {
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
        if def.accumulator != ACC {
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

        if def.absolute != ADDR {
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
        if def.absolute_x != ADDR_X {
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
        if def.absolute_y != ADDR_Y {
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
        if def.relative != RELATIVE {
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
        if def.indirect != IND {
            lines.push(format!(
                "
                pub fn {0}_ind(&mut self, address_name: &str) -> &mut Self {{
                    self.{0}(AddressMode::Indirect(AddressReference::new(address_name)))
                }}
                ",
                def.instruction.to_string()
            ));
        }
        if def.indexed_indirect != IND_X {
            lines.push(format!(
                "
                pub fn {0}_ind_x(&mut self, address_name: &str) -> &mut Self {{
                    self.{0}(AddressMode::IndexedIndirect(AddressReference::new(address_name)))
                }}
                ",
                def.instruction.to_string()
            ));
        }
        if def.indirect_indexed != IND_Y {
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

    let result = lines.join("\n");
    result.parse().unwrap()
}

#[proc_macro]
pub fn codegen_instruction_tests(_input: TokenStream) -> TokenStream {
    let mut lines = Vec::<String>::default();

    for def in isa_6502() {
        if def.implied != IMPLIED {
            lines.push(format!(
                "
                #[test]
                fn {0}() {{
                    test_first(instructions!({0}), Operation::{1}, AddressMode::Implied);
                }}
                ",
                def.instruction.to_string(),
                def.instruction.to_uppercase()
            ));
            continue;
        }
        lines.push(format!(
            "
            mod {0} {{
                use c64_assembler::{{
                    instruction::operation::Operation,
                    memory::{{address_mode::*, label::AddressReference}},
        }};
                use c64_assembler_macro::instructions;

                use crate::test_first;
                const OP: Operation = Operation::{1};
            ",
            def.instruction.to_string(),
            def.instruction.to_string().to_uppercase()
        ));

        if def.immediate != IMM {
            lines.push(format!(
                "
                #[test]
                fn imm() {{
                    test_first(
                        instructions!({0} #99),
                        OP,
                        AddressMode::Immediate(Immediate::Byte(99)),
                    );
                    test_first(
                        instructions!({0} #$99),
                        OP,
                        AddressMode::Immediate(Immediate::Byte(153)),
                    )
                }}

                #[test]
                fn imm_low() {{
                    test_first(
                        instructions!({0} #<test),
                        OP,
                        AddressMode::Immediate(Immediate::Low(AddressReference::new(\"test\"))),
                    );
                }}

                #[test]
                fn imm_high() {{
                    test_first(
                        instructions!({0} #>test),
                        OP,
                        AddressMode::Immediate(Immediate::High(AddressReference::new(\"test\"))),
                    );
                }}
                ",
                def.instruction.to_string()
            ));
        }

        if def.accumulator != ACC {
            lines.push(format!(
                "
                #[test]
                fn acc() {{
                    test_first(
                        instructions!({0} a),
                        OP,
                        AddressMode::Accumulator,
                    );
                }}
                ",
                def.instruction.to_string()
            ));
        }

        if def.absolute != ADDR || def.zeropage != ZERO {
            lines.push(format!(
                "
                #[test]
                fn addr() {{
                    test_first(
                        instructions!({0} test),
                        OP,
                        AddressMode::Absolute(AddressReference::new(&\"test\")),
                    );
                }}

                #[test]
                fn addr_offs() {{
                    test_first(
                        instructions!({0} test+1),
                        OP,
                        AddressMode::Absolute(AddressReference::with_offset(&\"test\", 1)),
                    );
                }}
                ",
                def.instruction.to_string()
            ));
        }
        if def.relative != RELATIVE {
            lines.push(format!(
                "
                #[test]
                fn addr() {{
                    test_first(
                        instructions!({0} test),
                        OP,
                        AddressMode::Relative(AddressReference::new(&\"test\")),
                    );
                }}

                #[test]
                fn addr_offs() {{
                    test_first(
                        instructions!({0} test+1),
                        OP,
                        AddressMode::Relative(AddressReference::with_offset(&\"test\", 1)),
                    );
                }}
                ",
                def.instruction.to_string()
            ));
        }

        /*
        if def.absolute_x != UNUSED ||def.zeropage_x != UNUSED{
            lines.push(format!(
                "
                #[test]
                fn addr_x() {{
                    test_first(
                        instructions!({0} test,x),
                        OP,
                        AddressMode::AbsoluteX(AddressReference::new(&\"test\")),
                    );
                }}
                ",
                def.instruction.to_string()
            ));
        }
         */
        /*
        if def.absolute_y != UNUSED ||def.zeropage_y != UNUSED{
            lines.push(format!(
                "
                #[test]
                fn addr_y() {{
                    test_first(
                        instructions!({0} test,y),
                        OP,
                        AddressMode::AbsoluteY(AddressReference::new(&\"test\")),
                    );
                }}
                ",
                def.instruction.to_string()
            ));
        }
        */
        /*
        if def.indirect != UNUSED {
            lines.push(format!(
                "
                #[test]
                fn ind() {{
                    test_first(
                        instructions!({0} (test)),
                        OP,
                        AddressMode::Indirect(AddressReference::new(&\"test\")),
                    );
                }}
                ",
                def.instruction.to_string()
            ));
        }
        if def.indexed_indirect != UNUSED {
            lines.push(format!(
                "
                #[test]
                fn ind_x() {{
                    test_first(
                        instructions!({0} (test,x)),
                        OP,
                        AddressMode::IndexedIndirect(AddressReference::new(&\"test\")),
                    );
                }}
                ",
                def.instruction.to_string()
            ));
        }

        if def.indirect_indexed != UNUSED {
            lines.push(format!(
                "
                #[test]
                fn ind_y() {{
                    test_first(
                        instructions!({0} (test),y),
                        OP,
                        AddressMode::IndirectIndexed(AddressReference::new(&\"test\")),
                    );
                }}
                ",
                def.instruction.to_string()
            ));
        }
        */

        // Close module
        lines.push(format!(
            "
            }}
            "
        ));
    }

    lines.join("\n").parse().unwrap()
}

#[proc_macro]
pub fn codegen_program_instruction_to_byte_code(_input: TokenStream) -> TokenStream {
    let mut lines = Vec::<String>::default();
    lines.push(format!("match &instruction.operation {{"));

    for def in isa_6502() {
        lines.push(format!(
            "
        Operation::{0} => {{
            self.add_byte_code(
                application,
                &instruction.address_mode,
                {1},
                {2},
                {3},
                {4},
                {5},
                {6},
                {7},
                {8},
                {9},
                {10},
                {11},
                {12},
                {13}
            )
        }}
        ",
            def.instruction.to_uppercase(),
            if def.implied == IMPLIED {
                "UNUSED".to_string()
            } else {
                format!("{}", def.instruction.to_uppercase())
            },
            if def.immediate == IMM {
                "UNUSED".to_string()
            } else {
                format!("{}_IMMEDIATE", def.instruction.to_uppercase())
            },
            if def.accumulator == ACC {
                "UNUSED".to_string()
            } else {
                format!(
                    "{}_ACCUMULATOR
                ",
                    def.instruction.to_uppercase()
                )
            },
            if def.absolute == ADDR {
                "UNUSED".to_string()
            } else {
                format!("{}_ABSOLUTE", def.instruction.to_uppercase())
            },
            if def.absolute_x == ADDR_X {
                "UNUSED".to_string()
            } else {
                format!("{}_ABSOLUTE_X", def.instruction.to_uppercase())
            },
            if def.absolute_y == ADDR_Y {
                "UNUSED".to_string()
            } else {
                format!("{}_ABSOLUTE_Y", def.instruction.to_uppercase())
            },
            if def.zeropage == ZERO {
                "UNUSED".to_string()
            } else {
                format!("{}_ZEROPAGE", def.instruction.to_uppercase())
            },
            if def.zeropage_x == ZERO_X {
                "UNUSED".to_string()
            } else {
                format!("{}_ZEROPAGE_X", def.instruction.to_uppercase())
            },
            if def.zeropage_y == ZERO_Y {
                "UNUSED".to_string()
            } else {
                format!("{}_ZEROPAGE_Y", def.instruction.to_uppercase())
            },
            if def.relative == RELATIVE {
                "UNUSED".to_string()
            } else {
                format!("{}_RELATIVE", def.instruction.to_uppercase())
            },
            if def.indirect == IND {
                "UNUSED".to_string()
            } else {
                format!("{}_INDIRECT", def.instruction.to_uppercase())
            },
            if def.indexed_indirect == IND_X {
                "UNUSED".to_string()
            } else {
                format!("{}_INDIRECT_INDEXED", def.instruction.to_uppercase())
            },
            if def.indirect_indexed == IND_Y {
                "UNUSED".to_string()
            } else {
                format!("{}_INDEXED_INDIRECT", def.instruction.to_uppercase())
            },
        ));
    }

    lines.push(format!(
        "
        Operation::Raw(bytes) => {{
            self.add_bytes(bytes);
        }}
        Operation::Label(_) => {{
            // Intentionally empty.
        }}
    }}"
    ));

    lines.join("\n").parse().unwrap()
}
