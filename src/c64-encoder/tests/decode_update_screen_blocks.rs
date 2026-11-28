use c64_assembler::{
    builder::{ApplicationBuilder, InstructionBuilder, ModuleBuilder},
    generator::{Generator, ProgramGenerator},
    validator::{AssemblerResult, Validator},
    Module,
};
use c64_encoder::{
    command::{
        all_decoder_modules,
        modules::{decoder_module, BlockScratchBuilder, CurrentPTR, EngineBuilder, UpdateScreenBlocksDecoder},
        update_screen_blocks::{BlockKind, UpdateScreenBlocks, SCREEN_CHARS, SCREEN_HEIGHT, SCREEN_WIDTH},
        Command, DecoderModule,
    },
    encoder::Encoder,
};
use mos6502::{
    cpu::CPU,
    instruction::Nmos6502,
    memory::{Bus, Memory},
};

const STREAM_ADDRESS: u16 = 0x0400;
const PROGRAM_ADDRESS: u16 = 0x0800;

fn resource_table_module() -> Module {
    ModuleBuilder::default()
        .name("resource_data")
        .instructions(
            InstructionBuilder::default()
                .label("resource_table")
                .raw(&[0; 7])
                .build(),
        )
        .build()
}

fn build_program(kind: BlockKind) -> AssemblerResult<Vec<u8>> {
    let process = format!("update_screen_blocks_{}__process", kind.name());
    let mut builder = ApplicationBuilder::default();
    let application = builder
        .define_address("CURRENT_PTR", 0xFE)
        .define_address("SCREEN_CHAR_PTR", 0xFC)
        .define_address("SCREEN_CHARS_PAGE0", 0xC000)
        .add_block_scratch_addresses()
        .module(
            ModuleBuilder::default()
                .instructions(
                    InstructionBuilder::default()
                        .jsr_addr(process.as_str())
                        .raw(&[0xFF])
                        .build(),
                )
                .build(),
        )
        .module(UpdateScreenBlocksDecoder::module())
        .module(CurrentPTR::module());

    for kind in BlockKind::ALL {
        application.module(decoder_module(kind));
    }

    let application = application.build()?;
    application.validate()?;
    ProgramGenerator::default().generate(application)
}

fn screens() -> ([u8; SCREEN_CHARS], [u8; SCREEN_CHARS]) {
    let from = [0x11_u8; SCREEN_CHARS];
    let mut to = from;

    // Changes spread over several outer bands, several inner bands, and several
    // cells inside one block mask. This exercises multi-byte bitmaps and masks.
    let changes = [
        (0, 0, 0xA1),
        (1, 1, 0xA2),
        (2, 2, 0xA3),
        (4, 1, 0xA4),
        (5, 2, 0xA5),
        (38, 2, 0xA6),
        (39, 3, 0xA7),
        (0, 7, 0xB1),
        (6, 8, 0xB2),
        (20, 10, 0xB3),
        (21, 11, 0xB4),
        (38, 12, 0xB5),
        (39, 13, 0xB6),
        (2, 20, 0xC1),
        (3, 21, 0xC2),
        (36, 23, 0xC3),
        (37, 23, 0xC4),
    ];
    for (x, y, value) in changes {
        to[y * SCREEN_WIDTH + x] = value;
    }

    (from, to)
}

fn decode(kind: BlockKind, from: &[u8; SCREEN_CHARS], to: &[u8; SCREEN_CHARS]) -> AssemblerResult<Vec<u8>> {
    let command = Command::UpdateScreenBlocks(UpdateScreenBlocks::transition(from, to, kind));
    let mut stream = vec![0_u8; command.byte_size()];
    command.encode(&mut stream);

    let program = build_program(kind)?;
    let mut cpu = CPU::new(Memory::new(), Nmos6502);
    cpu.memory
        .set_bytes(0x00FE, &[STREAM_ADDRESS as u8, (STREAM_ADDRESS >> 8) as u8]);
    cpu.memory.set_bytes(PROGRAM_ADDRESS, &program[2..]);
    cpu.memory.set_bytes(STREAM_ADDRESS, &stream);
    cpu.memory.set_bytes(0xC000, from);
    cpu.registers.program_counter = PROGRAM_ADDRESS;

    cpu.run();
    assert_eq!(cpu.registers.program_counter, PROGRAM_ADDRESS + 3, "kind={kind:?}");

    let decoded = (0..SCREEN_CHARS)
        .map(|offset| cpu.memory.get_byte(0xC000 + offset as u16))
        .collect::<Vec<_>>();

    let end = STREAM_ADDRESS + stream.len() as u16;
    assert_eq!((end & 0xFF) as u8, cpu.memory.get_byte(0x00FE), "kind={kind:?}");
    assert_eq!((end >> 8) as u8, cpu.memory.get_byte(0x00FF), "kind={kind:?}");

    Ok(decoded)
}

#[test]
fn decodes_every_block_kind() -> AssemblerResult<()> {
    let (from, to) = screens();
    for kind in BlockKind::ALL {
        let decoded = decode(kind, &from, &to)?;
        assert_eq!(decoded.as_slice(), to.as_slice(), "kind={kind:?}");
    }
    Ok(())
}

#[test]
fn partial_height_kinds_ignore_row_24() -> AssemblerResult<()> {
    let mut from = [0x11_u8; SCREEN_CHARS];
    let mut to = from;
    let row_24 = (SCREEN_HEIGHT - 1) * SCREEN_WIDTH + 7;
    to[row_24] = 0x99;

    for kind in BlockKind::ALL {
        if !kind.ignores_last_screen_row() {
            continue;
        }
        let decoded = decode(kind, &from, &to)?;
        assert_eq!(decoded[row_24], from[row_24], "kind={kind:?}");
        assert_eq!(&decoded[..row_24], &to[..row_24], "kind={kind:?}");
        assert_eq!(&decoded[row_24 + 1..], &to[row_24 + 1..], "kind={kind:?}");
    }

    from[row_24] = 0x99;
    assert_eq!(from[row_24], to[row_24]);
    Ok(())
}

#[test]
fn height_five_kinds_write_row_24() -> AssemblerResult<()> {
    let from = [0x11_u8; SCREEN_CHARS];
    let mut to = from;
    to[(SCREEN_HEIGHT - 1) * SCREEN_WIDTH + 39] = 0x42;

    for kind in BlockKind::ALL {
        if kind.ignores_last_screen_row() {
            continue;
        }
        let decoded = decode(kind, &from, &to)?;
        assert_eq!(decoded.as_slice(), to.as_slice(), "kind={kind:?}");
    }
    Ok(())
}

#[test]
fn all_block_opcodes_are_registered() {
    let modules = all_decoder_modules();
    for kind in BlockKind::ALL {
        let name = format!("update_screen_blocks_{}", kind.name());
        assert!(
            modules
                .iter()
                .any(|(opcode, module)| *opcode == kind.opcode() && module.name == name),
            "missing decoder for {kind:?}"
        );
    }
}

#[test]
fn dispatcher_decodes_block_command() -> AssemblerResult<()> {
    let application = ApplicationBuilder::default()
        .include_vic2_defines()
        .module(
            ModuleBuilder::default()
                .instructions(
                    InstructionBuilder::default()
                        .jsr_addr("engine__frame_command__switch")
                        .raw(&[0xFF])
                        .build(),
                )
                .build(),
        )
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
    let program = ProgramGenerator::default().generate(application)?;

    let kind = BlockKind::Row4x5;
    let (from, to) = screens();
    let command = Command::UpdateScreenBlocks(UpdateScreenBlocks::transition(&from, &to, kind));
    let mut stream = vec![0_u8; command.byte_size()];
    command.encode(&mut stream);

    let mut cpu = CPU::new(Memory::new(), Nmos6502);
    cpu.memory
        .set_bytes(0x00FE, &[STREAM_ADDRESS as u8, (STREAM_ADDRESS >> 8) as u8]);
    cpu.memory.set_bytes(PROGRAM_ADDRESS, &program[2..]);
    cpu.memory.set_bytes(STREAM_ADDRESS, &stream);
    cpu.memory.set_bytes(0x3000, &from);
    cpu.registers.program_counter = PROGRAM_ADDRESS;
    cpu.run();

    assert_eq!(cpu.registers.program_counter, PROGRAM_ADDRESS + 3);
    let end = STREAM_ADDRESS + stream.len() as u16;
    assert_eq!(cpu.memory.get_byte(0x00FE), (end & 0xFF) as u8);
    assert_eq!(cpu.memory.get_byte(0x00FF), (end >> 8) as u8);
    let decoded = (0..SCREEN_CHARS)
        .map(|offset| cpu.memory.get_byte(0x3000 + offset as u16))
        .collect::<Vec<_>>();
    assert_eq!(decoded.as_slice(), to.as_slice());
    Ok(())
}

#[test]
fn engine_with_block_decoders_validates() -> AssemblerResult<()> {
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
    ProgramGenerator::default().generate(application)?;
    Ok(())
}
