use c64_assembler::{
    builder::instruction::Instructions, instruction::operation::Operation,
    memory::address_mode::AddressMode,
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

#[test]
fn brk() {
    test_first(instructions!(brk), Operation::BRK, AddressMode::Implied);
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

#[test]
fn dex() {
    test_first(instructions!(dex), Operation::DEX, AddressMode::Implied);
}

#[test]
fn dey() {
    test_first(instructions!(dey), Operation::DEY, AddressMode::Implied);
}

#[test]
fn inx() {
    test_first(instructions!(inx), Operation::INX, AddressMode::Implied);
}

#[test]
fn iny() {
    test_first(instructions!(iny), Operation::INY, AddressMode::Implied);
}

#[test]
fn nop() {
    test_first(instructions!(nop), Operation::NOP, AddressMode::Implied);
}

#[test]
fn pha() {
    test_first(instructions!(pha), Operation::PHA, AddressMode::Implied);
}

#[test]
fn psr() {
    test_first(instructions!(psr), Operation::PSR, AddressMode::Implied);
}

#[test]
fn pla() {
    test_first(instructions!(pla), Operation::PLA, AddressMode::Implied);
}

#[test]
fn plp() {
    test_first(instructions!(plp), Operation::PLP, AddressMode::Implied);
}

#[test]
fn rti() {
    test_first(instructions!(rti), Operation::RTI, AddressMode::Implied);
}

#[test]
fn sed() {
    test_first(instructions!(sed), Operation::SED, AddressMode::Implied);
}

#[test]
fn sei() {
    test_first(instructions!(sei), Operation::SEI, AddressMode::Implied);
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

#[test]
fn clc() {
    test_first(instructions!(clc), Operation::CLC, AddressMode::Implied);
}

#[test]
fn rts() {
    test_first(instructions!(rts), Operation::RTS, AddressMode::Implied);
}

mod lda {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{
            address_mode::{AddressMode, Immediate},
            label::AddressReference,
        },
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;

    #[test]
    fn lda_imm() {
        test_first(
            instructions!(lda #99),
            Operation::LDA,
            AddressMode::Immediate(Immediate::Byte(99)),
        );
        test_first(
            instructions!(lda #$99),
            Operation::LDA,
            AddressMode::Immediate(Immediate::Byte(153)),
        )
    }

    #[test]
    fn lda_imm_low() {
        test_first(
            instructions!(lda #<test),
            Operation::LDA,
            AddressMode::Immediate(Immediate::Low(AddressReference::new("test"))),
        );
    }

    #[test]
    fn lda_imm_high() {
        test_first(
            instructions!(lda #>test),
            Operation::LDA,
            AddressMode::Immediate(Immediate::High(AddressReference::new("test"))),
        );
    }

    #[test]
    fn lda_addr() {
        test_first(
            instructions!(lda test),
            Operation::LDA,
            AddressMode::Absolute(AddressReference::new("test")),
        );
    }
    #[test]
    fn lda_addr_offs() {
        test_first(
            instructions!(lda test+1),
            Operation::LDA,
            AddressMode::Absolute(AddressReference::with_offset("test", 1)),
        );
    }
    #[test]
    fn lda_addr_x() {
        test_first(
            instructions!(lda test,x),
            Operation::LDA,
            AddressMode::AbsoluteX(AddressReference::new("test")),
        );
    }
    #[test]
    fn lda_addr_y() {
        test_first(
            instructions!(lda test,y),
            Operation::LDA,
            AddressMode::AbsoluteY(AddressReference::new("test")),
        );
    }
    /*
    #[test]
    fn lda_ind_x() {
        test_first(
            instructions!(lda(test, x)),
            Operation::LDA,
            AddressMode::IndexedIndirect(AddressReference::new("test")),
        );
    }

    #[test]
    fn lda_ind_y() {
        test_first(
            instructions!(lda(test), y),
            Operation::LDA,
            AddressMode::IndirectIndexed(AddressReference::new("test")),
        );
    }
    */
}
