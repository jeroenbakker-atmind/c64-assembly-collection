use c64_assembler_6502::{
    isa_6502,
    opcodes::{OpCode, NO_IMPLIED},
};

fn main() {
    fn format_opcode(result: &mut Vec<String>, instruction: &str, opcode: OpCode, post: &str) {
        if opcode != NO_IMPLIED {
            result.push(format!(
                "const {}_{}:OpCode = 0x{:02x};",
                instruction.to_string().to_uppercase(),
                post.to_string(),
                opcode
            ));
        }
    }

    let mut lines = Vec::<String>::default();
    for def in isa_6502() {
        format_opcode(&mut lines, &def.instruction, def.implied, "IMPLIED");
        format_opcode(&mut lines, &def.instruction, def.immediate, "IMMEDIATE");
        format_opcode(&mut lines, &def.instruction, def.accumulator, "ACCUMULATOR");
        format_opcode(&mut lines, &def.instruction, def.absolute, "ABSOLUTE");
        format_opcode(&mut lines, &def.instruction, def.absolute_x, "ABSOLUTE_X");
        format_opcode(&mut lines, &def.instruction, def.absolute_y, "ABSOLUTE_Y");
        format_opcode(&mut lines, &def.instruction, def.zeropage, "ZEROPAGE");
        format_opcode(&mut lines, &def.instruction, def.zeropage_x, "ZEROPAGE_X");
        format_opcode(&mut lines, &def.instruction, def.zeropage_y, "ZEROPAGE_Y");
        format_opcode(&mut lines, &def.instruction, def.relative, "RELATIVE");
        format_opcode(&mut lines, &def.instruction, def.indirect, "INDIRECT");
        format_opcode(&mut lines, &def.instruction, def.indexed_indirect, "INDEXED_INDIRECT");
        format_opcode(&mut lines, &def.instruction, def.indirect_indexed, "INDIRECT_INDEXED");
    }

    println!("{}", lines.join("\n"));
}
