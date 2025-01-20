use c64_assembler_6502::isa::{isa_6502, OpCode, NO_IMMEDIATE};

fn main() {
    fn format_opcode(result: &mut Vec<String>, instruction: &str, opcode: OpCode, post: &str) {
        if opcode != NO_IMMEDIATE {
            result.push(format!(
                "const {}{}:OpCode = 0x{:02x};",
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
        format_opcode(&mut lines, &def.instruction, def.accumulator, &"_ACCUMULATOR");
        format_opcode(&mut lines, &def.instruction, def.absolute, &"_ABSOLUTE");
        format_opcode(&mut lines, &def.instruction, def.absolute_x, &"_ABSOLUTE_X");
        format_opcode(&mut lines, &def.instruction, def.absolute_y, &"_ABSOLUTE_Y");
        format_opcode(&mut lines, &def.instruction, def.zeropage, &"_ZEROPAGE");
        format_opcode(&mut lines, &def.instruction, def.zeropage_x, &"_ZEROPAGE_X");
        format_opcode(&mut lines, &def.instruction, def.zeropage_y, &"_ZEROPAGE_Y");
        format_opcode(&mut lines, &def.instruction, def.relative, &"_RELATIVE");
        format_opcode(&mut lines, &def.instruction, def.indirect, &"_INDIRECT");
        format_opcode(&mut lines, &def.instruction, def.indexed_indirect, &"_INDEXED_INDIRECT");
        format_opcode(&mut lines, &def.instruction, def.indirect_indexed, &"_INDIRECT_INDEXED");
    }
    println!("{}", lines.join("\n"));
}
