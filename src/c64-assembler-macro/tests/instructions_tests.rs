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

macro_rules! _accumulator_test {
    ($op:ident) => {
    #[test]
    fn acc() {
        test_first(
            instructions!($op a),
            OP,
            AddressMode::Accumulator,
        );
    }
    }
}

macro_rules! _immediate_test{
    ($op:ident) => {
    #[test]
    fn imm() {
        test_first(
            instructions!($op #99),
            OP,
            AddressMode::Immediate(Immediate::Byte(99)),
        );
        /*
        test_first(
            instructions!($op #$99),
            OP,
            AddressMode::Immediate(Immediate::Byte(153)),
        )
        */
    }

    #[test]
    fn imm_low() {
        test_first(
            instructions!($op #<test),
            OP,
            AddressMode::Immediate(Immediate::Low(AddressReference::new("test"))),
        );
    }

    #[test]
    fn imm_high() {
        test_first(
            instructions!($op #>test),
            OP,
            AddressMode::Immediate(Immediate::High(AddressReference::new("test"))),
        );
    }
    }
}

macro_rules! __absolute_x_test {
    ($op:ident) => {
    #[test]
    fn addr_x() {
        test_first(
            instructions!($op test,x),
            OP,
            AddressMode::AbsoluteX(AddressReference::new("test")),
        );
    }
    }
}
macro_rules! __absolute_y_test {
    ($op:ident) => {
    #[test]
    fn addr_y() {
        test_first(
            instructions!($op test,y),
            OP,
            AddressMode::AbsoluteY(AddressReference::new("test")),
        );
    }
    }
}

macro_rules! __absolute_test {
    ($op:ident) => {
    #[test]
    fn addr() {
        test_first(
            instructions!($op test),
            OP,
            AddressMode::Absolute(AddressReference::new("test")),
        );
    }
    #[test]
    fn addr_offs() {
        test_first(
            instructions!($op test+1),
            OP,
            AddressMode::Absolute(AddressReference::with_offset("test", 1)),
        );
    }
    }
}

macro_rules! _absolute_x_test {
    ($op:ident) => {
        __absolute_test!($op);
        __absolute_x_test!($op);
    };
}

macro_rules! _absolute_y_test {
    ($op:ident) => {
        __absolute_test!($op);
        __absolute_y_test!($op);
    };
}

macro_rules! _absolute_test {
    ($op:ident) => {
        __absolute_test!($op);
        __absolute_x_test!($op);
        __absolute_y_test!($op);
    };
}

macro_rules! _indirect_test {
    ($op:ident) => {
        #[test]
        fn ind_x() {
            /*
            test_first(
                instructions!($op (test),x),
                OP,
                AddressMode::IndirectIndexed(AddressReference::new("test")),
            );
            */
        }

        #[test]
        fn ind_y() {
            /*
            test_first(
                instructions!($op (test,y)),
                OP,
                AddressMode::IndexedIndirect(AddressReference::new("test")),
            );
            */
        }
    };
}
macro_rules! _module_header {
    ($OP:ident) => {
        use c64_assembler::{
            instruction::operation::Operation,
            memory::{address_mode::*, label::AddressReference},
        };
        use c64_assembler_macro::instructions;

        use crate::test_first;
        const OP: Operation = Operation::$OP;
    };
}

macro_rules! immediate_absolute_indirect_test {
    ($op:ident, $OP:ident) => {
        mod $op {
            _module_header!($OP);
            _immediate_test!($op);
            _absolute_test!($op);
            _indirect_test!($op);
        }
    };
}

macro_rules! absolute_indirect_test {
    ($op:ident, $OP:ident) => {
        mod $op {
            _module_header!($OP);
            _absolute_test!($op);
            _indirect_test!($op);
        }
    };
}

macro_rules! accumulator_absolute_x {
    ($op:ident, $OP:ident) => {
        mod $op {
            _module_header!($OP);
            _accumulator_test!($op);
            _absolute_x_test!($op);
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

immediate_absolute_indirect_test!(adc, ADC);
// immediate_absolute_indirect_test!(and, AND);
accumulator_absolute_x!(asl, ASL);
immediate_absolute_indirect_test!(lda, LDA);
absolute_indirect_test!(sta, STA);
