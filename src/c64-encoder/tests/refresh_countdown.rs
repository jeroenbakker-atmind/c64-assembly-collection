use c64_assembler::{
    builder::{ApplicationBuilder, InstructionBuilder, ModuleBuilder},
    generator::{DasmGenerator, Generator, ProgramGenerator},
    validator::{AssemblerResult, Validator},
    Instructions,
};
use c64_encoder::command::{
    modules::{EngineBuilder, Refresh, RefreshBuilder},
    DecoderModule,
};
use mos6502::{
    cpu::CPU,
    instruction::Nmos6502,
    memory::{Bus, Memory},
};

const PROGRAM_ADDRESS: u16 = 0x0800;

fn build_refresh_program(stub: Instructions) -> AssemblerResult<Vec<u8>> {
    let application = ApplicationBuilder::default()
        .add_refresh_addresses()
        .include_vic2_defines()
        .module(ModuleBuilder::default().instructions(stub).build())
        .module(Refresh::module())
        .build()?;
    application.validate()?;
    ProgramGenerator::default().generate(application)
}

fn run_refresh_stub(stub: Instructions, remaining_address: u16, initial: u8) -> AssemblerResult<u8> {
    let program = build_refresh_program(stub)?;
    let mut cpu = CPU::new(Memory::new(), Nmos6502);
    cpu.memory.set_bytes(PROGRAM_ADDRESS, &program[2..]);
    cpu.memory.set_byte(remaining_address, initial);
    cpu.registers.program_counter = PROGRAM_ADDRESS;
    cpu.run();
    Ok(cpu.memory.get_byte(remaining_address))
}

fn run_refresh_calls(initial: u8, calls: usize) -> AssemblerResult<u8> {
    let mut stub = InstructionBuilder::default();
    for _ in 0..calls {
        stub.jsr_addr("engine__refresh__on_refresh");
    }
    let stub_size = 3 * calls as u16;
    stub.raw(&[0xFF]);
    let remaining_address = PROGRAM_ADDRESS + stub_size + 1;
    run_refresh_stub(stub.build(), remaining_address, initial)
}

#[test]
fn refresh_countdown_saturates_at_zero() -> AssemblerResult<()> {
    // One refresh per IRQ; four refreshes implement the authored 50/4 framerate.
    assert_eq!(3, run_refresh_calls(4, 1)?);
    assert_eq!(1, run_refresh_calls(4, 3)?);
    assert_eq!(0, run_refresh_calls(4, 4)?);

    // Extra refreshes during a long decode must not wrap the byte to 255.
    assert_eq!(0, run_refresh_calls(0, 6)?);
    assert_eq!(0, run_refresh_calls(2, 5)?);
    Ok(())
}

#[test]
fn wait_reloads_four_refreshes_at_zero() -> AssemblerResult<()> {
    let stub = InstructionBuilder::default()
        .jsr_addr("engine__refresh__wait")
        .raw(&[0xFF])
        .build();
    let remaining_address = PROGRAM_ADDRESS + 3 + 1;
    assert_eq!(4, run_refresh_stub(stub, remaining_address, 0)?);
    Ok(())
}

#[test]
fn engine_uses_irq_countdown_instead_of_vblank_polling() -> AssemblerResult<()> {
    let application = ApplicationBuilder::default()
        .include_vic2_defines()
        .module(
            ModuleBuilder::default()
                .name("engine_data")
                .instructions(
                    InstructionBuilder::default()
                        .label("engine_data")
                        .raw(&[0x01, 0x00])
                        .build(),
                )
                .build(),
        )
        .add_engine()
        .build()?;
    application.validate()?;
    let source = DasmGenerator::default().generate(application)?;

    assert!(source.contains("engine__refresh__irq:"));
    assert!(source.contains("  jsr engine__refresh__on_refresh"));
    assert!(source.contains("  asl VIC2_IRQ_STATUS"));
    assert!(source.contains("  jmp C64_IRQ_EPLOGUE"));
    assert_eq!(source.matches("  jsr engine__refresh__wait").count(), 1);
    assert!(!source.contains("engine__vblank__wait"));
    Ok(())
}
