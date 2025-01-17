use c64_assembler::{
    builder::instruction::Instructions, instruction::operation::Operation, memory::address_mode::AddressMode,
};
use c64_assembler_6502::codegen_instruction_tests;
use c64_assembler_macro::instructions;

fn test_first(instructions: Instructions, operation: Operation, address_mode: AddressMode) {
    let instruction = instructions.instructions.first().unwrap();
    assert_eq!(operation, instruction.operation);
    assert_eq!(address_mode, instruction.address_mode);
}

#[test]
fn label() {
    test_first(
        instructions!(test:),
        Operation::Label("test".to_string()),
        AddressMode::Implied,
    );
}

codegen_instruction_tests! {}
