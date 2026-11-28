use std::collections::{BTreeMap, HashSet, VecDeque};
use std::fmt;

use c64::create_disk::MasteredRange;

use crate::builder::demo::DemoBuilder;

use super::model::{Sequence, SequenceStep};
use super::report::{annotate_timing, ResourceReport, TimingStatus};
use super::resources::{DiskLocation, DiskResource, LoadRange, ResourceId, ResourceRegion, SlotId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MasterPlannerConfig {
    pub frame_region: ResourceRegion,
    pub music_region: ResourceRegion,
    pub slot_count: u8,
    pub slot_size: u16,
    pub condition_status_base: u16,
    pub interleave: u8,
    pub usable_sector_zero: bool,
    pub throughput_bytes_per_second: u32,
    pub refreshes_per_frame: u8,
    pub safety_margin_frames: u8,
}

impl Default for MasterPlannerConfig {
    fn default() -> Self {
        Self {
            frame_region: ResourceRegion::new(0x4000, 0x6000, false),
            music_region: ResourceRegion::new(0xA000, 0x2000, true),
            slot_count: 6,
            slot_size: 0x1000,
            condition_status_base: 0x3E00,
            interleave: 7,
            usable_sector_zero: false,
            throughput_bytes_per_second: 4096,
            refreshes_per_frame: 4,
            safety_margin_frames: 2,
        }
    }
}

impl MasterPlannerConfig {
    pub fn validate(&self) -> Result<(), PlanError> {
        if self.slot_count == 0 {
            return Err(PlanError::InvalidConfig("slot_count must be non-zero"));
        }
        if self.slot_size == 0 {
            return Err(PlanError::InvalidConfig("slot_size must be non-zero"));
        }
        if self.interleave == 0 {
            return Err(PlanError::InvalidConfig("interleave must be non-zero"));
        }
        if self.throughput_bytes_per_second == 0 {
            return Err(PlanError::InvalidConfig("throughput_bytes_per_second must be non-zero"));
        }

        let frame_end = self.frame_region.end_exclusive();
        let expected_frame_end = self.frame_region.base as u32 + self.slot_count as u32 * self.slot_size as u32;
        if frame_end < expected_frame_end {
            return Err(PlanError::InvalidConfig(
                "frame_region is smaller than slot_count * slot_size",
            ));
        }
        if frame_end > 0xA000 {
            return Err(PlanError::InvalidConfig("frame_region must stay below $A000"));
        }

        let music_end = self.music_region.end_exclusive();
        if music_end > 0xC000 {
            return Err(PlanError::InvalidConfig("music_region must stay below $C000"));
        }
        if (self.music_region.base as u32) < frame_end {
            return Err(PlanError::InvalidConfig("music_region overlaps frame_region"));
        }

        for sectors in [21u16, 19, 18, 17] {
            let modulus = if self.usable_sector_zero { sectors } else { sectors - 1 } as u32;
            if modulus == 0 || gcd(self.interleave as u32, modulus) != 1 {
                return Err(PlanError::InvalidConfig(
                    "interleave is not coprime with a sector modulus",
                ));
            }
        }

        Ok(())
    }
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

fn track_at(index: usize) -> Option<u8> {
    match index {
        0..=16 => Some((index + 1) as u8),
        17..=33 => Some((index + 2) as u8),
        _ => None,
    }
}

fn sectors_per_track(track: u8) -> u16 {
    match track {
        1..=17 => 21,
        18..=24 => 19,
        25..=30 => 18,
        31..=35 => 17,
        _ => 0,
    }
}

#[derive(Debug, Clone)]
struct SectorCursor {
    config: MasterPlannerConfig,
    track_index: usize,
    position_in_track: usize,
}

impl SectorCursor {
    fn new(config: MasterPlannerConfig) -> Self {
        Self {
            config,
            track_index: 0,
            position_in_track: 0,
        }
    }

    fn allocate(&mut self, count: u16) -> Result<Vec<DiskLocation>, PlanError> {
        let mut result = Vec::with_capacity(count as usize);
        for _ in 0..count {
            result.push(self.next()?);
        }
        Ok(result)
    }

    fn next(&mut self) -> Result<DiskLocation, PlanError> {
        loop {
            let track =
                track_at(self.track_index).ok_or_else(|| PlanError::InvalidConfig("disk geometry exhausted"))?;
            let total = sectors_per_track(track) as usize;
            let usable = if self.config.usable_sector_zero {
                total
            } else {
                total - 1
            };
            if self.position_in_track >= usable {
                self.track_index += 1;
                self.position_in_track = 0;
                continue;
            }

            let logical = self.position_in_track;
            let sector = if self.config.usable_sector_zero {
                logical as u8
            } else {
                (1 + ((logical * self.config.interleave as usize) % (total - 1))) as u8
            };
            self.position_in_track += 1;
            return Ok(DiskLocation { track, sector });
        }
    }
}

#[derive(Debug)]
pub enum PlanError {
    InvalidConfig(&'static str),
    InvalidResource(String),
    DuplicateName(String),
    OutOfMemory(String),
    TooManyResources,
    SectorCountOverflow(String),
    SlotExhausted(String),
    InvalidSlotLifetime(String),
}

impl fmt::Display for PlanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlanError::InvalidConfig(message) => write!(f, "invalid planner config: {message}"),
            PlanError::InvalidResource(message) => write!(f, "invalid resource: {message}"),
            PlanError::DuplicateName(name) => write!(f, "duplicate resource name: {name}"),
            PlanError::OutOfMemory(message) => write!(f, "resource does not fit: {message}"),
            PlanError::TooManyResources => write!(f, "more than 255 resources"),
            PlanError::SectorCountOverflow(name) => {
                write!(f, "resource {name} needs more than 255 sectors")
            }
            PlanError::SlotExhausted(name) => {
                write!(f, "no free frame slot remains for resource {name}")
            }
            PlanError::InvalidSlotLifetime(message) => {
                write!(f, "invalid frame slot lifetime: {message}")
            }
        }
    }
}

impl std::error::Error for PlanError {}

/// Immutable assembler constants emitted alongside the sequence.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GeneratedConstants {
    values: BTreeMap<String, u16>,
}

impl GeneratedConstants {
    pub fn get(&self, name: &str) -> Option<u16> {
        self.values.get(name).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &u16)> {
        self.values.iter()
    }

    pub fn emit_defines(&self) -> String {
        let mut out = String::new();
        for (name, value) in &self.values {
            out.push_str(&format!("{name} = ${value:04X}\n"));
        }
        out
    }
}

/// Build-time planner that turns encoded resources into a deterministic master plan.
#[derive(Debug, Clone)]
pub struct MasterPlanner {
    pub config: MasterPlannerConfig,
    resources: Vec<DiskResource>,
    constants: BTreeMap<String, u16>,
    frame_resource_index: usize,
    blob_offset: usize,
}

impl MasterPlanner {
    pub fn new(config: MasterPlannerConfig) -> Result<Self, PlanError> {
        config.validate()?;
        let mut constants = BTreeMap::new();
        constants.insert("FRAME_SLOT_COUNT".to_string(), config.slot_count as u16);
        constants.insert("FRAME_SLOT_SIZE".to_string(), config.slot_size);
        constants.insert("MUSIC_BASE".to_string(), config.music_region.base);
        constants.insert("CONDITION_STATUS_BASE".to_string(), config.condition_status_base);
        constants.insert(
            "BANK_OUT_REQUIRED".to_string(),
            u16::from(config.music_region.requires_bank_out),
        );
        constants.insert("REFRESHES_PER_FRAME".to_string(), config.refreshes_per_frame as u16);
        constants.insert("ACT_GAP_REFRESHES".to_string(), 0);
        Ok(Self {
            config,
            resources: Vec::new(),
            constants,
            frame_resource_index: 0,
            blob_offset: 0,
        })
    }

    pub fn with_defaults() -> Result<Self, PlanError> {
        Self::new(MasterPlannerConfig::default())
    }

    pub fn set_constant(&mut self, name: impl Into<String>, value: u16) {
        self.constants.insert(name.into(), value);
    }

    pub fn add_frame_resource(
        &mut self,
        name: impl Into<String>,
        act: u32,
        first_frame: u16,
        last_frame: u16,
        encoded_bytes: Vec<u8>,
    ) -> Result<ResourceId, PlanError> {
        let name = name.into();
        self.check_name(&name)?;
        if last_frame < first_frame {
            return Err(PlanError::InvalidResource(format!("{name}: last_frame < first_frame")));
        }
        if encoded_bytes.is_empty() || encoded_bytes.len() > self.config.slot_size as usize {
            return Err(PlanError::OutOfMemory(format!(
                "{name}: {} bytes exceeds frame slot size {}",
                encoded_bytes.len(),
                self.config.slot_size
            )));
        }

        let slot = (self.frame_resource_index % self.config.slot_count as usize) as SlotId;
        let dest = self.config.frame_region.base + slot as u16 * self.config.slot_size;
        if !self.config.frame_region.contains_range(dest, encoded_bytes.len()) {
            return Err(PlanError::OutOfMemory(format!(
                "{name}: destination ${dest:04X} outside frame region"
            )));
        }

        let resource = DiskResource {
            id: self.next_id()?,
            name,
            encoded_bytes,
            frame: Some(super::resources::FrameRange {
                first: first_frame,
                last: last_frame,
                act,
            }),
            dest,
            slot: Some(slot),
        };
        let id = resource.id;
        self.resources.push(resource);
        self.frame_resource_index += 1;
        Ok(id)
    }

    pub fn add_blob(&mut self, name: impl Into<String>, encoded_bytes: Vec<u8>) -> Result<ResourceId, PlanError> {
        let name = name.into();
        self.check_name(&name)?;
        if encoded_bytes.is_empty() {
            return Err(PlanError::InvalidResource(format!(
                "{name}: empty resources are not supported"
            )));
        }
        if self.blob_offset + encoded_bytes.len() > self.config.music_region.max_bytes {
            return Err(PlanError::OutOfMemory(format!(
                "{name}: blob region needs {} bytes, region has {}",
                self.blob_offset + encoded_bytes.len(),
                self.config.music_region.max_bytes
            )));
        }

        let dest = self.config.music_region.base + self.blob_offset as u16;
        let resource = DiskResource {
            id: self.next_id()?,
            name,
            encoded_bytes,
            frame: None,
            dest,
            slot: None,
        };
        let id = resource.id;
        self.resources.push(resource);
        self.blob_offset += self.resources.last().unwrap().encoded_bytes.len();
        Ok(id)
    }

    /// Splits one encoded act into frame resources at frame and slot boundaries.
    pub fn add_demo_resources(
        &mut self,
        act: u32,
        base_name: &str,
        demo: &DemoBuilder,
    ) -> Result<Vec<ResourceId>, PlanError> {
        if demo.frames.len() > u16::MAX as usize {
            return Err(PlanError::InvalidResource(format!(
                "{base_name}: more than {} frames",
                u16::MAX
            )));
        }

        let mut ids = Vec::new();
        let mut current = Vec::new();
        let mut first = 1u16;
        let mut last = 1u16;
        let mut part = 0usize;

        macro_rules! flush {
            () => {{
                if !current.is_empty() {
                    let name = format!("{base_name}_p{part}");
                    let id = self.add_frame_resource(name, act, first, last, std::mem::take(&mut current))?;
                    ids.push(id);
                    part += 1;
                }
            }};
        }

        for (index, frame) in demo.frames.iter().enumerate() {
            let frame_number = index as u16 + 1;
            let bytes = frame.frame_record_bytes();
            if bytes.len() > self.config.slot_size as usize {
                return Err(PlanError::OutOfMemory(format!(
                    "{base_name} frame {frame_number} is {} bytes, slot is {}",
                    bytes.len(),
                    self.config.slot_size
                )));
            }
            if !current.is_empty() && current.len() + bytes.len() > self.config.slot_size as usize {
                flush!();
                first = frame_number;
            }
            if current.is_empty() {
                first = frame_number;
            }
            current.extend_from_slice(&bytes);
            last = frame_number;
        }
        flush!();
        let _ = part;

        Ok(ids)
    }

    pub fn id(&self, name: &str) -> Result<ResourceId, PlanError> {
        self.resources
            .iter()
            .find(|resource| resource.name == name)
            .map(|resource| resource.id)
            .ok_or_else(|| PlanError::InvalidResource(format!("unknown resource {name}")))
    }

    pub fn frames(&self, name: &str) -> Result<u16, PlanError> {
        self.resources
            .iter()
            .find(|resource| resource.name == name)
            .and_then(|resource| resource.frame.as_ref())
            .map(|frame| frame.frame_count())
            .ok_or_else(|| PlanError::InvalidResource(format!("no frame range for {name}")))
    }

    pub fn build(&self) -> Result<ResourcePlan, PlanError> {
        self.config.validate()?;
        let mut cursor = SectorCursor::new(self.config);
        let mut ranges = Vec::with_capacity(self.resources.len());
        let mut sector_image = Vec::new();
        let mut constants = self.constants.clone();
        let mut reports = Vec::with_capacity(self.resources.len());

        for resource in &self.resources {
            let payload_bytes = resource.encoded_bytes.len();
            let sector_count = (payload_bytes + 255) / 256;
            if sector_count > u8::MAX as usize {
                return Err(PlanError::SectorCountOverflow(resource.name.clone()));
            }
            let locations = cursor.allocate(sector_count as u16)?;
            let start = locations[0];

            let mut padded = resource.encoded_bytes.clone();
            padded.resize(sector_count * 256, 0);
            for sector in padded.chunks_exact(256) {
                let mut block = [0u8; 256];
                block.copy_from_slice(sector);
                sector_image.push(block);
            }

            let symbol = symbol_name(&resource.name);
            constants.insert(format!("R_{symbol}"), resource.id as u16);
            constants.insert(format!("DEST_{symbol}"), resource.dest);
            constants.insert(format!("SECTORS_{symbol}"), sector_count as u16);
            constants.insert(format!("PAYLOAD_{symbol}"), payload_bytes as u16);
            if let Some(frame) = resource.frame.as_ref() {
                constants.insert(format!("FRAMES_{symbol}"), frame.frame_count());
            }

            let need_ms = (sector_count as f32 * 256.0 * 1000.0) / self.config.throughput_bytes_per_second as f32;
            reports.push(ResourceReport {
                resource_id: resource.id,
                name: resource.name.clone(),
                act: resource.frame.map(|frame| frame.act),
                frame_count: resource.frame.map(|frame| frame.frame_count()),
                bytes: payload_bytes,
                sectors: sector_count as u8,
                start,
                dest: resource.dest,
                lead_frames: None,
                need_ms,
                margin_ms: None,
                status: TimingStatus::Boot,
            });
            ranges.push(LoadRange {
                resource_id: resource.id,
                name: resource.name.clone(),
                start,
                sector_count: sector_count as u8,
                dest: resource.dest,
                payload_bytes: payload_bytes as u16,
            });
        }

        Ok(ResourcePlan {
            resources: self.resources.clone(),
            ranges,
            sector_image,
            constants: GeneratedConstants { values: constants },
            reports,
            config: self.config,
        })
    }

    fn next_id(&self) -> Result<ResourceId, PlanError> {
        if self.resources.len() >= u8::MAX as usize {
            Err(PlanError::TooManyResources)
        } else {
            Ok(self.resources.len() as ResourceId)
        }
    }

    fn check_name(&self, name: &str) -> Result<(), PlanError> {
        if name.is_empty() {
            return Err(PlanError::InvalidResource("name must not be empty".to_string()));
        }
        if self.resources.iter().any(|resource| resource.name == name) {
            return Err(PlanError::DuplicateName(name.to_string()));
        }
        Ok(())
    }
}

pub fn symbol_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct ResourcePlan {
    pub resources: Vec<DiskResource>,
    pub ranges: Vec<LoadRange>,
    pub sector_image: Vec<[u8; 256]>,
    pub constants: GeneratedConstants,
    pub reports: Vec<ResourceReport>,
    pub config: MasterPlannerConfig,
}

impl ResourcePlan {
    pub fn id(&self, name: &str) -> ResourceId {
        self.resources
            .iter()
            .find(|resource| resource.name == name)
            .unwrap_or_else(|| panic!("unknown resource {name}"))
            .id
    }

    pub fn frames(&self, name: &str) -> u16 {
        self.resources
            .iter()
            .find(|resource| resource.name == name)
            .and_then(|resource| resource.frame.as_ref())
            .unwrap_or_else(|| panic!("no frame range for resource {name}"))
            .frame_count()
    }

    pub fn value(&self, name: &str) -> u16 {
        self.constants
            .get(name)
            .unwrap_or_else(|| panic!("unknown constant {name}"))
    }

    pub fn encode_resource_table(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.ranges.len() * 7);
        for range in &self.ranges {
            result.push(range.start.track);
            result.push(range.start.sector);
            result.push(range.sector_count);
            result.push(range.dest as u8);
            result.push((range.dest >> 8) as u8);
            result.push(range.payload_bytes as u8);
            result.push((range.payload_bytes >> 8) as u8);
        }
        result
    }

    pub fn mastered_ranges(&self) -> Vec<MasteredRange> {
        let locations = self
            .all_locations()
            .expect("a built plan always has a valid sector cursor");
        let mut result = Vec::with_capacity(self.ranges.len());
        let mut offset = 0usize;
        for range in &self.ranges {
            let count = range.sector_count as usize;
            result.push(MasteredRange {
                locations: locations[offset..offset + count]
                    .iter()
                    .map(|location| (location.track, location.sector))
                    .collect(),
                sectors: self.sector_image[offset..offset + count].to_vec(),
            });
            offset += count;
        }
        result
    }

    /// Replays the planner's sector cursor so mastering, tests, and diagnostics
    /// see the exact chain implied by every `LoadRange`.
    pub fn all_locations(&self) -> Result<Vec<DiskLocation>, PlanError> {
        self.config.validate()?;
        let mut cursor = SectorCursor::new(self.config);
        cursor.allocate(self.sector_image.len() as u16)
    }

    pub fn annotate_timing(&mut self, sequence: &Sequence) {
        annotate_timing(
            &mut self.reports,
            sequence,
            self.config.refreshes_per_frame,
            self.config.safety_margin_frames,
        );
    }

    /// FNV-1a over names, placement, and payloads; used to detect plan drift.
    pub fn plan_hash(&self) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for resource in &self.resources {
            mix(&mut hash, resource.name.as_bytes());
            mix_u16(&mut hash, resource.dest);
            mix(&mut hash, &[resource.slot.unwrap_or(u8::MAX)]);
            if let Some(frame) = resource.frame.as_ref() {
                mix_u16(&mut hash, frame.first);
                mix_u16(&mut hash, frame.last);
                mix_u16(&mut hash, frame.act as u16);
            }
            mix_u16(&mut hash, resource.encoded_bytes.len() as u16);
            mix(&mut hash, &resource.encoded_bytes);
        }
        for range in &self.ranges {
            mix(
                &mut hash,
                &[
                    range.start.track,
                    range.start.sector,
                    range.sector_count,
                    range.payload_bytes as u8,
                    (range.payload_bytes >> 8) as u8,
                ],
            );
        }
        hash
    }

    /// Assign frame slots from explicit sequence lifetimes.
    ///
    /// A frame resource owns a slot from its `Load`/`StreamPart` until its
    /// `Recycle`. Slots are reused only after the explicit release, so the
    /// allocator cannot mistake "most recently played" for "still live".
    pub fn allocate_slots(&mut self, sequence: &Sequence) -> Result<(), PlanError> {
        let mut free: VecDeque<SlotId> = (0..self.config.slot_count).collect();
        let mut live = BTreeMap::new();
        let mut assigned = BTreeMap::new();
        let mut retired = HashSet::new();
        allocate_slots_in_steps(
            &self.resources,
            &sequence.steps,
            &mut free,
            &mut live,
            &mut assigned,
            &mut retired,
            false,
        )?;

        let base = self.config.frame_region.base;
        let slot_size = self.config.slot_size;
        for resource in &mut self.resources {
            if resource.frame.is_none() {
                continue;
            }
            let slot = *assigned.get(&resource.id).ok_or_else(|| {
                PlanError::InvalidSlotLifetime(format!("frame resource {} is never loaded", resource.name))
            })?;
            resource.slot = Some(slot);
            resource.dest = base + slot as u16 * slot_size;
        }

        let destinations: BTreeMap<ResourceId, u16> = self
            .resources
            .iter()
            .map(|resource| (resource.id, resource.dest))
            .collect();
        for range in &mut self.ranges {
            if let Some(dest) = destinations.get(&range.resource_id) {
                range.dest = *dest;
            }
        }
        for report in &mut self.reports {
            if let Some(dest) = destinations.get(&report.resource_id) {
                report.dest = *dest;
            }
        }
        for resource in &self.resources {
            if resource.frame.is_some() {
                let key = format!("DEST_{}", symbol_name(&resource.name));
                self.constants.values.insert(key, resource.dest);
            }
        }
        Ok(())
    }
}

fn mix(hash: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *hash ^= *byte as u64;
        *hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
}

fn mix_u16(hash: &mut u64, value: u16) {
    mix(hash, &[value as u8, (value >> 8) as u8]);
}

fn allocate_slots_in_steps(
    resources: &[DiskResource],
    steps: &[SequenceStep],
    free: &mut VecDeque<SlotId>,
    live: &mut BTreeMap<ResourceId, SlotId>,
    assigned: &mut BTreeMap<ResourceId, SlotId>,
    retired: &mut HashSet<ResourceId>,
    in_loop: bool,
) -> Result<(), PlanError> {
    for step in steps {
        match step {
            SequenceStep::Load(resource) | SequenceStep::StreamPart { next: resource, .. } => {
                if in_loop {
                    return Err(PlanError::InvalidSlotLifetime(
                        "loads must not be created inside a loop body".to_string(),
                    ));
                }
                allocate_one_slot(resources, *resource, free, live, assigned, retired)?;
                if let SequenceStep::StreamPart { previous, .. } = step {
                    if !live.contains_key(previous) {
                        return Err(PlanError::InvalidSlotLifetime(format!(
                            "resource {previous} is replayed before it is loaded"
                        )));
                    }
                }
            }
            SequenceStep::Recycle(resource) => {
                if in_loop {
                    return Err(PlanError::InvalidSlotLifetime(
                        "recycles must not be hidden inside a loop body".to_string(),
                    ));
                }
                let resource = find_resource(resources, *resource)?;
                if resource.frame.is_none() {
                    return Err(PlanError::InvalidSlotLifetime(format!(
                        "resource {} is not a frame slot",
                        resource.name
                    )));
                }
                if retired.contains(&resource.id) {
                    return Err(PlanError::InvalidSlotLifetime(format!(
                        "resource {} is recycled twice",
                        resource.name
                    )));
                }
                let slot = live.remove(&resource.id).ok_or_else(|| {
                    PlanError::InvalidSlotLifetime(format!("resource {} is recycled while not live", resource.name))
                })?;
                free.push_back(slot);
                retired.insert(resource.id);
            }
            SequenceStep::PlayFrames { resource, .. } => {
                if !live.contains_key(resource) {
                    return Err(PlanError::InvalidSlotLifetime(format!(
                        "resource {resource} is played before load or after recycle"
                    )));
                }
            }
            SequenceStep::RepeatCount { body, .. } | SequenceStep::RepeatUntil { body, .. } => {
                allocate_slots_in_steps(resources, body, free, live, assigned, retired, true)?;
            }
            SequenceStep::Init | SequenceStep::WaitLoaded | SequenceStep::WaitRefreshes(_) | SequenceStep::End => {}
        }
    }
    Ok(())
}

fn allocate_one_slot(
    resources: &[DiskResource],
    id: ResourceId,
    free: &mut VecDeque<SlotId>,
    live: &mut BTreeMap<ResourceId, SlotId>,
    assigned: &mut BTreeMap<ResourceId, SlotId>,
    retired: &mut HashSet<ResourceId>,
) -> Result<(), PlanError> {
    let resource = find_resource(resources, id)?;
    if resource.frame.is_none() {
        return Ok(());
    }
    if live.contains_key(&id) {
        return Err(PlanError::InvalidSlotLifetime(format!(
            "resource {} is loaded twice without recycle",
            resource.name
        )));
    }
    if retired.contains(&id) {
        return Err(PlanError::InvalidSlotLifetime(format!(
            "resource {} is loaded after recycle",
            resource.name
        )));
    }
    let slot = free
        .pop_front()
        .ok_or_else(|| PlanError::SlotExhausted(resource.name.clone()))?;
    live.insert(id, slot);
    assigned.insert(id, slot);
    Ok(())
}

fn find_resource(resources: &[DiskResource], id: ResourceId) -> Result<&DiskResource, PlanError> {
    resources
        .iter()
        .find(|resource| resource.id == id)
        .ok_or_else(|| PlanError::InvalidResource(format!("unknown resource id {id}")))
}
