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
