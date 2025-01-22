use c64_assembler_6502::isa_6502;

fn main() {
    let mut lines = Vec::<String>::default();
    lines.push(format!("match &instruction.operation {{"));

    for def in isa_6502() {
        lines.push(format!(
            "
        Operation::{0} => {{
            self.add_byte_code(
                application,
                &instruction.address_mode,
                &OPCODES_{0}
            )
        }}
        ",
            def.instruction.to_uppercase(),
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

    print!("{}", lines.join("\n"))
}
