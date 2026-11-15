use c64_assembler::{
    builder::{ApplicationBuilder, InstructionBuilder, ModuleBuilder},
    generator::{Generator, ProgramGenerator},
    validator::AssemblerResult,
};
use c64_encoder::{
    command::{
        modules::CurrentPTR,
        partial_update_text_mode::{PartialUpdateTextModeScreen, UpdateSingleChar},
        DecoderModule,
    },
    encoder::Encoder,
};
use mos6502::{
    cpu::CPU,
    instruction::Nmos6502,
    memory::{Bus, Memory},
};

fn build_program() -> AssemblerResult<Vec<u8>> {
    let application = ApplicationBuilder::default()
        .define_address("CURRENT_PTR", 0xFE)
        .define_address("SCREEN_CHAR_PTR", 0xFC)
        .define_address("SCRATCH_SPACE_00", 0xFB)
        .define_address("SCREEN_CHARS_PAGE0", 0xC000)
        .module(
            ModuleBuilder::default()
                .instructions(
                    InstructionBuilder::default()
                        .jsr_addr("partial_update_text_mode__process")
                        .raw(&[0xFF])
                        .build(),
                )
                .build(),
        )
        .module(CurrentPTR::module())
        .module(PartialUpdateTextModeScreen::module())
        .build()?;

    ProgramGenerator::default().generate(application)
}

#[test]
fn decode_partial_update_text_mode() -> AssemblerResult<()> {
    let update = PartialUpdateTextModeScreen {
        changes: vec![
            UpdateSingleChar { offset: 0, char: 0x12 },
            UpdateSingleChar { offset: 1, char: 0x34 },
            UpdateSingleChar {
                offset: 300,
                char: 0x56,
            },
            UpdateSingleChar {
                offset: 999,
                char: 0x78,
            },
        ],
    };
    let mut command = vec![0; update.byte_size() + 1];
    update.encode(&mut command[1..]);

    let program = build_program()?;
    let mut cpu = CPU::new(Memory::new(), Nmos6502);
    cpu.memory.set_bytes(0x00FE, &[0x00, 0x04]);
    cpu.memory.set_bytes(0x0800, &program[2..]);
    cpu.memory.set_bytes(0x0400, &command);
    cpu.memory.set_bytes(0xC000, &vec![0xDE; 1000]);
    cpu.registers.program_counter = 0x0800;

    cpu.run();

    assert_eq!(0x12, cpu.memory.get_byte(0xC000));
    assert_eq!(0x34, cpu.memory.get_byte(0xC001));
    assert_eq!(0x56, cpu.memory.get_byte(0xC12C));
    assert_eq!(0x78, cpu.memory.get_byte(0xC3E7));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC002));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC12B));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC3E6));

    let end = 0x0400 + command.len() as u16;
    assert_eq!((end & 0xFF) as u8, cpu.memory.get_byte(0x00FE));
    assert_eq!((end >> 8) as u8, cpu.memory.get_byte(0x00FF));

    Ok(())
}

#[test]
fn decode_partial_update_text_mode_without_changes() -> AssemblerResult<()> {
    let update = PartialUpdateTextModeScreen { changes: vec![] };
    let mut command = vec![0; update.byte_size() + 1];
    update.encode(&mut command[1..]);

    let program = build_program()?;
    let mut cpu = CPU::new(Memory::new(), Nmos6502);
    cpu.memory.set_bytes(0x00FE, &[0x00, 0x04]);
    cpu.memory.set_bytes(0x0800, &program[2..]);
    cpu.memory.set_bytes(0x0400, &command);
    cpu.memory.set_bytes(0xC000, &vec![0xDE; 1000]);
    cpu.registers.program_counter = 0x0800;

    cpu.run();

    assert_eq!(0xDE, cpu.memory.get_byte(0xC000));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC3E7));

    let end = 0x0400 + command.len() as u16;
    assert_eq!((end & 0xFF) as u8, cpu.memory.get_byte(0x00FE));
    assert_eq!((end >> 8) as u8, cpu.memory.get_byte(0x00FF));

    Ok(())
}
