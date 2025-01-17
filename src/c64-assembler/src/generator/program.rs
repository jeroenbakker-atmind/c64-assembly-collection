use c64_assembler_6502::{codegen_opcodes, codegen_program_instruction_to_byte_code};

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
        codegen_program_instruction_to_byte_code! {}
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
