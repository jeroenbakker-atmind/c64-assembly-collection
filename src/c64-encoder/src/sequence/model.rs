use super::resources::ResourceId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    MusicBefore(u8),
    DiskTurned,
    /// Z=1 once the current `Load` has completed.
    LoadComplete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SequenceStep {
    Init,
    Load(ResourceId),
    WaitLoaded,
    /// Start loading `next` while replaying `previous` until the load completes.
    StreamPart {
        next: ResourceId,
        previous: ResourceId,
        frames: u16,
    },
    PlayFrames {
        resource: ResourceId,
        offset: u16,
        count: u16,
    },
    WaitRefreshes(u8),
    RepeatCount {
        count: u16,
        body: Vec<SequenceStep>,
    },
    RepeatUntil {
        condition: Condition,
        body: Vec<SequenceStep>,
    },
    End,
}

/// Build-time AST for the intro sequence.
///
/// This is not executed by the C64. `SequenceEmitter` turns it into 6502
/// instructions that are included in the PRG.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sequence {
    pub name: String,
    pub steps: Vec<SequenceStep>,
}

impl Sequence {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            steps: Vec::new(),
        }
    }

    pub fn init(mut self) -> Self {
        self.steps.push(SequenceStep::Init);
        self
    }

    pub fn load(mut self, resource: ResourceId) -> Self {
        self.steps.push(SequenceStep::Load(resource));
        self
    }

    pub fn wait_loaded(mut self) -> Self {
        self.steps.push(SequenceStep::WaitLoaded);
        self
    }

    /// Load the next part while the previous part is replayed during the transfer.
    pub fn stream_part(mut self, next: ResourceId, previous: ResourceId, frames: u16) -> Self {
        self.steps.push(SequenceStep::StreamPart { next, previous, frames });
        self
    }

    pub fn play(mut self, resource: ResourceId, offset: u16, count: u16) -> Self {
        self.steps.push(SequenceStep::PlayFrames {
            resource,
            offset,
            count,
        });
        self
    }

    pub fn wait_refreshes(mut self, refreshes: u8) -> Self {
        self.steps.push(SequenceStep::WaitRefreshes(refreshes));
        self
    }

    pub fn repeat_count<F>(mut self, count: u16, body: F) -> Self
    where
        F: FnOnce(Sequence) -> Sequence,
    {
        let body = body(Sequence::new("_repeat_body"));
        self.steps.push(SequenceStep::RepeatCount {
            count,
            body: body.steps,
        });
        self
    }

    pub fn repeat_until<F>(mut self, condition: Condition, body: F) -> Self
    where
        F: FnOnce(Sequence) -> Sequence,
    {
        let body = body(Sequence::new("_until_body"));
        self.steps.push(SequenceStep::RepeatUntil {
            condition,
            body: body.steps,
        });
        self
    }

    pub fn end(mut self) -> Self {
        self.steps.push(SequenceStep::End);
        self
    }
}
