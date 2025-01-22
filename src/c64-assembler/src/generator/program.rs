use c64_assembler_6502::{instruction::InstructionDef, opcodes::*};

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
        match &instruction.operation {
            Operation::ADC => self.add_byte_code(application, &instruction.address_mode, &OPCODES_ADC),

            Operation::AND => self.add_byte_code(application, &instruction.address_mode, &OPCODES_AND),

            Operation::ASL => self.add_byte_code(application, &instruction.address_mode, &OPCODES_ASL),

            Operation::BCC => self.add_byte_code(application, &instruction.address_mode, &OPCODES_BCC),

            Operation::BCS => self.add_byte_code(application, &instruction.address_mode, &OPCODES_BCS),

            Operation::BEQ => self.add_byte_code(application, &instruction.address_mode, &OPCODES_BEQ),

            Operation::BIT => self.add_byte_code(application, &instruction.address_mode, &OPCODES_BIT),

            Operation::BMI => self.add_byte_code(application, &instruction.address_mode, &OPCODES_BMI),

            Operation::BNE => self.add_byte_code(application, &instruction.address_mode, &OPCODES_BNE),

            Operation::BPL => self.add_byte_code(application, &instruction.address_mode, &OPCODES_BPL),

            Operation::BRK => self.add_byte_code(application, &instruction.address_mode, &OPCODES_BRK),

            Operation::BVC => self.add_byte_code(application, &instruction.address_mode, &OPCODES_BVC),

            Operation::BVS => self.add_byte_code(application, &instruction.address_mode, &OPCODES_BVS),

            Operation::CLD => self.add_byte_code(application, &instruction.address_mode, &OPCODES_CLD),

            Operation::CLI => self.add_byte_code(application, &instruction.address_mode, &OPCODES_CLI),

            Operation::CLV => self.add_byte_code(application, &instruction.address_mode, &OPCODES_CLV),

            Operation::CMP => self.add_byte_code(application, &instruction.address_mode, &OPCODES_CMP),

            Operation::CPX => self.add_byte_code(application, &instruction.address_mode, &OPCODES_CPX),

            Operation::CPY => self.add_byte_code(application, &instruction.address_mode, &OPCODES_CPY),

            Operation::DEC => self.add_byte_code(application, &instruction.address_mode, &OPCODES_DEC),

            Operation::DEX => self.add_byte_code(application, &instruction.address_mode, &OPCODES_DEX),

            Operation::DEY => self.add_byte_code(application, &instruction.address_mode, &OPCODES_DEY),

            Operation::EOR => self.add_byte_code(application, &instruction.address_mode, &OPCODES_EOR),

            Operation::INC => self.add_byte_code(application, &instruction.address_mode, &OPCODES_INC),

            Operation::INX => self.add_byte_code(application, &instruction.address_mode, &OPCODES_INX),

            Operation::INY => self.add_byte_code(application, &instruction.address_mode, &OPCODES_INY),

            Operation::LDX => self.add_byte_code(application, &instruction.address_mode, &OPCODES_LDX),

            Operation::LSR => self.add_byte_code(application, &instruction.address_mode, &OPCODES_LSR),

            Operation::NOP => self.add_byte_code(application, &instruction.address_mode, &OPCODES_NOP),

            Operation::ORA => self.add_byte_code(application, &instruction.address_mode, &OPCODES_ORA),

            Operation::PHA => self.add_byte_code(application, &instruction.address_mode, &OPCODES_PHA),

            Operation::PHP => self.add_byte_code(application, &instruction.address_mode, &OPCODES_PHP),

            Operation::PLA => self.add_byte_code(application, &instruction.address_mode, &OPCODES_PLA),

            Operation::PLP => self.add_byte_code(application, &instruction.address_mode, &OPCODES_PLP),

            Operation::ROL => self.add_byte_code(application, &instruction.address_mode, &OPCODES_ROL),

            Operation::ROR => self.add_byte_code(application, &instruction.address_mode, &OPCODES_ROR),

            Operation::RTI => self.add_byte_code(application, &instruction.address_mode, &OPCODES_RTI),

            Operation::SBC => self.add_byte_code(application, &instruction.address_mode, &OPCODES_SBC),

            Operation::SED => self.add_byte_code(application, &instruction.address_mode, &OPCODES_SED),

            Operation::SEI => self.add_byte_code(application, &instruction.address_mode, &OPCODES_SEI),

            Operation::STX => self.add_byte_code(application, &instruction.address_mode, &OPCODES_STX),

            Operation::STY => self.add_byte_code(application, &instruction.address_mode, &OPCODES_STY),

            Operation::TAX => self.add_byte_code(application, &instruction.address_mode, &OPCODES_TAX),

            Operation::TAY => self.add_byte_code(application, &instruction.address_mode, &OPCODES_TAY),

            Operation::TSX => self.add_byte_code(application, &instruction.address_mode, &OPCODES_TSX),

            Operation::TXA => self.add_byte_code(application, &instruction.address_mode, &OPCODES_TXA),

            Operation::TXS => self.add_byte_code(application, &instruction.address_mode, &OPCODES_TXS),

            Operation::TYA => self.add_byte_code(application, &instruction.address_mode, &OPCODES_TYA),

            Operation::LDA => self.add_byte_code(application, &instruction.address_mode, &OPCODES_LDA),

            Operation::LDY => self.add_byte_code(application, &instruction.address_mode, &OPCODES_LDY),

            Operation::STA => self.add_byte_code(application, &instruction.address_mode, &OPCODES_STA),

            Operation::JMP => self.add_byte_code(application, &instruction.address_mode, &OPCODES_JMP),

            Operation::JSR => self.add_byte_code(application, &instruction.address_mode, &OPCODES_JSR),

            Operation::SEC => self.add_byte_code(application, &instruction.address_mode, &OPCODES_SEC),

            Operation::CLC => self.add_byte_code(application, &instruction.address_mode, &OPCODES_CLC),

            Operation::RTS => self.add_byte_code(application, &instruction.address_mode, &OPCODES_RTS),

            Operation::Raw(bytes) => {
                self.add_bytes(bytes);
            }
            Operation::Label(_) => {
                // Intentionally empty.
            }
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
