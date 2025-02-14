use c64_assembler::{
    builder::{ApplicationBuilder, InstructionBuilder, ModuleBuilder},
    generator::{Generator, ProgramGenerator},
    validator::AssemblerResult,
};
use c64_encoder::command::{modules::DecodeU16Char, DecoderModule};
use mos6502::{
    cpu::CPU,
    instruction::Nmos6502,
    memory::{Bus, Memory},
};

#[test]
fn decode_u16_char() -> AssemblerResult<()> {
    let application = ApplicationBuilder::default()
        .define_address("CHAR_DECODE_SRC_PTR", 0xFC)
        .define_address("CHAR_DECODE_DST_PTR", 0xFE)
        .module(
            ModuleBuilder::default()
                .instructions(
                    InstructionBuilder::default()
                        .jsr_addr("char__decode_u16")
                        .raw(&[0xFF])
                        .build(),
                )
                .build(),
        )
        .module(DecodeU16Char::module())
        .build()?;
    let bytes = ProgramGenerator::default().generate(application)?;

    let mut cpu = CPU::new(Memory::new(), Nmos6502);
    cpu.memory.set_bytes(0x00FC, &[0x00, 0x04, 0x00, 0x05]);
    cpu.memory.set_bytes(0x0800, &bytes[2..]);
    cpu.memory.set_byte(0xd020, 0xFF);
    cpu.registers.program_counter = 0x0800;

    cpu.memory.set_bytes(0x0400, &[0b10100101, 0b00110001]);
    cpu.run();
    assert_eq!(0b11001100, cpu.memory.get_byte(0x0500));
    assert_eq!(0b11001100, cpu.memory.get_byte(0x0501));
    assert_eq!(0b00110011, cpu.memory.get_byte(0x0502));
    assert_eq!(0b00110011, cpu.memory.get_byte(0x0503));
    assert_eq!(0b00001111, cpu.memory.get_byte(0x0504));
    assert_eq!(0b00001111, cpu.memory.get_byte(0x0505));
    assert_eq!(0b00000011, cpu.memory.get_byte(0x0506));
    assert_eq!(0b00000011, cpu.memory.get_byte(0x0507));

    Ok(())
}
