//! Update screen characters using delta blocks with two-level bitmaps and an intra-block mask.
//!
//! See `.designs/block-compressions.md` for the full packet description.
//!
//! A block is `W x H` screen cells. Row-first commands iterate block-rows and then block-columns;
//! column-first commands iterate block-columns and then block-rows. An outer bitmap marks the
//! active bands, and every active band has its own inner bitmap marking the present blocks.
//! Every present block stores a `W * H` bit mask followed by one byte for each set mask bit,
//! taken from the target screen in row-major order.

use crate::encoder::{writer::Writer, Encoder};

use super::{
    UPDATE_SCREEN_BLOCKS_COL_2X3, UPDATE_SCREEN_BLOCKS_COL_2X5, UPDATE_SCREEN_BLOCKS_COL_4X3,
    UPDATE_SCREEN_BLOCKS_COL_4X4, UPDATE_SCREEN_BLOCKS_COL_4X5, UPDATE_SCREEN_BLOCKS_ROW_2X3,
    UPDATE_SCREEN_BLOCKS_ROW_2X5, UPDATE_SCREEN_BLOCKS_ROW_4X3, UPDATE_SCREEN_BLOCKS_ROW_4X4,
    UPDATE_SCREEN_BLOCKS_ROW_4X5,
};

pub const SCREEN_WIDTH: usize = 40;
pub const SCREEN_HEIGHT: usize = 25;
pub const SCREEN_CHARS: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BlockKind {
    Row2x3,
    Row4x3,
    Row2x5,
    Row4x5,
    Col2x3,
    Col4x3,
    Col2x5,
    Col4x5,
    Row4x4,
    Col4x4,
}

impl BlockKind {
    pub const ALL: [BlockKind; 10] = [
        BlockKind::Row2x3,
        BlockKind::Row4x3,
        BlockKind::Row2x5,
        BlockKind::Row4x5,
        BlockKind::Col2x3,
        BlockKind::Col4x3,
        BlockKind::Col2x5,
        BlockKind::Col4x5,
        BlockKind::Row4x4,
        BlockKind::Col4x4,
    ];

    pub fn from_opcode(opcode: u8) -> Option<BlockKind> {
        match opcode {
            UPDATE_SCREEN_BLOCKS_ROW_2X3 => Some(BlockKind::Row2x3),
            UPDATE_SCREEN_BLOCKS_ROW_4X3 => Some(BlockKind::Row4x3),
            UPDATE_SCREEN_BLOCKS_ROW_2X5 => Some(BlockKind::Row2x5),
            UPDATE_SCREEN_BLOCKS_ROW_4X5 => Some(BlockKind::Row4x5),
            UPDATE_SCREEN_BLOCKS_COL_2X3 => Some(BlockKind::Col2x3),
            UPDATE_SCREEN_BLOCKS_COL_4X3 => Some(BlockKind::Col4x3),
            UPDATE_SCREEN_BLOCKS_COL_2X5 => Some(BlockKind::Col2x5),
            UPDATE_SCREEN_BLOCKS_COL_4X5 => Some(BlockKind::Col4x5),
            UPDATE_SCREEN_BLOCKS_ROW_4X4 => Some(BlockKind::Row4x4),
            UPDATE_SCREEN_BLOCKS_COL_4X4 => Some(BlockKind::Col4x4),
            _ => None,
        }
    }

    pub fn opcode(self) -> u8 {
        match self {
            BlockKind::Row2x3 => UPDATE_SCREEN_BLOCKS_ROW_2X3,
            BlockKind::Row4x3 => UPDATE_SCREEN_BLOCKS_ROW_4X3,
            BlockKind::Row2x5 => UPDATE_SCREEN_BLOCKS_ROW_2X5,
            BlockKind::Row4x5 => UPDATE_SCREEN_BLOCKS_ROW_4X5,
            BlockKind::Col2x3 => UPDATE_SCREEN_BLOCKS_COL_2X3,
            BlockKind::Col4x3 => UPDATE_SCREEN_BLOCKS_COL_4X3,
            BlockKind::Col2x5 => UPDATE_SCREEN_BLOCKS_COL_2X5,
            BlockKind::Col4x5 => UPDATE_SCREEN_BLOCKS_COL_4X5,
            BlockKind::Row4x4 => UPDATE_SCREEN_BLOCKS_ROW_4X4,
            BlockKind::Col4x4 => UPDATE_SCREEN_BLOCKS_COL_4X4,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            BlockKind::Row2x3 => "row2x3",
            BlockKind::Row4x3 => "row4x3",
            BlockKind::Row2x5 => "row2x5",
            BlockKind::Row4x5 => "row4x5",
            BlockKind::Col2x3 => "col2x3",
            BlockKind::Col4x3 => "col4x3",
            BlockKind::Col2x5 => "col2x5",
            BlockKind::Col4x5 => "col4x5",
            BlockKind::Row4x4 => "row4x4",
            BlockKind::Col4x4 => "col4x4",
        }
    }

    pub fn width(self) -> usize {
        match self {
            BlockKind::Row2x3 | BlockKind::Row2x5 | BlockKind::Col2x3 | BlockKind::Col2x5 => 2,
            BlockKind::Row4x3
            | BlockKind::Row4x5
            | BlockKind::Col4x3
            | BlockKind::Col4x5
            | BlockKind::Row4x4
            | BlockKind::Col4x4 => 4,
        }
    }

    pub fn height(self) -> usize {
        match self {
            BlockKind::Row2x3 | BlockKind::Row4x3 | BlockKind::Col2x3 | BlockKind::Col4x3 => 3,
            BlockKind::Row2x5 | BlockKind::Row4x5 | BlockKind::Col2x5 | BlockKind::Col4x5 => 5,
            BlockKind::Row4x4 | BlockKind::Col4x4 => 4,
        }
    }

    pub fn column_first(self) -> bool {
        matches!(
            self,
            BlockKind::Col2x3 | BlockKind::Col4x3 | BlockKind::Col2x5 | BlockKind::Col4x5 | BlockKind::Col4x4
        )
    }

    /// Number of bands in the outer bitmap: block-rows for row-first, block-columns for column-first.
    pub fn outer_bands(self) -> usize {
        if self.column_first() {
            SCREEN_WIDTH / self.width()
        } else {
            SCREEN_HEIGHT / self.height()
        }
    }

    /// Number of bands in the inner bitmap: block-columns for row-first, block-rows for column-first.
    pub fn inner_bands(self) -> usize {
        if self.column_first() {
            SCREEN_HEIGHT / self.height()
        } else {
            SCREEN_WIDTH / self.width()
        }
    }

    pub fn outer_bytes(self) -> usize {
        bitmap_byte_count(self.outer_bands())
    }

    pub fn inner_bytes(self) -> usize {
        bitmap_byte_count(self.inner_bands())
    }

    pub fn cell_count(self) -> usize {
        self.width() * self.height()
    }

    /// Number of bytes in the intra-block mask: one bit per cell in row-major order.
    pub fn mask_bytes(self) -> usize {
        bitmap_byte_count(self.cell_count())
    }

    /// True when the block height does not divide the 25-row screen, leaving row 24 uncovered.
    pub fn ignores_last_screen_row(self) -> bool {
        SCREEN_HEIGHT % self.height() != 0
    }

    /// True when `from` and `to` differ on a screen row this command cannot encode.
    pub fn misses_changes(self, from: &[u8; SCREEN_CHARS], to: &[u8; SCREEN_CHARS]) -> bool {
        if !self.ignores_last_screen_row() {
            return false;
        }
        let first = (SCREEN_HEIGHT - 1) * SCREEN_WIDTH;
        from[first..] != to[first..]
    }
}

fn bitmap_byte_count(num_bits: usize) -> usize {
    (num_bits + 7) / 8
}

fn bitmap_bytes(active_indices: &[usize], num_bytes: usize) -> Vec<u8> {
    let mut result = vec![0_u8; num_bytes];
    for &index in active_indices {
        result[index / 8] |= 1 << (index % 8);
    }
    result
}

/// A single present block: its index in the inner bitmap, the changed-cell mask, and target chars.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresentBlock {
    pub inner: usize,
    pub mask: Vec<u8>,
    pub chars: Vec<u8>,
}

/// An active outer band and the present blocks inside it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockBand {
    pub outer: usize,
    pub blocks: Vec<PresentBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateScreenBlocks {
    pub kind: BlockKind,
    pub bands: Vec<BlockBand>,
}

impl UpdateScreenBlocks {
    /// Build the command from the differences between `from` and `to`.
    ///
    /// Variants whose height does not divide 25 ignore screen row 24; callers must check
    /// [`BlockKind::misses_changes`] and keep those cells on another command.
    pub fn transition(from: &[u8; SCREEN_CHARS], to: &[u8; SCREEN_CHARS], kind: BlockKind) -> Self {
        let width = kind.width();
        let height = kind.height();
        let covered_rows = (SCREEN_HEIGHT / height) * height;

        // Per outer band and inner band, remember the top-left cell of the block.
        let mut present = vec![vec![None; kind.inner_bands()]; kind.outer_bands()];
        for y in 0..covered_rows {
            for x in 0..SCREEN_WIDTH {
                let offset = y * SCREEN_WIDTH + x;
                if from[offset] == to[offset] {
                    continue;
                }
                let (outer, inner) = if kind.column_first() {
                    (x / width, y / height)
                } else {
                    (y / height, x / width)
                };
                present[outer][inner] = Some((x / width * width, y / height * height));
            }
        }

        let mut bands = Vec::new();
        for (outer, inner_blocks) in present.iter().enumerate() {
            let mut blocks = Vec::new();
            for (inner, origin) in inner_blocks.iter().enumerate() {
                let Some((x0, y0)) = origin else {
                    continue;
                };
                let mut mask = vec![0_u8; kind.mask_bytes()];
                let mut chars = Vec::new();
                for dy in 0..height {
                    for dx in 0..width {
                        let offset = (y0 + dy) * SCREEN_WIDTH + x0 + dx;
                        if from[offset] == to[offset] {
                            continue;
                        }
                        let bit = dy * width + dx;
                        mask[bit / 8] |= 1 << (bit % 8);
                        chars.push(to[offset]);
                    }
                }
                blocks.push(PresentBlock { inner, mask, chars });
            }
            if !blocks.is_empty() {
                bands.push(BlockBand { outer, blocks });
            }
        }

        Self { kind, bands }
    }

    pub fn present_block_count(&self) -> usize {
        self.bands.iter().map(|band| band.blocks.len()).sum()
    }

    /// Decode this command's payload into the given screen, returning the number of bytes read.
    pub fn decode_payload(&self, payload: &[u8], screen: &mut [u8; SCREEN_CHARS]) -> usize {
        let mut offset = 0;
        decode_block_payload(self.kind, payload, &mut offset, screen);
        offset
    }
}

/// Decode a block payload starting at `*offset`, advancing it past the consumed bytes.
///
/// Used by both the round-trip tests and the Rust evaluator so there is one definition
/// of the packet semantics.
pub fn decode_block_payload(kind: BlockKind, payload: &[u8], offset: &mut usize, screen: &mut [u8; SCREEN_CHARS]) {
    let mut read_byte = || {
        let value = payload[*offset];
        *offset += 1;
        value
    };

    let outer_bitmap = (0..kind.outer_bytes()).map(|_| read_byte()).collect::<Vec<_>>();
    for (outer_byte, band_present) in outer_bitmap.iter().enumerate() {
        for bit in 0..8 {
            let outer_index = outer_byte * 8 + bit;
            if outer_index >= kind.outer_bands() {
                break;
            }
            if band_present & (1 << bit) == 0 {
                continue;
            }
            let inner_bitmap = (0..kind.inner_bytes()).map(|_| read_byte()).collect::<Vec<_>>();
            for (inner_byte_index, inner_byte) in inner_bitmap.iter().enumerate() {
                for inner_bit in 0..8 {
                    let inner_index = inner_byte_index * 8 + inner_bit;
                    if inner_index >= kind.inner_bands() || inner_byte & (1 << inner_bit) == 0 {
                        continue;
                    }
                    let mask = (0..kind.mask_bytes()).map(|_| read_byte()).collect::<Vec<_>>();
                    for cell in 0..kind.cell_count() {
                        if mask[cell / 8] & (1 << (cell % 8)) == 0 {
                            continue;
                        }
                        let dx = cell % kind.width();
                        let dy = cell / kind.width();
                        let (x, y) = if kind.column_first() {
                            (outer_index * kind.width() + dx, inner_index * kind.height() + dy)
                        } else {
                            (inner_index * kind.width() + dx, outer_index * kind.height() + dy)
                        };
                        screen[y * SCREEN_WIDTH + x] = read_byte();
                    }
                }
            }
        }
    }
}

impl Encoder for UpdateScreenBlocks {
    fn byte_size(&self) -> usize {
        self.kind.outer_bytes()
            + self
                .bands
                .iter()
                .map(|band| {
                    self.kind.inner_bytes()
                        + band
                            .blocks
                            .iter()
                            .map(|block| block.mask.len() + block.chars.len())
                            .sum::<usize>()
                })
                .sum::<usize>()
    }

    fn encode<'a>(&self, encoded_data: &'a mut [u8]) -> &'a mut [u8] {
        debug_assert!(encoded_data.len() >= self.byte_size());

        let mut encoded_data = encoded_data;
        let outer_indices = self.bands.iter().map(|band| band.outer).collect::<Vec<_>>();
        for byte in bitmap_bytes(&outer_indices, self.kind.outer_bytes()) {
            encoded_data = encoded_data.add(&byte);
        }
        for band in &self.bands {
            let inner_indices = band.blocks.iter().map(|block| block.inner).collect::<Vec<_>>();
            for byte in bitmap_bytes(&inner_indices, self.kind.inner_bytes()) {
                encoded_data = encoded_data.add(&byte);
            }
            for block in &band.blocks {
                for bit in &block.mask {
                    encoded_data = encoded_data.add(bit);
                }
                for character in &block.chars {
                    encoded_data = encoded_data.add(character);
                }
            }
        }
        encoded_data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn screens() -> ([u8; SCREEN_CHARS], [u8; SCREEN_CHARS]) {
        let from = [0x11_u8; SCREEN_CHARS];
        let mut to = from;
        to[0] = 0xAA;
        to[1] = 0xBB;
        (from, to)
    }

    #[test]
    fn every_kind_has_a_unique_opcode() {
        let mut opcodes = BlockKind::ALL.map(|kind| kind.opcode()).to_vec();
        opcodes.sort_unstable();
        opcodes.dedup();
        assert_eq!(opcodes.len(), 10);
        for opcode in opcodes {
            assert_eq!(BlockKind::from_opcode(opcode).map(|kind| kind.opcode()), Some(opcode));
        }
        assert_eq!(BlockKind::from_opcode(34), None);
        assert_eq!(BlockKind::from_opcode(43), Some(BlockKind::Row4x4));
        assert_eq!(BlockKind::from_opcode(44), Some(BlockKind::Col4x4));
    }

    #[test]
    fn byte_size_matches_encoded_length() {
        let (from, to) = screens();
        for kind in BlockKind::ALL {
            let command = UpdateScreenBlocks::transition(&from, &to, kind);
            let mut bytes = vec![0_u8; command.byte_size()];
            let remaining = command.encode(&mut bytes);
            assert!(remaining.is_empty());
        }
    }

    #[test]
    fn round_trip_all_kinds() {
        let (from, to) = screens();
        for kind in BlockKind::ALL {
            let command = UpdateScreenBlocks::transition(&from, &to, kind);
            let mut payload = vec![0_u8; command.byte_size()];
            command.encode(&mut payload);

            let mut decoded = from;
            let consumed = command.decode_payload(&payload, &mut decoded);
            assert_eq!(consumed, payload.len(), "kind={:?}", kind);
            assert_eq!(decoded, to, "kind={:?}", kind);
        }
    }

    #[test]
    fn mask_contains_only_changed_cells_in_row_major_order() {
        let (from, to) = screens();
        let command = UpdateScreenBlocks::transition(&from, &to, BlockKind::Row2x3);
        let mut payload = vec![0_u8; command.byte_size()];
        command.encode(&mut payload);

        // outer row bitmap, inner column bitmap (3 bytes), then the 2x3 mask and chars.
        assert_eq!(payload[0], 0b0000_0001);
        assert_eq!(payload[1], 0b0000_0001);
        assert_eq!(payload[4], 0b0000_0011, "changed cells at bit 0 and bit 1");
        assert_eq!(payload[5], 0xAA);
        assert_eq!(payload[6], 0xBB);
    }

    #[test]
    fn column_first_has_one_row_bitmap_per_active_column() {
        let mut from = [0_u8; SCREEN_CHARS];
        let mut to = from;
        // Column 0 has changes in two block-rows; column 1 has a change in one.
        to[0] = 1;
        to[6 * SCREEN_WIDTH] = 2;
        to[2] = 3;
        from[0] = 9;
        from[6 * SCREEN_WIDTH] = 9;
        from[2] = 9;

        let command = UpdateScreenBlocks::transition(&from, &to, BlockKind::Col2x3);
        assert_eq!(command.bands.len(), 2);
        assert_eq!(command.bands[0].outer, 0);
        assert_eq!(
            command.bands[0].blocks.iter().map(|b| b.inner).collect::<Vec<_>>(),
            vec![0, 2]
        );
        assert_eq!(command.bands[1].outer, 1);
        assert_eq!(
            command.bands[1].blocks.iter().map(|b| b.inner).collect::<Vec<_>>(),
            vec![0]
        );

        let mut payload = vec![0_u8; command.byte_size()];
        command.encode(&mut payload);
        assert_eq!(payload[0], 0b0000_0011, "outer column bitmap");
        assert_eq!(payload[3], 0b0000_0101, "row bitmap for column 0");
        // 3 outer bytes + 1 row bitmap + two (1-byte mask + 1 char) blocks.
        assert_eq!(payload[8], 0b0000_0001, "row bitmap for column 1");
    }

    #[test]
    fn partial_height_ignores_last_screen_row() {
        let mut from = [0_u8; SCREEN_CHARS];
        let to = from;
        from[(SCREEN_HEIGHT - 1) * SCREEN_WIDTH] = 0xFF;
        assert!(BlockKind::Row2x3.misses_changes(&from, &to));
        assert!(BlockKind::Row4x4.misses_changes(&from, &to));
        assert!(!BlockKind::Row2x5.misses_changes(&from, &to));

        let command = UpdateScreenBlocks::transition(&from, &to, BlockKind::Row2x3);
        assert!(command.bands.is_empty());
    }
}
