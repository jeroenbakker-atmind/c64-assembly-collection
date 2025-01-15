mod isa;

use isa::{isa_6502, OpCode, UNUSED};
use proc_macro::TokenStream;

#[proc_macro]
pub fn codegen_opcodes(_input: TokenStream) -> TokenStream {
    fn format_opcode(result: &mut Vec<String>, instruction: &str, opcode: OpCode, post: &str) {
        if opcode != UNUSED {
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
        if def.implied != UNUSED {
            lines.push(format!(
                "
                /// Record a new {0} instruction (addressing mode is implied).
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
        if def.immediate != UNUSED {
            lines.push(format!(
                "
                pub fn {0}_imm(&mut self, byte: u8) -> &mut Self {{
                    self.{0}(AddressMode::Immediate(Immediate::Byte(byte)))
                }}
                pub fn {0}_imm_low(&mut self, address_name: &str) -> &mut Self {{
                    self.{0}(AddressMode::Immediate(Immediate::Low(
                        AddressReference::new(address_name)
                    )))
                }}
                pub fn {0}_imm_high(&mut self, address_name: &str) -> &mut Self {{
                    self.{0}(AddressMode::Immediate(Immediate::High(
                        AddressReference::new(address_name)
                    )))
                }}
                ",
                def.instruction.to_string()
            ));
        }
        if def.accumulator != UNUSED {
            lines.push(format!(
                "
                pub fn {0}_acc(&mut self) -> &mut Self {{
                    self.{0}(AddressMode::Accumulator)
                }}
                ",
                def.instruction.to_string()
            ));
        }

        if def.absolute != UNUSED {
            lines.push(format!(
                "
                pub fn {0}_addr(&mut self, address_name: &str) -> &mut Self {{
                    self.{0}(AddressMode::Absolute(AddressReference::new(address_name)))
                }}
                pub fn {0}_addr_offs(&mut self, address_name: &str, offset: Address) -> &mut Self {{
                    self.{0}(AddressMode::Absolute(AddressReference::with_offset(
                        address_name, offset
                    )))
                }}
                ",
                def.instruction.to_string()
            ));
        }
        if def.absolute_x != UNUSED {
            lines.push(format!(
                "
                pub fn {0}_addr_x(&mut self, address_name: &str) -> &mut Self {{
                    self.{0}(AddressMode::AbsoluteX(AddressReference::new(address_name)))
                }}
                ",
                def.instruction.to_string()
            ));
        }
        if def.absolute_y != UNUSED {
            lines.push(format!(
                "
                pub fn {0}_addr_y(&mut self, address_name: &str) -> &mut Self {{
                    self.{0}(AddressMode::AbsoluteY(AddressReference::new(address_name)))
                }}
                ",
                def.instruction.to_string()
            ));
        }
        if def.indirect != UNUSED {
            lines.push(format!(
                "
                pub fn {0}_ind(&mut self, address_name: &str) -> &mut Self {{
                    self.{0}(AddressMode::Indirect(AddressReference::new(address_name)))
                }}
                ",
                def.instruction.to_string()
            ));
        }
        if def.indexed_indirect != UNUSED {
            lines.push(format!(
                "
                pub fn {0}_ind_x(&mut self, address_name: &str) -> &mut Self {{
                    self.{0}(AddressMode::IndexedIndirect(AddressReference::new(address_name)))
                }}
                ",
                def.instruction.to_string()
            ));
        }
        if def.indirect_indexed != UNUSED {
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
        if def.implied != UNUSED {
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
        
/*
        if def.immediate != UNUSED {
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
 */
/* 
        if def.accumulator != UNUSED {
            lines.push(format!(
                "
                #[test]
                fn acc() {{
                    test_first(
                        instructions!({0} A),
                        OP,
                        AddressMode::Accumulator,
                    );
                }}
                ",
                def.instruction.to_string()
            ));
        }
        */
        /* 
        if def.absolute != UNUSED ||def.zeropage != UNUSED{
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
        */

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
        #[ignore]
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