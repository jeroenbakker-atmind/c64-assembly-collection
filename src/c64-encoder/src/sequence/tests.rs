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
fn mastered_ranges_match_the_planned_sector_cursor() {
    let plan = example_plan();
    let expected_locations: Vec<_> = plan
        .all_locations()
        .unwrap()
        .iter()
        .map(|location| (location.track, location.sector))
        .collect();
    let ranges = plan.mastered_ranges();
    let locations: Vec<_> = ranges
        .iter()
        .flat_map(|range| range.locations.iter().copied())
        .collect();

    assert_eq!(locations, expected_locations);

    let mut offset = 0usize;
    for range in &ranges {
        assert_eq!(range.sectors.len(), range.locations.len());
        for sector in &range.sectors {
            assert_eq!(&sector[..], &plan.sector_image[offset][..]);
            offset += 1;
        }
    }
    assert_eq!(offset, plan.sector_image.len());
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
fn validation_allows_replaying_a_loaded_part_while_another_load_is_pending() {
    let plan = example_plan();
    let sequence = Sequence::new("intro")
        .init()
        .load(plan.id("act1_p1"))
        .repeat_until(Condition::LoadComplete, |body| body.play(plan.id("act1_p0"), 0, 10))
        .wait_loaded()
        .end();
    assert!(SequenceEmitter::new(&plan).emit(&sequence).is_ok());

    let completing_load_in_loop = Sequence::new("bad")
        .init()
        .load(plan.id("act1_p1"))
        .repeat_until(Condition::LoadComplete, |body| body.wait_loaded())
        .end();
    assert!(SequenceEmitter::validate(&completing_load_in_loop).is_err());
}

#[test]
fn stream_part_emits_load_replay_loop_and_wait() {
    let plan = example_plan();
    let sequence = Sequence::new("intro")
        .init()
        .stream_part(plan.id("act1_p1"), plan.id("act1_p0"), 12)
        .end();

    let text = SequenceEmitter::new(&plan).emit_asm_text(&sequence).unwrap();
    assert!(text.contains("    jsr engine__loader__start"));
    assert!(text.contains("seq_stream_0:"));
    assert!(text.contains("    jsr engine__condition__load_complete"));
    assert!(text.contains("    beq seq_stream_0_done"));
    assert!(text.contains("    jsr engine__resource__use"));
    assert!(text.contains("    jsr engine__frame__play_n"));
    assert!(text.contains("    jmp seq_stream_0"));
    assert!(text.contains("seq_stream_0_done:"));
    assert!(text.contains("    jsr engine__loader__wait_loaded"));

    let instructions = SequenceEmitter::new(&plan).emit(&sequence).unwrap();
    assert_eq!(
        jsr_targets(&instructions),
        vec![
            "engine__init",
            "engine__loader__init",
            "engine__condition__init",
            "engine__loader__start",
            "engine__condition__load_complete",
            "engine__resource__use",
            "engine__frame__play_n",
            "engine__loader__wait_loaded",
            "engine__loader__deinit",
            "engine__deinit",
        ]
    );
}

#[test]
fn stream_part_rejects_invalid_sources_and_pending_loads() {
    let self_stream = Sequence::new("bad").init().stream_part(0, 0, 1).end();
    assert!(SequenceEmitter::validate(&self_stream).is_err());

    let zero_frames = Sequence::new("bad").init().stream_part(1, 0, 0).end();
    assert!(SequenceEmitter::validate(&zero_frames).is_err());

    let pending_load = Sequence::new("bad").init().load(0).stream_part(1, 0, 1).end();
    assert!(SequenceEmitter::validate(&pending_load).is_err());

    let plan = example_plan();
    let unknown = Sequence::new("bad").init().stream_part(99, plan.id("act1_p0"), 1).end();
    assert!(SequenceEmitter::new(&plan).emit(&unknown).is_err());
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
fn condition_loops_emit_labels_condition_jsrs_and_back_edges() {
    let plan = example_plan();
    let disk_sequence = Sequence::new("wait_disk")
        .init()
        .repeat_until(Condition::DiskTurned, |body| body.play(plan.id("act1_p0"), 0, 10))
        .end();

    let disk_text = SequenceEmitter::new(&plan).emit_asm_text(&disk_sequence).unwrap();
    assert!(disk_text.contains("seq_until_0:"));
    assert!(disk_text.contains("    jsr engine__condition__disk_turned"));
    assert!(disk_text.contains("    beq seq_until_0_done"));
    assert!(disk_text.contains("    jmp seq_until_0"));
    assert!(disk_text.contains("seq_until_0_done:"));

    let disk_instructions = SequenceEmitter::new(&plan).emit(&disk_sequence).unwrap();
    let targets = jsr_targets(&disk_instructions);
    assert!(targets.contains(&"engine__condition__disk_turned".to_string()));
    assert!(targets.contains(&"engine__resource__use".to_string()));
    assert!(targets.contains(&"engine__frame__play_n".to_string()));

    let music_sequence = Sequence::new("wait_music")
        .init()
        .repeat_until(Condition::MusicBefore(7), |body| body.play(plan.id("act1_p0"), 0, 10))
        .end();

    let music_text = SequenceEmitter::new(&plan).emit_asm_text(&music_sequence).unwrap();
    assert!(music_text.contains("seq_until_0:"));
    assert!(music_text.contains("    lda #7"));
    assert!(music_text.contains("    jsr engine__condition__music_before"));
    assert!(music_text.contains("    beq seq_until_0_done"));
    assert!(music_text.contains("    jmp seq_until_0"));
    assert!(music_text.contains("seq_until_0_done:"));

    let load_sequence = Sequence::new("wait_load")
        .init()
        .repeat_until(Condition::LoadComplete, |body| body.play(plan.id("act1_p0"), 0, 10))
        .end();
    let load_text = SequenceEmitter::new(&plan).emit_asm_text(&load_sequence).unwrap();
    assert!(load_text.contains("    jsr engine__condition__load_complete"));
    assert!(load_text.contains("    beq seq_until_0_done"));
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
fn stream_part_reports_lead_frames_for_the_next_resource() {
    let mut plan = example_plan();
    let sequence = Sequence::new("intro")
        .init()
        .stream_part(plan.id("music0"), plan.id("act1_p0"), 12)
        .end();

    plan.annotate_timing(&sequence);
    assert_eq!(plan.reports[2].lead_frames, Some(12));
    assert!(plan.reports[2].margin_ms.is_some());
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
