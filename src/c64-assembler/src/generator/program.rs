use crate::{
    builder::{
        application_builder::ApplicationBuilder, instruction_builder::InstructionBuilder,
        module_builder::ModuleBuilder,
    },
    instruction::{operation::Operation, Instruction},
    memory::address_mode::AddressMode,
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
        const LDA_IMMEDIATE: u8 = 0xA9;
        const STA_ABSOLUTE: u8 = 0x8D;
        const RTS: u8 = 0x60;

        match (&instruction.operation, &instruction.address_mode) {
            (Operation::Raw(bytes), AddressMode::Implied) => {
                for byte in bytes {
                    self.add_u8(*byte);
                }
            }
            (Operation::LoadAccumulator, AddressMode::Immediate(byte)) => {
                self.add_u8(LDA_IMMEDIATE);
                self.add_u8(*byte);
            }
            (Operation::StoreAccumulator, AddressMode::Absolute(address_reference)) => {
                self.add_u8(STA_ABSOLUTE);
                self.add_u16(application.address(address_reference));
            }
            (Operation::Return, AddressMode::Implied) => {
                self.add_u8(RTS);
            }
            (Operation::Label(_), AddressMode::Implied) => {
                // Intentionally empty.
            }

            _ => unimplemented!(),
        }
    }
}

impl ProgramGenerator {
    fn add_u8(&mut self, byte: u8) {
        self.output.push(byte);
    }

    fn add_u16(&mut self, word: u16) {
        self.add_u8((word & 0xFF) as u8);
        self.add_u8((word >> 8) as u8);
    }
}
