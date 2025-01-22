use c64_assembler::{
    builder::instruction::Instructions, instruction::operation::Operation, memory::address_mode::AddressMode,
};
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

mod adc {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::ADC;

    #[test]
    fn imm() {
        test_first(instructions!(adc #99), OP, AddressMode::Immediate(Immediate::Byte(99)));
        test_first(
            instructions!(adc #$99),
            OP,
            AddressMode::Immediate(Immediate::Byte(153)),
        )
    }

    #[test]
    fn imm_low() {
        test_first(
            instructions!(adc #<test),
            OP,
            AddressMode::Immediate(Immediate::Low(AddressReference::new("test"))),
        );
    }

    #[test]
    fn imm_high() {
        test_first(
            instructions!(adc #>test),
            OP,
            AddressMode::Immediate(Immediate::High(AddressReference::new("test"))),
        );
    }

    #[test]
    fn addr() {
        test_first(
            instructions!(adc test),
            OP,
            AddressMode::Absolute(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(adc test+1),
            OP,
            AddressMode::Absolute(AddressReference::with_offset(&"test", 1)),
        );
    }

    #[test]
    fn addr_x() {
        test_first(
            instructions!(adc test,x),
            OP,
            AddressMode::AbsoluteX(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_y() {
        test_first(
            instructions!(adc test,y),
            OP,
            AddressMode::AbsoluteY(AddressReference::new(&"test")),
        );
    }
}

mod and {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::AND;

    #[test]
    fn imm() {
        test_first(instructions!(and #99), OP, AddressMode::Immediate(Immediate::Byte(99)));
        test_first(
            instructions!(and #$99),
            OP,
            AddressMode::Immediate(Immediate::Byte(153)),
        )
    }

    #[test]
    fn imm_low() {
        test_first(
            instructions!(and #<test),
            OP,
            AddressMode::Immediate(Immediate::Low(AddressReference::new("test"))),
        );
    }

    #[test]
    fn imm_high() {
        test_first(
            instructions!(and #>test),
            OP,
            AddressMode::Immediate(Immediate::High(AddressReference::new("test"))),
        );
    }

    #[test]
    fn addr() {
        test_first(
            instructions!(and test),
            OP,
            AddressMode::Absolute(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(and test+1),
            OP,
            AddressMode::Absolute(AddressReference::with_offset(&"test", 1)),
        );
    }

    #[test]
    fn addr_x() {
        test_first(
            instructions!(and test,x),
            OP,
            AddressMode::AbsoluteX(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_y() {
        test_first(
            instructions!(and test,y),
            OP,
            AddressMode::AbsoluteY(AddressReference::new(&"test")),
        );
    }
}

mod asl {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::ASL;

    #[test]
    fn acc() {
        test_first(instructions!(asl a), OP, AddressMode::Accumulator);
    }

    #[test]
    fn addr() {
        test_first(
            instructions!(asl test),
            OP,
            AddressMode::Absolute(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(asl test+1),
            OP,
            AddressMode::Absolute(AddressReference::with_offset(&"test", 1)),
        );
    }

    #[test]
    fn addr_x() {
        test_first(
            instructions!(asl test,x),
            OP,
            AddressMode::AbsoluteX(AddressReference::new(&"test")),
        );
    }
}

mod bcc {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::BCC;

    #[test]
    fn addr() {
        test_first(
            instructions!(bcc test),
            OP,
            AddressMode::Relative(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(bcc test+1),
            OP,
            AddressMode::Relative(AddressReference::with_offset(&"test", 1)),
        );
    }
}

mod bcs {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::BCS;

    #[test]
    fn addr() {
        test_first(
            instructions!(bcs test),
            OP,
            AddressMode::Relative(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(bcs test+1),
            OP,
            AddressMode::Relative(AddressReference::with_offset(&"test", 1)),
        );
    }
}

mod beq {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::BEQ;

    #[test]
    fn addr() {
        test_first(
            instructions!(beq test),
            OP,
            AddressMode::Relative(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(beq test+1),
            OP,
            AddressMode::Relative(AddressReference::with_offset(&"test", 1)),
        );
    }
}

mod bit {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::BIT;

    #[test]
    fn addr() {
        test_first(
            instructions!(bit test),
            OP,
            AddressMode::Absolute(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(bit test+1),
            OP,
            AddressMode::Absolute(AddressReference::with_offset(&"test", 1)),
        );
    }
}

mod bmi {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::BMI;

    #[test]
    fn addr() {
        test_first(
            instructions!(bmi test),
            OP,
            AddressMode::Relative(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(bmi test+1),
            OP,
            AddressMode::Relative(AddressReference::with_offset(&"test", 1)),
        );
    }
}

mod bne {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::BNE;

    #[test]
    fn addr() {
        test_first(
            instructions!(bne test),
            OP,
            AddressMode::Relative(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(bne test+1),
            OP,
            AddressMode::Relative(AddressReference::with_offset(&"test", 1)),
        );
    }
}

mod bpl {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::BPL;

    #[test]
    fn addr() {
        test_first(
            instructions!(bpl test),
            OP,
            AddressMode::Relative(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(bpl test+1),
            OP,
            AddressMode::Relative(AddressReference::with_offset(&"test", 1)),
        );
    }
}

#[test]
fn brk() {
    test_first(instructions!(brk), Operation::BRK, AddressMode::Implied);
}

mod bvc {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::BVC;

    #[test]
    fn addr() {
        test_first(
            instructions!(bvc test),
            OP,
            AddressMode::Relative(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(bvc test+1),
            OP,
            AddressMode::Relative(AddressReference::with_offset(&"test", 1)),
        );
    }
}

mod bvs {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::BVS;

    #[test]
    fn addr() {
        test_first(
            instructions!(bvs test),
            OP,
            AddressMode::Relative(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(bvs test+1),
            OP,
            AddressMode::Relative(AddressReference::with_offset(&"test", 1)),
        );
    }
}

#[test]
fn clc() {
    test_first(instructions!(clc), Operation::CLC, AddressMode::Implied);
}

#[test]
fn cld() {
    test_first(instructions!(cld), Operation::CLD, AddressMode::Implied);
}

#[test]
fn cli() {
    test_first(instructions!(cli), Operation::CLI, AddressMode::Implied);
}

#[test]
fn clv() {
    test_first(instructions!(clv), Operation::CLV, AddressMode::Implied);
}

mod cmp {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::CMP;

    #[test]
    fn imm() {
        test_first(instructions!(cmp #99), OP, AddressMode::Immediate(Immediate::Byte(99)));
        test_first(
            instructions!(cmp #$99),
            OP,
            AddressMode::Immediate(Immediate::Byte(153)),
        )
    }

    #[test]
    fn imm_low() {
        test_first(
            instructions!(cmp #<test),
            OP,
            AddressMode::Immediate(Immediate::Low(AddressReference::new("test"))),
        );
    }

    #[test]
    fn imm_high() {
        test_first(
            instructions!(cmp #>test),
            OP,
            AddressMode::Immediate(Immediate::High(AddressReference::new("test"))),
        );
    }

    #[test]
    fn addr() {
        test_first(
            instructions!(cmp test),
            OP,
            AddressMode::Absolute(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(cmp test+1),
            OP,
            AddressMode::Absolute(AddressReference::with_offset(&"test", 1)),
        );
    }

    #[test]
    fn addr_x() {
        test_first(
            instructions!(cmp test,x),
            OP,
            AddressMode::AbsoluteX(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_y() {
        test_first(
            instructions!(cmp test,y),
            OP,
            AddressMode::AbsoluteY(AddressReference::new(&"test")),
        );
    }
}

mod cpx {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::CPX;

    #[test]
    fn imm() {
        test_first(instructions!(cpx #99), OP, AddressMode::Immediate(Immediate::Byte(99)));
        test_first(
            instructions!(cpx #$99),
            OP,
            AddressMode::Immediate(Immediate::Byte(153)),
        )
    }

    #[test]
    fn imm_low() {
        test_first(
            instructions!(cpx #<test),
            OP,
            AddressMode::Immediate(Immediate::Low(AddressReference::new("test"))),
        );
    }

    #[test]
    fn imm_high() {
        test_first(
            instructions!(cpx #>test),
            OP,
            AddressMode::Immediate(Immediate::High(AddressReference::new("test"))),
        );
    }

    #[test]
    fn addr() {
        test_first(
            instructions!(cpx test),
            OP,
            AddressMode::Absolute(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(cpx test+1),
            OP,
            AddressMode::Absolute(AddressReference::with_offset(&"test", 1)),
        );
    }
}

mod cpy {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::CPY;

    #[test]
    fn imm() {
        test_first(instructions!(cpy #99), OP, AddressMode::Immediate(Immediate::Byte(99)));
        test_first(
            instructions!(cpy #$99),
            OP,
            AddressMode::Immediate(Immediate::Byte(153)),
        )
    }

    #[test]
    fn imm_low() {
        test_first(
            instructions!(cpy #<test),
            OP,
            AddressMode::Immediate(Immediate::Low(AddressReference::new("test"))),
        );
    }

    #[test]
    fn imm_high() {
        test_first(
            instructions!(cpy #>test),
            OP,
            AddressMode::Immediate(Immediate::High(AddressReference::new("test"))),
        );
    }

    #[test]
    fn addr() {
        test_first(
            instructions!(cpy test),
            OP,
            AddressMode::Absolute(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(cpy test+1),
            OP,
            AddressMode::Absolute(AddressReference::with_offset(&"test", 1)),
        );
    }
}

mod dec {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::DEC;

    #[test]
    fn addr() {
        test_first(
            instructions!(dec test),
            OP,
            AddressMode::Absolute(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(dec test+1),
            OP,
            AddressMode::Absolute(AddressReference::with_offset(&"test", 1)),
        );
    }

    #[test]
    fn addr_x() {
        test_first(
            instructions!(dec test,x),
            OP,
            AddressMode::AbsoluteX(AddressReference::new(&"test")),
        );
    }
}

#[test]
fn dex() {
    test_first(instructions!(dex), Operation::DEX, AddressMode::Implied);
}

#[test]
fn dey() {
    test_first(instructions!(dey), Operation::DEY, AddressMode::Implied);
}

mod eor {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::EOR;

    #[test]
    fn imm() {
        test_first(instructions!(eor #99), OP, AddressMode::Immediate(Immediate::Byte(99)));
        test_first(
            instructions!(eor #$99),
            OP,
            AddressMode::Immediate(Immediate::Byte(153)),
        )
    }

    #[test]
    fn imm_low() {
        test_first(
            instructions!(eor #<test),
            OP,
            AddressMode::Immediate(Immediate::Low(AddressReference::new("test"))),
        );
    }

    #[test]
    fn imm_high() {
        test_first(
            instructions!(eor #>test),
            OP,
            AddressMode::Immediate(Immediate::High(AddressReference::new("test"))),
        );
    }

    #[test]
    fn addr() {
        test_first(
            instructions!(eor test),
            OP,
            AddressMode::Absolute(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(eor test+1),
            OP,
            AddressMode::Absolute(AddressReference::with_offset(&"test", 1)),
        );
    }

    #[test]
    fn addr_x() {
        test_first(
            instructions!(eor test,x),
            OP,
            AddressMode::AbsoluteX(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_y() {
        test_first(
            instructions!(eor test,y),
            OP,
            AddressMode::AbsoluteY(AddressReference::new(&"test")),
        );
    }
}

mod inc {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::INC;

    #[test]
    fn addr() {
        test_first(
            instructions!(inc test),
            OP,
            AddressMode::Absolute(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(inc test+1),
            OP,
            AddressMode::Absolute(AddressReference::with_offset(&"test", 1)),
        );
    }

    #[test]
    fn addr_x() {
        test_first(
            instructions!(inc test,x),
            OP,
            AddressMode::AbsoluteX(AddressReference::new(&"test")),
        );
    }
}

#[test]
fn inx() {
    test_first(instructions!(inx), Operation::INX, AddressMode::Implied);
}

#[test]
fn iny() {
    test_first(instructions!(iny), Operation::INY, AddressMode::Implied);
}

mod jmp {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::JMP;

    #[test]
    fn addr() {
        test_first(
            instructions!(jmp test),
            OP,
            AddressMode::Absolute(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(jmp test+1),
            OP,
            AddressMode::Absolute(AddressReference::with_offset(&"test", 1)),
        );
    }
}

mod jsr {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::JSR;

    #[test]
    fn addr() {
        test_first(
            instructions!(jsr test),
            OP,
            AddressMode::Absolute(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(jsr test+1),
            OP,
            AddressMode::Absolute(AddressReference::with_offset(&"test", 1)),
        );
    }
}

mod lda {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::LDA;

    #[test]
    fn imm() {
        test_first(instructions!(lda #99), OP, AddressMode::Immediate(Immediate::Byte(99)));
        test_first(
            instructions!(lda #$99),
            OP,
            AddressMode::Immediate(Immediate::Byte(153)),
        )
    }

    #[test]
    fn imm_low() {
        test_first(
            instructions!(lda #<test),
            OP,
            AddressMode::Immediate(Immediate::Low(AddressReference::new("test"))),
        );
    }

    #[test]
    fn imm_high() {
        test_first(
            instructions!(lda #>test),
            OP,
            AddressMode::Immediate(Immediate::High(AddressReference::new("test"))),
        );
    }

    #[test]
    fn addr() {
        test_first(
            instructions!(lda test),
            OP,
            AddressMode::Absolute(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(lda test+1),
            OP,
            AddressMode::Absolute(AddressReference::with_offset(&"test", 1)),
        );
    }

    #[test]
    fn addr_x() {
        test_first(
            instructions!(lda test,x),
            OP,
            AddressMode::AbsoluteX(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_y() {
        test_first(
            instructions!(lda test,y),
            OP,
            AddressMode::AbsoluteY(AddressReference::new(&"test")),
        );
    }
}

mod ldx {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::LDX;

    #[test]
    fn imm() {
        test_first(instructions!(ldx #99), OP, AddressMode::Immediate(Immediate::Byte(99)));
        test_first(
            instructions!(ldx #$99),
            OP,
            AddressMode::Immediate(Immediate::Byte(153)),
        )
    }

    #[test]
    fn imm_low() {
        test_first(
            instructions!(ldx #<test),
            OP,
            AddressMode::Immediate(Immediate::Low(AddressReference::new("test"))),
        );
    }

    #[test]
    fn imm_high() {
        test_first(
            instructions!(ldx #>test),
            OP,
            AddressMode::Immediate(Immediate::High(AddressReference::new("test"))),
        );
    }

    #[test]
    fn addr() {
        test_first(
            instructions!(ldx test),
            OP,
            AddressMode::Absolute(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(ldx test+1),
            OP,
            AddressMode::Absolute(AddressReference::with_offset(&"test", 1)),
        );
    }

    #[test]
    fn addr_y() {
        test_first(
            instructions!(ldx test,y),
            OP,
            AddressMode::AbsoluteY(AddressReference::new(&"test")),
        );
    }
}

mod ldy {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::LDY;

    #[test]
    fn imm() {
        test_first(instructions!(ldy #99), OP, AddressMode::Immediate(Immediate::Byte(99)));
        test_first(
            instructions!(ldy #$99),
            OP,
            AddressMode::Immediate(Immediate::Byte(153)),
        )
    }

    #[test]
    fn imm_low() {
        test_first(
            instructions!(ldy #<test),
            OP,
            AddressMode::Immediate(Immediate::Low(AddressReference::new("test"))),
        );
    }

    #[test]
    fn imm_high() {
        test_first(
            instructions!(ldy #>test),
            OP,
            AddressMode::Immediate(Immediate::High(AddressReference::new("test"))),
        );
    }

    #[test]
    fn addr() {
        test_first(
            instructions!(ldy test),
            OP,
            AddressMode::Absolute(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(ldy test+1),
            OP,
            AddressMode::Absolute(AddressReference::with_offset(&"test", 1)),
        );
    }

    #[test]
    fn addr_x() {
        test_first(
            instructions!(ldy test,x),
            OP,
            AddressMode::AbsoluteX(AddressReference::new(&"test")),
        );
    }
}

mod lsr {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::LSR;

    #[test]
    fn acc() {
        test_first(instructions!(lsr a), OP, AddressMode::Accumulator);
    }

    #[test]
    fn addr() {
        test_first(
            instructions!(lsr test),
            OP,
            AddressMode::Absolute(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(lsr test+1),
            OP,
            AddressMode::Absolute(AddressReference::with_offset(&"test", 1)),
        );
    }

    #[test]
    fn addr_x() {
        test_first(
            instructions!(lsr test,x),
            OP,
            AddressMode::AbsoluteX(AddressReference::new(&"test")),
        );
    }
}

#[test]
fn nop() {
    test_first(instructions!(nop), Operation::NOP, AddressMode::Implied);
}

mod ora {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::ORA;

    #[test]
    fn imm() {
        test_first(instructions!(ora #99), OP, AddressMode::Immediate(Immediate::Byte(99)));
        test_first(
            instructions!(ora #$99),
            OP,
            AddressMode::Immediate(Immediate::Byte(153)),
        )
    }

    #[test]
    fn imm_low() {
        test_first(
            instructions!(ora #<test),
            OP,
            AddressMode::Immediate(Immediate::Low(AddressReference::new("test"))),
        );
    }

    #[test]
    fn imm_high() {
        test_first(
            instructions!(ora #>test),
            OP,
            AddressMode::Immediate(Immediate::High(AddressReference::new("test"))),
        );
    }

    #[test]
    fn addr() {
        test_first(
            instructions!(ora test),
            OP,
            AddressMode::Absolute(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(ora test+1),
            OP,
            AddressMode::Absolute(AddressReference::with_offset(&"test", 1)),
        );
    }

    #[test]
    fn addr_x() {
        test_first(
            instructions!(ora test,x),
            OP,
            AddressMode::AbsoluteX(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_y() {
        test_first(
            instructions!(ora test,y),
            OP,
            AddressMode::AbsoluteY(AddressReference::new(&"test")),
        );
    }
}

#[test]
fn pha() {
    test_first(instructions!(pha), Operation::PHA, AddressMode::Implied);
}

#[test]
fn php() {
    test_first(instructions!(php), Operation::PHP, AddressMode::Implied);
}

#[test]
fn pla() {
    test_first(instructions!(pla), Operation::PLA, AddressMode::Implied);
}

#[test]
fn plp() {
    test_first(instructions!(plp), Operation::PLP, AddressMode::Implied);
}

mod rol {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::ROL;

    #[test]
    fn acc() {
        test_first(instructions!(rol a), OP, AddressMode::Accumulator);
    }

    #[test]
    fn addr() {
        test_first(
            instructions!(rol test),
            OP,
            AddressMode::Absolute(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(rol test+1),
            OP,
            AddressMode::Absolute(AddressReference::with_offset(&"test", 1)),
        );
    }

    #[test]
    fn addr_x() {
        test_first(
            instructions!(rol test,x),
            OP,
            AddressMode::AbsoluteX(AddressReference::new(&"test")),
        );
    }
}

mod ror {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::ROR;

    #[test]
    fn acc() {
        test_first(instructions!(ror a), OP, AddressMode::Accumulator);
    }

    #[test]
    fn addr() {
        test_first(
            instructions!(ror test),
            OP,
            AddressMode::Absolute(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(ror test+1),
            OP,
            AddressMode::Absolute(AddressReference::with_offset(&"test", 1)),
        );
    }

    #[test]
    fn addr_x() {
        test_first(
            instructions!(ror test,x),
            OP,
            AddressMode::AbsoluteX(AddressReference::new(&"test")),
        );
    }
}

#[test]
fn rti() {
    test_first(instructions!(rti), Operation::RTI, AddressMode::Implied);
}

#[test]
fn rts() {
    test_first(instructions!(rts), Operation::RTS, AddressMode::Implied);
}

mod sbc {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::SBC;

    #[test]
    fn imm() {
        test_first(instructions!(sbc #99), OP, AddressMode::Immediate(Immediate::Byte(99)));
        test_first(
            instructions!(sbc #$99),
            OP,
            AddressMode::Immediate(Immediate::Byte(153)),
        )
    }

    #[test]
    fn imm_low() {
        test_first(
            instructions!(sbc #<test),
            OP,
            AddressMode::Immediate(Immediate::Low(AddressReference::new("test"))),
        );
    }

    #[test]
    fn imm_high() {
        test_first(
            instructions!(sbc #>test),
            OP,
            AddressMode::Immediate(Immediate::High(AddressReference::new("test"))),
        );
    }

    #[test]
    fn addr() {
        test_first(
            instructions!(sbc test),
            OP,
            AddressMode::Absolute(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(sbc test+1),
            OP,
            AddressMode::Absolute(AddressReference::with_offset(&"test", 1)),
        );
    }

    #[test]
    fn addr_x() {
        test_first(
            instructions!(sbc test,x),
            OP,
            AddressMode::AbsoluteX(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_y() {
        test_first(
            instructions!(sbc test,y),
            OP,
            AddressMode::AbsoluteY(AddressReference::new(&"test")),
        );
    }
}

#[test]
fn sec() {
    test_first(instructions!(sec), Operation::SEC, AddressMode::Implied);
}

#[test]
fn sed() {
    test_first(instructions!(sed), Operation::SED, AddressMode::Implied);
}

#[test]
fn sei() {
    test_first(instructions!(sei), Operation::SEI, AddressMode::Implied);
}

mod sta {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::STA;

    #[test]
    fn addr() {
        test_first(
            instructions!(sta test),
            OP,
            AddressMode::Absolute(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(sta test+1),
            OP,
            AddressMode::Absolute(AddressReference::with_offset(&"test", 1)),
        );
    }

    #[test]
    fn addr_x() {
        test_first(
            instructions!(sta test,x),
            OP,
            AddressMode::AbsoluteX(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_y() {
        test_first(
            instructions!(sta test,y),
            OP,
            AddressMode::AbsoluteY(AddressReference::new(&"test")),
        );
    }
}

mod stx {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::STX;

    #[test]
    fn addr() {
        test_first(
            instructions!(stx test),
            OP,
            AddressMode::Absolute(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(stx test+1),
            OP,
            AddressMode::Absolute(AddressReference::with_offset(&"test", 1)),
        );
    }

    #[test]
    fn addr_y() {
        test_first(
            instructions!(stx test,y),
            OP,
            AddressMode::AbsoluteY(AddressReference::new(&"test")),
        );
    }
}

mod sty {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{address_mode::*, label::AddressReference},
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::STY;

    #[test]
    fn addr() {
        test_first(
            instructions!(sty test),
            OP,
            AddressMode::Absolute(AddressReference::new(&"test")),
        );
    }

    #[test]
    fn addr_offs() {
        test_first(
            instructions!(sty test+1),
            OP,
            AddressMode::Absolute(AddressReference::with_offset(&"test", 1)),
        );
    }

    #[test]
    fn addr_x() {
        test_first(
            instructions!(sty test,x),
            OP,
            AddressMode::AbsoluteX(AddressReference::new(&"test")),
        );
    }
}

#[test]
fn tax() {
    test_first(instructions!(tax), Operation::TAX, AddressMode::Implied);
}

#[test]
fn tay() {
    test_first(instructions!(tay), Operation::TAY, AddressMode::Implied);
}

#[test]
fn tsx() {
    test_first(instructions!(tsx), Operation::TSX, AddressMode::Implied);
}

#[test]
fn txa() {
    test_first(instructions!(txa), Operation::TXA, AddressMode::Implied);
}

#[test]
fn txs() {
    test_first(instructions!(txs), Operation::TXS, AddressMode::Implied);
}

#[test]
fn tya() {
    test_first(instructions!(tya), Operation::TYA, AddressMode::Implied);
}
