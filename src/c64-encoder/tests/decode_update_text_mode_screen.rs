use c64_assembler::{
    builder::{ApplicationBuilder, InstructionBuilder, ModuleBuilder},
    generator::{Generator, ProgramGenerator},
    validator::{AssemblerResult, Validator},
};
use c64_encoder::command::{modules::CurrentPTR, update_text_mode_screen::UpdateTextModeScreen, DecoderModule};
use mos6502::{
    cpu::CPU,
    instruction::Nmos6502,
    memory::{Bus, Memory},
};

const STREAM_ADDRESS: u16 = 0x0400;
const PROGRAM_ADDRESS: u16 = 0x0800;

#[test]
fn decodes_full_screen_command() -> AssemblerResult<()> {
    let application = ApplicationBuilder::default()
        .define_address("CURRENT_PTR", 0xFE)
        .define_address("SCREEN_CHAR_PTR", 0xFC)
        .define_address("SCREEN_CHARS_PAGE0", 0xC000)
        .module(
            ModuleBuilder::default()
                .instructions(
                    InstructionBuilder::default()
                        .jsr_addr("update_text_mode_screen__process")
                        .raw(&[0xFF])
                        .build(),
                )
                .build(),
        )
        .module(UpdateTextModeScreen::module())
        .module(CurrentPTR::module())
        .build()?;
    application.validate()?;
    let program = ProgramGenerator::default().generate(application)?;

    let chars = (0_u8..=255).cycle().take(1000).collect::<Vec<_>>();
    let mut stream = vec![0x20_u8];
    stream.extend_from_slice(&chars);

    let mut cpu = CPU::new(Memory::new(), Nmos6502);
    cpu.memory
        .set_bytes(0x00FE, &[STREAM_ADDRESS as u8, (STREAM_ADDRESS >> 8) as u8]);
    cpu.memory.set_bytes(PROGRAM_ADDRESS, &program[2..]);
    cpu.memory.set_bytes(STREAM_ADDRESS, &stream);
    cpu.memory.set_bytes(0xC000, &[0x11_u8; 1000]);
    cpu.registers.program_counter = PROGRAM_ADDRESS;
    cpu.run();

    assert_eq!(cpu.registers.program_counter, PROGRAM_ADDRESS + 3);
    let end = STREAM_ADDRESS + stream.len() as u16;
    assert_eq!(cpu.memory.get_byte(0x00FE), (end & 0xFF) as u8);
    assert_eq!(cpu.memory.get_byte(0x00FF), (end >> 8) as u8);
    let decoded = (0..1000)
        .map(|offset| cpu.memory.get_byte(0xC000 + offset as u16))
        .collect::<Vec<_>>();
    assert_eq!(decoded, chars);
    Ok(())
}
