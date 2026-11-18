use std::collections::{HashMap, HashSet};

use c64_encoder::{builder::demo::DemoBuilder, charmap::encoding::encode_char, command::Command};

/// Charset-upload churn in one act's final stream.
#[derive(Debug, Clone, Default)]
pub struct CharRedundancyStats {
    pub first_uploads: usize,
    pub reuploads_after_eviction: usize,
    pub duplicate_while_resident: usize,
    pub same_slot_rewrites: usize,
    pub first_upload_bytes: usize,
    pub reupload_bytes: usize,
    pub duplicate_bytes: usize,
    pub same_slot_bytes: usize,
    pub command_overhead_bytes: usize,
    pub unique_glyphs: usize,
}

impl CharRedundancyStats {
    /// Walk the final stream and classify every charset upload against the charset state it sees.
    ///
    /// The comparison is on the encoded 16-bit glyph value, because that is what is actually
    /// written to charset memory. The decoder starts with an all-zero charset.
    pub fn scan(demo: &DemoBuilder) -> Self {
        let mut stats = Self::default();
        let mut charset = [0_u16; 256];
        let mut resident = HashMap::<u16, HashSet<usize>>::new();
        for slot in 0..256 {
            resident.entry(0).or_default().insert(slot);
        }
        let mut uploaded_glyphs = HashSet::<u16>::new();

        for frame in &demo.frames {
            for command in &frame.commands {
                let uploads: Vec<(usize, u16, usize)> = match command {
                    Command::UpdateCharsU16Encoded(update) => {
                        stats.command_overhead_bytes += 1;
                        update
                            .chars
                            .iter()
                            .map(|update| (update.char as usize, encode_char(update.data), 3))
                            .collect()
                    }
                    Command::UpdateCharsRangedU16Encoded(update) => {
                        stats.command_overhead_bytes += 2;
                        let offset = update.offset as usize;
                        update
                            .chars
                            .iter()
                            .enumerate()
                            .map(|(index, update)| (offset + index, encode_char(update.data), 2))
                            .collect()
                    }
                    _ => continue,
                };

                for (slot, glyph, bytes) in uploads {
                    if let Some(slots) = resident.get(&glyph) {
                        if slots.len() > 1 || !slots.contains(&slot) {
                            stats.duplicate_while_resident += 1;
                            stats.duplicate_bytes += bytes;
                        } else {
                            stats.same_slot_rewrites += 1;
                            stats.same_slot_bytes += bytes;
                        }
                    } else if uploaded_glyphs.contains(&glyph) {
                        stats.reuploads_after_eviction += 1;
                        stats.reupload_bytes += bytes;
                    } else {
                        stats.first_uploads += 1;
                        stats.first_upload_bytes += bytes;
                    }
                    uploaded_glyphs.insert(glyph);

                    let old_glyph = charset[slot];
                    let remove_old_key = if let Some(old_slots) = resident.get_mut(&old_glyph) {
                        old_slots.remove(&slot);
                        old_slots.is_empty()
                    } else {
                        false
                    };
                    if remove_old_key {
                        resident.remove(&old_glyph);
                    }
                    charset[slot] = glyph;
                    resident.entry(glyph).or_default().insert(slot);
                }
            }
        }

        stats.unique_glyphs = uploaded_glyphs.len();
        stats
    }

    pub fn uploads(&self) -> usize {
        self.first_uploads + self.reuploads_after_eviction + self.duplicate_while_resident + self.same_slot_rewrites
    }

    pub fn redundant_uploads(&self) -> usize {
        self.reuploads_after_eviction + self.duplicate_while_resident + self.same_slot_rewrites
    }

    pub fn payload_bytes(&self) -> usize {
        self.first_upload_bytes + self.reupload_bytes + self.duplicate_bytes + self.same_slot_bytes
    }

    pub fn redundant_bytes(&self) -> usize {
        self.reupload_bytes + self.duplicate_bytes + self.same_slot_bytes
    }

    pub fn redundant_percentage(&self) -> f64 {
        let payload = self.payload_bytes();
        if payload == 0 {
            0.0
        } else {
            100.0 * self.redundant_bytes() as f64 / payload as f64
        }
    }

    pub fn add(&mut self, other: &Self) {
        self.first_uploads += other.first_uploads;
        self.reuploads_after_eviction += other.reuploads_after_eviction;
        self.duplicate_while_resident += other.duplicate_while_resident;
        self.same_slot_rewrites += other.same_slot_rewrites;
        self.first_upload_bytes += other.first_upload_bytes;
        self.reupload_bytes += other.reupload_bytes;
        self.duplicate_bytes += other.duplicate_bytes;
        self.same_slot_bytes += other.same_slot_bytes;
        self.command_overhead_bytes += other.command_overhead_bytes;
    }
}
