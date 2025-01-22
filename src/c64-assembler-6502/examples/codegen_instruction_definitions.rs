use c64_assembler_6502::{
    instruction::InstructionDef,
    isa_6502,
    opcodes::{
        OpCode, NO_ABSOLUTE, NO_ABSOLUTE_X, NO_ABSOLUTE_Y, NO_ACCUMULATOR, NO_IMMEDIATE, NO_IMPLIED,
        NO_INDEXED_INDIRECT, NO_INDIRECT, NO_INDIRECT_INDEXED, NO_RELATIVE, NO_ZEROPAGE, NO_ZEROPAGE_X, NO_ZEROPAGE_Y,
    },
};

fn main() {
    fn format(def: &InstructionDef, op_code: OpCode, no_opcode: OpCode, opcode_str: &str) -> String {
        if op_code == no_opcode {
            format!("NO_{}", opcode_str)
        } else {
            format!("{}_{}", def.instruction.to_uppercase(), opcode_str)
        }
    }

    fn format_opcodes(result: &mut Vec<String>, def: &InstructionDef) {
        let line = format!(
            "pub const OPCODES_{0}:InstructionDef = InstructionDef {{
    instruction: \"{1}\",
    implied: {2},
    immediate: {3},
    accumulator: {4},
    absolute: {5},
    absolute_x: {6},
    absolute_y: {7},
    zeropage: {8},
    zeropage_x: {9},
    zeropage_y: {10},
    relative: {11},
    indirect: {12},
    indexed_indirect: {13},
    indirect_indexed: {14},
}};",
            def.instruction.to_uppercase(),
            def.instruction.to_string(),
            format(def, def.implied, NO_IMPLIED, "IMPLIED"),
            format(def, def.immediate, NO_IMMEDIATE, "IMMEDIATE"),
            format(def, def.accumulator, NO_ACCUMULATOR, "ACCUMULATOR"),
            format(def, def.absolute, NO_ABSOLUTE, "ABSOLUTE"),
            format(def, def.absolute_x, NO_ABSOLUTE_X, "ABSOLUTE_X"),
            format(def, def.absolute_y, NO_ABSOLUTE_Y, "ABSOLUTE_Y"),
            format(def, def.zeropage, NO_ZEROPAGE, "ZEROPAGE"),
            format(def, def.zeropage_x, NO_ZEROPAGE_X, "ZEROPAGE_X"),
            format(def, def.zeropage_y, NO_ZEROPAGE_Y, "ZEROPAGE_Y"),
            format(def, def.relative, NO_RELATIVE, "RELATIVE"),
            format(def, def.indirect, NO_INDIRECT, "INDIRECT"),
            format(def, def.indexed_indirect, NO_INDEXED_INDIRECT, "INDEXED_INDIRECT"),
            format(def, def.indirect_indexed, NO_INDIRECT_INDEXED, "INDIRECT_INDEXED"),
        );
        result.push(line);
    }

    let mut lines = Vec::<String>::default();
    for def in &isa_6502() {
        format_opcodes(&mut lines, def);
    }

    println!("{}", lines.join("\n"));
}
