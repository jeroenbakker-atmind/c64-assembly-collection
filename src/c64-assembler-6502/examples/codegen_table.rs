use c64_assembler_6502::{
    isa_6502,
    opcodes::{
        NO_ABSOLUTE, NO_ABSOLUTE_X, NO_ABSOLUTE_Y, NO_ACCUMULATOR, NO_IMMEDIATE, NO_IMPLIED, NO_INDEXED_INDIRECT,
        NO_INDIRECT, NO_INDIRECT_INDEXED, NO_RELATIVE, NO_ZEROPAGE, NO_ZEROPAGE_X, NO_ZEROPAGE_Y,
    },
};

fn main() {
    println!("//! | **Instruction** | **Implied** | **Immediate** | **Accumulator** | **Absolute** | **Absolute,X** | **Absolute,Y** | **Zero Page** | **Zero Page,X** | **Zero Page,Y** | **Relative** | **Indirect** | **Indirect,X** | **Indirect,Y** |");
    println!("//! | --------------- | ----------- | ------------- | --------------- | ------------ | -------------- | -------------- | ------------- | --------------- | --------------- | ------------ | ------------ | -------------- | -------------- |");
    for instruction in isa_6502() {
        println!("//! | {:15} | {:11} | {:13} | {:15} | {:12} | {:14} | {:14} | {:13} | {:15} | {:15} | {:12} | {:12} | {:14} | {:14} |", instruction.instruction.to_uppercase(),
            if instruction.implied == NO_IMPLIED{"".to_string()}else {format!("0x{:02X}", instruction.implied)},
            if instruction.immediate == NO_IMMEDIATE{"".to_string()}else {format!("0x{:02X}", instruction.immediate)},
            if instruction.accumulator == NO_ACCUMULATOR{"".to_string()}else {format!("0x{:02X}", instruction.accumulator)},
            if instruction.absolute == NO_ABSOLUTE{"".to_string()}else {format!("0x{:02X}", instruction.absolute)},
            if instruction.absolute_x == NO_ABSOLUTE_X{"".to_string()}else {format!("0x{:02X}", instruction.absolute_x)},
            if instruction.absolute_y == NO_ABSOLUTE_Y{"".to_string()}else {format!("0x{:02X}", instruction.absolute_y)},
            if instruction.zeropage == NO_ZEROPAGE{"".to_string()}else {format!("0x{:02X}", instruction.zeropage)},
            if instruction.zeropage_x == NO_ZEROPAGE_X{"".to_string()}else {format!("0x{:02X}", instruction.zeropage_x)},
            if instruction.zeropage_y == NO_ZEROPAGE_Y{"".to_string()}else {format!("0x{:02X}", instruction.zeropage_y)},
            if instruction.relative == NO_RELATIVE{"".to_string()}else {format!("0x{:02X}", instruction.relative)},
            if instruction.indirect == NO_INDIRECT{"".to_string()}else {format!("0x{:02X}", instruction.indirect)},
            if instruction.indexed_indirect == NO_INDEXED_INDIRECT{"".to_string()}else {format!("0x{:02X}", instruction.indexed_indirect)},
            if instruction.indirect_indexed == NO_INDIRECT_INDEXED{"".to_string()}else {format!("0x{:02X}", instruction.indirect_indexed)},
        );
    }
}
