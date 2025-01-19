use c64_assembler_6502::isa::{isa_6502, NO_IMMEDIATE, NO_IMPLIED};

fn main() {
    println!("| **Instruction** | **Implied** | **Immediate** | **Accumulator** | **Absolute** | **Absolute,X** | **Absolute,Y** | **Zero Page** | **Zero Page,X** | **Relative** | **Indirect** | **Indirect,X** | **Indirect,Y** |");
    println!("| --------------- | ----------- | ------------- | --------------- | ------------ | -------------- | -------------- | ------------- | --------------- | ------------ | ------------ | -------------- | -------------- |");
    for instruction in isa_6502() {
        println!("| {:15} | {:11} | {:13} | --------------- | ------------ | -------------- | -------------- | ------------- | --------------- | ------------ | ------------ | -------------- | -------------- |", instruction.instruction.to_uppercase(),
            if instruction.implied == NO_IMPLIED{"".to_string()}else {format!("0x{:02X}", instruction.implied)},
            if instruction.immediate == NO_IMMEDIATE{"".to_string()}else {format!("0x{:02X}", instruction.immediate)},
        );
    }
    println!("| --------------- | ----------- | ------------- | --------------- | ------------ | -------------- | -------------- | ------------- | --------------- | ------------ | ------------ | -------------- | -------------- |");
}
