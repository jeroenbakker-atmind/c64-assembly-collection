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

macro_rules! implied_test {
    ($op:ident, $OP:ident) => {
        #[test]
        fn $op() {
            test_first(instructions!($op), Operation::$OP, AddressMode::Implied);
        }
    };
}

implied_test!(brk, BRK);
implied_test!(cld, CLD);
implied_test!(cli, CLI);
implied_test!(clv, CLV);
implied_test!(dex, DEX);
implied_test!(dey, DEY);
implied_test!(inx, INX);
implied_test!(iny, INY);
implied_test!(nop, NOP);
implied_test!(pha, PHA);
implied_test!(psr, PSR);
implied_test!(pla, PLA);
implied_test!(plp, PLP);
implied_test!(rti, RTI);
implied_test!(sed, SED);
implied_test!(sei, SEI);
implied_test!(tax, TAX);
implied_test!(tay, TAY);
implied_test!(tsx, TSX);
implied_test!(txs, TXS);
implied_test!(txa, TXA);
implied_test!(tya, TYA);
implied_test!(clc, CLC);
implied_test!(rts, RTS);

mod adc {
    use c64_assembler::{
        instruction::operation::Operation,
        memory::{
            address_mode::{AddressMode, Immediate},
            label::AddressReference,
        },
    };
    use c64_assembler_macro::instructions;

    use crate::test_first;
    const OP: Operation = Operation::ADC;

    #[test]
    fn adc_imm() {
        test_first(
            instructions!(adc #99),
            OP,
            AddressMode::Immediate(Immediate::Byte(99)),
        );
        test_first(
            instructions!(adc #$99),
            OP,
            AddressMode::Immediate(Immediate::Byte(153)),
        )
    }

    #[test]
    fn adc_imm_low() {
        test_first(
            instructions!(adc #<test),
            OP,
            AddressMode::Immediate(Immediate::Low(AddressReference::new("test"))),
        );
    }

    #[test]
    fn adc_imm_high() {
        test_first(
            instructions!(adc #>test),
            OP,
            AddressMode::Immediate(Immediate::High(AddressReference::new("test"))),
        );
    }

    #[test]
    fn adc_addr() {
        test_first(
            instructions!(adc test),
            OP,
            AddressMode::Absolute(AddressReference::new("test")),
        );
    }
    #[test]
    fn adc_addr_offs() {
        test_first(
            instructions!(adc test+1),
            OP,
            AddressMode::Absolute(AddressReference::with_offset("test", 1)),
        );
    }
    #[test]
    fn adc_addr_x() {
        test_first(
            instructions!(adc test,x),
            OP,
            AddressMode::AbsoluteX(AddressReference::new("test")),
        );
    }
    #[test]
    fn adc_addr_y() {
        test_first(
            instructions!(adc test,y),
            OP,
            AddressMode::AbsoluteY(AddressReference::new("test")),
        );
    }
    /*
    #[test]
    fn adc_ind_x() {
        test_first(
            instructions!(adc(test, x)),
            OP,
            AddressMode::IndexedIndirect(AddressReference::new("test")),
        );
    }

    #[test]
    fn adc_ind_y() {
        test_first(
            instructions!(adc(test), y),
            OP,
            AddressMode::IndirectIndexed(AddressReference::new("test")),
        );
    }
    */
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
    const OP: Operation = Operation::LDA;

    #[test]
    fn lda_imm() {
        test_first(
            instructions!(lda #99),
            OP,
            AddressMode::Immediate(Immediate::Byte(99)),
        );
        test_first(
            instructions!(lda #$99),
            OP,
            AddressMode::Immediate(Immediate::Byte(153)),
        )
    }

    #[test]
    fn lda_imm_low() {
        test_first(
            instructions!(lda #<test),
            OP,
            AddressMode::Immediate(Immediate::Low(AddressReference::new("test"))),
        );
    }

    #[test]
    fn lda_imm_high() {
        test_first(
            instructions!(lda #>test),
            OP,
            AddressMode::Immediate(Immediate::High(AddressReference::new("test"))),
        );
    }

    #[test]
    fn lda_addr() {
        test_first(
            instructions!(lda test),
            OP,
            AddressMode::Absolute(AddressReference::new("test")),
        );
    }
    #[test]
    fn lda_addr_offs() {
        test_first(
            instructions!(lda test+1),
            OP,
            AddressMode::Absolute(AddressReference::with_offset("test", 1)),
        );
    }
    #[test]
    fn lda_addr_x() {
        test_first(
            instructions!(lda test,x),
            OP,
            AddressMode::AbsoluteX(AddressReference::new("test")),
        );
    }
    #[test]
    fn lda_addr_y() {
        test_first(
            instructions!(lda test,y),
            OP,
            AddressMode::AbsoluteY(AddressReference::new("test")),
        );
    }
    /*
    #[test]
    fn lda_ind_x() {
        test_first(
            instructions!(lda(test, x)),
            OP,
            AddressMode::IndexedIndirect(AddressReference::new("test")),
        );
    }

    #[test]
    fn lda_ind_y() {
        test_first(
            instructions!(lda(test), y),
            OP,
            AddressMode::IndirectIndexed(AddressReference::new("test")),
        );
    }
    */
}
