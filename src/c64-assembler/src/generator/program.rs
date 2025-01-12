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
        // Use https://www.c64-wiki.com/wiki/
        const UNUSED: u8 = 0x00;
        const BRK: u8 = 0x00;
        const ADC_IMMEDIATE: u8 = 0x69;
        const ADC_ABSOLUTE: u8 = 0x6D;
        const ADC_ABSOLUTE_X: u8 = 0x7D;
        const ADC_ABSOLUTE_Y: u8 = 0x79;
        const ADC_ZEROPAGE: u8 = 0x65;
        const ADC_ZEROPAGE_X: u8 = 0x75;
        const ADC_INDEXED_INDIRECT: u8 = 0x61;
        const ADC_INDIRECT_INDEXED: u8 = 0x71;
        const AND_IMMEDIATE: u8 = 0x29;
        const AND_ABSOLUTE: u8 = 0x2D;
        const AND_ABSOLUTE_X: u8 = 0x3D;
        const AND_ABSOLUTE_Y: u8 = 0x39;
        const AND_ZEROPAGE: u8 = 0x25;
        const AND_ZEROPAGE_X: u8 = 0x35;
        const AND_INDEXED_INDIRECT: u8 = 0x21;
        const AND_INDIRECT_INDEXED: u8 = 0x31;
        const ASL_IMPLIED: u8 = 0x0A;
        const ASL_ABSOLUTE: u8 = 0x0E;
        const ASL_ABSOLUTE_X: u8 = 0x1E;
        const ASL_ZEROPAGE: u8 = 0x06;
        const ASL_ZEROPAGE_X: u8 = 0x16;
        const BCC: u8 = 0x90;
        const BCS: u8 = 0xB0;
        const BEQ: u8 = 0xF0;
        const BIT_ZEROPAGE: u8 = 0x24;
        const BIT_ABSOLUTE: u8 = 0x2C;
        const BMI: u8 = 0x30;
        const BNE: u8 = 0xD0;
        const BPL: u8 = 0x10;
        const BVC: u8 = 0x50;
        const BVS: u8 = 0x70;
        const CLD: u8 = 0xD8;
        const CLI: u8 = 0x58;
        const CLV: u8 = 0xB8;

        const CMP_IMMEDIATE: u8 = 0xC9;
        const CMP_ABSOLUTE: u8 = 0xCD;
        const CMP_ABSOLUTE_X: u8 = 0xDD;
        const CMP_ABSOLUTE_Y: u8 = 0xD9;
        const CMP_ZEROPAGE: u8 = 0xC5;
        const CMP_ZEROPAGE_X: u8 = 0xD5;
        const CMP_INDEXED_INDIRECT: u8 = 0xC1;
        const CMP_INDIRECT_INDEXED: u8 = 0xD1;
        const CPX_IMMEDIATE: u8 = 0xE0;
        const CPX_ZEROPAGE: u8 = 0xE4;
        const CPX_ABSOLUTE: u8 = 0xEC;
        const CPY_IMMEDIATE: u8 = 0xC0;
        const CPY_ZEROPAGE: u8 = 0xC4;
        const CPY_ABSOLUTE: u8 = 0xCC;
        const DEC_ZEROPAGE: u8 = 0xC6;
        const DEC_ZEROPAGE_X: u8 = 0xD6;
        const DEC_ABSOLUTE: u8 = 0xCE;
        const DEC_ABSOLUTE_X: u8 = 0xDE;

        const DEX: u8 = 0xCA;
        const DEY: u8 = 0x88;
        const EOR_IMMEDIATE: u8 = 0x49;
        const EOR_ABSOLUTE: u8 = 0x4D;
        const EOR_ABSOLUTE_X: u8 = 0x5D;
        const EOR_ABSOLUTE_Y: u8 = 0x59;
        const EOR_ZEROPAGE: u8 = 0x45;
        const EOR_ZEROPAGE_X: u8 = 0x55;
        const EOR_INDEXED_INDIRECT: u8 = 0x41;
        const EOR_INDIRECT_INDEXED: u8 = 0x51;
        const INC_ZEROPAGE: u8 = 0xE6;
        const INC_ZEROPAGE_X: u8 = 0xF6;
        const INC_ABSOLUTE: u8 = 0xEE;
        const INC_ABSOLUTE_X: u8 = 0xFE;
        const INX: u8 = 0xE8;
        const INY: u8 = 0xC8;
        const JMP_INDIRECT: u8 = 0x6C;
        const JMP_ABSOLUTE: u8 = 0x4C;
        const LDA_IMMEDIATE: u8 = 0xA9;
        const LDA_ABSOLUTE: u8 = 0xAD;
        const LDA_ABSOLUTE_X: u8 = 0xBD;
        const LDA_ABSOLUTE_Y: u8 = 0xB9;
        const LDA_ZEROPAGE: u8 = 0xA5;
        const LDA_ZEROPAGE_X: u8 = 0xB5;
        const LDA_INDEXED_INDIRECT: u8 = 0xA1;
        const LDA_INDIRECT_INDEXED: u8 = 0xB1;
        const LDX_IMMEDIATE: u8 = 0xA2;
        const LDX_ABSOLUTE: u8 = 0xAE;
        const LDX_ABSOLUTE_Y: u8 = 0xBE;
        const LDX_ZEROPAGE: u8 = 0xA6;
        const LDX_ZEROPAGE_Y: u8 = 0xB6;
        const LDY_IMMEDIATE: u8 = 0xA0;
        const LDY_ABSOLUTE: u8 = 0xAC;
        const LDY_ABSOLUTE_X: u8 = 0xBC;
        const LDY_ZEROPAGE: u8 = 0xA4;
        const LDY_ZEROPAGE_X: u8 = 0xB4;
        const LSR_ACCUMULATOR: u8 = 0x4A;
        const LSR_ABSOLUTE: u8 = 0x4E;
        const LSR_ABSOLUTE_X: u8 = 0x5E;
        const LSR_ZEROPAGE: u8 = 0x46;
        const LSR_ZEROPAGE_X: u8 = 0x56;
        const NOP: u8 = 0xEA;
        const ORA_IMMEDIATE: u8 = 0x09;
        const ORA_ABSOLUTE: u8 = 0x0D;
        const ORA_ABSOLUTE_X: u8 = 0x1D;
        const ORA_ABSOLUTE_Y: u8 = 0x19;
        const ORA_ZEROPAGE: u8 = 0x05;
        const ORA_ZEROPAGE_X: u8 = 0x15;
        const ORA_INDEXED_INDIRECT: u8 = 0x01;
        const ORA_INDIRECT_INDEXED: u8 = 0x11;
        const PHA: u8 = 0x48;
        const PSR: u8 = 0x08;
        const PLA: u8 = 0x68;
        const PLP: u8 = 0x28;
        const ROL_ACCUMULATOR: u8 = 0x2A;
        const ROL_ABSOLUTE: u8 = 0x2E;
        const ROL_ABSOLUTE_X: u8 = 0x3E;
        const ROL_ZEROPAGE: u8 = 0x26;
        const ROL_ZEROPAGE_X: u8 = 0x36;
        const ROR_ACCUMULATOR: u8 = 0x6A;
        const ROR_ABSOLUTE: u8 = 0x6E;
        const ROR_ABSOLUTE_X: u8 = 0x7E;
        const ROR_ZEROPAGE: u8 = 0x66;
        const ROR_ZEROPAGE_X: u8 = 0x76;
        const RTI: u8 = 0x40;
        const SBC_IMMEDIATE: u8 = 0xE9;
        const SBC_ABSOLUTE: u8 = 0xED;
        const SBC_ABSOLUTE_X: u8 = 0xFD;
        const SBC_ABSOLUTE_Y: u8 = 0xF9;
        const SBC_ZEROPAGE: u8 = 0xE5;
        const SBC_ZEROPAGE_X: u8 = 0xF5;
        const SBC_INDEXED_INDIRECT: u8 = 0xE1;
        const SBC_INDIRECT_INDEXED: u8 = 0xF1;
        const SEC: u8 = 0x38;
        const SED: u8 = 0xF8;
        const SEI: u8 = 0x78;
        const STA_ABSOLUTE: u8 = 0x8D;
        const STA_ABSOLUTE_X: u8 = 0x9D;
        const STA_ABSOLUTE_Y: u8 = 0x99;
        const STA_ZEROPAGE: u8 = 0x85;
        const STA_ZEROPAGE_X: u8 = 0x95;
        const STA_INDEXED_INDIRECT: u8 = 0x81;
        const STA_INDIRECT_INDEXED: u8 = 0x91;
        const STX_ABSOLUTE: u8 = 0x8E;
        const STX_ZEROPAGE: u8 = 0x86;
        const STX_ZEROPAGE_Y: u8 = 0x96;
        const STY_ABSOLUTE: u8 = 0x8C;
        const STY_ZEROPAGE: u8 = 0x84;
        const STY_ZEROPAGE_Y: u8 = 0x94;
        const TAX: u8 = 0xAA;
        const TAY: u8 = 0xA8;
        const TSX: u8 = 0xBA;
        const TXA: u8 = 0x8A;
        const TXS: u8 = 0x9A;
        const TYA: u8 = 0x98;
        const JSR_ABSOLUTE: u8 = 0x20;
        const RTS: u8 = 0x60;
        const CLC: u8 = 0x18;

        match &instruction.operation {
            Operation::ADC => {
                self.with_absolute(
                    application,
                    &instruction.address_mode,
                    ADC_IMMEDIATE,
                    ADC_ABSOLUTE,
                    ADC_ABSOLUTE_X,
                    ADC_ABSOLUTE_Y,
                    ADC_ZEROPAGE,
                    ADC_ZEROPAGE_X,
                    ADC_INDEXED_INDIRECT,
                    ADC_INDIRECT_INDEXED,
                );
            }
            Operation::LDA => {
                self.with_absolute(
                    application,
                    &instruction.address_mode,
                    LDA_IMMEDIATE,
                    LDA_ABSOLUTE,
                    LDA_ABSOLUTE_X,
                    LDA_ABSOLUTE_Y,
                    LDA_ZEROPAGE,
                    LDA_ZEROPAGE_X,
                    LDA_INDEXED_INDIRECT,
                    LDA_INDIRECT_INDEXED,
                );
            }

            Operation::LDY => {
                self.with_absolute(
                    application,
                    &instruction.address_mode,
                    LDY_IMMEDIATE,
                    LDY_ABSOLUTE,
                    LDY_ABSOLUTE_X,
                    UNUSED,
                    LDY_ZEROPAGE,
                    LDY_ZEROPAGE_X,
                    UNUSED,
                    UNUSED,
                );
            }

            Operation::STA => self.with_absolute(
                application,
                &instruction.address_mode,
                UNUSED,
                STA_ABSOLUTE,
                STA_ABSOLUTE_X,
                STA_ABSOLUTE_Y,
                STA_ZEROPAGE,
                STA_ZEROPAGE_X,
                STA_INDEXED_INDIRECT,
                STA_INDIRECT_INDEXED,
            ),
            Operation::RTS => {
                self.add_u8(RTS);
            }
            Operation::Raw(bytes) => self.add_bytes(bytes),
            Operation::Label(_) => {
                // Intentionally empty.
            }
            Operation::AND => todo!(),
            Operation::ASL => todo!(),
            Operation::BCC => todo!(),
            Operation::BCS => todo!(),
            Operation::JMP => todo!(),
            Operation::JSR => self.with_absolute(
                application,
                &instruction.address_mode,
                UNUSED,
                JSR_ABSOLUTE,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
                UNUSED,
            ),
            Operation::PSR => self.add_u8(PSR),
            Operation::SEC => self.add_u8(SEC),
            Operation::CLC => self.add_u8(CLC),
            Operation::BEQ => todo!(),
            Operation::BIT => todo!(),
            Operation::BMI => todo!(),
            Operation::BNE => self.with_relative(application, &instruction.address_mode, BNE),
            Operation::BPL => todo!(),
            Operation::BRK => todo!(),
            Operation::BVC => todo!(),
            Operation::BVS => todo!(),
            Operation::CLD => todo!(),
            Operation::CLI => todo!(),
            Operation::CLV => todo!(),
            Operation::CMP => self.with_absolute(
                application,
                &instruction.address_mode,
                CMP_IMMEDIATE,
                CMP_ABSOLUTE,
                CMP_ABSOLUTE_X,
                CMP_ABSOLUTE_Y,
                CMP_ZEROPAGE,
                CMP_ZEROPAGE_X,
                UNUSED,
                UNUSED,
            ),
            Operation::CPX => self.with_absolute(
                application,
                &instruction.address_mode,
                CPX_IMMEDIATE,
                CPX_ABSOLUTE,
                UNUSED,
                UNUSED,
                CPX_ZEROPAGE,
                UNUSED,
                UNUSED,
                UNUSED,
            ),
            Operation::CPY => self.with_absolute(
                application,
                &instruction.address_mode,
                CPY_IMMEDIATE,
                CPY_ABSOLUTE,
                UNUSED,
                UNUSED,
                CPY_ZEROPAGE,
                UNUSED,
                UNUSED,
                UNUSED,
            ),
            Operation::DEC => todo!(),
            Operation::DEX => self.add_u8(DEX),
            Operation::DEY => self.add_u8(DEY),
            Operation::EOR => todo!(),
            Operation::INC => todo!(),
            Operation::INX => self.add_u8(INX),
            Operation::INY => self.add_u8(INY),
            Operation::LDX => todo!(),
            Operation::LSR => todo!(),
            Operation::NOP => todo!(),
            Operation::ORA => todo!(),
            Operation::PHA => todo!(),
            Operation::PHP => todo!(),
            Operation::PLA => todo!(),
            Operation::PLP => todo!(),
            Operation::ROL => todo!(),
            Operation::ROR => todo!(),
            Operation::RTI => todo!(),
            Operation::SBC => self.with_absolute(
                application,
                &instruction.address_mode,
                SBC_IMMEDIATE,
                SBC_ABSOLUTE,
                SBC_ABSOLUTE_X,
                SBC_ABSOLUTE_Y,
                SBC_ZEROPAGE,
                SBC_ZEROPAGE_X,
                SBC_INDEXED_INDIRECT,
                SBC_INDIRECT_INDEXED,
            ),
            Operation::SED => todo!(),
            Operation::SEI => todo!(),
            Operation::STX => todo!(),
            Operation::STY => todo!(),
            Operation::TAX => todo!(),
            Operation::TAY => todo!(),
            Operation::TSX => todo!(),
            Operation::TXA => todo!(),
            Operation::TXS => todo!(),
            Operation::TYA => todo!(),
        }
    }

    fn with_relative(
        &mut self,
        application: &Application,
        address_mode: &AddressMode,
        relative: u8,
    ) {
        match address_mode {
            AddressMode::Relative(address_reference) => {
                self.add_u8(relative);
                let address = application.address(address_reference);
                let current_instruction =
                    application.entry_point + self.output.len() as Address - 2;
                let next_instruction = current_instruction + 2;
                let relative_address = (address as i32 - next_instruction as i32) as i8;
                self.add_u8(relative_address as u8);
            }
            _ => unreachable!(),
        }
    }
    fn with_absolute(
        &mut self,
        application: &Application,
        address_mode: &AddressMode,
        immediate: u8,
        absolute: u8,
        absolute_x: u8,
        absolute_y: u8,
        zeropage: u8,
        zeropage_x: u8,
        indexed_indirect: u8,
        indirect_indexed: u8,
    ) {
        match address_mode {
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
            AddressMode::Absolute(address_reference) => {
                let address = application.address(address_reference);
                if zeropage != 0x00 && address.is_zeropage() {
                    self.add_u8(zeropage);
                    self.add_u8(application.address(address_reference).low());
                } else {
                    self.add_u8(absolute);
                    self.add_u16(address);
                }
            }
            AddressMode::AbsoluteX(address_reference) => {
                let address = application.address(address_reference);
                if zeropage_x != 0x00 && address.is_zeropage() {
                    self.add_u8(zeropage_x);
                    self.add_u8(address.low());
                } else {
                    self.add_u8(absolute_x);
                    self.add_u16(address);
                }
            }
            AddressMode::AbsoluteY(address_reference) => {
                let address = application.address(address_reference);
                self.add_u8(absolute_y);
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
            _ => {}
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
