use c64_assembler_6502::{
    isa_6502,
    opcodes::{
        NO_ABSOLUTE, NO_ABSOLUTE_X, NO_ABSOLUTE_Y, NO_ACCUMULATOR, NO_IMMEDIATE, NO_IMPLIED, NO_RELATIVE, NO_ZEROPAGE,
        NO_ZEROPAGE_X, NO_ZEROPAGE_Y,
    },
};

fn main() {
    let mut lines = Vec::<String>::default();

    for def in isa_6502() {
        if def.implied != NO_IMPLIED {
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

        if def.immediate != NO_IMMEDIATE {
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

        if def.accumulator != NO_ACCUMULATOR {
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

        if def.absolute != NO_ABSOLUTE || def.zeropage != NO_ZEROPAGE {
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
        if def.relative != NO_RELATIVE {
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

        if def.absolute_x != NO_ABSOLUTE_X || def.zeropage_x != NO_ZEROPAGE_X {
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
        if def.absolute_y != NO_ABSOLUTE_Y || def.zeropage_y != NO_ZEROPAGE_Y {
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

    print!("{}", lines.join("\n"));
}
