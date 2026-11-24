use c64_assembler::{
    builder::{ApplicationBuilder, FunctionBuilder, InstructionBuilder, ModuleBuilder},
    Module,
};

use crate::command::DecoderModule;

pub trait BankBuilder {
    fn add_bank_addresses(&mut self) -> &mut Self;
}

impl BankBuilder for ApplicationBuilder {
    fn add_bank_addresses(&mut self) -> &mut Self {
        self.define_address("PROCESSOR_PORT_DATA", 0x0001)
            .define_address("PROCESSOR_PORT_DDR", 0x0000)
            .define_address("BANK_PORT_DATA_SAVE", 0x3E02)
            .define_address("BANK_PORT_DDR_SAVE", 0x3E03)
    }
}

pub struct Bank {}

impl DecoderModule for Bank {
    fn module() -> Module {
        ModuleBuilder::default()
            .name("engine__bank")
            .function(
                FunctionBuilder::default()
                    .name("engine__bank__init")
                    .doc(&[
                        "Bank out BASIC ROM so the $A000..$BFFF music region is visible RAM.",
                        "",
                        "The original processor-port data and direction registers are saved",
                        "for `engine__bank__restore`.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_addr("PROCESSOR_PORT_DATA")
                            .sta_addr("BANK_PORT_DATA_SAVE")
                            .lda_addr("PROCESSOR_PORT_DDR")
                            .sta_addr("BANK_PORT_DDR_SAVE")
                            .ora_imm(0b0000_1000)
                            .sta_addr("PROCESSOR_PORT_DDR")
                            .lda_addr("BANK_PORT_DATA_SAVE")
                            .and_imm(0b1111_0111)
                            .sta_addr("PROCESSOR_PORT_DATA")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__bank__restore")
                    .doc(&["Restore the processor port saved by `engine__bank__init`."])
                    .instructions(
                        InstructionBuilder::default()
                            .lda_addr("BANK_PORT_DDR_SAVE")
                            .sta_addr("PROCESSOR_PORT_DDR")
                            .lda_addr("BANK_PORT_DATA_SAVE")
                            .sta_addr("PROCESSOR_PORT_DATA")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .build()
    }
}
