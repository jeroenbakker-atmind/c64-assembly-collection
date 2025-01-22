use c64_assembler_6502::{
    instruction::InstructionDef,
    opcodes::{NO_ZEROPAGE, NO_ZEROPAGE_X, NO_ZEROPAGE_Y},
};

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
        match (&instruction.operation.definition(), &instruction.operation) {
            (Some(definition), _) => {
                self.add_byte_code(application, &instruction.address_mode, definition);
            }
            (None, Operation::Label(_)) => {
                // Labels don't have bytes in the byte stream, they are only markers
            }
            (None, Operation::Raw(bytes)) => {
                self.add_bytes(bytes);
            }

            (_, _) => panic!(),
        }
    }

    fn add_byte_code(&mut self, application: &Application, address_mode: &AddressMode, instruction: &InstructionDef) {
        match address_mode {
            AddressMode::Implied => {
                self.add_u8(instruction.implied);
            }
            AddressMode::Immediate(Immediate::Byte(byte)) => {
                self.add_u8(instruction.immediate);
                self.add_u8(*byte);
            }
            AddressMode::Immediate(Immediate::Low(address_reference)) => {
                self.add_u8(instruction.immediate);
                self.add_u8(application.address(address_reference).low());
            }
            AddressMode::Immediate(Immediate::High(address_reference)) => {
                self.add_u8(instruction.immediate);
                self.add_u8(application.address(address_reference).high());
            }
            AddressMode::Accumulator => {
                self.add_u8(instruction.accumulator);
            }
            AddressMode::Absolute(address_reference) => {
                let address = application.address(address_reference);
                if instruction.zeropage != NO_ZEROPAGE && address.is_zeropage() {
                    self.add_u8(instruction.zeropage);
                    self.add_u8(application.address(address_reference).low());
                } else {
                    self.add_u8(instruction.absolute);
                    self.add_u16(address);
                }
            }
            AddressMode::AbsoluteX(address_reference) => {
                let address = application.address(address_reference);
                if instruction.zeropage_x != NO_ZEROPAGE_X && address.is_zeropage() {
                    self.add_u8(instruction.zeropage_x);
                    self.add_u8(address.low());
                } else {
                    self.add_u8(instruction.absolute_x);
                    self.add_u16(address);
                }
            }
            AddressMode::AbsoluteY(address_reference) => {
                let address = application.address(address_reference);
                if instruction.zeropage_y != NO_ZEROPAGE_Y && address.is_zeropage() {
                    self.add_u8(instruction.zeropage_y);
                    self.add_u8(address.low());
                } else {
                    self.add_u8(instruction.absolute_y);
                    self.add_u16(address);
                }
                self.add_u8(instruction.absolute_y);
                self.add_u16(address);
            }
            AddressMode::Relative(address_reference) => {
                self.add_u8(instruction.relative);
                let address = application.address(address_reference);
                let current_instruction = application.entry_point + self.output.len() as Address - 2;
                let next_instruction = current_instruction + 2;
                let relative_address = (address as i32 - next_instruction as i32) as i8;
                self.add_u8(relative_address as u8);
            }
            AddressMode::Indirect(address_reference) => {
                let address = application.address(address_reference);
                self.add_u8(instruction.indirect);
                self.add_u16(address);
            }
            AddressMode::IndexedIndirect(address_reference) => {
                let address = application.address(address_reference);
                assert!(address.is_zeropage());
                self.add_u8(instruction.indexed_indirect);
                self.add_u8(address.low());
            }
            AddressMode::IndirectIndexed(address_reference) => {
                let address = application.address(address_reference);
                self.add_u8(instruction.indirect_indexed);
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
