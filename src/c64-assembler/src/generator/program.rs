use crate::{
    builder::{
        application_builder::ApplicationBuilder, instruction_builder::InstructionBuilder,
        module_builder::ModuleBuilder,
    },
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

    fn generate(mut self, application: ApplicationBuilder) -> Self::Output {
        self.add_u16(application.entry_point);
        for module in &application.modules {
            self.generate_module(&application, module);
        }
        self.output
    }
}

impl ProgramGenerator {
    fn generate_module(&mut self, application: &ApplicationBuilder, module: &ModuleBuilder) {
        self.generate_instructions(application, &module.instructions);
        for function in &module.functions {
            self.generate_instructions(application, &function.instructions);
        }
    }

    fn generate_instructions(
        &mut self,
        application: &ApplicationBuilder,
        instructions: &InstructionBuilder,
    ) {
        for instruction in &instructions.instructions {
            self.generate_instruction(application, instruction);
        }
    }

    fn generate_instruction(
        &mut self,
        application: &ApplicationBuilder,
        instruction: &Instruction,
    ) {
        // Use https://www.c64-wiki.com/wiki/
        const UNUSED: u8 = 0x00;
        const ADC_IMMEDIATE: u8 = 0x69;
        const ADC_ABSOLUTE: u8 = 0x6D;
        const ADC_ABSOLUTE_X: u8 = 0x7D;
        const ADC_ABSOLUTE_Y: u8 = 0x79;
        const ADC_ZEROPAGE: u8 = 0x65;
        const ADC_ZEROPAGE_X: u8 = 0x75;
        const ADC_INDEXED_INDIRECT: u8 = 0x61;
        const ADC_INDIRECT_INDEXED: u8 = 0x71;
        const LDA_IMMEDIATE: u8 = 0xA9;
        const LDA_ABSOLUTE: u8 = 0xAD;
        const LDA_ABSOLUTE_X: u8 = 0xBD;
        const LDA_ABSOLUTE_Y: u8 = 0xB9;
        const LDA_ZEROPAGE: u8 = 0xA5;
        const LDA_ZEROPAGE_X: u8 = 0xB5;
        const LDA_INDEXED_INDIRECT: u8 = 0xA1;
        const LDA_INDIRECT_INDEXED: u8 = 0xB1;
        const STA_ABSOLUTE: u8 = 0x8D;
        const STA_ABSOLUTE_X: u8 = 0x9D;
        const STA_ABSOLUTE_Y: u8 = 0x99;
        const STA_ZEROPAGE: u8 = 0x85;
        const STA_ZEROPAGE_X: u8 = 0x95;
        const STA_INDEXED_INDIRECT: u8 = 0x81;
        const STA_INDIRECT_INDEXED: u8 = 0x91;
        const RTS: u8 = 0x60;

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
            Operation::JSR => todo!(),
            Operation::SEC => todo!(),
            Operation::CLC => todo!(),
        }
    }

    fn with_absolute(
        &mut self,
        application: &ApplicationBuilder,
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
                if address.is_zeropage() {
                    self.add_u8(zeropage);
                    self.add_u8(application.address(address_reference).low());
                } else {
                    self.add_u8(absolute);
                    self.add_u16(address);
                }
            }
            AddressMode::AbsoluteX(address_reference) => {
                let address = application.address(address_reference);
                if address.is_zeropage() {
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
