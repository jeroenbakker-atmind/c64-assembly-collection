#[macro_export]
macro_rules! isa_6502 {
    () => {
        (
            (
                instruction: "lda",
                operation: "LDA"
                implied: 0xFF,
                accumulator: 0xFF,
                absolute: 0xAD,
                absolute_x: 0xBD,
                absolute_y: 0xB9,
                zeropage: 0xA5,
                zeropage_x: 0x5,
                indexed_indirect: 0xA1,
                indirect_indexed: 0xB1,
            ),
        )
    }
}

macro_rules! operation_attribute_codegen {
    () =>{
        
    }
}