use c64_assembler_6502::isa::OpCode;

use crate::{
    builder::{application::Application, instruction::Instructions, module::Module},
    instruction::{operation::Operation, Instruction},
    memory::{
        address_mode::{AddressMode, Immediate},
        Address, ZeroPage,
    },
};

use super::Generator;

#[derive(Default, Debug)]
pub struct ProgramGenerator {
    output: Vec<u8>,
}

impl Generator for ProgramGenerator {
    type Output = Vec<u8>;

    fn generate(mut self, application: Application) -> Self::Output {
        self.add_u16(application.entry_point);
        for module in &application.modules {
            self.generate_module(&application, module);
        }
        self.output
    }
}

const ADC_IMMEDIATE: OpCode = 0x69;
const ADC_ABSOLUTE: OpCode = 0x6d;
const ADC_ABSOLUTE_X: OpCode = 0x7d;
const ADC_ABSOLUTE_Y: OpCode = 0x79;
const ADC_ZEROPAGE: OpCode = 0x65;
const ADC_ZEROPAGE_X: OpCode = 0x75;
const ADC_INDEXED_INDIRECT: OpCode = 0x61;
const ADC_INDIRECT_INDEXED: OpCode = 0x71;
const AND_IMMEDIATE: OpCode = 0x29;
const AND_ABSOLUTE: OpCode = 0x2d;
const AND_ABSOLUTE_X: OpCode = 0x3d;
const AND_ABSOLUTE_Y: OpCode = 0x39;
const AND_ZEROPAGE: OpCode = 0x25;
const AND_ZEROPAGE_X: OpCode = 0x35;
const AND_INDEXED_INDIRECT: OpCode = 0x21;
const AND_INDIRECT_INDEXED: OpCode = 0x01;
const ASL_ACCUMULATOR: OpCode = 0x0a;
const ASL_ABSOLUTE: OpCode = 0x0e;
const ASL_ABSOLUTE_X: OpCode = 0x1e;
const ASL_ZEROPAGE: OpCode = 0x06;
const ASL_ZEROPAGE_X: OpCode = 0x16;
const BCC_RELATIVE: OpCode = 0x90;
const BCS_RELATIVE: OpCode = 0xb0;
const BEQ_RELATIVE: OpCode = 0xf0;
const BIT_ABSOLUTE: OpCode = 0x2c;
const BIT_ZEROPAGE: OpCode = 0x24;
const BMI_RELATIVE: OpCode = 0x30;
const BNE_RELATIVE: OpCode = 0xd0;
const BPL_RELATIVE: OpCode = 0x10;
const BRK: OpCode = 0x00;
const BVC_RELATIVE: OpCode = 0x50;
const BVS_RELATIVE: OpCode = 0x70;
const CLC: OpCode = 0x18;
const CLD: OpCode = 0xd8;
const CLI: OpCode = 0x58;
const CLV: OpCode = 0xb8;
const CMP_IMMEDIATE: OpCode = 0xc9;
const CMP_ABSOLUTE: OpCode = 0xcd;
const CMP_ABSOLUTE_X: OpCode = 0xdd;
const CMP_ABSOLUTE_Y: OpCode = 0xd9;
const CMP_ZEROPAGE: OpCode = 0xc5;
const CMP_ZEROPAGE_X: OpCode = 0xd5;
const CMP_INDEXED_INDIRECT: OpCode = 0xc1;
const CMP_INDIRECT_INDEXED: OpCode = 0xd1;
const CPX_IMMEDIATE: OpCode = 0xe0;
const CPX_ABSOLUTE: OpCode = 0xec;
const CPX_ZEROPAGE: OpCode = 0xe4;
const CPY_IMMEDIATE: OpCode = 0xc0;
const CPY_ABSOLUTE: OpCode = 0xcc;
const CPY_ZEROPAGE: OpCode = 0xc4;
const DEC_ABSOLUTE: OpCode = 0xce;
const DEC_ABSOLUTE_X: OpCode = 0xde;
const DEC_ZEROPAGE: OpCode = 0xc6;
const DEC_ZEROPAGE_X: OpCode = 0xd6;
const DEX: OpCode = 0xca;
const DEY: OpCode = 0x88;
const EOR_IMMEDIATE: OpCode = 0x49;
const EOR_ABSOLUTE: OpCode = 0x4d;
const EOR_ABSOLUTE_X: OpCode = 0x5d;
const EOR_ABSOLUTE_Y: OpCode = 0x59;
const EOR_ZEROPAGE: OpCode = 0x45;
const EOR_ZEROPAGE_X: OpCode = 0x55;
const EOR_INDEXED_INDIRECT: OpCode = 0x41;
const EOR_INDIRECT_INDEXED: OpCode = 0x51;
const INC_ABSOLUTE: OpCode = 0xee;
const INC_ABSOLUTE_X: OpCode = 0xfe;
const INC_ZEROPAGE: OpCode = 0xe6;
const INC_ZEROPAGE_X: OpCode = 0xf6;
const INX: OpCode = 0xe8;
const INY: OpCode = 0xc8;
const JMP_ABSOLUTE: OpCode = 0x4c;
const JMP_INDIRECT: OpCode = 0x6c;
const JSR_ABSOLUTE: OpCode = 0x20;
const LDA_IMMEDIATE: OpCode = 0xa9;
const LDA_ABSOLUTE: OpCode = 0xad;
const LDA_ABSOLUTE_X: OpCode = 0xbd;
const LDA_ABSOLUTE_Y: OpCode = 0xb9;
const LDA_ZEROPAGE: OpCode = 0xa5;
const LDA_ZEROPAGE_X: OpCode = 0xb5;
const LDA_INDEXED_INDIRECT: OpCode = 0xa1;
const LDA_INDIRECT_INDEXED: OpCode = 0xb1;
const LDX_IMMEDIATE: OpCode = 0xa2;
const LDX_ABSOLUTE: OpCode = 0xae;
const LDX_ABSOLUTE_Y: OpCode = 0xbe;
const LDX_ZEROPAGE: OpCode = 0xa6;
const LDX_ZEROPAGE_Y: OpCode = 0xb6;
const LDY_IMMEDIATE: OpCode = 0xa0;
const LDY_ABSOLUTE: OpCode = 0xac;
const LDY_ABSOLUTE_X: OpCode = 0xbc;
const LDY_ZEROPAGE: OpCode = 0xa4;
const LDY_ZEROPAGE_X: OpCode = 0xb4;
const LSR_ACCUMULATOR: OpCode = 0x4a;
const LSR_ABSOLUTE: OpCode = 0x4e;
const LSR_ABSOLUTE_X: OpCode = 0x5e;
const LSR_ZEROPAGE: OpCode = 0x46;
const LSR_ZEROPAGE_X: OpCode = 0x56;
const NOP: OpCode = 0xea;
const ORA_IMMEDIATE: OpCode = 0x09;
const ORA_ABSOLUTE: OpCode = 0x0d;
const ORA_ABSOLUTE_X: OpCode = 0x1d;
const ORA_ABSOLUTE_Y: OpCode = 0x19;
const ORA_ZEROPAGE: OpCode = 0x05;
const ORA_ZEROPAGE_X: OpCode = 0x15;
const ORA_INDEXED_INDIRECT: OpCode = 0x01;
const ORA_INDIRECT_INDEXED: OpCode = 0x11;
const PHA: OpCode = 0x48;
const PHP: OpCode = 0x08;
const PLA: OpCode = 0x68;
const PLP: OpCode = 0x28;
const ROL_ACCUMULATOR: OpCode = 0x2a;
const ROL_ABSOLUTE: OpCode = 0x2e;
const ROL_ABSOLUTE_X: OpCode = 0x3e;
const ROL_ZEROPAGE: OpCode = 0x26;
const ROL_ZEROPAGE_X: OpCode = 0x36;
const ROR_ACCUMULATOR: OpCode = 0x6a;
const ROR_ABSOLUTE: OpCode = 0x6e;
const ROR_ABSOLUTE_X: OpCode = 0x7e;
const ROR_ZEROPAGE: OpCode = 0x66;
const ROR_ZEROPAGE_X: OpCode = 0x76;
const RTI: OpCode = 0x40;
const RTS: OpCode = 0x60;
const SBC_IMMEDIATE: OpCode = 0xe9;
const SBC_ABSOLUTE: OpCode = 0xed;
const SBC_ABSOLUTE_X: OpCode = 0xfd;
const SBC_ABSOLUTE_Y: OpCode = 0xf9;
const SBC_ZEROPAGE: OpCode = 0xe5;
const SBC_ZEROPAGE_X: OpCode = 0xf5;
const SBC_INDEXED_INDIRECT: OpCode = 0xe1;
const SBC_INDIRECT_INDEXED: OpCode = 0xf1;
const SEC: OpCode = 0x38;
const SED: OpCode = 0xf8;
const SEI: OpCode = 0x78;
const STA_ABSOLUTE: OpCode = 0x8d;
const STA_ABSOLUTE_X: OpCode = 0x9d;
const STA_ABSOLUTE_Y: OpCode = 0x99;
const STA_ZEROPAGE: OpCode = 0x85;
const STA_ZEROPAGE_X: OpCode = 0x95;
const STA_INDEXED_INDIRECT: OpCode = 0x81;
const STA_INDIRECT_INDEXED: OpCode = 0x91;
const STX_ABSOLUTE: OpCode = 0x8e;
const STX_ZEROPAGE: OpCode = 0x86;
const STX_ZEROPAGE_Y: OpCode = 0x96;
const STY_ABSOLUTE: OpCode = 0x8c;
const STY_ZEROPAGE: OpCode = 0x84;
const STY_ZEROPAGE_X: OpCode = 0x94;
const TAX: OpCode = 0xaa;
const TAY: OpCode = 0xa8;
const TSX: OpCode = 0xba;
const TXA: OpCode = 0x8a;
const TXS: OpCode = 0x9a;
const TYA: OpCode = 0x98;

const UNUSED: u8 = 0xFF;

impl ProgramGenerator {
    fn generate_module(&mut self, application: &Application, module: &Module) {
        self.generate_instructions(application, &module.instructions);
        for function in &module.functions {
            self.generate_instructions(application, &function.instructions);
        }
    }

    fn generate_instructions(&mut self, application: &Application, instructions: &Instructions) {
        for instruction in &instructions.instructions {
            self.generate_instruction(application, instruction);
        }
    }

    fn generate_instruction(&mut self, application: &Application, instruction: &Instruction) {
        match &instruction.operation {
            Operation::ADC => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                ADC_IMMEDIATE,
                UNUSED,
                ADC_ABSOLUTE,
                ADC_ABSOLUTE_X,
                ADC_ABSOLUTE_Y,
                ADC_ZEROPAGE,
                ADC_ZEROPAGE_X,
                UNUSED,
                UNUSED,
                UNUSED,
                ADC_INDIRECT_INDEXED,
                ADC_INDEXED_INDIRECT,
            ),

            Operation::AND => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                AND_IMMEDIATE,
                UNUSED,
                AND_ABSOLUTE,
                AND_ABSOLUTE_X,
                AND_ABSOLUTE_Y,
                AND_ZEROPAGE,
                AND_ZEROPAGE_X,
                UNUSED,
                UNUSED,
                UNUSED,
                AND_INDIRECT_INDEXED,
                AND_INDEXED_INDIRECT,
            ),

            Operation::ASL => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                UNUSED,
                ASL_ACCUMULATOR,
                ASL_ABSOLUTE,
                ASL_ABSOLUTE_X,
                UNUSED,
                ASL_ZEROPAGE,
                ASL_ZEROPAGE_X,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::BCC => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                BCC_RELATIVE,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::BCS => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                BCS_RELATIVE,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::BEQ => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                BEQ_RELATIVE,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::BIT => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                UNUSED,
                UNUSED,
                BIT_ABSOLUTE,
                UNUSED,
                UNUSED,
                BIT_ZEROPAGE,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::BMI => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                BMI_RELATIVE,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::BNE => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                BNE_RELATIVE,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::BPL => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                BPL_RELATIVE,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::BRK => self.add_byte_code(
                application,
                &instruction.address_mode,
                BRK,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::BVC => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                BVC_RELATIVE,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::BVS => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                BVS_RELATIVE,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::CLC => self.add_byte_code(
                application,
                &instruction.address_mode,
                CLC,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::CLD => self.add_byte_code(
                application,
                &instruction.address_mode,
                CLD,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::CLI => self.add_byte_code(
                application,
                &instruction.address_mode,
                CLI,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::CLV => self.add_byte_code(
                application,
                &instruction.address_mode,
                CLV,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::CMP => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                CMP_IMMEDIATE,
                UNUSED,
                CMP_ABSOLUTE,
                CMP_ABSOLUTE_X,
                CMP_ABSOLUTE_Y,
                CMP_ZEROPAGE,
                CMP_ZEROPAGE_X,
                UNUSED,
                UNUSED,
                UNUSED,
                CMP_INDIRECT_INDEXED,
                CMP_INDEXED_INDIRECT,
            ),

            Operation::CPX => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                CPX_IMMEDIATE,
                UNUSED,
                CPX_ABSOLUTE,
                UNUSED,
                UNUSED,
                CPX_ZEROPAGE,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::CPY => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                CPY_IMMEDIATE,
                UNUSED,
                CPY_ABSOLUTE,
                UNUSED,
                UNUSED,
                CPY_ZEROPAGE,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::DEC => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                UNUSED,
                UNUSED,
                DEC_ABSOLUTE,
                DEC_ABSOLUTE_X,
                UNUSED,
                DEC_ZEROPAGE,
                DEC_ZEROPAGE_X,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::DEX => self.add_byte_code(
                application,
                &instruction.address_mode,
                DEX,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::DEY => self.add_byte_code(
                application,
                &instruction.address_mode,
                DEY,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::EOR => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                EOR_IMMEDIATE,
                UNUSED,
                EOR_ABSOLUTE,
                EOR_ABSOLUTE_X,
                EOR_ABSOLUTE_Y,
                EOR_ZEROPAGE,
                EOR_ZEROPAGE_X,
                UNUSED,
                UNUSED,
                UNUSED,
                EOR_INDIRECT_INDEXED,
                EOR_INDEXED_INDIRECT,
            ),

            Operation::INC => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                UNUSED,
                UNUSED,
                INC_ABSOLUTE,
                INC_ABSOLUTE_X,
                UNUSED,
                INC_ZEROPAGE,
                INC_ZEROPAGE_X,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::INX => self.add_byte_code(
                application,
                &instruction.address_mode,
                INX,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::INY => self.add_byte_code(
                application,
                &instruction.address_mode,
                INY,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::JMP => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                UNUSED,
                UNUSED,
                JMP_ABSOLUTE,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                JMP_INDIRECT,
                UNUSED,
                UNUSED,
            ),

            Operation::JSR => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                UNUSED,
                UNUSED,
                JSR_ABSOLUTE,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::LDA => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                LDA_IMMEDIATE,
                UNUSED,
                LDA_ABSOLUTE,
                LDA_ABSOLUTE_X,
                LDA_ABSOLUTE_Y,
                LDA_ZEROPAGE,
                LDA_ZEROPAGE_X,
                UNUSED,
                UNUSED,
                UNUSED,
                LDA_INDIRECT_INDEXED,
                LDA_INDEXED_INDIRECT,
            ),

            Operation::LDX => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                LDX_IMMEDIATE,
                UNUSED,
                LDX_ABSOLUTE,
                UNUSED,
                LDX_ABSOLUTE_Y,
                LDX_ZEROPAGE,
                UNUSED,
                LDX_ZEROPAGE_Y,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::LDY => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                LDY_IMMEDIATE,
                UNUSED,
                LDY_ABSOLUTE,
                LDY_ABSOLUTE_X,
                UNUSED,
                LDY_ZEROPAGE,
                LDY_ZEROPAGE_X,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::LSR => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                UNUSED,
                LSR_ACCUMULATOR,
                LSR_ABSOLUTE,
                LSR_ABSOLUTE_X,
                UNUSED,
                LSR_ZEROPAGE,
                LSR_ZEROPAGE_X,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::NOP => self.add_byte_code(
                application,
                &instruction.address_mode,
                NOP,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::ORA => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                ORA_IMMEDIATE,
                UNUSED,
                ORA_ABSOLUTE,
                ORA_ABSOLUTE_X,
                ORA_ABSOLUTE_Y,
                ORA_ZEROPAGE,
                ORA_ZEROPAGE_X,
                UNUSED,
                UNUSED,
                UNUSED,
                ORA_INDIRECT_INDEXED,
                ORA_INDEXED_INDIRECT,
            ),

            Operation::PHA => self.add_byte_code(
                application,
                &instruction.address_mode,
                PHA,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::PHP => self.add_byte_code(
                application,
                &instruction.address_mode,
                PHP,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::PLA => self.add_byte_code(
                application,
                &instruction.address_mode,
                PLA,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::PLP => self.add_byte_code(
                application,
                &instruction.address_mode,
                PLP,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::ROL => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                UNUSED,
                ROL_ACCUMULATOR,
                ROL_ABSOLUTE,
                ROL_ABSOLUTE_X,
                UNUSED,
                ROL_ZEROPAGE,
                ROL_ZEROPAGE_X,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::ROR => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                UNUSED,
                ROR_ACCUMULATOR,
                ROR_ABSOLUTE,
                ROR_ABSOLUTE_X,
                UNUSED,
                ROR_ZEROPAGE,
                ROR_ZEROPAGE_X,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::RTI => self.add_byte_code(
                application,
                &instruction.address_mode,
                RTI,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::RTS => self.add_byte_code(
                application,
                &instruction.address_mode,
                RTS,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::SBC => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                SBC_IMMEDIATE,
                UNUSED,
                SBC_ABSOLUTE,
                SBC_ABSOLUTE_X,
                SBC_ABSOLUTE_Y,
                SBC_ZEROPAGE,
                SBC_ZEROPAGE_X,
                UNUSED,
                UNUSED,
                UNUSED,
                SBC_INDIRECT_INDEXED,
                SBC_INDEXED_INDIRECT,
            ),

            Operation::SEC => self.add_byte_code(
                application,
                &instruction.address_mode,
                SEC,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::SED => self.add_byte_code(
                application,
                &instruction.address_mode,
                SED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::SEI => self.add_byte_code(
                application,
                &instruction.address_mode,
                SEI,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::STA => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                UNUSED,
                UNUSED,
                STA_ABSOLUTE,
                STA_ABSOLUTE_X,
                STA_ABSOLUTE_Y,
                STA_ZEROPAGE,
                STA_ZEROPAGE_X,
                UNUSED,
                UNUSED,
                UNUSED,
                STA_INDIRECT_INDEXED,
                STA_INDEXED_INDIRECT,
            ),

            Operation::STX => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                UNUSED,
                UNUSED,
                STX_ABSOLUTE,
                UNUSED,
                UNUSED,
                STX_ZEROPAGE,
                UNUSED,
                STX_ZEROPAGE_Y,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::STY => self.add_byte_code(
                application,
                &instruction.address_mode,
                UNUSED,
                UNUSED,
                UNUSED,
                STY_ABSOLUTE,
                UNUSED,
                UNUSED,
                STY_ZEROPAGE,
                STY_ZEROPAGE_X,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::TAX => self.add_byte_code(
                application,
                &instruction.address_mode,
                TAX,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::TAY => self.add_byte_code(
                application,
                &instruction.address_mode,
                TAY,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::TSX => self.add_byte_code(
                application,
                &instruction.address_mode,
                TSX,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::TXA => self.add_byte_code(
                application,
                &instruction.address_mode,
                TXA,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::TXS => self.add_byte_code(
                application,
                &instruction.address_mode,
                TXS,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::TYA => self.add_byte_code(
                application,
                &instruction.address_mode,
                TYA,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),

            Operation::Raw(bytes) => {
                self.add_bytes(bytes);
            }
            Operation::Label(_) => {
                // Intentionally empty.
            }
        }
    }

    fn add_byte_code(
        &mut self,
        application: &Application,
        address_mode: &AddressMode,
        implied: u8,
        immediate: u8,
        accumulator: u8,
        absolute: u8,
        absolute_x: u8,
        absolute_y: u8,
        zeropage: u8,
        zeropage_x: u8,
        zeropage_y: u8,
        relative: u8,
        indirect: u8,
        indexed_indirect: u8,
        indirect_indexed: u8,
    ) {
        match address_mode {
            AddressMode::Implied => {
                self.add_u8(implied);
            }
            AddressMode::Immediate(Immediate::Byte(byte)) => {
                self.add_u8(immediate);
                self.add_u8(*byte);
            }
            AddressMode::Immediate(Immediate::Low(address_reference)) => {
                self.add_u8(immediate);
                self.add_u8(application.address(address_reference).low());
            }
            AddressMode::Immediate(Immediate::High(address_reference)) => {
                self.add_u8(immediate);
                self.add_u8(application.address(address_reference).high());
            }
            AddressMode::Accumulator => {
                self.add_u8(accumulator);
            }
            AddressMode::Absolute(address_reference) => {
                let address = application.address(address_reference);
                if zeropage != UNUSED && address.is_zeropage() {
                    self.add_u8(zeropage);
                    self.add_u8(application.address(address_reference).low());
                } else {
                    self.add_u8(absolute);
                    self.add_u16(address);
                }
            }
            AddressMode::AbsoluteX(address_reference) => {
                let address = application.address(address_reference);
                if zeropage_x != UNUSED && address.is_zeropage() {
                    self.add_u8(zeropage_x);
                    self.add_u8(address.low());
                } else {
                    self.add_u8(absolute_x);
                    self.add_u16(address);
                }
            }
            AddressMode::AbsoluteY(address_reference) => {
                let address = application.address(address_reference);
                if zeropage_y != UNUSED && address.is_zeropage() {
                    self.add_u8(zeropage_y);
                    self.add_u8(address.low());
                } else {
                    self.add_u8(absolute_y);
                    self.add_u16(address);
                }
                self.add_u8(absolute_y);
                self.add_u16(address);
            }
            AddressMode::Relative(address_reference) => {
                self.add_u8(relative);
                let address = application.address(address_reference);
                let current_instruction = application.entry_point + self.output.len() as Address - 2;
                let next_instruction = current_instruction + 2;
                let relative_address = (address as i32 - next_instruction as i32) as i8;
                self.add_u8(relative_address as u8);
            }
            AddressMode::Indirect(address_reference) => {
                let address = application.address(address_reference);
                self.add_u8(indirect);
                self.add_u16(address);
            }
            AddressMode::IndexedIndirect(address_reference) => {
                let address = application.address(address_reference);
                assert!(address.is_zeropage());
                self.add_u8(indexed_indirect);
                self.add_u8(address.low());
            }
            AddressMode::IndirectIndexed(address_reference) => {
                let address = application.address(address_reference);
                self.add_u8(indirect_indexed);
                self.add_u8(address.low());
            }
        };
    }
}

impl ProgramGenerator {
    fn add_u8(&mut self, byte: u8) {
        self.output.push(byte);
    }

    fn add_u16(&mut self, address: Address) {
        self.add_u8(address.low());
        self.add_u8(address.high());
    }

    fn add_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.output.push(*byte);
        }
    }
}
