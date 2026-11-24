use c64_assembler::{
    builder::{ApplicationBuilder, FunctionBuilder, InstructionBuilder, ModuleBuilder},
    Module,
};

use crate::command::DecoderModule;

/// Resource row layout: track, first sector, sector count, dest lo/hi,
/// payload lo/hi. The name is a build-time concept and is not stored.
pub const RESOURCE_TABLE_ENTRY_SIZE: u8 = 7;
pub const RESOURCE_TRACK_OFFSET: u8 = 0;
pub const RESOURCE_SECTOR_OFFSET: u8 = 1;
pub const RESOURCE_SECTOR_COUNT_OFFSET: u8 = 2;
pub const RESOURCE_DEST_LO_OFFSET: u8 = 3;
pub const RESOURCE_DEST_HI_OFFSET: u8 = 4;
pub const RESOURCE_PAYLOAD_LO_OFFSET: u8 = 5;
pub const RESOURCE_PAYLOAD_HI_OFFSET: u8 = 6;

pub trait ResourceBuilder {
    fn add_resource_addresses(&mut self) -> &mut Self;
}

impl ResourceBuilder for ApplicationBuilder {
    fn add_resource_addresses(&mut self) -> &mut Self {
        self.define_address("RESOURCE_SCRATCH_ID", 0x87)
            .define_address("RESOURCE_SCRATCH_OFFSET_LO", 0x88)
            .define_address("RESOURCE_SCRATCH_OFFSET_HI", 0x89)
    }
}

/// Minimal table for legacy applications that use `add_engine` but do not
/// load disk resources. The final intro replaces this with the generated table.
pub fn dummy_resource_table_module() -> Module {
    ModuleBuilder::default()
        .name("resource_data")
        .instructions(
            InstructionBuilder::default()
                .label("resource_table")
                .raw(&[0; RESOURCE_TABLE_ENTRY_SIZE as usize])
                .build(),
        )
        .build()
}

pub struct Resource {}

impl DecoderModule for Resource {
    fn module() -> Module {
        ModuleBuilder::default()
            .name("engine__resource")
            .function(
                FunctionBuilder::default()
                    .name("engine__resource__index")
                    .doc(&[
                        "Convert a resource id in A into its seven-byte table index.",
                        "",
                        "Multiplication is done as id * 8 - id, avoiding a lookup table.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .sta_addr("RESOURCE_SCRATCH_ID")
                            .asl_acc()
                            .asl_acc()
                            .asl_acc()
                            .sec()
                            .sbc_addr("RESOURCE_SCRATCH_ID")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__resource__dest")
                    .doc(&[
                        "Look up the destination of the resource id in A.",
                        "",
                        "Returns the low byte in X and the high byte in A.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .jsr_addr("engine__resource__index")
                            .tay()
                            .iny()
                            .iny()
                            .iny()
                            .lda_addr_y("resource_table")
                            .tax()
                            .iny()
                            .lda_addr_y("resource_table")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__resource__sector_count")
                    .doc(&["Return the derived sector count of the resource id in A."])
                    .instructions(
                        InstructionBuilder::default()
                            .jsr_addr("engine__resource__index")
                            .tay()
                            .iny()
                            .iny()
                            .lda_addr_y("resource_table")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__resource__payload_bytes")
                    .doc(&[
                        "Look up the exact payload size of the resource id in A.",
                        "",
                        "Returns the low byte in X and the high byte in A.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .jsr_addr("engine__resource__index")
                            .tay()
                            .iny()
                            .iny()
                            .iny()
                            .iny()
                            .iny()
                            .lda_addr_y("resource_table")
                            .tax()
                            .iny()
                            .lda_addr_y("resource_table")
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .function(
                FunctionBuilder::default()
                    .name("engine__resource__use")
                    .doc(&[
                        "Point CURRENT_PTR at a resource frame offset.",
                        "",
                        "A = resource id, X = offset low byte, Y = offset high byte.",
                    ])
                    .instructions(
                        InstructionBuilder::default()
                            .sta_addr("RESOURCE_SCRATCH_ID")
                            .stx_addr("RESOURCE_SCRATCH_OFFSET_LO")
                            .sty_addr("RESOURCE_SCRATCH_OFFSET_HI")
                            .lda_addr("RESOURCE_SCRATCH_ID")
                            .jsr_addr("engine__resource__index")
                            .tay()
                            .iny()
                            .iny()
                            .iny()
                            .lda_addr_y("resource_table")
                            .clc()
                            .adc_addr("RESOURCE_SCRATCH_OFFSET_LO")
                            .sta_addr("CURRENT_PTR")
                            .iny()
                            .lda_addr_y("resource_table")
                            .adc_addr("RESOURCE_SCRATCH_OFFSET_HI")
                            .sta_addr_offs("CURRENT_PTR", 1)
                            .rts()
                            .build(),
                    )
                    .build(),
            )
            .build()
    }
}
