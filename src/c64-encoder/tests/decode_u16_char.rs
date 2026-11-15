use c64_assembler::{
    builder::{ApplicationBuilder, InstructionBuilder, ModuleBuilder},
    generator::{print_hexdump, Generator, ProgramGenerator},
    validator::AssemblerResult,
};
use c64_encoder::{
    charmap::encoding::{decode_char, encode_char},
    command::{
        modules::{CurrentPTR, DecodeU16Char},
        update_chars::{UpdateChar, UpdateCharsU16Encoded},
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
fn decode_update_chars_ranged_u16_crosses_page() -> AssemblerResult<()> {
    let encoded_chars = [(0u8, 0x1234_u16), (1u8, 0xABCD_u16)];
    let update_chars = UpdateCharsRangedU16Encoded {
        // Character 16 has bit 4 set, which used to corrupt the charset page, and
        // its destination ends at an address where the increment needs to carry.
        offset: 16,
        chars: encoded_chars
            .iter()
            .map(|(_, encoded)| UpdateCharRanged {
                data: decode_char(*encoded),
            })
            .collect(),
    };
    let mut command = vec![0; update_chars.byte_size() + 1];
    update_chars.encode(&mut command[1..]);

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

    let table = [
        0b00000000, 0b00000011, 0b00001100, 0b00001111, 0b00110000, 0b00110011, 0b00111100, 0b00111111, 0b11000000,
        0b11000011, 0b11001100, 0b11001111, 0b11110000, 0b11110011, 0b11111100, 0b11111111,
    ];
    for (char_offset, encoded) in encoded_chars {
        let low = (encoded & 0xFF) as usize;
        let high = (encoded >> 8) as usize;
        let expected = [
            table[low >> 4],
            table[low >> 4],
            table[low & 0x0F],
            table[low & 0x0F],
            table[high >> 4],
            table[high >> 4],
            table[high & 0x0F],
            table[high & 0x0F],
        ];
        let base = 0x0500 + (16 + u16::from(char_offset)) * 8;
        for (index, expected_byte) in expected.iter().enumerate() {
            let address = base + index as u16;
            assert_eq!(
                *expected_byte,
                cpu.memory.get_byte(address),
                "character {} at {address:04X}",
                16 + char_offset
            );
        }
    }

    Ok(())
}

#[test]
fn decode_update_chars_u16() -> AssemblerResult<()> {
    let encoded_chars = [
        (0u8, 0x1234_u16),
        // Characters 16..31 have bit 4 set. This used to corrupt the charset page
        // because the carry of the five LSRs was not cleared before adding the page.
        (16u8, 0xABCD_u16),
        (17u8, 0x00FF_u16),
        (33u8, 0x8001_u16),
    ];
    let update_chars = UpdateCharsU16Encoded {
        chars: encoded_chars
            .iter()
            .map(|(char, encoded)| UpdateChar {
                char: *char,
                data: decode_char(*encoded),
            })
            .collect(),
    };
    let mut command = vec![0; update_chars.byte_size() + 1];
    update_chars.encode(&mut command[1..]);

    let application = ApplicationBuilder::default()
        .define_address("CHAR_DECODE_SRC_PTR", 0xFE)
        .define_address("CHAR_DECODE_DST_PTR", 0xFC)
        .define_address("CURRENT_PTR", 0xFE)
        .define_address("CHARSET_PTR_PAGE0", 0x0500)
        .module(
            ModuleBuilder::default()
                .instructions(
                    InstructionBuilder::default()
                        .jsr_addr("update_chars_u16__process")
                        .raw(&[0xFF])
                        .build(),
                )
                .build(),
        )
        .module(CurrentPTR::module())
        .module(UpdateCharsU16Encoded::module())
        .module(DecodeU16Char::module())
        .build()?;
    let bytes = ProgramGenerator::default().generate(application)?;

    let mut cpu = CPU::new(Memory::new(), Nmos6502);
    cpu.memory.set_bytes(0x00FE, &[0x00, 0x04]);
    cpu.memory.set_bytes(0x0800, &bytes[2..]);
    cpu.memory.set_bytes(0x0400, &command);
    cpu.registers.program_counter = 0x0800;

    cpu.run();

    let table = [
        0b00000000, 0b00000011, 0b00001100, 0b00001111, 0b00110000, 0b00110011, 0b00111100, 0b00111111, 0b11000000,
        0b11000011, 0b11001100, 0b11001111, 0b11110000, 0b11110011, 0b11111100, 0b11111111,
    ];
    for (char, encoded) in encoded_chars {
        let low = (encoded & 0xFF) as usize;
        let high = (encoded >> 8) as usize;
        let expected = [
            table[low >> 4],
            table[low >> 4],
            table[low & 0x0F],
            table[low & 0x0F],
            table[high >> 4],
            table[high >> 4],
            table[high & 0x0F],
            table[high & 0x0F],
        ];
        let base = 0x0500 + u16::from(char) * 8;
        for (index, expected_byte) in expected.iter().enumerate() {
            let address = base + index as u16;
            assert_eq!(
                *expected_byte,
                cpu.memory.get_byte(address),
                "character {char} at {address:04X}"
            );
        }
    }

    let end = 0x0400 + command.len() as u16;
    assert_eq!((end & 0xFF) as u8, cpu.memory.get_byte(0x00FE));
    assert_eq!((end >> 8) as u8, cpu.memory.get_byte(0x00FF));

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
