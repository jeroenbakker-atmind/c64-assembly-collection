use c64_assembler::{builder::InstructionBuilder, Instructions};

use super::{
    model::{Condition, Sequence, SequenceStep},
    plan::ResourcePlan,
    resources::ResourceId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmitError {
    Validation(String),
    Plan(String),
}

impl std::fmt::Display for EmitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmitError::Validation(message) => write!(f, "invalid sequence: {message}"),
            EmitError::Plan(message) => write!(f, "sequence does not match plan: {message}"),
        }
    }
}

impl std::error::Error for EmitError {}

/// Emits a Rust `Sequence` AST as ordinary 6502 assembler instructions.
pub struct SequenceEmitter<'a> {
    plan: &'a ResourcePlan,
    label_counter: usize,
}

impl<'a> SequenceEmitter<'a> {
    pub fn new(plan: &'a ResourcePlan) -> Self {
        Self { plan, label_counter: 0 }
    }

    /// Validates structural rules without requiring a resource plan.
    pub fn validate(sequence: &Sequence) -> Result<(), EmitError> {
        if !matches!(sequence.steps.first(), Some(SequenceStep::Init)) {
            return Err(EmitError::Validation("sequence must start with Init".to_string()));
        }
        let mut active = None;
        validate_steps(&sequence.steps, &mut active, false)
    }

    pub fn emit(&mut self, sequence: &Sequence) -> Result<Instructions, EmitError> {
        self.validate_with_plan(sequence)?;
        let mut builder = InstructionBuilder::default();
        builder.label(&sequence.name);
        self.emit_steps(&sequence.steps, &mut builder)?;
        Ok(builder.build())
    }

    /// Optional readable output for debugging; it is not executed at runtime.
    pub fn emit_asm_text(&mut self, sequence: &Sequence) -> Result<String, EmitError> {
        self.validate_with_plan(sequence)?;
        let mut lines = vec![format!("{}:", sequence.name)];
        self.emit_text_steps(&sequence.steps, &mut lines)?;
        Ok(lines.join("\n"))
    }

    fn validate_with_plan(&self, sequence: &Sequence) -> Result<(), EmitError> {
        Self::validate(sequence)?;
        for step in &sequence.steps {
            self.validate_resources(step)?;
        }
        Ok(())
    }

    fn validate_resources(&self, step: &SequenceStep) -> Result<(), EmitError> {
        match step {
            SequenceStep::Load(resource) => self.check_resource(*resource),
            SequenceStep::StreamPart { next, previous, .. } => {
                self.check_resource(*next)?;
                self.check_resource(*previous)
            }
            SequenceStep::Recycle(resource) | SequenceStep::PlayFrames { resource, .. } => {
                self.check_resource(*resource)
            }
            SequenceStep::RepeatCount { body, .. } | SequenceStep::RepeatUntil { body, .. } => {
                for nested in body {
                    self.validate_resources(nested)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn check_resource(&self, resource: ResourceId) -> Result<(), EmitError> {
        if self.plan.resources.iter().any(|item| item.id == resource) {
            Ok(())
        } else {
            Err(EmitError::Plan(format!("unknown resource id {resource}")))
        }
    }

    fn emit_steps(&mut self, steps: &[SequenceStep], builder: &mut InstructionBuilder) -> Result<(), EmitError> {
        for step in steps {
            match step {
                SequenceStep::Init => {
                    builder
                        .jsr_addr("engine__init")
                        .jsr_addr("engine__loader__init")
                        .jsr_addr("engine__condition__init");
                }
                SequenceStep::Load(resource) => {
                    builder.lda_imm(*resource).jsr_addr("engine__loader__start");
                }
                SequenceStep::WaitLoaded => {
                    builder.jsr_addr("engine__loader__wait_loaded");
                }
                SequenceStep::StreamPart { next, previous, frames } => {
                    let label = self.unique_label("stream");
                    let done_label = format!("{label}_done");
                    builder.lda_imm(*next).jsr_addr("engine__loader__start");
                    builder.label(&label);
                    builder.jsr_addr("engine__condition__load_complete");
                    builder.beq_addr(&done_label);
                    builder
                        .lda_imm(*previous)
                        .ldx_imm(0)
                        .ldy_imm(0)
                        .jsr_addr("engine__resource__use")
                        .lda_imm(*frames as u8)
                        .ldx_imm((*frames >> 8) as u8)
                        .jsr_addr("engine__frame__play_n");
                    builder.jmp_addr(&label).label(&done_label);
                    builder.jsr_addr("engine__loader__wait_loaded");
                }
                SequenceStep::Recycle(_) => {}
                SequenceStep::PlayFrames {
                    resource,
                    offset,
                    count,
                } => {
                    builder
                        .lda_imm(*resource)
                        .ldx_imm(*offset as u8)
                        .ldy_imm((*offset >> 8) as u8)
                        .jsr_addr("engine__resource__use")
                        .lda_imm(*count as u8)
                        .ldx_imm((*count >> 8) as u8)
                        .jsr_addr("engine__frame__play_n");
                }
                SequenceStep::WaitRefreshes(refreshes) => {
                    builder.lda_imm(*refreshes).jsr_addr("engine__refresh__wait_n");
                }
                SequenceStep::RepeatCount { count, body } => {
                    builder.ldx_imm(*count as u8);
                    let label = self.unique_label("repeat");
                    builder.label(&label);
                    // Body primitives clobber X, so preserve the loop counter on
                    // the stack around the emitted body.
                    builder.txa().pha();
                    self.emit_steps(body, builder)?;
                    builder.pla().tax().dex().bne_addr(&label);
                }
                SequenceStep::RepeatUntil { condition, body } => {
                    let label = self.unique_label("until");
                    let done_label = format!("{label}_done");
                    builder.label(&label);
                    match condition {
                        Condition::MusicBefore(position) => {
                            builder.lda_imm(*position).jsr_addr("engine__condition__music_before");
                        }
                        Condition::DiskTurned => {
                            builder.jsr_addr("engine__condition__disk_turned");
                        }
                        Condition::LoadComplete => {
                            builder.jsr_addr("engine__condition__load_complete");
                        }
                    }
                    builder.beq_addr(&done_label);
                    self.emit_steps(body, builder)?;
                    builder.jmp_addr(&label).label(&done_label);
                }
                SequenceStep::End => {
                    builder
                        .jsr_addr("engine__loader__deinit")
                        .jsr_addr("engine__deinit")
                        .rts();
                }
            }
        }
        Ok(())
    }

    fn emit_text_steps(&mut self, steps: &[SequenceStep], lines: &mut Vec<String>) -> Result<(), EmitError> {
        for step in steps {
            match step {
                SequenceStep::Init => {
                    lines.push("    jsr engine__init".to_string());
                    lines.push("    jsr engine__loader__init".to_string());
                    lines.push("    jsr engine__condition__init".to_string());
                }
                SequenceStep::Load(resource) => {
                    lines.push(format!("    lda #{resource}"));
                    lines.push("    jsr engine__loader__start".to_string());
                }
                SequenceStep::WaitLoaded => {
                    lines.push("    jsr engine__loader__wait_loaded".to_string());
                }
                SequenceStep::StreamPart { next, previous, frames } => {
                    let label = self.unique_label("stream");
                    let done_label = format!("{label}_done");
                    lines.push(format!("    lda #{next}"));
                    lines.push("    jsr engine__loader__start".to_string());
                    lines.push(format!("{label}:"));
                    lines.push("    jsr engine__condition__load_complete".to_string());
                    lines.push(format!("    beq {done_label}"));
                    lines.push(format!("    lda #{previous}"));
                    lines.push("    ldx #$00".to_string());
                    lines.push("    ldy #$00".to_string());
                    lines.push("    jsr engine__resource__use".to_string());
                    lines.push(format!("    lda #<{frames}"));
                    lines.push(format!("    ldx #>{frames}"));
                    lines.push("    jsr engine__frame__play_n".to_string());
                    lines.push(format!("    jmp {label}"));
                    lines.push(format!("{done_label}:"));
                    lines.push("    jsr engine__loader__wait_loaded".to_string());
                }
                SequenceStep::Recycle(resource) => {
                    lines.push(format!("    ; recycle {resource} (build-time slot release)"));
                }
                SequenceStep::PlayFrames {
                    resource,
                    offset,
                    count,
                } => {
                    lines.push(format!("    lda #{resource}"));
                    lines.push(format!("    ldx #${:02X}", *offset as u8));
                    lines.push(format!("    ldy #${:02X}", (*offset >> 8) as u8));
                    lines.push("    jsr engine__resource__use".to_string());
                    lines.push(format!("    lda #<{count}"));
                    lines.push(format!("    ldx #>{count}"));
                    lines.push("    jsr engine__frame__play_n".to_string());
                }
                SequenceStep::WaitRefreshes(refreshes) => {
                    lines.push(format!("    lda #{refreshes}"));
                    lines.push("    jsr engine__refresh__wait_n".to_string());
                }
                SequenceStep::RepeatCount { count, body } => {
                    lines.push(format!("    ldx #{count}"));
                    let label = self.unique_label("repeat");
                    lines.push(format!("{label}:"));
                    lines.push("    txa".to_string());
                    lines.push("    pha".to_string());
                    self.emit_text_steps(body, lines)?;
                    lines.push("    pla".to_string());
                    lines.push("    tax".to_string());
                    lines.push("    dex".to_string());
                    lines.push(format!("    bne {label}"));
                }
                SequenceStep::RepeatUntil { condition, body } => {
                    let label = self.unique_label("until");
                    let done_label = format!("{label}_done");
                    lines.push(format!("{label}:"));
                    match condition {
                        Condition::MusicBefore(position) => {
                            lines.push(format!("    lda #{position}"));
                            lines.push("    jsr engine__condition__music_before".to_string());
                        }
                        Condition::DiskTurned => {
                            lines.push("    jsr engine__condition__disk_turned".to_string());
                        }
                        Condition::LoadComplete => {
                            lines.push("    jsr engine__condition__load_complete".to_string());
                        }
                    }
                    lines.push(format!("    beq {done_label}"));
                    self.emit_text_steps(body, lines)?;
                    lines.push(format!("    jmp {label}"));
                    lines.push(format!("{done_label}:"));
                }
                SequenceStep::End => {
                    lines.push("    jsr engine__loader__deinit".to_string());
                    lines.push("    jsr engine__deinit".to_string());
                    lines.push("    rts".to_string());
                }
            }
        }
        Ok(())
    }

    fn unique_label(&mut self, prefix: &str) -> String {
        let label = format!("seq_{prefix}_{}", self.label_counter);
        self.label_counter += 1;
        label
    }
}

fn validate_steps(steps: &[SequenceStep], active: &mut Option<ResourceId>, in_loop: bool) -> Result<(), EmitError> {
    for (index, step) in steps.iter().enumerate() {
        match step {
            SequenceStep::Init => {
                if index != 0 {
                    return Err(EmitError::Validation("Init must be the first step".to_string()));
                }
            }
            SequenceStep::Load(resource) => {
                if active.is_some() {
                    return Err(EmitError::Validation(
                        "Load emitted while another load is pending".to_string(),
                    ));
                }
                *active = Some(*resource);
            }
            SequenceStep::WaitLoaded => {
                if active.take().is_none() {
                    return Err(EmitError::Validation("WaitLoaded without a pending Load".to_string()));
                }
            }
            SequenceStep::StreamPart { next, previous, frames } => {
                if in_loop {
                    return Err(EmitError::Validation(
                        "StreamPart must not appear in a loop body".to_string(),
                    ));
                }
                if *next == *previous {
                    return Err(EmitError::Validation(
                        "StreamPart cannot replay the resource it is loading".to_string(),
                    ));
                }
                if *frames == 0 {
                    return Err(EmitError::Validation("StreamPart frames must be non-zero".to_string()));
                }
                if active.is_some() {
                    return Err(EmitError::Validation(
                        "StreamPart emitted while another load is pending".to_string(),
                    ));
                }
                // The emitted sequence starts and waits for its own load.
                *active = Some(*next);
                *active = None;
            }
            SequenceStep::Recycle(_) => {
                if in_loop {
                    return Err(EmitError::Validation(
                        "Recycle must not appear in a loop body".to_string(),
                    ));
                }
                if active.is_some() {
                    return Err(EmitError::Validation(
                        "Recycle emitted while a load is pending".to_string(),
                    ));
                }
            }
            SequenceStep::PlayFrames { resource, count, .. } => {
                if *count == 0 {
                    return Err(EmitError::Validation("PlayFrames count must be non-zero".to_string()));
                }
                if *active == Some(*resource) {
                    return Err(EmitError::Validation(format!(
                        "PlayFrames uses resource {resource} while it is loading"
                    )));
                }
            }
            SequenceStep::WaitRefreshes(refreshes) => {
                if *refreshes == 0 {
                    return Err(EmitError::Validation(
                        "WaitRefreshes count must be non-zero".to_string(),
                    ));
                }
            }
            SequenceStep::RepeatCount { count, body } => {
                if *count == 0 {
                    return Err(EmitError::Validation("RepeatCount count must be non-zero".to_string()));
                }
                if *count > u8::MAX as u16 {
                    return Err(EmitError::Validation("RepeatCount count must fit in X".to_string()));
                }
                validate_loop_body(body, active)?;
            }
            SequenceStep::RepeatUntil { body, .. } => validate_loop_body(body, active)?,
            SequenceStep::End => {
                if in_loop {
                    return Err(EmitError::Validation(
                        "End must only appear at the top level".to_string(),
                    ));
                }
                if active.is_some() {
                    return Err(EmitError::Validation("End reached with a pending Load".to_string()));
                }
                if index + 1 != steps.len() {
                    return Err(EmitError::Validation("End must be the final step".to_string()));
                }
            }
        }
    }
    Ok(())
}

fn validate_loop_body(body: &[SequenceStep], active: &mut Option<ResourceId>) -> Result<(), EmitError> {
    // A load may already be in flight when a loop starts: playing the previous
    // part until that load completes is the intended interleave. What must not
    // happen is a load being *created* or *completed* inside the loop.
    let active_on_entry = *active;
    validate_steps(body, active, true)?;
    if *active != active_on_entry {
        return Err(EmitError::Validation("loop body changes a pending Load".to_string()));
    }
    Ok(())
}
