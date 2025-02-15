use c64_assembler::{
    builder::{ApplicationBuilder, InstructionBuilder, ModuleBuilder},
    generator::{print_hexdump, Generator, ProgramGenerator},
    validator::AssemblerResult,
};
use c64_encoder::{
    charmap::encoding::{decode_char, encode_char},
    command::{
        modules::{CurrentPTR, DecodeU16Char},
        update_chars_ranged::{UpdateCharRanged, UpdateCharsRangedU16Encoded},
        DecoderModule,
    },
    encoder::Encoder,
};
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

#[test]
fn decode_update_chars_ranged_u16() -> AssemblerResult<()> {
    let update_chars = UpdateCharsRangedU16Encoded {
        offset: 2,
        chars: vec![
            UpdateCharRanged {
                data: 0b1100110011001100001100110011001111001100110011000011001100110011,
            },
            UpdateCharRanged {
                data: 0b1111000011110000111100001111000000001111000011110000111100001111,
            },
        ],
    };
    let mut command = vec![0; update_chars.byte_size() + 1];
    update_chars.encode(&mut command[1..]);
    print_hexdump(&command);
    for b in &command {
        println!("{:08b}", b);
    }

    let application = ApplicationBuilder::default()
        .define_address("CHAR_DECODE_SRC_PTR", 0xFE)
        .define_address("CHAR_DECODE_DST_PTR", 0xFC)
        .define_address("CURRENT_PTR", 0xFE)
        .define_address("CHARSET_PTR_PAGE0", 0x0500)
        .module(
            ModuleBuilder::default()
                .instructions(
                    InstructionBuilder::default()
                        .jsr_addr("update_chars_ranged_u16__process")
                        .raw(&[0xFF])
                        .build(),
                )
                .build(),
        )
        .module(CurrentPTR::module())
        .module(UpdateCharsRangedU16Encoded::module())
        .module(DecodeU16Char::module())
        .build()?;
    let bytes = ProgramGenerator::default().generate(application)?;

    let mut cpu = CPU::new(Memory::new(), Nmos6502);
    cpu.memory.set_bytes(0x00FE, &[0x00, 0x04]);
    cpu.memory.set_bytes(0x0800, &bytes[2..]);
    cpu.memory.set_bytes(0x0400, &command);
    cpu.registers.program_counter = 0x0800;

    cpu.run();
    for address in 0x0500..0x0530 {
        println!("{address:04X}: {:08b}", cpu.memory.get_byte(address));
    }

    assert_eq!(0b11001100, cpu.memory.get_byte(0x0510));
    assert_eq!(0b11001100, cpu.memory.get_byte(0x0511));
    assert_eq!(0b00110011, cpu.memory.get_byte(0x0512));
    assert_eq!(0b00110011, cpu.memory.get_byte(0x0513));
    assert_eq!(0b11001100, cpu.memory.get_byte(0x0514));
    assert_eq!(0b11001100, cpu.memory.get_byte(0x0515));
    assert_eq!(0b00110011, cpu.memory.get_byte(0x0516));
    assert_eq!(0b00110011, cpu.memory.get_byte(0x0517));

    assert_eq!(0b11110000, cpu.memory.get_byte(0x0518));
    assert_eq!(0b11110000, cpu.memory.get_byte(0x0519));
    assert_eq!(0b11110000, cpu.memory.get_byte(0x051A));
    assert_eq!(0b11110000, cpu.memory.get_byte(0x051B));
    assert_eq!(0b00001111, cpu.memory.get_byte(0x051C));
    assert_eq!(0b00001111, cpu.memory.get_byte(0x051D));
    assert_eq!(0b00001111, cpu.memory.get_byte(0x051E));
    assert_eq!(0b00001111, cpu.memory.get_byte(0x051F));

    Ok(())
}

#[test]
fn decode_u16_roundtrip() {
    for encoded in 0..=65535_u16 {
        let decoded = decode_char(encoded);
        let encoded_again = encode_char(decoded);
        assert_eq!(encoded, encoded_again, "{} {} {}", encoded, decoded, encoded_again);
    }
}
