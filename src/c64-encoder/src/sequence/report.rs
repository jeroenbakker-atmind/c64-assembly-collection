use super::{
    model::{Sequence, SequenceStep},
    resources::{DiskLocation, ResourceId},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimingStatus {
    Boot,
    Ok,
    Tight,
    UnderrunRisk,
}

impl TimingStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TimingStatus::Boot => "boot",
            TimingStatus::Ok => "OK",
            TimingStatus::Tight => "TIGHT",
            TimingStatus::UnderrunRisk => "UNDERRUN_RISK",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResourceReport {
    pub resource_id: ResourceId,
    pub name: String,
    pub act: Option<u32>,
    pub frame_count: Option<u16>,
    pub bytes: usize,
    pub sectors: u8,
    pub start: DiskLocation,
    pub dest: u16,
    pub lead_frames: Option<u16>,
    pub need_ms: f32,
    pub margin_ms: Option<f32>,
    pub status: TimingStatus,
}

impl ResourceReport {
    pub fn markdown_table(reports: &[ResourceReport]) -> String {
        let mut out = String::new();
        out.push_str("| resource | id | act | frames | bytes | sectors | t/s start | dest | lead frames | need ms | margin ms | status |\n");
        out.push_str("|---------:|---:|----:|-------:|------:|--------:|----------:|-----:|------------:|--------:|----------:|:------:|\n");
        for report in reports {
            let act = report.act.map(|act| act.to_string()).unwrap_or_else(|| "-".to_string());
            let frames = report
                .frame_count
                .map(|count| count.to_string())
                .unwrap_or_else(|| "-".to_string());
            let lead = report
                .lead_frames
                .map(|frames| frames.to_string())
                .unwrap_or_else(|| "-".to_string());
            let margin = report
                .margin_ms
                .map(|margin| format!("{margin:.0}"))
                .unwrap_or_else(|| "-".to_string());
            out.push_str(&format!(
                "| {:<8} | {:>2} | {:>3} | {:>6} | {:>5} | {:>7} | {}/{} | {:>4X} | {:>11} | {:>7.0} | {:>9} | {} |\n",
                report.name,
                report.resource_id,
                act,
                frames,
                report.bytes,
                report.sectors,
                report.start.track,
                report.start.sector,
                report.dest,
                lead,
                report.need_ms,
                margin,
                report.status.as_str(),
            ));
        }
        out
    }
}

/// Adds sequence-derived timing information to resource reports.
pub fn annotate_timing(
    reports: &mut [ResourceReport],
    sequence: &Sequence,
    refreshes_per_frame: u8,
    safety_margin_frames: u8,
) {
    let mut state = TimingWalkState::default();
    walk_steps(&sequence.steps, &mut state);
    let frame_ms = refreshes_per_frame as f32 * 20.0;

    for report in reports.iter_mut() {
        if let Some(lead_frames) = state.lead_frames.get(&report.resource_id).copied() {
            report.lead_frames = Some(lead_frames);
            let available_ms = lead_frames as f32 * frame_ms;
            let margin_ms = available_ms - report.need_ms;
            report.margin_ms = Some(margin_ms);
            report.status = if margin_ms < 0.0 {
                TimingStatus::UnderrunRisk
            } else if margin_ms < safety_margin_frames as f32 * frame_ms {
                TimingStatus::Tight
            } else {
                TimingStatus::Ok
            };
        }
    }
}

#[derive(Default)]
struct TimingWalkState {
    pending: Option<(ResourceId, u16)>,
    lead_frames: std::collections::BTreeMap<ResourceId, u16>,
}

fn walk_steps(steps: &[SequenceStep], state: &mut TimingWalkState) {
    for step in steps {
        match step {
            SequenceStep::Load(resource) => {
                state.pending = Some((*resource, 0));
            }
            SequenceStep::PlayFrames { count, .. } => {
                if let Some((_, lead)) = state.pending.as_mut() {
                    *lead = lead.saturating_add(*count);
                }
            }
            SequenceStep::WaitLoaded => {
                if let Some((resource, lead)) = state.pending.take() {
                    state.lead_frames.insert(resource, lead);
                }
            }
            SequenceStep::RepeatCount { body, .. } | SequenceStep::RepeatUntil { body, .. } => {
                let mut nested = TimingWalkState::default();
                walk_steps(body, &mut nested);
                for (resource, lead) in nested.lead_frames {
                    state.lead_frames.insert(resource, lead);
                }
            }
            _ => {}
        }
    }
}
