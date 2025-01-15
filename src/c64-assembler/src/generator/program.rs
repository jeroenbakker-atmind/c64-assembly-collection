use c64_assembler_6502::codegen_opcodes;

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

codegen_opcodes!();

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
            Operation::SEC => self.add_u8(SEC),
            Operation::CLC => self.add_u8(CLC),
            Operation::BEQ => todo!(),
            Operation::BIT => todo!(),
            Operation::BMI => todo!(),
            Operation::BNE => {
                self.with_relative(application, &instruction.address_mode, BNE_ABSOLUTE)
            }
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
