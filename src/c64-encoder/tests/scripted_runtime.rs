use c64_assembler::{
    builder::{ApplicationBuilder, InstructionBuilder, ModuleBuilder},
    generator::{DasmGenerator, Generator, ProgramGenerator},
    validator::{AssemblerResult, Validator},
    Instructions,
};
use c64_encoder::command::{
    modules::{
        dummy_resource_table_module, BankBuilder, ConditionBuilder, EngineBuilder, FramePlayBuilder, Irq,
        LoaderBackend, LoaderBuilder, LoaderRegularBuilder, RefreshBuilder, RefreshWaitN, RegularLoaderBackend,
        ResourceBuilder, StubLoaderBackend,
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
const SECTOR_CHAIN_RESULT_BASE: u16 = 0x3F00;
const CIA2_DATA_PORT: u16 = 0xDD00;
const CIA2_IRQ_CONTROL: u16 = 0xDD0D;
const CIA2_TIMER_A_CONTROL: u16 = 0xDD0E;
const CIA2_TIMER_B_CONTROL: u16 = 0xDD0F;
const LOADER_STATUS: u16 = 0x80;
const LOADER_TEST_CALLS: u16 = 0x3E10;
const LOADER_IEC_TRACK: u16 = 0x3E26;
const LOADER_IEC_SECTOR: u16 = 0x3E27;
const LOADER_IEC_TENS: u16 = 0x3E2A;
const LOADER_IEC_MODULUS: u16 = 0x3E2C;
const LOADER_IEC_POSITION: u16 = 0x3E35;
const ENGINE_NMI_TIMER_B_PENDING: u16 = 0x3E34;
const NMI_FRAME_A: u8 = 0xC3;
const NMI_FRAME_X: u8 = 0xA7;
const NMI_FRAME_Y: u8 = 0x5E;

#[derive(Clone, Debug, Default)]
struct IcrBus {
    memory: Memory,
    irq_control_writes: Vec<u8>,
    timer_control_writes: Vec<u8>,
    port_writes: Vec<u8>,
}

impl Bus for IcrBus {
    fn get_byte(&mut self, address: u16) -> u8 {
        self.memory.get_byte(address)
    }

    fn set_byte(&mut self, address: u16, value: u8) {
        match address {
            CIA2_DATA_PORT => self.port_writes.push(value),
            CIA2_IRQ_CONTROL => self.irq_control_writes.push(value),
            CIA2_TIMER_A_CONTROL | CIA2_TIMER_B_CONTROL => self.timer_control_writes.push(value),
            _ => {}
        }
        self.memory.set_byte(address, value);
    }
}

/// Seed the emulator's RTI frame at `$01F7..$01F9` (status/PC).
///
/// The KERNAL NMI entry at `$FE43` is only `SEI; JMP ($0318)`, so the engine
/// handler owns the A/X/Y frame. Callers set those registers on the CPU before
/// running, with SP left at `$F6`; the handler restores them and RTI leaves SP
/// at `$F9`.
fn seed_interrupt_frame(bus: &mut IcrBus, stop_address: u16) {
    bus.memory.set_byte(0x01F7, 0x20);
    bus.memory.set_byte(0x01F8, stop_address as u8);
    bus.memory.set_byte(0x01F9, (stop_address >> 8) as u8);
}

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

fn run_program_with_stub_source(application: c64_assembler::Application) -> AssemblerResult<CPU<Memory, Nmos6502>> {
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
    Ok(cpu)
}

fn loader_play_application(complete_load: bool) -> AssemblerResult<c64_assembler::Application> {
    let mut main = InstructionBuilder::default();
    main.lda_imm(1)
        .jsr_addr("engine__loader__start")
        .lda_imm(2)
        .ldx_imm(0)
        .jsr_addr("engine__frame__play_n");
    if complete_load {
        main.jsr_addr("engine__loader__on_irq")
            .jsr_addr("engine__loader__wait_loaded");
    }
    let main = main.raw(&[0xFF]).build();

    let mut frame_process = InstructionBuilder::default();
    frame_process
        .label("engine__frame__process")
        .inc_addr("FRAME_TEST_COUNTER")
        .rts();

    ApplicationBuilder::default()
        .define_address("FRAME_TEST_COUNTER", STUB_COUNTER)
        .define_address("CURRENT_PTR", 0xFE)
        .add_loader_addresses()
        .add_resource_addresses()
        .add_frame_play_addresses()
        .module(ModuleBuilder::default().instructions(main).build())
        .module(resource_table_module())
        .module(c64_encoder::command::modules::Resource::module())
        .module(c64_encoder::command::modules::Loader::module())
        .module(StubLoaderBackend::module())
        .module(c64_encoder::command::modules::FramePlay::module())
        .module(ModuleBuilder::default().instructions(frame_process.build()).build())
        .build()
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
fn resource_lookups_expose_physical_location() -> AssemblerResult<()> {
    let stub = InstructionBuilder::default()
        .lda_imm(1)
        .jsr_addr("engine__resource__track")
        .sta_addr("TEST_TRACK")
        .lda_imm(1)
        .jsr_addr("engine__resource__sector")
        .sta_addr("TEST_SECTOR")
        .lda_imm(1)
        .jsr_addr("engine__resource__sector_count")
        .sta_addr("TEST_SECTOR_COUNT")
        .raw(&[0xFF])
        .build();
    let application = ApplicationBuilder::default()
        .define_address("TEST_TRACK", STUB_COUNTER)
        .define_address("TEST_SECTOR", STUB_COUNTER + 1)
        .define_address("TEST_SECTOR_COUNT", STUB_COUNTER + 2)
        .define_address("CURRENT_PTR", 0xFE)
        .add_resource_addresses()
        .module(ModuleBuilder::default().instructions(stub).build())
        .module(resource_table_module())
        .module(c64_encoder::command::modules::Resource::module())
        .build()?;
    application.validate()?;
    let mut cpu = run_program(application)?;

    assert_eq!(cpu.memory.get_byte(STUB_COUNTER), 2);
    assert_eq!(cpu.memory.get_byte(STUB_COUNTER + 1), 21);
    assert_eq!(cpu.memory.get_byte(STUB_COUNTER + 2), 1);
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
        .module(StubLoaderBackend::module())
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
fn frames_play_while_a_load_is_still_busy() -> AssemblerResult<()> {
    let application = loader_play_application(false)?;
    application.validate()?;
    let mut cpu = run_program_with_stub_source(application)?;

    assert_eq!(cpu.memory.get_byte(0x80), 2);
    assert_eq!(cpu.memory.get_byte(STUB_COUNTER), 2);
    assert_eq!(cpu.memory.get_byte(RESOURCE_DEST_1), 0);
    Ok(())
}

#[test]
fn loader_completes_after_frames_have_played() -> AssemblerResult<()> {
    let application = loader_play_application(true)?;
    application.validate()?;
    let mut cpu = run_program_with_stub_source(application)?;

    assert_eq!(cpu.memory.get_byte(0x80), 1);
    assert_eq!(cpu.memory.get_byte(STUB_COUNTER), 2);
    assert_eq!(cpu.memory.get_byte(RESOURCE_DEST_1), 0xDD);
    assert_eq!(cpu.memory.get_byte(RESOURCE_DEST_1 + 1), 0xEE);
    Ok(())
}

#[test]
fn regular_store_skips_padding_after_payload_is_complete() -> AssemblerResult<()> {
    let main = InstructionBuilder::default()
        .lda_imm(0)
        .sta_addr("LOADER_REMAINING")
        .sta_addr_offs("LOADER_REMAINING", 1)
        .lda_imm(0xAA)
        .sta_addr("LOADER_IEC_BYTE")
        .lda_imm_low("PAD_TEST")
        .sta_addr("LOADER_DST_PTR")
        .lda_imm_high("PAD_TEST")
        .sta_addr_offs("LOADER_DST_PTR", 1)
        .jsr_addr("engine__loader__regular__store_sector_byte")
        .lda_addr("PAD_TEST")
        .sta_addr("PAD_RESULT")
        .lda_addr("LOADER_REMAINING")
        .sta_addr("PAD_REMAINING")
        .raw(&[0xFF])
        .build();
    let pad = ModuleBuilder::default()
        .instructions(InstructionBuilder::default().label("PAD_TEST").raw(&[0x55]).build())
        .build();
    let application = ApplicationBuilder::default()
        .define_address("PAD_RESULT", STUB_COUNTER)
        .define_address("PAD_REMAINING", STUB_COUNTER + 1)
        .define_address("CURRENT_PTR", 0xFE)
        .define_address("CIA2_IRQ_CONTROL", 0xDD0D)
        .add_loader_addresses()
        .add_loader_regular_addresses()
        .add_resource_addresses()
        .add_condition_addresses()
        .module(ModuleBuilder::default().instructions(main).build())
        .module(pad)
        .module(resource_table_module())
        .module(c64_encoder::command::modules::Resource::module())
        .module(c64_encoder::command::modules::Condition::module())
        .module(RegularLoaderBackend::module())
        .build()?;
    application.validate()?;
    let mut cpu = run_program(application)?;

    assert_eq!(cpu.memory.get_byte(STUB_COUNTER), 0x55);
    assert_eq!(cpu.memory.get_byte(STUB_COUNTER + 1), 0);
    Ok(())
}

#[test]
fn regular_turnaround_resets_the_receive_index() -> AssemblerResult<()> {
    let main = InstructionBuilder::default()
        .lda_imm(12)
        .sta_addr("LOADER_IEC_INDEX")
        .jsr_addr("engine__loader__regular__prepare_turnaround")
        .lda_addr("LOADER_IEC_INDEX")
        .sta_addr("TURNAROUND_INDEX")
        .lda_addr("LOADER_IEC_TIMEOUT_HI")
        .sta_addr("TURNAROUND_TIMEOUT_HI")
        .raw(&[0xFF])
        .build();
    let application = ApplicationBuilder::default()
        .define_address("TURNAROUND_INDEX", STUB_COUNTER)
        .define_address("TURNAROUND_TIMEOUT_HI", STUB_COUNTER + 1)
        .define_address("CURRENT_PTR", 0xFE)
        .define_address("CIA2_IRQ_CONTROL", 0xDD0D)
        .add_loader_addresses()
        .add_loader_regular_addresses()
        .add_resource_addresses()
        .add_condition_addresses()
        .module(ModuleBuilder::default().instructions(main).build())
        .module(resource_table_module())
        .module(c64_encoder::command::modules::Resource::module())
        .module(c64_encoder::command::modules::Condition::module())
        .module(RegularLoaderBackend::module())
        .build()?;
    application.validate()?;
    let mut cpu = run_program(application)?;

    assert_eq!(cpu.memory.get_byte(STUB_COUNTER), 0);
    assert_eq!(cpu.memory.get_byte(STUB_COUNTER + 1), 40);
    Ok(())
}

#[test]
fn regular_advance_sector_preserves_the_interleaved_sector_offset() -> AssemblerResult<()> {
    const RESULT_SECTOR: u16 = STUB_COUNTER;
    const RESULT_TRACK: u16 = STUB_COUNTER + 1;
    const RESULT_MODULUS: u16 = STUB_COUNTER + 2;

    let main = InstructionBuilder::default()
        .jsr_addr("engine__loader__regular__advance_sector")
        .lda_addr("LOADER_IEC_SECTOR")
        .sta_addr("RESULT_SECTOR")
        .lda_addr("LOADER_IEC_TRACK")
        .sta_addr("RESULT_TRACK")
        .lda_addr("LOADER_IEC_MODULUS")
        .sta_addr("RESULT_MODULUS")
        .raw(&[0xFF])
        .build();
    let application = ApplicationBuilder::default()
        .define_address("CURRENT_PTR", 0xFE)
        .define_address("CIA2_IRQ_CONTROL", CIA2_IRQ_CONTROL)
        .define_address("RESULT_SECTOR", RESULT_SECTOR)
        .define_address("RESULT_TRACK", RESULT_TRACK)
        .define_address("RESULT_MODULUS", RESULT_MODULUS)
        .add_loader_addresses()
        .add_loader_regular_addresses()
        .add_resource_addresses()
        .add_condition_addresses()
        .module(ModuleBuilder::default().instructions(main).build())
        .module(resource_table_module())
        .module(c64_encoder::command::modules::Resource::module())
        .module(c64_encoder::command::modules::Condition::module())
        .module(RegularLoaderBackend::module())
        .build()?;
    application.validate()?;
    let program = ProgramGenerator::default().generate(application)?;
    let mut cpu = CPU::new(Memory::new(), Nmos6502);
    cpu.memory.set_bytes(PROGRAM_ADDRESS, &program[2..]);
    cpu.memory.set_byte(LOADER_IEC_TRACK, 1);
    cpu.memory.set_byte(LOADER_IEC_SECTOR, 1);
    cpu.memory.set_byte(LOADER_IEC_TENS, 0);
    cpu.memory.set_byte(LOADER_IEC_POSITION, 0);
    cpu.memory.set_byte(LOADER_IEC_MODULUS, 20);
    cpu.registers.program_counter = PROGRAM_ADDRESS;
    cpu.run();

    assert_eq!(cpu.memory.get_byte(RESULT_SECTOR), 8);
    assert_eq!(cpu.memory.get_byte(RESULT_TRACK), 1);
    assert_eq!(cpu.memory.get_byte(RESULT_MODULUS), 20);
    Ok(())
}

#[test]
fn regular_advance_sector_walks_the_full_mastered_interleave() -> AssemblerResult<()> {
    let mut main = InstructionBuilder::default();
    for index in 0..32_u16 {
        main.jsr_addr("engine__loader__regular__advance_sector")
            .lda_addr("LOADER_IEC_SECTOR")
            .sta_addr_offs("SECTOR_CHAIN_RESULT_BASE", index)
            .lda_addr("LOADER_IEC_TRACK")
            .sta_addr_offs("SECTOR_CHAIN_RESULT_BASE", index + 32);
    }
    main.raw(&[0xFF]);

    let application = ApplicationBuilder::default()
        .define_address("CURRENT_PTR", 0xFE)
        .define_address("CIA2_IRQ_CONTROL", CIA2_IRQ_CONTROL)
        .define_address("SECTOR_CHAIN_RESULT_BASE", SECTOR_CHAIN_RESULT_BASE)
        .add_loader_addresses()
        .add_loader_regular_addresses()
        .add_resource_addresses()
        .add_condition_addresses()
        .module(ModuleBuilder::default().instructions(main.build()).build())
        .module(resource_table_module())
        .module(c64_encoder::command::modules::Resource::module())
        .module(c64_encoder::command::modules::Condition::module())
        .module(RegularLoaderBackend::module())
        .build()?;
    application.validate()?;
    let program = ProgramGenerator::default().generate(application)?;
    let mut cpu = CPU::new(Memory::new(), Nmos6502);
    cpu.memory.set_bytes(PROGRAM_ADDRESS, &program[2..]);
    cpu.memory.set_byte(LOADER_IEC_TRACK, 1);
    cpu.memory.set_byte(LOADER_IEC_SECTOR, 1);
    cpu.memory.set_byte(LOADER_IEC_TENS, 0);
    cpu.memory.set_byte(LOADER_IEC_POSITION, 0);
    cpu.memory.set_byte(LOADER_IEC_MODULUS, 20);
    cpu.registers.program_counter = PROGRAM_ADDRESS;
    cpu.run();

    let expected: [(u8, u8); 32] = [
        (1, 8),
        (1, 15),
        (1, 2),
        (1, 9),
        (1, 16),
        (1, 3),
        (1, 10),
        (1, 17),
        (1, 4),
        (1, 11),
        (1, 18),
        (1, 5),
        (1, 12),
        (1, 19),
        (1, 6),
        (1, 13),
        (1, 20),
        (1, 7),
        (1, 14),
        (2, 1),
        (2, 8),
        (2, 15),
        (2, 2),
        (2, 9),
        (2, 16),
        (2, 3),
        (2, 10),
        (2, 17),
        (2, 4),
        (2, 11),
        (2, 18),
        (2, 5),
    ];
    for (index, (track, sector)) in expected.iter().enumerate() {
        assert_eq!(
            cpu.memory.get_byte(SECTOR_CHAIN_RESULT_BASE + index as u16),
            *sector,
            "sector at chain position {index}"
        );
        assert_eq!(
            cpu.memory.get_byte(SECTOR_CHAIN_RESULT_BASE + 32 + index as u16),
            *track,
            "track at chain position {index}"
        );
    }
    Ok(())
}

#[test]
fn regular_init_position_recovers_a_non_zero_starting_sector() -> AssemblerResult<()> {
    const RESULT_SECTOR: u16 = STUB_COUNTER;
    const RESULT_TRACK: u16 = STUB_COUNTER + 1;
    const RESULT_POSITION: u16 = STUB_COUNTER + 2;

    let main = InstructionBuilder::default()
        .lda_addr("LOADER_IEC_TRACK")
        .jsr_addr("engine__loader__regular__track_modulus")
        .jsr_addr("engine__loader__regular__init_position")
        .lda_addr("LOADER_IEC_POSITION")
        .sta_addr("RESULT_POSITION")
        .jsr_addr("engine__loader__regular__advance_sector")
        .lda_addr("LOADER_IEC_SECTOR")
        .sta_addr("RESULT_SECTOR")
        .lda_addr("LOADER_IEC_TRACK")
        .sta_addr("RESULT_TRACK")
        .raw(&[0xFF])
        .build();
    let application = ApplicationBuilder::default()
        .define_address("CURRENT_PTR", 0xFE)
        .define_address("CIA2_IRQ_CONTROL", CIA2_IRQ_CONTROL)
        .define_address("RESULT_SECTOR", RESULT_SECTOR)
        .define_address("RESULT_TRACK", RESULT_TRACK)
        .define_address("RESULT_POSITION", RESULT_POSITION)
        .add_loader_addresses()
        .add_loader_regular_addresses()
        .add_resource_addresses()
        .add_condition_addresses()
        .module(ModuleBuilder::default().instructions(main).build())
        .module(resource_table_module())
        .module(c64_encoder::command::modules::Resource::module())
        .module(c64_encoder::command::modules::Condition::module())
        .module(RegularLoaderBackend::module())
        .build()?;
    application.validate()?;
    let program = ProgramGenerator::default().generate(application)?;
    let mut cpu = CPU::new(Memory::new(), Nmos6502);
    cpu.memory.set_bytes(PROGRAM_ADDRESS, &program[2..]);
    cpu.memory.set_byte(LOADER_IEC_TRACK, 1);
    cpu.memory.set_byte(LOADER_IEC_SECTOR, 13);
    cpu.registers.program_counter = PROGRAM_ADDRESS;
    cpu.run();

    assert_eq!(cpu.memory.get_byte(RESULT_POSITION), 16);
    assert_eq!(cpu.memory.get_byte(RESULT_SECTOR), 20);
    assert_eq!(cpu.memory.get_byte(RESULT_TRACK), 1);
    Ok(())
}

#[test]
fn engine_init_maps_the_custom_charset_at_c800() -> AssemblerResult<()> {
    let application = ApplicationBuilder::default()
        .include_vic2_defines()
        .module(resource_table_module())
        .add_engine_with_loader::<StubLoaderBackend>()
        .build()?;
    application.validate()?;
    let source = DasmGenerator::default().generate(application)?;
    let init = source
        .split("engine__init:")
        .nth(1)
        .expect("missing engine__init")
        .split("; --- Function end: ENGINE__INIT ---")
        .next()
        .expect("unterminated engine__init");

    assert!(
        init.contains("lda #$03"),
        "engine init must select screen $C000 and charset $C800: {init}"
    );
    assert!(
        init.contains("sta VIC2_MEMORY_SETUP"),
        "missing VIC memory setup: {init}"
    );
    Ok(())
}

#[test]
fn regular_timer_mask_is_set_on_enable_and_cleared_on_disable() -> AssemblerResult<()> {
    let main = InstructionBuilder::default()
        .jsr_addr("engine__loader__regular__enable_timer")
        .jsr_addr("engine__loader__regular__disable_timer")
        .jsr_addr("engine__loader__regular__enable_timer")
        .raw(&[0xFF])
        .build();
    let application = ApplicationBuilder::default()
        .define_address("CIA2_IRQ_CONTROL", CIA2_IRQ_CONTROL)
        .define_address("CURRENT_PTR", 0xFE)
        .add_loader_addresses()
        .add_loader_regular_addresses()
        .add_resource_addresses()
        .add_condition_addresses()
        .module(ModuleBuilder::default().instructions(main).build())
        .module(resource_table_module())
        .module(c64_encoder::command::modules::Resource::module())
        .module(c64_encoder::command::modules::Condition::module())
        .module(RegularLoaderBackend::module())
        .build()?;
    application.validate()?;
    let program = ProgramGenerator::default().generate(application)?;
    let mut bus = IcrBus::default();
    bus.set_bytes(PROGRAM_ADDRESS, &program[2..]);
    let mut cpu = CPU::new(bus, Nmos6502);
    cpu.registers.program_counter = PROGRAM_ADDRESS;
    cpu.run();

    assert_eq!(cpu.memory.irq_control_writes, vec![0x81, 0x03, 0x81]);
    assert_eq!(
        cpu.memory.timer_control_writes,
        vec![0x00, 0x01, 0x00, 0x00, 0x00, 0x01]
    );
    Ok(())
}

#[test]
fn regular_loader_preserves_the_engine_vic_bank_on_port_writes() -> AssemblerResult<()> {
    let main = InstructionBuilder::default()
        .jsr_addr("engine__loader__backend__init")
        .lda_imm(0b0001_0000)
        .jsr_addr("engine__loader__regular__line")
        .raw(&[0xFF])
        .build();
    let application = ApplicationBuilder::default()
        .define_address("CURRENT_PTR", 0xFE)
        .define_address("CIA2_IRQ_CONTROL", CIA2_IRQ_CONTROL)
        .add_loader_addresses()
        .add_loader_regular_addresses()
        .add_resource_addresses()
        .add_condition_addresses()
        .module(ModuleBuilder::default().instructions(main).build())
        .module(resource_table_module())
        .module(c64_encoder::command::modules::Resource::module())
        .module(c64_encoder::command::modules::Condition::module())
        .module(RegularLoaderBackend::module())
        .build()?;
    application.validate()?;
    let program = ProgramGenerator::default().generate(application)?;
    let mut bus = IcrBus::default();
    // `engine__init` selected VIC bank 3 by writing zero to bits 0-1.
    bus.memory.set_byte(CIA2_DATA_PORT, 0);
    bus.set_bytes(PROGRAM_ADDRESS, &program[2..]);
    let mut cpu = CPU::new(bus, Nmos6502);
    cpu.registers.program_counter = PROGRAM_ADDRESS;
    cpu.run();

    assert_eq!(cpu.memory.get_byte(0x3E37), 0x04);
    assert_eq!(cpu.memory.port_writes, vec![0x04, 0x14]);
    assert_eq!(cpu.memory.get_byte(CIA2_DATA_PORT), 0x14);
    Ok(())
}

#[test]
fn nmi_handler_saves_its_own_frame_and_a_timer_b_handoff_flag() -> AssemblerResult<()> {
    let hooks = InstructionBuilder::default()
        .label("engine__loader__on_irq")
        .rts()
        .label("engine__refresh__on_refresh")
        .rts()
        .build();
    let application = ApplicationBuilder::default()
        .define_address("CIA2_IRQ_CONTROL", CIA2_IRQ_CONTROL)
        .define_address("CIA2_TIMER_A_CONTROL", CIA2_TIMER_A_CONTROL)
        .define_address("CIA2_TIMER_A_LO", 0xDD04)
        .define_address("CIA2_TIMER_A_HI", 0xDD05)
        .define_address("CIA2_TIMER_B_CONTROL", 0xDD0F)
        .define_address("CIA2_TIMER_B_LO", 0xDD06)
        .define_address("CIA2_TIMER_B_HI", 0xDD07)
        .define_address("ENGINE_NMI_ACTIVE", 0x3E31)
        .define_address("ENGINE_NMI_TIMER_B_PENDING", ENGINE_NMI_TIMER_B_PENDING)
        .define_address("LOADER_STATUS", 0x80)
        .define_address("VIC2_IRQ_STATUS", 0xD019)
        .define_address("C64_IRQ_EPLOGUE", 0xEA81)
        .module(ModuleBuilder::default().instructions(hooks).build())
        .module(Irq::module())
        .build()?;
    application.validate()?;
    let source = DasmGenerator::default().generate(application)?;

    assert!(source.contains(
        "engine__irq__nmi_handler:\n  pha\n  inc ENGINE_NMI_ACTIVE\n  lda #$00\n  sta CIA2_TIMER_A_CONTROL\n  sta CIA2_TIMER_B_CONTROL\n  lda CIA2_IRQ_CONTROL"
    ));
    // Dense loader NMIs can postpone the raster IRQ indefinitely, so each NMI
    // acknowledges a pending raster latch and keeps the refresh countdown moving.
    assert!(source.contains("  lda VIC2_IRQ_STATUS\n  and #$01\n  beq engine__irq__nmi_handler__no_raster"));
    assert!(source.contains("  jsr engine__refresh__on_refresh\n  lda #$FF\n  sta VIC2_IRQ_STATUS"));
    assert!(source.contains(
        "engine__irq__nmi_handler__no_raster:\n  lda ENGINE_NMI_ACTIVE\n  cmp #$01\n  bne engine__irq__nmi_handler__nested"
    ));
    assert!(source.contains("  lda ENGINE_NMI_TIMER_B_PENDING\n  bne engine__irq__nmi_handler__timer_b"));
    assert!(source.contains("engine__irq__nmi_handler__nested:\n  dec ENGINE_NMI_ACTIVE\n  pla\n  rti"));
    assert!(source.contains(
        "engine__irq__nmi_handler__timer_b:\n  lda #$00\n  sta ENGINE_NMI_TIMER_B_PENDING\n  lda #$02\n  sta CIA2_IRQ_CONTROL\n  lda #$00\n  sta CIA2_TIMER_B_CONTROL\n  lda #$32\n  sta CIA2_TIMER_A_LO"
    ));
    assert!(source.contains(
        "engine__irq__nmi_handler__owner_done:\n  lda LOADER_STATUS\n  and #$02\n  beq engine__irq__nmi_handler__clear_guard"
    ));
    assert!(source.contains("  lda #$01\n  sta ENGINE_NMI_TIMER_B_PENDING\n  lda #$09\n  sta CIA2_TIMER_B_CONTROL"));
    assert!(source.contains("engine__irq__nmi_handler__clear_guard:\n  lda #$00\n  sta ENGINE_NMI_ACTIVE\n  pla\n  tay\n  pla\n  tax\n  pla\n  rti"));
    assert!(!source.contains("engine__irq__nmi_handler__nested:\n  dec ENGINE_NMI_ACTIVE\n  jmp engine__irq__epilogue"));
    assert!(!source.contains("sta ENGINE_NMI_A\n"));
    assert!(!source.contains("lda ENGINE_NMI_A\n"));
    Ok(())
}

#[test]
fn nmi_handler_schedules_the_next_phase_only_while_busy() -> AssemblerResult<()> {
    const STOP_ADDRESS: u16 = 0x0200;

    let main = InstructionBuilder::default()
        .jmp_addr("engine__irq__nmi_handler")
        .build();
    let hooks = InstructionBuilder::default()
        .label("engine__loader__on_irq")
        .inc_addr("LOADER_TEST_CALLS")
        .rts()
        .label("engine__refresh__on_refresh")
        .rts()
        .build();
    let application = ApplicationBuilder::default()
        .define_address("CIA2_IRQ_CONTROL", CIA2_IRQ_CONTROL)
        .define_address("CIA2_TIMER_A_CONTROL", CIA2_TIMER_A_CONTROL)
        .define_address("CIA2_TIMER_A_LO", 0xDD04)
        .define_address("CIA2_TIMER_A_HI", 0xDD05)
        .define_address("CIA2_TIMER_B_CONTROL", 0xDD0F)
        .define_address("CIA2_TIMER_B_LO", 0xDD06)
        .define_address("CIA2_TIMER_B_HI", 0xDD07)
        .define_address("ENGINE_NMI_ACTIVE", 0x3E31)
        .define_address("ENGINE_NMI_TIMER_B_PENDING", ENGINE_NMI_TIMER_B_PENDING)
        .define_address("LOADER_STATUS", LOADER_STATUS)
        .define_address("LOADER_TEST_CALLS", LOADER_TEST_CALLS)
        .define_address("VIC2_IRQ_STATUS", 0xD019)
        .define_address("C64_IRQ_EPLOGUE", 0xEA81)
        .module(ModuleBuilder::default().instructions(main).build())
        .module(ModuleBuilder::default().instructions(hooks).build())
        .module(Irq::module())
        .build()?;
    application.validate()?;

    let program = ProgramGenerator::default().generate(application)?;
    let mut bus = IcrBus::default();
    bus.memory.set_byte(LOADER_STATUS, 0b0000_0010);
    bus.memory.set_byte(STOP_ADDRESS, 0xFF);
    seed_interrupt_frame(&mut bus, STOP_ADDRESS);
    bus.set_bytes(PROGRAM_ADDRESS, &program[2..]);

    let mut cpu = CPU::new(bus, Nmos6502);
    cpu.registers.stack_pointer.0 = 0xF6;
    cpu.registers.accumulator = NMI_FRAME_A;
    cpu.registers.index_x = NMI_FRAME_X;
    cpu.registers.index_y = NMI_FRAME_Y;
    cpu.registers.program_counter = PROGRAM_ADDRESS;
    cpu.run();

    assert_eq!(cpu.memory.irq_control_writes, vec![0x02, 0x82]);
    assert_eq!(cpu.memory.timer_control_writes, vec![0x00, 0x00, 0x00, 0x09]);
    assert_eq!(cpu.memory.get_byte(0x3E31), 0);
    assert_eq!(cpu.memory.get_byte(ENGINE_NMI_TIMER_B_PENDING), 1);
    assert_eq!(cpu.memory.get_byte(LOADER_TEST_CALLS), 1);
    assert_eq!(cpu.registers.accumulator, NMI_FRAME_A);
    assert_eq!(cpu.registers.index_x, NMI_FRAME_X);
    assert_eq!(cpu.registers.index_y, NMI_FRAME_Y);
    assert_eq!(cpu.registers.stack_pointer.0, 0xF9);
    Ok(())
}

#[test]
fn nested_nmi_entry_does_not_run_the_loader_or_disturb_the_owner_stack() -> AssemblerResult<()> {
    const STOP_ADDRESS: u16 = 0x0200;

    let main = InstructionBuilder::default()
        .jmp_addr("engine__irq__nmi_handler")
        .build();
    let hooks = InstructionBuilder::default()
        .label("engine__loader__on_irq")
        .inc_addr("LOADER_TEST_CALLS")
        .rts()
        .label("engine__refresh__on_refresh")
        .rts()
        .build();
    let application = ApplicationBuilder::default()
        .define_address("CIA2_IRQ_CONTROL", CIA2_IRQ_CONTROL)
        .define_address("CIA2_TIMER_A_CONTROL", CIA2_TIMER_A_CONTROL)
        .define_address("CIA2_TIMER_A_LO", 0xDD04)
        .define_address("CIA2_TIMER_A_HI", 0xDD05)
        .define_address("CIA2_TIMER_B_CONTROL", CIA2_TIMER_B_CONTROL)
        .define_address("CIA2_TIMER_B_LO", 0xDD06)
        .define_address("CIA2_TIMER_B_HI", 0xDD07)
        .define_address("ENGINE_NMI_ACTIVE", 0x3E31)
        .define_address("ENGINE_NMI_TIMER_B_PENDING", ENGINE_NMI_TIMER_B_PENDING)
        .define_address("LOADER_STATUS", 0x80)
        .define_address("LOADER_TEST_CALLS", LOADER_TEST_CALLS)
        .define_address("VIC2_IRQ_STATUS", 0xD019)
        .define_address("C64_IRQ_EPLOGUE", 0xEA81)
        .module(ModuleBuilder::default().instructions(main).build())
        .module(ModuleBuilder::default().instructions(hooks).build())
        .module(Irq::module())
        .build()?;
    application.validate()?;

    let program = ProgramGenerator::default().generate(application)?;
    let mut bus = IcrBus::default();
    bus.memory.set_byte(0x3E31, 1);
    bus.memory.set_byte(LOADER_STATUS, 0b0000_0010);
    bus.memory.set_byte(STOP_ADDRESS, 0xFF);
    seed_interrupt_frame(&mut bus, STOP_ADDRESS);
    bus.set_bytes(PROGRAM_ADDRESS, &program[2..]);

    let mut cpu = CPU::new(bus, Nmos6502);
    cpu.registers.stack_pointer.0 = 0xF6;
    cpu.registers.accumulator = NMI_FRAME_A;
    cpu.registers.index_x = NMI_FRAME_X;
    cpu.registers.index_y = NMI_FRAME_Y;
    cpu.registers.program_counter = PROGRAM_ADDRESS;
    cpu.run();

    assert!(cpu.memory.irq_control_writes.is_empty());
    assert_eq!(cpu.memory.timer_control_writes, vec![0x00, 0x00]);
    assert_eq!(cpu.memory.get_byte(0x3E31), 1);
    assert_eq!(cpu.memory.get_byte(ENGINE_NMI_TIMER_B_PENDING), 0);
    assert_eq!(cpu.memory.get_byte(LOADER_TEST_CALLS), 0);
    assert_eq!(cpu.registers.accumulator, NMI_FRAME_A);
    assert_eq!(cpu.registers.index_x, NMI_FRAME_X);
    assert_eq!(cpu.registers.index_y, NMI_FRAME_Y);
    assert_eq!(cpu.registers.stack_pointer.0, 0xF9);
    Ok(())
}

#[test]
fn timer_b_nmi_restarts_timer_a_without_running_the_loader() -> AssemblerResult<()> {
    const STOP_ADDRESS: u16 = 0x0200;

    let main = InstructionBuilder::default()
        .jmp_addr("engine__irq__nmi_handler")
        .build();
    let hooks = InstructionBuilder::default()
        .label("engine__loader__on_irq")
        .inc_addr("LOADER_TEST_CALLS")
        .rts()
        .label("engine__refresh__on_refresh")
        .rts()
        .build();
    let application = ApplicationBuilder::default()
        .define_address("CIA2_IRQ_CONTROL", CIA2_IRQ_CONTROL)
        .define_address("CIA2_TIMER_A_CONTROL", CIA2_TIMER_A_CONTROL)
        .define_address("CIA2_TIMER_A_LO", 0xDD04)
        .define_address("CIA2_TIMER_A_HI", 0xDD05)
        .define_address("CIA2_TIMER_B_CONTROL", CIA2_TIMER_B_CONTROL)
        .define_address("CIA2_TIMER_B_LO", 0xDD06)
        .define_address("CIA2_TIMER_B_HI", 0xDD07)
        .define_address("ENGINE_NMI_ACTIVE", 0x3E31)
        .define_address("ENGINE_NMI_TIMER_B_PENDING", ENGINE_NMI_TIMER_B_PENDING)
        .define_address("LOADER_STATUS", 0x80)
        .define_address("LOADER_TEST_CALLS", LOADER_TEST_CALLS)
        .define_address("VIC2_IRQ_STATUS", 0xD019)
        .define_address("C64_IRQ_EPLOGUE", 0xEA81)
        .module(ModuleBuilder::default().instructions(main).build())
        .module(ModuleBuilder::default().instructions(hooks).build())
        .module(Irq::module())
        .build()?;
    application.validate()?;

    let program = ProgramGenerator::default().generate(application)?;
    let mut bus = IcrBus::default();
    bus.memory.set_byte(ENGINE_NMI_TIMER_B_PENDING, 1);
    bus.memory.set_byte(0x3E31, 0);
    bus.memory.set_byte(LOADER_STATUS, 0b0000_0010);
    bus.memory.set_byte(STOP_ADDRESS, 0xFF);
    seed_interrupt_frame(&mut bus, STOP_ADDRESS);
    bus.set_bytes(PROGRAM_ADDRESS, &program[2..]);

    let mut cpu = CPU::new(bus, Nmos6502);
    cpu.registers.stack_pointer.0 = 0xF6;
    cpu.registers.accumulator = NMI_FRAME_A;
    cpu.registers.index_x = NMI_FRAME_X;
    cpu.registers.index_y = NMI_FRAME_Y;
    cpu.registers.program_counter = PROGRAM_ADDRESS;
    cpu.run();

    assert_eq!(cpu.memory.irq_control_writes, vec![0x02, 0x81]);
    assert_eq!(cpu.memory.timer_control_writes, vec![0x00, 0x00, 0x00, 0x01]);
    assert_eq!(cpu.memory.get_byte(0xDD04), 50);
    assert_eq!(cpu.memory.get_byte(0xDD05), 0);
    assert_eq!(cpu.memory.get_byte(0x3E31), 0);
    assert_eq!(cpu.memory.get_byte(ENGINE_NMI_TIMER_B_PENDING), 0);
    assert_eq!(cpu.memory.get_byte(LOADER_TEST_CALLS), 0);
    assert_eq!(cpu.registers.accumulator, NMI_FRAME_A);
    assert_eq!(cpu.registers.index_x, NMI_FRAME_X);
    assert_eq!(cpu.registers.index_y, NMI_FRAME_Y);
    assert_eq!(cpu.registers.stack_pointer.0, 0xF9);
    Ok(())
}

#[test]
fn regular_loader_state_lives_outside_kernal_zero_page() -> AssemblerResult<()> {
    let application = ApplicationBuilder::default()
        .add_loader_regular_addresses()
        .add_refresh_addresses()
        .build()?;

    let address = |name: &str| match application
        .defines
        .iter()
        .find(|define| define.name == name)
        .unwrap_or_else(|| panic!("missing define {name}"))
        .value
    {
        c64_assembler::memory::define::Value::Address(address)
        | c64_assembler::memory::define::Value::Zeropage(address) => address,
    };

    assert_eq!(address("LOADER_IEC_STATE"), 0x3E20);
    assert_eq!(address("LOADER_IEC_TIMEOUT"), 0x3E2E);
    assert_eq!(address("REFRESH_WAIT_COUNT"), 0x3E2F);
    for name in [
        "LOADER_IEC_STATE",
        "LOADER_IEC_PHASE",
        "LOADER_IEC_BIT",
        "LOADER_IEC_BYTE",
        "LOADER_IEC_FLAGS",
        "LOADER_IEC_PORT",
        "LOADER_IEC_PORT_BASE",
        "LOADER_IEC_TRACK",
        "LOADER_IEC_SECTOR",
        "LOADER_IEC_SECTOR_LEFT",
        "LOADER_IEC_INDEX",
        "LOADER_IEC_TENS",
        "LOADER_IEC_ONES",
        "LOADER_IEC_MODULUS",
        "LOADER_IEC_TIMEOUT_HI",
        "LOADER_IEC_TIMEOUT",
        "REFRESH_WAIT_COUNT",
    ] {
        assert!(address(name) >= 0x0100, "{name} must not use KERNAL/BASIC zero page");
    }
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
    loader_status: u8,
    expected_disk_turned: u8,
) -> AssemblerResult<bool> {
    let application = ApplicationBuilder::default()
        .define_address("LOADER_STATUS", 0x80)
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
    cpu.memory.set_byte(0x80, loader_status);
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
    assert!(run_condition_stub(music_before.clone(), 2, 0, 0, 0)?);
    assert!(!run_condition_stub(music_before.clone(), 5, 0, 0, 0)?);
    assert!(!run_condition_stub(music_before, 6, 0, 0, 0)?);

    let disk_turned = InstructionBuilder::default()
        .jsr_addr("engine__condition__disk_turned")
        .raw(&[0xFF])
        .build();
    assert!(!run_condition_stub(disk_turned.clone(), 0, 0, 0, 0)?);
    assert!(run_condition_stub(disk_turned, 0, 1, 0, 1)?);

    let marked = InstructionBuilder::default()
        .jsr_addr("engine__condition__mark_disk_turned")
        .jsr_addr("engine__condition__disk_turned")
        .raw(&[0xFF])
        .build();
    assert!(run_condition_stub(marked, 0, 0, 0, 1)?);

    let load_complete = InstructionBuilder::default()
        .jsr_addr("engine__condition__load_complete")
        .raw(&[0xFF])
        .build();
    assert!(run_condition_stub(load_complete.clone(), 0, 0, 0, 0)?);
    assert!(!run_condition_stub(load_complete, 0, 0, 2, 0)?);
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
        "engine__condition__load_complete:",
        "engine__loader__regular__enable_timer:",
        "engine__loader__regular__disable_timer:",
        "engine__loader__regular__timeout_tick:",
        "engine__loader__regular__send_phase:",
        "engine__loader__regular__recv_phase:",
        "engine__loader__regular__turnaround_phase:",
        "engine__loader__regular__on_send_done:",
        "engine__loader__regular__on_recv_done:",
        "engine__loader__regular__prepare_listen_buffer:",
        "engine__loader__regular__prepare_open_buffer:",
        "engine__loader__regular__prepare_name_hash:",
        "engine__loader__regular__prepare_name_two:",
        "engine__loader__regular__prepare_unlisten_buffer:",
        "engine__loader__regular__prepare_listen_command:",
        "engine__loader__regular__prepare_open_command:",
        "engine__loader__regular__prepare_command_byte:",
        "engine__loader__regular__prepare_unlisten_command:",
        "engine__loader__regular__prepare_turnaround:",
        "engine__loader__regular__prepare_untalk:",
        "engine__loader__regular__sector_done:",
        "engine__bank__init:",
        "engine__bank__restore:",
        "engine__irq__handler:",
        "engine__irq__nmi_handler:",
        "resource_table:",
    ] {
        assert!(source.contains(label), "missing {label}");
    }
    assert!(source.contains("; --- Module begin: ENGINE__LOADER__BACKEND_REGULAR ---"));
    assert!(!source.contains("ENGINE__LOADER__BACKEND_STUB"));
    assert!(source.contains("CIA2_TIMER_A_CONTROL"));
    assert!(source.contains("CIA2_TIMER_B_CONTROL"));
    assert!(source.contains("  lda #$FF\n  sta VIC2_IRQ_STATUS"));
    // The raster source is VIC IRQ enable bit 0 and status latch bit 0. Bit 7
    // is only the global IRQ flag, so testing it mistakes other VIC IRQs.
    assert!(source.contains("  lda #$01\n  sta VIC2_IRQ_ENABLE"));
    assert!(source.contains("  lda VIC2_IRQ_STATUS\n  and #$01\n  beq engine__irq__handler__clear_latches"));
    // Acknowledge every VIC latch on both paths. If the raster latch is clear
    // but another VIC latch is set, the global IRQ flag can stay asserted and
    // starve playback; only refresh accounting is conditional on bit 0.
    assert!(source.contains("  beq engine__irq__handler__clear_latches"));
    assert!(source.contains("engine__irq__handler__clear_latches:\n  lda #$FF\n  sta VIC2_IRQ_STATUS"));
    assert!(source.contains("  jmp engine__irq__epilogue"));
    assert!(source.contains("  jsr engine__loader__on_irq"));
    assert!(!source.contains("LOADER_WAIT_REQUIRED"));
    assert!(!source.contains("SEQUENCE_PTR"));
    assert!(!source.contains("engine__sequence__opcode"));

    // The first send phase asserts ATN and CLK together, then waits for the
    // listener ACK before sending any data bits.
    assert!(source.contains("  jsr engine__loader__regular__atn_mask"));
    assert!(source.contains("  ora #$10"));
    assert!(source.contains("  jsr engine__loader__regular__line"));

    // Timeout-driven EOI is reachable, and a normal first CLK-low goes to
    // bit reception instead of skipping the EOI branch.
    assert!(source.contains("engine__loader__regular__recv_phase__eoi_start:"));
    assert!(source.contains("  jmp engine__loader__regular__recv_phase__eoi_start"));
    assert!(source.contains("  jmp engine__loader__regular__recv_phase__bit_loop"));

    // Each received bit must resume at the sampler. Branching back to
    // `bit_loop` would reset BIT to eight on every bit and never reach the
    // byte ACK. A single read and inline X countdown keep the sampler tight
    // enough for short CLK edges; a JSR timeout check misses the eighth bit.
    assert!(source.contains("engine__loader__regular__recv_phase__bit_low_wait:\n  lda CIA2_DATA_PORT\n  asl"));
    assert!(source.contains("  bmi engine__loader__regular__recv_phase__bit_low_tick\n  dec LOADER_IEC_BIT\n  bne engine__loader__regular__recv_phase__bit_high_wait\n  jmp engine__loader__regular__recv_phase__byte_ack"));
    assert!(source.contains("engine__loader__regular__recv_phase__bit_high_tick:\n  dex\n  bne engine__loader__regular__recv_phase__bit_high_wait"));
    assert!(source.contains("engine__loader__regular__recv_phase__bit_low_tick:\n  dex\n  bne engine__loader__regular__recv_phase__bit_low_wait"));
    Ok(())
}

#[test]
fn explicit_stub_backend_selection_keeps_the_shared_loader_contract() -> AssemblerResult<()> {
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
        .add_engine_with_loader::<StubLoaderBackend>()
        .build()?;
    application.validate()?;
    let source = DasmGenerator::default().generate(application)?;

    assert!(source.contains("; --- Module begin: ENGINE__LOADER__BACKEND_STUB ---"));
    assert!(!source.contains("ENGINE__LOADER__BACKEND_REGULAR"));
    for label in [
        "engine__loader__start:",
        "engine__loader__on_irq:",
        "engine__loader__wait_loaded:",
        "engine__loader__deinit:",
        "engine__resource__use:",
    ] {
        assert!(source.contains(label), "missing {label}");
    }
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
