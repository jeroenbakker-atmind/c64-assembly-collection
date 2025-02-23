use std::iter::repeat_n;

use c64_assembler::{
    builder::{ApplicationBuilder, InstructionBuilder, ModuleBuilder},
    generator::{Generator, ProgramGenerator},
    validator::{AssemblerResult, Validator},
};
use c64_encoder::command::{
    modules::{CurrentPTR, ScreenCharPTR},
    update_screen_chars_rle::{
        UpdateScreenCharsRLE, RLE_MASK_AUTO_INCREMENT, RLE_MASK_SKIP_VALUES, RLE_MASK_UPDATE_VALUES,
        RLE_MASK_UPDATE_WITH_SINGLE_VALUE,
    },
    DecoderModule,
};
use mos6502::{
    cpu::CPU,
    instruction::{Instruction, Nmos6502},
    memory::{Bus, Memory},
};

fn build_program() -> AssemblerResult<Vec<u8>> {
    let application = ApplicationBuilder::default()
        .define_address("CURRENT_PTR", 0xFE)
        .define_address("SCREEN_CHAR_PTR", 0xFC)
        .define_address("SCREEN_CHARS_PAGE0", 0xC000)
        .define_address("SCRATCH_SPACE_00", 0xFB)
        .include_vic2_defines()
        .module(
            ModuleBuilder::default()
                .instructions(
                    InstructionBuilder::default()
                        .jsr_addr("update_screen_chars_rle__process")
                        .raw(&[0xFF])
                        .build(),
                )
                .build(),
        )
        .module(UpdateScreenCharsRLE::module())
        .module(CurrentPTR::module())
        .module(ScreenCharPTR::module())
        .build()?;

    application.validate()?;

    ProgramGenerator::default().generate(application)
}

fn run_debug(cpu: &mut CPU<Memory, Nmos6502>) {
    let mut prev_pc = cpu.registers.program_counter;
    let mut indent = 0;
    while let Some(decoded_instr) = cpu.fetch_next_and_decode() {
        let registers = cpu.registers.clone();
        cpu.execute_instruction(decoded_instr);
        println!(
            "{:04X}: {}{:?} a={}, x={}, y={}, sp={}",
            prev_pc,
            repeat_n("  ", indent).collect::<String>(),
            decoded_instr,
            registers.accumulator,
            registers.index_x,
            registers.index_y,
            registers.stack_pointer.0
        );

        match decoded_instr.0 {
            Instruction::JSR => {
                indent += 1;
            }
            Instruction::RTS => {
                indent -= 1;
            }
            _ => {}
        }
        prev_pc = registers.program_counter;
    }

    for address in 0xc000..0xc010 {
        let byte = cpu.memory.get_byte(address);
        println!("{:04X}: {:02X}", address, byte);
    }
}

#[test]
fn screen_char_rle_skip_values() -> AssemblerResult<()> {
    let decode_stream = vec![
        // One packet
        0x01,
        // Packet: Skip 4
        RLE_MASK_SKIP_VALUES | 0x04,
    ];
    let program = build_program()?;

    let mut cpu = CPU::new(Memory::new(), Nmos6502);
    cpu.memory.set_bytes(0x00FE, &[0x00, 0x04]);
    cpu.memory.set_bytes(0x0800, &program[2..]);
    cpu.registers.program_counter = 0x0800;

    cpu.memory.set_bytes(0x0401, &decode_stream);
    cpu.memory.set_bytes(0xC000, &vec![0xDE; 1024]);

    run_debug(&mut cpu);

    assert_eq!(0x04, cpu.memory.get_byte(0x00FC));
    assert_eq!(0xC0, cpu.memory.get_byte(0x00FD));
    assert_eq!(0x03, cpu.memory.get_byte(0x00FE));
    assert_eq!(0x04, cpu.memory.get_byte(0x00FF));

    assert_eq!(0xDE, cpu.memory.get_byte(0xC000));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC001));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC002));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC003));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC004));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC005));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC006));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC007));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC008));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC009));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00A));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00B));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00C));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00D));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00E));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00F));

    Ok(())
}

#[test]
fn screen_char_rle_single_value() -> AssemblerResult<()> {
    let decode_stream = vec![
        // One packet
        0x01,
        // Packet: Update 4 values with 0xAD
        RLE_MASK_UPDATE_WITH_SINGLE_VALUE | 0x04,
        0xAD,
    ];
    let program = build_program()?;

    let mut cpu = CPU::new(Memory::new(), Nmos6502);
    cpu.memory.set_bytes(0x00FE, &[0x00, 0x04]);
    cpu.memory.set_bytes(0x0800, &program[2..]);
    cpu.registers.program_counter = 0x0800;

    cpu.memory.set_bytes(0x0401, &decode_stream);
    cpu.memory.set_bytes(0xC000, &vec![0xDE; 1024]);

    run_debug(&mut cpu);

    assert_eq!(0x04, cpu.memory.get_byte(0x00FC));
    assert_eq!(0xC0, cpu.memory.get_byte(0x00FD));
    assert_eq!(0x04, cpu.memory.get_byte(0x00FE));
    assert_eq!(0x04, cpu.memory.get_byte(0x00FF));

    assert_eq!(0xAD, cpu.memory.get_byte(0xC000));
    assert_eq!(0xAD, cpu.memory.get_byte(0xC001));
    assert_eq!(0xAD, cpu.memory.get_byte(0xC002));
    assert_eq!(0xAD, cpu.memory.get_byte(0xC003));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC004));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC005));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC006));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC007));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC008));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC009));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00A));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00B));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00C));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00D));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00E));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00F));

    Ok(())
}

#[test]
fn screen_char_rle_auto_increment() -> AssemblerResult<()> {
    let decode_stream = vec![
        // One packet
        0x01,
        RLE_MASK_AUTO_INCREMENT | 0x04,
        0xAD,
    ];
    let program = build_program()?;

    let mut cpu = CPU::new(Memory::new(), Nmos6502);
    cpu.memory.set_bytes(0x00FE, &[0x00, 0x04]);
    cpu.memory.set_bytes(0x0800, &program[2..]);
    cpu.registers.program_counter = 0x0800;

    cpu.memory.set_bytes(0x0401, &decode_stream);
    cpu.memory.set_bytes(0xC000, &vec![0xDE; 1024]);

    run_debug(&mut cpu);

    assert_eq!(0x04, cpu.memory.get_byte(0x00FC));
    assert_eq!(0xC0, cpu.memory.get_byte(0x00FD));
    assert_eq!(0x04, cpu.memory.get_byte(0x00FE));
    assert_eq!(0x04, cpu.memory.get_byte(0x00FF));

    assert_eq!(0xAD, cpu.memory.get_byte(0xC000));
    assert_eq!(0xAE, cpu.memory.get_byte(0xC001));
    assert_eq!(0xAF, cpu.memory.get_byte(0xC002));
    assert_eq!(0xB0, cpu.memory.get_byte(0xC003));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC004));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC005));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC006));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC007));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC008));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC009));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00A));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00B));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00C));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00D));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00E));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00F));

    Ok(())
}

#[test]
fn screen_char_rle_values() -> AssemblerResult<()> {
    let decode_stream = vec![
        // One packet
        0x01,
        RLE_MASK_UPDATE_VALUES | 0x04,
        0xDA,
        0xDA,
        0xA4,
        0x4A,
    ];
    let program = build_program()?;

    let mut cpu = CPU::new(Memory::new(), Nmos6502);
    cpu.memory.set_bytes(0x00FE, &[0x00, 0x04]);
    cpu.memory.set_bytes(0x0800, &program[2..]);
    cpu.registers.program_counter = 0x0800;

    cpu.memory.set_bytes(0x0401, &decode_stream);
    cpu.memory.set_bytes(0xC000, &vec![0xDE; 1024]);

    run_debug(&mut cpu);

    assert_eq!(0x04, cpu.memory.get_byte(0x00FC));
    assert_eq!(0xC0, cpu.memory.get_byte(0x00FD));
    assert_eq!(0x07, cpu.memory.get_byte(0x00FE));
    assert_eq!(0x04, cpu.memory.get_byte(0x00FF));

    assert_eq!(0xDA, cpu.memory.get_byte(0xC000));
    assert_eq!(0xDA, cpu.memory.get_byte(0xC001));
    assert_eq!(0xA4, cpu.memory.get_byte(0xC002));
    assert_eq!(0x4A, cpu.memory.get_byte(0xC003));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC004));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC005));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC006));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC007));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC008));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC009));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00A));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00B));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00C));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00D));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00E));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00F));

    Ok(())
}

#[test]
fn screen_char_rle_skip_and_single_value() -> AssemblerResult<()> {
    let decode_stream = vec![
        // One packet
        0x02,
        RLE_MASK_SKIP_VALUES | 0x05,
        RLE_MASK_UPDATE_WITH_SINGLE_VALUE | 0x03,
        0xAD,
    ];
    let program = build_program()?;

    let mut cpu = CPU::new(Memory::new(), Nmos6502);
    cpu.memory.set_bytes(0x00FE, &[0x00, 0x04]);
    cpu.memory.set_bytes(0x0800, &program[2..]);
    cpu.registers.program_counter = 0x0800;

    cpu.memory.set_bytes(0x0401, &decode_stream);
    cpu.memory.set_bytes(0xC000, &vec![0xDE; 1024]);

    run_debug(&mut cpu);

    assert_eq!(0x08, cpu.memory.get_byte(0x00FC));
    assert_eq!(0xC0, cpu.memory.get_byte(0x00FD));
    assert_eq!(0x05, cpu.memory.get_byte(0x00FE));
    assert_eq!(0x04, cpu.memory.get_byte(0x00FF));

    assert_eq!(0xDE, cpu.memory.get_byte(0xC000));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC001));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC002));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC003));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC004));
    assert_eq!(0xAD, cpu.memory.get_byte(0xC005));
    assert_eq!(0xAD, cpu.memory.get_byte(0xC006));
    assert_eq!(0xAD, cpu.memory.get_byte(0xC007));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC008));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC009));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00A));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00B));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00C));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00D));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00E));
    assert_eq!(0xDE, cpu.memory.get_byte(0xC00F));

    Ok(())
}
