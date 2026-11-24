/// Stable numeric resource identifier used by the emitted sequence.
pub type ResourceId = u8;

/// Frame-slot identifier within `$4000..$9FFF`.
pub type SlotId = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DiskLocation {
    pub track: u8,
    pub sector: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameRange {
    pub first: u16,
    pub last: u16,
    pub act: u32,
}

impl FrameRange {
    pub fn frame_count(&self) -> u16 {
        self.last.saturating_sub(self.first) + 1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceRegion {
    pub base: u16,
    pub max_bytes: usize,
    pub requires_bank_out: bool,
}

impl ResourceRegion {
    pub const fn new(base: u16, max_bytes: usize, requires_bank_out: bool) -> Self {
        Self {
            base,
            max_bytes,
            requires_bank_out,
        }
    }

    pub fn end_exclusive(&self) -> u32 {
        self.base as u32 + self.max_bytes as u32
    }

    pub fn contains(&self, address: u16) -> bool {
        let address = address as u32;
        address >= self.base as u32 && address < self.end_exclusive()
    }

    pub fn contains_range(&self, address: u16, len: usize) -> bool {
        let end = address as u32 + len as u32;
        address as u32 >= self.base as u32 && end <= self.end_exclusive()
    }
}

/// Build-time description of one disk resource.
///
/// `name` is a diagnostic/symbol name only; it is not stored in the PRG table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiskResource {
    pub id: ResourceId,
    pub name: String,
    pub encoded_bytes: Vec<u8>,
    pub frame: Option<FrameRange>,
    pub dest: u16,
    pub slot: Option<SlotId>,
}

/// Loader metadata derived from a `DiskResource`.
///
/// This is the Rust form of one 7-byte `resource_table` row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadRange {
    pub resource_id: ResourceId,
    pub name: String,
    pub start: DiskLocation,
    pub sector_count: u8,
    pub dest: u16,
    pub payload_bytes: u16,
}
