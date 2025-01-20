//! Internal crate for c64-assembler containing 6502 instruction set and code generators.
//!
//! Code generators for
//!
//! - generate op codes
//! - instruction builder functions including doctests
//! - instruction!() tests suite
//! - program byte code
//!

use c64_assembler_6502::isa::{
    isa_6502, NO_ABSOLUTE, NO_ABSOLUTE_X, NO_ABSOLUTE_Y, NO_ACCUMULATOR, NO_IMMEDIATE, NO_IMPLIED, NO_INDEXED_INDIRECT,
    NO_INDIRECT, NO_INDIRECT_INDEXED, NO_RELATIVE, NO_ZEROPAGE, NO_ZEROPAGE_X, NO_ZEROPAGE_Y,
};
use proc_macro::TokenStream;


#[proc_macro]
pub fn codegen_instruction_tests(_input: TokenStream) -> TokenStream {
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
            if def.implied == NO_IMPLIED {
                "UNUSED".to_string()
            } else {
                format!("{}", def.instruction.to_uppercase())
            },
            if def.immediate == NO_IMMEDIATE {
                "UNUSED".to_string()
            } else {
                format!("{}_IMMEDIATE", def.instruction.to_uppercase())
            },
            if def.accumulator == NO_ACCUMULATOR {
                "UNUSED".to_string()
            } else {
                format!(
                    "{}_ACCUMULATOR
                ",
                    def.instruction.to_uppercase()
                )
            },
            if def.absolute == NO_ABSOLUTE {
                "UNUSED".to_string()
            } else {
                format!("{}_ABSOLUTE", def.instruction.to_uppercase())
            },
            if def.absolute_x == NO_ABSOLUTE_X {
                "UNUSED".to_string()
            } else {
                format!("{}_ABSOLUTE_X", def.instruction.to_uppercase())
            },
            if def.absolute_y == NO_ABSOLUTE_Y {
                "UNUSED".to_string()
            } else {
                format!("{}_ABSOLUTE_Y", def.instruction.to_uppercase())
            },
            if def.zeropage == NO_ZEROPAGE {
                "UNUSED".to_string()
            } else {
                format!("{}_ZEROPAGE", def.instruction.to_uppercase())
            },
            if def.zeropage_x == NO_ZEROPAGE_X {
                "UNUSED".to_string()
            } else {
                format!("{}_ZEROPAGE_X", def.instruction.to_uppercase())
            },
            if def.zeropage_y == NO_ZEROPAGE_Y {
                "UNUSED".to_string()
            } else {
                format!("{}_ZEROPAGE_Y", def.instruction.to_uppercase())
            },
            if def.relative == NO_RELATIVE {
                "UNUSED".to_string()
            } else {
                format!("{}_RELATIVE", def.instruction.to_uppercase())
            },
            if def.indirect == NO_INDIRECT {
                "UNUSED".to_string()
            } else {
                format!("{}_INDIRECT", def.instruction.to_uppercase())
            },
            if def.indexed_indirect == NO_INDEXED_INDIRECT {
                "UNUSED".to_string()
            } else {
                format!("{}_INDIRECT_INDEXED", def.instruction.to_uppercase())
            },
            if def.indirect_indexed == NO_INDIRECT_INDEXED {
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
