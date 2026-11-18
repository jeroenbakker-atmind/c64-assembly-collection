use std::collections::HashMap;

use c64::markdown::table::{ColumnAlign, MarkdownTable};
use c64_encoder::{
    builder::demo::DemoBuilder,
    command::{
        update_screen_blocks::BlockKind, CLEAR_SCREEN_CHAR, PARTIAL_UPDATE_TEXT_MODE_SCREEN, SET_BORDER_COLOR,
        SET_PALETTE4, UPDATE_CHARS_RANGED_U16, UPDATE_CHARS_U16, UPDATE_SCREEN_CHARS_RLE, UPDATE_TEXT_MODE_SCREEN,
    },
    encoder::Encoder,
};

use crate::checks::char_redundancy::CharRedundancyStats;

#[derive(Debug, Clone, Default)]
pub struct CommandUse {
    pub count: usize,
    pub bytes: usize,
}

/// Statistics for the commands emitted in one act's final stream.
#[derive(Debug, Clone, Default)]
pub struct CommandSummary {
    pub act: u32,
    pub strategy: String,
    pub frames: usize,
    pub commands: usize,
    pub stream_bytes: usize,
    /// Keyed by opcode so the report cannot drift from the encoded stream.
    pub usage: HashMap<u8, CommandUse>,
    pub redundancy: CharRedundancyStats,
}

pub fn fmt_num(value: usize) -> String {
    let digits = value.to_string();
    let mut result = String::with_capacity(digits.len() + digits.len() / 3);
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index) % 3 == 0 {
            result.push(',');
        }
        result.push(digit);
    }
    result
}

pub fn build_command_summary(act: u32, strategy: &str, demo: &DemoBuilder, stream_bytes: usize) -> CommandSummary {
    let mut summary = CommandSummary {
        act,
        strategy: strategy.to_string(),
        frames: demo.frames.len(),
        stream_bytes,
        ..CommandSummary::default()
    };

    for frame in &demo.frames {
        for command in &frame.commands {
            let usage = summary.usage.entry(command.opcode()).or_default();
            usage.count += 1;
            usage.bytes += command.byte_size();
            summary.commands += 1;
        }
    }

    summary.redundancy = CharRedundancyStats::scan(demo);
    summary
}

/// All command opcodes in stream order, with the labels used by the encoder reports.
fn command_catalog() -> Vec<(u8, String)> {
    let mut result = vec![
        (CLEAR_SCREEN_CHAR, "clear_screen_chars".to_string()),
        (SET_PALETTE4, "set_palette4".to_string()),
        (SET_BORDER_COLOR, "set_border_color".to_string()),
        (UPDATE_CHARS_U16, "update_chars_u16".to_string()),
        (UPDATE_CHARS_RANGED_U16, "update_chars_ranged_u16".to_string()),
        (UPDATE_TEXT_MODE_SCREEN, "full".to_string()),
        (PARTIAL_UPDATE_TEXT_MODE_SCREEN, "partial".to_string()),
        (UPDATE_SCREEN_CHARS_RLE, "rle".to_string()),
    ];
    result.extend(
        BlockKind::ALL
            .iter()
            .map(|kind| (kind.opcode(), format!("block:{}", kind.name()))),
    );
    result
}

fn redundancy_row(label: &str, stats: &CharRedundancyStats, unique_glyphs: Option<usize>) -> Vec<String> {
    vec![
        label.to_string(),
        fmt_num(stats.uploads()),
        unique_glyphs.map(fmt_num).unwrap_or_else(|| "-".to_string()),
        fmt_num(stats.first_uploads),
        fmt_num(stats.reuploads_after_eviction),
        fmt_num(stats.duplicate_while_resident),
        fmt_num(stats.same_slot_rewrites),
        fmt_num(stats.redundant_uploads()),
        fmt_num(stats.payload_bytes()),
        fmt_num(stats.command_overhead_bytes),
        fmt_num(stats.redundant_bytes()),
        format!("{:.2}%", stats.redundant_percentage()),
    ]
}

pub fn print_command_summary(summaries: &[CommandSummary]) {
    println!();
    println!("## Command summary");
    println!();

    let mut stream_table = MarkdownTable::new(
        &["act", "strategy", "frames", "commands", "stream bytes"],
        &[
            ColumnAlign::Right,
            ColumnAlign::Left,
            ColumnAlign::Right,
            ColumnAlign::Right,
            ColumnAlign::Right,
        ],
    );

    let mut total_frames = 0;
    let mut total_commands = 0;
    let mut total_stream_bytes = 0;
    let mut totals = HashMap::<u8, CommandUse>::new();
    let mut redundancy_total = CharRedundancyStats::default();

    for summary in summaries {
        stream_table.add_row(vec![
            summary.act.to_string(),
            summary.strategy.clone(),
            fmt_num(summary.frames),
            fmt_num(summary.commands),
            fmt_num(summary.stream_bytes),
        ]);
        total_frames += summary.frames;
        total_commands += summary.commands;
        total_stream_bytes += summary.stream_bytes;

        for (opcode, usage) in &summary.usage {
            let total = totals.entry(*opcode).or_default();
            total.count += usage.count;
            total.bytes += usage.bytes;
        }
        redundancy_total.add(&summary.redundancy);
    }

    stream_table.add_row(vec![
        "**total**".to_string(),
        String::new(),
        format!("**{}**", fmt_num(total_frames)),
        format!("**{}**", fmt_num(total_commands)),
        format!("**{}**", fmt_num(total_stream_bytes)),
    ]);
    stream_table.print();

    println!();
    println!("### Emitted commands");
    println!();
    let mut commands_table = MarkdownTable::new(
        &["opcode", "command", "count", "bytes"],
        &[
            ColumnAlign::Right,
            ColumnAlign::Left,
            ColumnAlign::Right,
            ColumnAlign::Right,
        ],
    );
    for (opcode, label) in command_catalog() {
        let usage = totals.get(&opcode);
        commands_table.add_row(vec![
            opcode.to_string(),
            label,
            fmt_num(usage.map_or(0, |u| u.count)),
            fmt_num(usage.map_or(0, |u| u.bytes)),
        ]);
    }
    commands_table.print();

    println!();
    println!("### Charset upload redundancy");
    println!();
    let mut redundancy_table = MarkdownTable::new(
        &[
            "act",
            "uploads",
            "unique glyphs",
            "first uploads",
            "reuploads after eviction",
            "duplicates while resident",
            "same-slot rewrites",
            "redundant uploads",
            "upload payload B",
            "command overhead B",
            "redundant B",
            "redundant %",
        ],
        &[ColumnAlign::Right; 12],
    );
    for summary in summaries {
        redundancy_table.add_row(redundancy_row(
            &summary.act.to_string(),
            &summary.redundancy,
            Some(summary.redundancy.unique_glyphs),
        ));
    }
    redundancy_table.add_row(redundancy_row("**total**", &redundancy_total, None));
    redundancy_table.print();

    println!();
    println!(
        "A *reupload after eviction* is a glyph that was uploaded earlier, overwritten in its slot by a different glyph, and uploaded again later. *Duplicate while resident* means the same encoded glyph was already present in another slot at upload time."
    );
}
