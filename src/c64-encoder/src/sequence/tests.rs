use std::collections::HashSet;

use c64_assembler::{instruction::operation::Operation, memory::address_mode::AddressMode, Instructions};

use crate::builder::{demo::DemoBuilder, frame::FrameBuilder};

use super::{
    model::{Condition, Sequence},
    plan::{MasterPlanner, MasterPlannerConfig, ResourcePlan},
    report::ResourceReport,
    SequenceEmitter,
};

fn example_plan() -> ResourcePlan {
    let mut planner = MasterPlanner::with_defaults().unwrap();
    planner
        .add_frame_resource("act1_p0", 1, 1, 40, vec![0xAA; 4096])
        .unwrap();
    planner
        .add_frame_resource("act1_p1", 1, 41, 80, vec![0xBB; 2048])
        .unwrap();
    planner.add_blob("music0", vec![0xCC; 6144]).unwrap();
    planner.build().unwrap()
}

fn jsr_targets(instructions: &Instructions) -> Vec<String> {
    instructions
        .instructions
        .iter()
        .filter_map(
            |instruction| match (&instruction.operation, &instruction.address_mode) {
                (Operation::JSR, AddressMode::Absolute(reference)) => Some(reference.name.clone()),
                _ => None,
            },
        )
        .collect()
}

#[test]
fn sector_counts_and_destinations_are_derived_from_encoded_sizes() {
    let plan = example_plan();

    assert_eq!(plan.ranges.len(), 3);
    assert_eq!(plan.ranges[0].sector_count, 16);
    assert_eq!(plan.ranges[1].sector_count, 8);
    assert_eq!(plan.ranges[2].sector_count, 24);
    assert_eq!(plan.ranges[0].payload_bytes, 4096);
    assert_eq!(plan.ranges[0].dest, 0x4000);
    assert_eq!(plan.ranges[1].dest, 0x5000);
    assert_eq!(plan.ranges[2].dest, 0xA000);
    assert_eq!(plan.constants.get("FRAMES_ACT1_P0"), Some(40));
    assert_eq!(plan.constants.get("SECTORS_MUSIC0"), Some(24));
}

#[test]
fn resource_table_has_seven_bytes_per_resource() {
    let plan = example_plan();
    let table = plan.encode_resource_table();

    assert_eq!(table.len(), 21);
    assert_eq!(&table[0..7], &[1, 1, 16, 0x00, 0x40, 0x00, 0x10]);
    assert_eq!(&table[7..14], &[1, 13, 8, 0x00, 0x50, 0x00, 0x08]);
    assert_eq!(&table[14..21], &[2, 9, 24, 0x00, 0xA0, 0x00, 0x18]);
}

#[test]
fn geometry_skips_track_18_and_sector_zero() {
    let plan = example_plan();
    for location in plan.ranges.iter().map(|range| range.start) {
        assert_ne!(location.track, 18);
        assert_ne!(location.sector, 0);
    }
}

#[test]
fn sector_cursor_visits_every_usable_sector_exactly_once() {
    let mut planner = MasterPlanner::with_defaults().unwrap();
    for index in 0..40 {
        let bytes = if index < 39 { 4096 } else { 1536 };
        planner
            .add_frame_resource(
                format!("chunk{index}"),
                1,
                index as u16 * 10 + 1,
                index as u16 * 10 + 10,
                vec![index as u8; bytes],
            )
            .unwrap();
    }
    let plan = planner.build().unwrap();
    let locations = plan.all_locations().unwrap();
    let unique: HashSet<_> = locations.iter().collect();

    assert_eq!(locations.len(), 630);
    assert_eq!(unique.len(), locations.len());
    assert!(locations.iter().all(|location| location.track != 18));
    assert!(locations.iter().all(|location| location.sector != 0));
}

#[test]
fn bank_out_requirement_is_derived_from_the_music_region() {
    let plan = example_plan();
    assert_eq!(plan.constants.get("BANK_OUT_REQUIRED"), Some(1));
    assert_eq!(plan.constants.get("CONDITION_STATUS_BASE"), Some(0x3E00));
    assert_eq!(plan.ranges[2].dest, 0xA000);
    assert_eq!(
        plan.resources[2].dest as u32 + plan.resources[2].encoded_bytes.len() as u32,
        0xB800
    );
}

#[test]
fn sequence_names_stay_stable_when_sizes_change() {
    let small = example_plan();
    let mut planner = MasterPlanner::with_defaults().unwrap();
    planner
        .add_frame_resource("act1_p0", 1, 1, 40, vec![0xAA; 2048])
        .unwrap();
    planner
        .add_frame_resource("act1_p1", 1, 41, 80, vec![0xBB; 2048])
        .unwrap();
    planner.add_blob("music0", vec![0xCC; 6144]).unwrap();
    let large = planner.build().unwrap();

    assert_eq!(small.id("act1_p0"), large.id("act1_p0"));
    assert_eq!(small.constants.get("R_ACT1_P0"), large.constants.get("R_ACT1_P0"));
    assert_ne!(
        small.constants.get("SECTORS_ACT1_P0"),
        large.constants.get("SECTORS_ACT1_P0")
    );
}

#[test]
fn validation_accepts_balanced_sequence() {
    let plan = example_plan();
    let sequence = Sequence::new("intro")
        .init()
        .load(plan.id("music0"))
        .wait_loaded()
        .load(plan.id("act1_p0"))
        .wait_loaded()
        .play(plan.id("act1_p0"), 0, plan.frames("act1_p0"))
        .end();

    assert!(SequenceEmitter::new(&plan).emit(&sequence).is_ok());
}

#[test]
fn validation_rejects_unbalanced_loads() {
    let double_load = Sequence::new("bad").init().load(0).load(1).wait_loaded().end();
    assert!(SequenceEmitter::validate(&double_load).is_err());

    let wait_without_load = Sequence::new("bad").init().wait_loaded().end();
    assert!(SequenceEmitter::validate(&wait_without_load).is_err());

    let pending_at_end = Sequence::new("bad").init().load(0).end();
    assert!(SequenceEmitter::validate(&pending_at_end).is_err());

    let play_active_load = Sequence::new("bad").init().load(0).play(0, 0, 1).wait_loaded().end();
    assert!(SequenceEmitter::validate(&play_active_load).is_err());

    let leaking_loop = Sequence::new("bad").init().repeat_count(2, |body| body.load(0)).end();
    assert!(SequenceEmitter::validate(&leaking_loop).is_err());
}

#[test]
fn emitter_emits_expected_runtime_primitives() {
    let plan = example_plan();
    let sequence = Sequence::new("intro_main")
        .init()
        .load(plan.id("music0"))
        .wait_loaded()
        .load(plan.id("act1_p0"))
        .play(plan.id("act1_p1"), 0, plan.frames("act1_p1"))
        .wait_loaded()
        .wait_refreshes(5)
        .end();

    let instructions = SequenceEmitter::new(&plan).emit(&sequence).unwrap();
    let targets = jsr_targets(&instructions);
    assert_eq!(
        targets,
        vec![
            "engine__init",
            "engine__loader__init",
            "engine__condition__init",
            "engine__loader__start",
            "engine__loader__wait_loaded",
            "engine__loader__start",
            "engine__resource__use",
            "engine__frame__play_n",
            "engine__loader__wait_loaded",
            "engine__refresh__wait_n",
            "engine__loader__deinit",
            "engine__deinit",
        ]
    );
    assert!(!targets.iter().any(|target| target.contains("interpreter")));
}

#[test]
fn counted_loops_emit_labels_and_branches() {
    let plan = example_plan();
    let sequence = Sequence::new("title")
        .init()
        .repeat_count(3, |body| body.play(plan.id("act1_p0"), 0, 10))
        .end();

    let text = SequenceEmitter::new(&plan).emit_asm_text(&sequence).unwrap();
    assert!(text.contains("ldx #3"));
    assert!(text.contains("seq_repeat_0:"));
    assert!(text.contains("    txa"));
    assert!(text.contains("    pha"));
    assert!(text.contains("    pla"));
    assert!(text.contains("    tax"));
    assert!(text.contains("bne seq_repeat_0"));
}

#[test]
fn condition_loops_are_accepted_by_the_model_but_not_phase_one_emitter() {
    let plan = example_plan();
    let sequence = Sequence::new("wait")
        .init()
        .repeat_until(Condition::DiskTurned, |body| body.play(plan.id("act1_p0"), 0, 10))
        .end();

    let error = SequenceEmitter::new(&plan).emit(&sequence).unwrap_err();
    assert!(matches!(error, super::EmitError::UnsupportedCondition(_)));
    assert!(SequenceEmitter::validate(&sequence).is_ok());
}

#[test]
fn reports_get_lead_frames_from_the_sequence() {
    let mut plan = example_plan();
    let sequence = Sequence::new("intro")
        .init()
        .load(plan.id("music0"))
        .wait_loaded()
        .load(plan.id("act1_p0"))
        .play(plan.id("act1_p1"), 0, 20)
        .play(plan.id("act1_p1"), 0, 11)
        .wait_loaded()
        .end();

    plan.annotate_timing(&sequence);
    assert_eq!(plan.reports[0].lead_frames, Some(31));
    assert!(plan.reports[0].margin_ms.is_some());
    assert!(ResourceReport::markdown_table(&plan.reports).contains("music0"));
}

#[test]
fn demo_resources_split_on_frame_and_slot_boundaries() {
    let mut config = MasterPlannerConfig::default();
    config.slot_size = 32;
    let mut planner = MasterPlanner::new(config).unwrap();
    let mut demo = DemoBuilder::default();
    for _ in 0..12 {
        demo.frame(FrameBuilder::default().clear_screen_chars(0).build());
    }

    let ids = planner.add_demo_resources(1, "act1", &demo).unwrap();
    let plan = planner.build().unwrap();
    assert!(ids.len() > 1);
    assert_eq!(plan.resources[0].frame.unwrap().first, 1);
    assert_eq!(plan.resources[0].name, "act1_p0");
    assert!(plan.resources.iter().all(|resource| resource.slot.is_some()));
}

#[test]
fn plan_hash_is_deterministic_and_changes_with_payload() {
    let first = example_plan();
    let second = example_plan();
    let mut planner = MasterPlanner::with_defaults().unwrap();
    planner
        .add_frame_resource("act1_p0", 1, 1, 40, vec![0xAA; 4096])
        .unwrap();
    planner
        .add_frame_resource("act1_p1", 1, 41, 80, vec![0xBB; 2048])
        .unwrap();
    planner.add_blob("music0", vec![0xCC; 3072]).unwrap();
    let changed = planner.build().unwrap();

    assert_eq!(first.plan_hash(), second.plan_hash());
    assert_ne!(first.plan_hash(), changed.plan_hash());
}

#[test]
fn slot_lifetime_validation_rejects_overwriting_playing_frames() {
    let mut planner = MasterPlanner::with_defaults().unwrap();
    for index in 0..7 {
        planner
            .add_frame_resource(
                format!("chunk{index}"),
                1,
                index * 10 + 1,
                index * 10 + 10,
                vec![index as u8; 64],
            )
            .unwrap();
    }
    let plan = planner.build().unwrap();
    assert_eq!(plan.resources[6].slot, plan.resources[0].slot);

    let bad = Sequence::new("bad")
        .init()
        .play(plan.id("chunk0"), 0, 10)
        .load(plan.id("chunk6"))
        .wait_loaded()
        .play(plan.id("chunk6"), 0, 10)
        .end();
    assert!(plan.validate_sequence_slots(&bad).is_err());

    let good = Sequence::new("good")
        .init()
        .play(plan.id("chunk0"), 0, 10)
        .load(plan.id("chunk1"))
        .wait_loaded()
        .play(plan.id("chunk1"), 0, 10)
        .end();
    assert!(plan.validate_sequence_slots(&good).is_ok());
}
