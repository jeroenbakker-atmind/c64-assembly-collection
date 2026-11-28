use c64_assembler::{
    builder::{ApplicationBuilder, InstructionBuilder, ModuleBuilder},
    generator::{Generator, ProgramGenerator},
    validator::{AssemblerResult, Validator},
    Application,
};
use c64_encoder::{
    builder::demo::DemoBuilder,
    command::modules::EngineBuilder,
    sequence::{MasterPlanner, ResourceId, ResourcePlan, Sequence, SequenceEmitter},
};
use cube_encoder::encode_act_demo;

/// Refreshes to wait after reset before the first disk command. A real 1541
/// runs a long RAM self-test before it answers ATN; `engine__refresh__wait_n`
/// consumes four raster refreshes per call, so 20 calls allow ~1.6 s on PAL.
const DRIVE_READY_REFRESHES: u8 = 20;

/// The generated program must end before the bank-0 charset at `$2800`.
const PROGRAM_START: u16 = 0x0800;
const PROGRAM_LIMIT: u16 = 0x2800;

/// Everything the builder needs from the intro build: the PRG plus the
/// immutable master plan needed by disk mastering.
pub struct IntroBuild {
    pub prg: Vec<u8>,
    pub plan: ResourcePlan,
}

#[allow(dead_code)] // Kept as the simple PRG entry point for callers that do not need the plan.
pub fn intro_application() -> AssemblerResult<Vec<u8>> {
    Ok(build_intro()?.prg)
}

/// Encode the rendered cube acts and turn them into a scripted sequence.
pub fn build_intro() -> AssemblerResult<IntroBuild> {
    let act1 = encode_act_demo(1, 100);
    let act2 = encode_act_demo(2, 100);
    build_intro_from_demos(&act1.demo, &act2.demo)
}

fn build_intro_from_demos(act1: &DemoBuilder, act2: &DemoBuilder) -> AssemblerResult<IntroBuild> {
    let plan = build_plan(act1, act2);
    let sequence = build_sequence(&plan);
    let application = build_application(&plan, &sequence)?;
    application.validate()?;
    let prg = ProgramGenerator::default().generate(application)?;
    let program_end = PROGRAM_START + (prg.len() - 2) as u16;
    assert!(
        program_end <= PROGRAM_LIMIT,
        "intro program ends at ${program_end:04X}; charset starts at ${PROGRAM_LIMIT:04X}"
    );

    for report in &plan.reports {
        println!(
            "resource {:<12} id={:<2} frames={:<3} bytes={:<5} sectors={:<3} track={:<2} sector={:<2} ${:04X}",
            report.name,
            report.resource_id,
            report.frame_count.unwrap_or_default(),
            report.bytes,
            report.sectors,
            report.start.track,
            report.start.sector,
            report.dest,
        );
    }

    Ok(IntroBuild { prg, plan })
}

fn build_plan(act1: &DemoBuilder, act2: &DemoBuilder) -> ResourcePlan {
    let mut planner = MasterPlanner::with_defaults().expect("invalid master planner defaults");
    planner
        .add_demo_resources(1, "act1", act1)
        .expect("failed to plan act 1 frame resources");
    planner
        .add_demo_resources(2, "act2", act2)
        .expect("failed to plan act 2 frame resources");
    planner.build().expect("failed to build intro resource plan")
}

#[derive(Debug, Clone, Copy)]
struct FramePart {
    id: ResourceId,
    frames: u16,
}

fn build_sequence(plan: &ResourcePlan) -> Sequence {
    let mut parts = frame_parts(plan, "act1_p");
    parts.extend(frame_parts(plan, "act2_p"));

    let mut sequence = Sequence::new("intro_main").init().wait_refreshes(DRIVE_READY_REFRESHES);
    sequence = append_parts(sequence, &parts);
    sequence.end()
}

fn frame_parts(plan: &ResourcePlan, prefix: &str) -> Vec<FramePart> {
    plan.resources
        .iter()
        .filter(|resource| resource.name.starts_with(prefix))
        .filter_map(|resource| {
            resource.frame.map(|frame| FramePart {
                id: resource.id,
                frames: frame.frame_count(),
            })
        })
        .collect()
}

/// Loads one part at a time. `stream_part` owns the whole interleave: it starts
/// the next async load, replays the previous part until the load completes, and
/// waits for completion before returning. The final part plays on its own.
fn append_parts(sequence: Sequence, parts: &[FramePart]) -> Sequence {
    let Some((first, rest)) = parts.split_first() else {
        return sequence;
    };

    let mut sequence = sequence.load(first.id).wait_loaded();
    let mut previous = first;
    for next in rest {
        sequence = sequence.stream_part(next.id, previous.id, previous.frames);
        previous = next;
    }
    sequence.play(previous.id, 0, previous.frames)
}

fn build_application(plan: &ResourcePlan, sequence: &Sequence) -> AssemblerResult<Application> {
    let sequence_instructions = SequenceEmitter::new(plan)
        .emit(sequence)
        .expect("invalid scripted intro sequence");

    let main = ModuleBuilder::default()
        .name("main")
        .instructions(
            InstructionBuilder::default()
                .add_basic_header()
                .label("main_entry_point")
                .jsr_addr("intro_main")
                .rts()
                .build(),
        )
        .build();

    let sequence_module = ModuleBuilder::default()
        .name("intro_sequence")
        .instructions(sequence_instructions)
        .build();

    let resource_module = ModuleBuilder::default()
        .name("resource_data")
        .instructions(
            InstructionBuilder::default()
                .label("resource_table")
                .raw(&plan.encode_resource_table())
                .build(),
        )
        .build();

    // `add_engine` still installs the legacy stream entry points for other
    // applications. They need a resolvable `engine_data` label even though the
    // scripted sequence never calls them.
    let legacy_stream_placeholder = ModuleBuilder::default()
        .name("legacy_stream_placeholder")
        .instructions(InstructionBuilder::default().label("engine_data").raw(&[0, 0]).build())
        .build();

    let mut builder = ApplicationBuilder::default();
    builder.name("Intro").include_vic2_defines();
    for (name, value) in plan.constants.iter() {
        builder.define_address(name, *value);
    }

    Ok(builder
        .module(main)
        .module(sequence_module)
        .module(resource_module)
        .module(legacy_stream_placeholder)
        .add_engine()
        .build()?)
}

#[cfg(test)]
mod tests {
    use c64_assembler::generator::DasmGenerator;
    use c64_encoder::{builder::frame::FrameBuilder, command::update_text_mode_screen::UpdateTextModeScreen};

    use super::*;

    fn demo_with_frames(count: usize) -> DemoBuilder {
        let mut demo = DemoBuilder::default();
        for _ in 0..count {
            let mut frame = FrameBuilder::default();
            frame.update_text_mode_screen(UpdateTextModeScreen::filled(0x20));
            demo.frame(frame.build());
        }
        demo
    }

    #[test]
    fn emits_sequence_resource_table_and_constants_without_legacy_stream_call() {
        let plan = build_plan(&demo_with_frames(3), &demo_with_frames(1));
        let sequence = build_sequence(&plan);
        let application = build_application(&plan, &sequence).unwrap();
        application.validate().unwrap();
        let source = DasmGenerator::default().generate(application.clone()).unwrap();

        assert!(source.contains("intro_main:"));
        assert!(source.contains("jsr engine__loader__start"));
        assert!(source.contains("jsr engine__loader__wait_loaded"));
        assert!(source.contains("jsr engine__resource__use"));
        assert!(source.contains("jsr engine__frame__play_n"));
        assert!(source.contains("resource_table:"));
        assert!(application.defines.iter().any(|define| define.name == "R_ACT1_P0"));
        assert!(application
            .defines
            .iter()
            .any(|define| define.name == format!("FRAMES_ACT1_P0")));
        assert!(source.contains("  jsr intro_main"));
        assert!(!source.contains("  jsr engine__stream__init"));
        assert!(!source.contains("  jsr engine__play_stream"));

        let table = plan.encode_resource_table();
        assert_eq!(table.len(), plan.resources.len() * 7);
        assert_eq!(table[0], 1);
        assert_eq!(table[1], 1);
    }

    #[test]
    fn sequence_pipeline_plays_the_previous_part_while_the_next_loads() {
        let plan = build_plan(&demo_with_frames(5), &demo_with_frames(1));
        let steps = format!("{:?}", build_sequence(&plan).steps);
        let p0 = plan.id("act1_p0");
        let p1 = plan.id("act1_p1");
        let p2 = plan.id("act2_p0");
        let p0_frames = plan.frames("act1_p0");
        let p1_frames = plan.frames("act1_p1");
        let p2_frames = plan.frames("act2_p0");

        let load0 = format!("Load({p0})");
        let stream01 = format!("StreamPart {{ next: {p1}, previous: {p0}, frames: {p0_frames} }}");
        let stream12 = format!("StreamPart {{ next: {p2}, previous: {p1}, frames: {p1_frames} }}");
        let play2 = format!("PlayFrames {{ resource: {p2}, offset: 0, count: {p2_frames} }}");

        assert!(steps.find(&load0).unwrap() < steps.find(&stream01).unwrap());
        assert!(steps.find(&stream01).unwrap() < steps.find(&stream12).unwrap());
        assert!(steps.find(&stream12).unwrap() < steps.find(&play2).unwrap());
        assert!(steps.contains(&stream01));
        assert!(steps.contains(&stream12));
        assert!(!steps.contains("RepeatUntil { condition: LoadComplete"));
    }
}
