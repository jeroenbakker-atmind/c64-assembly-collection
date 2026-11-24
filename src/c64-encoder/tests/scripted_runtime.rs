use c64_assembler::{
    builder::{ApplicationBuilder, InstructionBuilder, ModuleBuilder},
    generator::{DasmGenerator, Generator, ProgramGenerator},
    validator::{AssemblerResult, Validator},
    Instructions,
};
use c64_encoder::command::{
    modules::{
        dummy_resource_table_module, BankBuilder, ConditionBuilder, EngineBuilder, FramePlayBuilder, LoaderBuilder,
        RefreshBuilder, RefreshWaitN, ResourceBuilder,
    },
    DecoderModule,
};
use mos6502::{
    cpu::CPU,
    instruction::Nmos6502,
    memory::{Bus, Memory},
    registers::Status,
};

const PROGRAM_ADDRESS: u16 = 0x0800;
const STUB_IMAGE_ADDRESS: u16 = 0x3000;
const RESOURCE_DEST_0: u16 = 0x4000;
const RESOURCE_DEST_1: u16 = 0x5000;
const STUB_COUNTER: u16 = 0x3E10;

fn resource_table() -> Vec<u8> {
    vec![
        1,
        1,
        1,
        RESOURCE_DEST_0 as u8,
        (RESOURCE_DEST_0 >> 8) as u8,
        3,
        0,
        2,
        21,
        1,
        RESOURCE_DEST_1 as u8,
        (RESOURCE_DEST_1 >> 8) as u8,
        4,
        0,
    ]
}

fn resource_table_module() -> c64_assembler::Module {
    ModuleBuilder::default()
        .name("resource_data")
        .instructions(
            InstructionBuilder::default()
                .label("resource_table")
                .raw(&resource_table())
                .build(),
        )
        .build()
}

fn run_program(application: c64_assembler::Application) -> AssemblerResult<CPU<Memory, Nmos6502>> {
    let program = ProgramGenerator::default().generate(application)?;
    let mut cpu = CPU::new(Memory::new(), Nmos6502);
    cpu.memory.set_bytes(PROGRAM_ADDRESS, &program[2..]);
    cpu.registers.program_counter = PROGRAM_ADDRESS;
    cpu.run();
    Ok(cpu)
}

#[test]
fn resource_use_points_current_ptr_at_destination_and_offset() -> AssemblerResult<()> {
    let stub = InstructionBuilder::default()
        .lda_imm(1)
        .ldx_imm(0x12)
        .ldy_imm(0x00)
        .jsr_addr("engine__resource__use")
        .raw(&[0xFF])
        .build();
    let application = ApplicationBuilder::default()
        .define_address("CURRENT_PTR", 0xFE)
        .add_resource_addresses()
        .module(ModuleBuilder::default().instructions(stub).build())
        .module(resource_table_module())
        .module(c64_encoder::command::modules::Resource::module())
        .build()?;
    application.validate()?;
    let mut cpu = run_program(application)?;

    assert_eq!(cpu.memory.get_byte(0xFE), 0x12);
    assert_eq!(cpu.memory.get_byte(0xFF), (RESOURCE_DEST_1 >> 8) as u8);
    Ok(())
}

#[test]
fn loader_start_on_irq_and_wait_copies_exact_payload() -> AssemblerResult<()> {
    let stub = InstructionBuilder::default()
        .lda_imm(1)
        .jsr_addr("engine__loader__start")
        .jsr_addr("engine__loader__on_irq")
        .jsr_addr("engine__loader__wait_loaded")
        .raw(&[0xFF])
        .build();
    let application = ApplicationBuilder::default()
        .define_address("CURRENT_PTR", 0xFE)
        .add_loader_addresses()
        .add_resource_addresses()
        .module(ModuleBuilder::default().instructions(stub).build())
        .module(resource_table_module())
        .module(c64_encoder::command::modules::Resource::module())
        .module(c64_encoder::command::modules::Loader::module())
        .build()?;
    application.validate()?;

    let program = ProgramGenerator::default().generate(application)?;
    let mut cpu = CPU::new(Memory::new(), Nmos6502);
    cpu.memory.set_bytes(PROGRAM_ADDRESS, &program[2..]);
    cpu.memory
        .set_bytes(STUB_IMAGE_ADDRESS, &[0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x11]);
    let resource_image = STUB_IMAGE_ADDRESS + 3;
    cpu.memory.set_byte(0x8C, resource_image as u8);
    cpu.memory.set_byte(0x8D, (resource_image >> 8) as u8);
    cpu.registers.program_counter = PROGRAM_ADDRESS;
    cpu.run();

    assert_eq!(cpu.memory.get_byte(0x80), 1);
    assert_eq!(cpu.memory.get_byte(0x81), 1);
    assert_eq!(cpu.memory.get_byte(0x84), 0);
    assert_eq!(cpu.memory.get_byte(0x85), 0);
    assert_eq!(cpu.memory.get_byte(RESOURCE_DEST_1), 0xDD);
    assert_eq!(cpu.memory.get_byte(RESOURCE_DEST_1 + 1), 0xEE);
    assert_eq!(cpu.memory.get_byte(RESOURCE_DEST_1 + 2), 0xFF);
    assert_eq!(cpu.memory.get_byte(RESOURCE_DEST_1 + 3), 0x11);
    Ok(())
}

#[test]
fn frame_play_n_calls_frame_process_exactly_n_times() -> AssemblerResult<()> {
    let mut stub = InstructionBuilder::default();
    stub.label("engine__frame__process")
        .inc_addr("FRAME_TEST_COUNTER")
        .rts();
    let main = InstructionBuilder::default()
        .lda_imm(3)
        .ldx_imm(0)
        .jsr_addr("engine__frame__play_n")
        .raw(&[0xFF])
        .build();
    let application = ApplicationBuilder::default()
        .define_address("FRAME_TEST_COUNTER", STUB_COUNTER)
        .add_frame_play_addresses()
        .module(ModuleBuilder::default().instructions(main).build())
        .module(ModuleBuilder::default().instructions(stub.build()).build())
        .module(c64_encoder::command::modules::FramePlay::module())
        .build()?;
    application.validate()?;
    let mut cpu = run_program(application)?;

    assert_eq!(cpu.memory.get_byte(STUB_COUNTER), 3);
    Ok(())
}

#[test]
fn refresh_wait_n_calls_refresh_wait_exactly_n_times() -> AssemblerResult<()> {
    let mut stub = InstructionBuilder::default();
    stub.label("engine__refresh__wait")
        .inc_addr("REFRESH_TEST_COUNTER")
        .rts();
    let main = InstructionBuilder::default()
        .lda_imm(5)
        .jsr_addr("engine__refresh__wait_n")
        .raw(&[0xFF])
        .build();
    let application = ApplicationBuilder::default()
        .define_address("REFRESH_TEST_COUNTER", STUB_COUNTER)
        .add_refresh_addresses()
        .module(ModuleBuilder::default().instructions(main).build())
        .module(ModuleBuilder::default().instructions(stub.build()).build())
        .module(RefreshWaitN::module())
        .build()?;
    application.validate()?;
    let mut cpu = run_program(application)?;

    assert_eq!(cpu.memory.get_byte(STUB_COUNTER), 5);
    Ok(())
}

fn run_condition_stub(
    stub: Instructions,
    music_position: u8,
    disk_turned: u8,
    expected_disk_turned: u8,
) -> AssemblerResult<bool> {
    let application = ApplicationBuilder::default()
        .add_condition_addresses()
        .module(ModuleBuilder::default().instructions(stub).build())
        .module(c64_encoder::command::modules::Condition::module())
        .build()?;
    application.validate()?;
    let program = ProgramGenerator::default().generate(application)?;
    let mut cpu = CPU::new(Memory::new(), Nmos6502);
    cpu.memory.set_bytes(PROGRAM_ADDRESS, &program[2..]);
    cpu.memory.set_byte(0x3E00, music_position);
    cpu.memory.set_byte(0x3E01, disk_turned);
    cpu.registers.program_counter = PROGRAM_ADDRESS;
    cpu.run();
    assert_eq!(cpu.memory.get_byte(0x3E00), music_position);
    assert_eq!(cpu.memory.get_byte(0x3E01), expected_disk_turned);
    Ok(cpu.registers.status.contains(Status::PS_ZERO))
}

#[test]
fn condition_routines_return_zero_flag_for_true() -> AssemblerResult<()> {
    let music_before = InstructionBuilder::default()
        .lda_imm(5)
        .jsr_addr("engine__condition__music_before")
        .raw(&[0xFF])
        .build();
    assert!(run_condition_stub(music_before.clone(), 2, 0, 0)?);
    assert!(!run_condition_stub(music_before.clone(), 5, 0, 0)?);
    assert!(!run_condition_stub(music_before, 6, 0, 0)?);

    let disk_turned = InstructionBuilder::default()
        .jsr_addr("engine__condition__disk_turned")
        .raw(&[0xFF])
        .build();
    assert!(!run_condition_stub(disk_turned.clone(), 0, 0, 0)?);
    assert!(run_condition_stub(disk_turned, 0, 1, 1)?);

    let marked = InstructionBuilder::default()
        .jsr_addr("engine__condition__mark_disk_turned")
        .jsr_addr("engine__condition__disk_turned")
        .raw(&[0xFF])
        .build();
    assert!(run_condition_stub(marked, 0, 0, 1)?);
    Ok(())
}

#[test]
fn full_runtime_application_has_expected_primitives_without_interpreter_state() -> AssemblerResult<()> {
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
        .module(resource_table_module())
        .add_engine()
        .build()?;
    application.validate()?;
    let source = DasmGenerator::default().generate(application)?;

    for label in [
        "engine__init:",
        "engine__stream__init:",
        "engine__play_stream:",
        "engine__frame__play_n:",
        "engine__resource__use:",
        "engine__loader__init:",
        "engine__loader__start:",
        "engine__loader__wait_loaded:",
        "engine__loader__on_irq:",
        "engine__loader__deinit:",
        "engine__condition__init:",
        "engine__condition__music_before:",
        "engine__condition__disk_turned:",
        "engine__bank__init:",
        "engine__bank__restore:",
        "engine__irq__handler:",
        "resource_table:",
    ] {
        assert!(source.contains(label), "missing {label}");
    }
    assert!(!source.contains("LOADER_WAIT_REQUIRED"));
    assert!(!source.contains("SEQUENCE_PTR"));
    assert!(!source.contains("engine__sequence__opcode"));
    Ok(())
}

#[test]
fn legacy_stream_application_validates_with_dummy_resource_table() -> AssemblerResult<()> {
    let main = ModuleBuilder::default()
        .instructions(
            InstructionBuilder::default()
                .jsr_addr("engine__init")
                .jsr_addr("engine__stream__init")
                .jsr_addr("engine__play_stream")
                .rts()
                .build(),
        )
        .build();
    let engine_data = ModuleBuilder::default()
        .instructions(
            InstructionBuilder::default()
                .label("engine_data")
                .raw(&[1, 0, 0])
                .build(),
        )
        .build();
    let application = ApplicationBuilder::default()
        .include_vic2_defines()
        .module(main)
        .module(engine_data)
        .module(dummy_resource_table_module())
        .add_engine()
        .build()?;
    application.validate()?;
    let source = DasmGenerator::default().generate(application)?;

    assert!(source.contains("  jsr engine__stream__init"));
    assert!(source.contains("  jsr engine__play_stream"));
    Ok(())
}

fn run_bank_stub(restore: bool) -> AssemblerResult<CPU<Memory, Nmos6502>> {
    let mut builder = InstructionBuilder::default();
    builder.jsr_addr("engine__bank__init");
    if restore {
        builder.jsr_addr("engine__bank__restore");
    }
    let stub = builder.raw(&[0xFF]).build();
    let application = ApplicationBuilder::default()
        .add_bank_addresses()
        .module(ModuleBuilder::default().instructions(stub).build())
        .module(c64_encoder::command::modules::Bank::module())
        .build()?;
    application.validate()?;

    let program = ProgramGenerator::default().generate(application)?;
    let mut cpu = CPU::new(Memory::new(), Nmos6502);
    cpu.memory.set_bytes(PROGRAM_ADDRESS, &program[2..]);
    cpu.memory.set_byte(0x0000, 0x05);
    cpu.memory.set_byte(0x0001, 0xFF);
    cpu.registers.program_counter = PROGRAM_ADDRESS;
    cpu.run();
    Ok(cpu)
}

#[test]
fn bank_init_banks_out_basic_rom() -> AssemblerResult<()> {
    let mut cpu = run_bank_stub(false)?;
    assert_eq!(cpu.memory.get_byte(0x0000), 0x0D);
    assert_eq!(cpu.memory.get_byte(0x0001), 0xF7);
    assert_eq!(cpu.memory.get_byte(0x3E02), 0xFF);
    assert_eq!(cpu.memory.get_byte(0x3E03), 0x05);
    Ok(())
}

#[test]
fn bank_restore_restores_processor_port() -> AssemblerResult<()> {
    let mut cpu = run_bank_stub(true)?;
    assert_eq!(cpu.memory.get_byte(0x0000), 0x05);
    assert_eq!(cpu.memory.get_byte(0x0001), 0xFF);
    Ok(())
}
