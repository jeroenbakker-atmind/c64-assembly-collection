# Delta Block Compression — Detailed Design

## 1. Goal & scope

**Goal:** add a delta block compression mode to the animation encoder that writes only the screen cells that changed since the previous frame, grouped into rectangular blocks, with two-level bitmaps to skip empty bands and empty blocks.

**Scope of this document:** packet format for the 10 new commands, encoder selection rules, Rust reference decoder behaviour, measured gain on the animation actually used by this repository, the C64/6502 decoder, and the staged implementation order.

**Implementation order:** the encoder and Rust decoder were built first so usage and benefit could be measured (see §7 and §8). Those measurements justified the C64/6502 decoder, which is now implemented and documented in Phase 3.

**Inspiration / surroundings:** this design fits the same command stream as `.designs/streaming.md` (frame/chunk based, `u16` frame count → `u16` command count → `opcode + payload`). It is an alternative to the existing `UpdateScreenCharsRLE` command for screen updates, and is selected by encoded size, exactly like the existing candidates in `choose_best_text_mode_update`.

## 2. Terminology & grid geometry

The VIC-II text screen is **40 columns × 25 rows = 1000 cells**. Each cell holds one screen character code (1 byte).

- **Block** — a rectangle of `W × H` screen cells. `W ∈ {2, 4}`, `H ∈ {3, 4, 5}`.
- **Block-row / block-column** — a band of block indices along the vertical / horizontal axis. A block-row is `H` screen rows tall; a block-column is `W` screen columns wide.
- **Row-first** — outer bitmap iterates block-rows (top → bottom); inner bitmap iterates block-columns (left → right) inside an active block-row.
- **Column-first** — outer bitmap iterates block-columns (left → right); inner bitmap iterates block-rows (top → bottom) inside an active block-column.
- **Outer / inner bitmap** — 1 bit per band / per block, packed into bytes. Bit `0` (LSB) of the first byte is band `0` (topmost or leftmost). Unused high bits in the last byte are `0`.
- **Intra-block mask** — `ceil(W·H/8)` bytes, one bit per cell in row-major order (bit 0 = top-left). A set bit means that cell changed.
- **Present block** — a block with at least one changed cell. Its payload is its **intra-block mask** followed by **one byte per set mask bit**, taken from the target screen in the same row-major order. Unchanged cells are not written.

| Block | W | H | Block-rows (`floor(25/H)`) | Block-cols (`40/W`) | Screen rows covered |
|---|---:|---:|---:|---:|---|
| 2×3 | 2 | 3 | 8 | 20 | 0–23 |
| 4×3 | 4 | 3 | 8 | 10 | 0–23 |
| 2×5 | 2 | 5 | 5 | 20 | 0–24 |
| 4×5 | 4 | 5 | 5 | 10 | 0–24 |
| 4×4 | 4 | 4 | 6 | 10 | 0–23 |

**Heights 3 and 4 ignore the last line.** `floor(25/3) = 8` and `floor(25/4) = 6` bands cover rows 0–23; screen row 24 is not representable by those commands. If a frame changes row 24, the encoder pairs the block command with a row-24-only partial update and includes its bytes in the size comparison. Height 5 covers all 25 rows.

**5 dimensions × 2 orientations = 10 encoding commands.**

## 3. Opcodes

Existing opcodes are 1, 2, 3, 16, 17, 32, 33, 34 (see `src/c64-encoder/src/command/mod.rs`). The contiguous range **35–44** keeps the new screen-update commands next to `UPDATE_SCREEN_CHARS_RLE = 34`.

| Opcode | Name | Orientation | W | H |
|---:|---|---|---:|---:|
| 35 | `UPDATE_SCREEN_BLOCKS_ROW_2X3` | row-first | 2 | 3 |
| 36 | `UPDATE_SCREEN_BLOCKS_ROW_4X3` | row-first | 4 | 3 |
| 37 | `UPDATE_SCREEN_BLOCKS_ROW_2X5` | row-first | 2 | 5 |
| 38 | `UPDATE_SCREEN_BLOCKS_ROW_4X5` | row-first | 4 | 5 |
| 39 | `UPDATE_SCREEN_BLOCKS_COL_2X3` | column-first | 2 | 3 |
| 40 | `UPDATE_SCREEN_BLOCKS_COL_4X3` | column-first | 4 | 3 |
| 41 | `UPDATE_SCREEN_BLOCKS_COL_2X5` | column-first | 2 | 5 |
| 42 | `UPDATE_SCREEN_BLOCKS_COL_4X5` | column-first | 4 | 5 |
| 43 | `UPDATE_SCREEN_BLOCKS_ROW_4X4` | row-first | 4 | 4 |
| 44 | `UPDATE_SCREEN_BLOCKS_COL_4X4` | column-first | 4 | 4 |

The C64 dispatcher is generated from `all_decoder_modules()`. Opcode 35–44 entries are now registered there, after the 6502 decoder modules were implemented. The Rust evaluator (§6) does not use that registry; it dispatches through `BlockKind::from_opcode()`.

## 4. Packet format

### 4.1 Common rules

1. Byte 0 is the opcode (35–44).
2. Then the **outer bitmap**: 1 byte for row-first (8, 6 or 5 used bits), 2 bytes for column-first with W=4 (10 used bits), 3 bytes for column-first with W=2 (20 used bits).
3. For each set bit in the outer bitmap, in increasing index order: the **inner bitmap** (1–3 bytes depending on orientation/dimension).
   - **Column-first explicitly:** the outer bitmap marks which block-columns have any changed block. **Every column with a set bit gets its own row bitmap**, immediately following the column bitmap, and that row bitmap is followed by the block payloads for that column only. Row bitmaps are never shared between columns; the group `(row bitmap, block payloads)` repeats once per active column.
   - Row-first is the mirror image: the outer bitmap marks active block-rows, and every active row gets its own column bitmap followed by that row's block payloads.
4. For each set bit in the inner bitmap, in increasing index order: `ceil(W·H/8)` mask bytes, then one target character byte for every set mask bit. Both mask and characters are row-major inside the block (top-left first, left→right then top→bottom).
5. Blocks are emitted in the same scan order as the bitmaps. There is no block count; the bitmaps are the complete structure.
6. Total size: `1 + ceil(outer_bits/8) + Σ_active_bands(ceil(inner_bits/8) + Σ_present_blocks(ceil(W·H/8) + changed_cells_in_block))`.

`active_outer_bands` is the number of set bits in the outer bitmap; `present_blocks` is the total number of set bits across all inner bitmaps. The mask is 1 byte for 2×3, 2 bytes for 4×3/2×5/4×4, and 3 bytes for 4×5.

### 4.2 Mermaid packet diagrams

Each diagram shows one active outer band and two present blocks; the inner-bitmap + block group repeats for every active outer band, and each block group repeats for every set inner bit. Bit ranges are illustrative: the number of changed-character bytes in a block is variable (one per set mask bit).

#### Opcode 35 — row-first 2×3

```mermaid
packet-beta
  0-7 "Opcode 35"
  8-15 "Active block-row bitmap (8 bits)"
  16-39 "Active block-col bitmap (20 used / 24 sent)"
  40-87 "Present block 0: mask(1B) + changed chars (2x3)"
  88-135 "Present block 1: mask(1B) + changed chars (2x3)"
```

#### Opcode 36 — row-first 4×3

```mermaid
packet-beta
  0-7 "Opcode 36"
  8-15 "Active block-row bitmap (8 bits)"
  16-31 "Active block-col bitmap (10 used / 16 sent)"
  32-127 "Present block 0: mask(2B) + changed chars (4x3)"
  128-223 "Present block 1: mask(2B) + changed chars (4x3)"
```

#### Opcode 37 — row-first 2×5

```mermaid
packet-beta
  0-7 "Opcode 37"
  8-15 "Active block-row bitmap (5 used / 8 sent)"
  16-39 "Active block-col bitmap (20 used / 24 sent)"
  40-119 "Present block 0: mask(2B) + changed chars (2x5)"
  120-199 "Present block 1: mask(2B) + changed chars (2x5)"
```

#### Opcode 38 — row-first 4×5

```mermaid
packet-beta
  0-7 "Opcode 38"
  8-15 "Active block-row bitmap (5 used / 8 sent)"
  16-31 "Active block-col bitmap (10 used / 16 sent)"
  32-191 "Present block 0: mask(3B) + changed chars (4x5)"
  192-351 "Present block 1: mask(3B) + changed chars (4x5)"
```

#### Opcode 39 — column-first 2×3

```mermaid
packet-beta
  0-7 "Opcode 39"
  8-31 "Active block-col bitmap (20 used / 24 sent)"
  32-39 "Active block-row bitmap (8 bits)"
  40-87 "Present block 0: mask(1B) + changed chars (2x3)"
  88-135 "Present block 1: mask(1B) + changed chars (2x3)"
```

#### Opcode 40 — column-first 4×3

```mermaid
packet-beta
  0-7 "Opcode 40"
  8-23 "Active block-col bitmap (10 used / 16 sent)"
  24-31 "Active block-row bitmap (8 bits)"
  32-127 "Present block 0: mask(2B) + changed chars (4x3)"
  128-223 "Present block 1: mask(2B) + changed chars (4x3)"
```

#### Opcode 41 — column-first 2×5

```mermaid
packet-beta
  0-7 "Opcode 41"
  8-31 "Active block-col bitmap (20 used / 24 sent)"
  32-39 "Active block-row bitmap (5 used / 8 sent)"
  40-119 "Present block 0: mask(2B) + changed chars (2x5)"
  120-199 "Present block 1: mask(2B) + changed chars (2x5)"
```

#### Opcode 42 — column-first 4×5

```mermaid
packet-beta
  0-7 "Opcode 42"
  8-23 "Active block-col bitmap (10 used / 16 sent)"
  24-31 "Active block-row bitmap (5 used / 8 sent)"
  32-191 "Present block 0: mask(3B) + changed chars (4x5)"
  192-351 "Present block 1: mask(3B) + changed chars (4x5)"
```

#### Opcode 43 — row-first 4×4

```mermaid
packet-beta
  0-7 "Opcode 43"
  8-15 "Active block-row bitmap (6 used / 8 sent)"
  16-31 "Active block-col bitmap (10 used / 16 sent)"
  32-159 "Present block 0: mask(2B) + changed chars (4x4)"
  160-287 "Present block 1: mask(2B) + changed chars (4x4)"
```

#### Opcode 44 — column-first 4×4

```mermaid
packet-beta
  0-7 "Opcode 44"
  8-23 "Active block-col bitmap (10 used / 16 sent)"
  24-31 "Active block-row bitmap (6 used / 8 sent)"
  32-159 "Present block 0: mask(2B) + changed chars (4x4)"
  160-287 "Present block 1: mask(2B) + changed chars (4x4)"
```

### 4.3 Worked example (opcode 35, row-first 2×3)

Target frame differs from the previous frame in exactly two cells: `(x=0,y=0)` and `(x=2,y=1)`.

- Block-row 0 (rows 0–2) is active; block-cols 0 and 1 are active.
- Outer bitmap = `0b0000_0001` → `0x01`.
- Inner bitmap = `0b0000_0000_0000_0011` → `0x03, 0x00, 0x00` (20 used bits, 24 sent).
- Block 0 covers `(x=0..1,y=0..2)`; only `(0,0)` changed, which is mask bit 0 → mask `0x01`, one char byte.
- Block 1 covers `(x=2..3,y=0..2)`; only `(2,1)` changed, which is mask bit 2 → mask `0x04`, one char byte.
- Encoded bytes: `23 01 03 00 00 01 <c(0,0)> 04 <c(2,1)>` (`0x23` = opcode 35; char values illustrative).

Only the two changed cells are written; the other 10 cells in those blocks are untouched.

**Column-first example (opcode 39, column-first 2×3).** Target frame differs at `(x=0,y=0)` and `(x=0,y=6)` — two block-rows in the same block-column 0 (columns 0–1).

- Outer (block-column) bitmap: only column 0 is active → `0x01, 0x00, 0x00` (20 used bits, 24 sent).
- Row bitmap **for column 0**: block-rows 0 and 2 are active → `0b0000_0101` = `0x05`.
- Block-row 0: `(0,0)` changed, mask bit 0 → `0x01` + one char byte.
- Block-row 2: `(0,6)` changed, mask bit 0 → `0x01` + one char byte.
- Bytes: `27 01 00 00 05 01 <c(0,0)> 01 <c(0,6)>`. If another column were active, its own row bitmap and payload group would follow immediately; the two groups are independent.

## 5. Encoder algorithm

### 5.1 Command construction (per candidate)

For a given `(W, H, orientation)`:

1. Mark `changed[y*40+x] = (from[y*40+x] != to[y*40+x])` for all cells in the covered region. For `H ∈ {3, 4}`, ignore `y == 24`.
2. Group changed cells by outer band index (`y/H` for row-first, `x/W` for column-first), then by inner band index (`x/W` for row-first, `y/H` for column-first).
3. For each present block, build the `ceil(W·H/8)`-byte intra-block mask in row-major order and collect the target characters for its set bits, also in row-major order.
4. Emit the outer bitmap, then for each active outer band its inner bitmap, then for each set inner bit the block's mask followed by its changed-character bytes.
5. Compute `byte_size()` from the formula in §4.1 (this includes the 1-byte opcode, matching `Command::byte_size()`).

### 5.2 Selection

`choose_best_text_mode_update_with_report(from_screen, to_screen, ...)` in `src/c64/examples/encode_cube.rs` compares:

- full `UpdateTextModeScreen`,
- clear + update,
- no-op,
- `PartialUpdateTextModeScreen::transition`,
- `UpdateScreenCharsRLE::transition`.

The 10 block candidates (5 dimensions × 2 orientations) are added to this comparison. The smallest `byte_size()` wins, preserving the existing behaviour that an earlier command remains the default when it is smaller.

**Row-24 handling (implemented):** for `H ∈ {3, 4}`, no changed cell with `y == 24` is silently dropped. If such a change exists, the candidate is paired with a `PartialUpdateTextModeScreen` that writes only row 24, and that command's bytes are included in the candidate's size. Height-5 variants need no fallback.

**Never emit an empty block command** (all-zero outer bitmap). The existing no-op candidate already covers that case; emitting `opcode + zero bitmap` would waste 1–3 bytes.

### 5.3 Code organisation (encoder and command model, phase 1)

- `src/c64-encoder/src/command/mod.rs`: 10 opcode constants (35–44), a `Command` variant carrying the block command, `Command::opcode()`, and `Encoder` wiring. Decoder registration was added later in Phase 3.
- `src/c64-encoder/src/command/update_screen_blocks.rs` containing:
  - `enum BlockKind` with the 10 variants (`Row2x3`, `Row4x3`, `Row2x5`, `Row4x5`, `Col2x3`, `Col4x3`, `Col2x5`, `Col4x5`, `Row4x4`, `Col4x4`) and `opcode()`, `width()`, `height()`, `column_first()`, `outer_bands()`, `inner_bands()`, `mask_bytes()`.
  - `PresentBlock { inner, mask, chars }` and `BlockBand { outer, blocks }`, held by `UpdateScreenBlocks { kind, bands }`.
  - `UpdateScreenBlocks::transition(from, to, kind) -> Self`.
  - `impl Encoder for UpdateScreenBlocks` (bit packing + mask/chars payload) and `decode_block_payload()` shared with the evaluator.
- `src/c64/examples/encode_cube.rs`: register the 10 candidates. The final all-acts command summary is printed by `c64-encoder-reporting` (§7.5).

## 6. Rust decoder (phase 2)

`evaluator::evaluate()` in `src/c64-encoder/src/evaluator/mod.rs` interprets the stream into per-frame `State` snapshots. The implementation uses one branch that dispatches through `BlockKind::from_opcode()` to a shared helper, `decode_block_payload()`:

```text
read outer_bitmap (outer_bytes bytes)
for outer_index in 0..outer_bands where bit set:
    read inner_bitmap (inner_bytes bytes)
    for inner_index in 0..inner_bands where bit set:
        read mask (mask_bytes bytes)
        for cell in 0..W*H where mask bit is set:
            dx = cell % W
            dy = cell / W
            x = column_first ? outer_index*W + dx : inner_index*W + dx
            y = column_first ? inner_index*H + dy : outer_index*H + dy
            state.text_screen.screen_chars[y*40 + x] = read_u8()
```

The decoder writes **only the set mask bits**, in the same row-major order used by the encoder. It must not clear or reset the screen first — the previous frame stays visible until the overwritten cells are written.

`read_u16` in the evaluator is little-endian (LOW then HIGH); the new branch only reads bytes, so it inherits that convention.

### Tests

- Unit tests for `UpdateScreenBlocks::transition`: correct band/block bitmaps, mask/char payload order, `byte_size()` matches `encode()` output length, and heights 3/4 ignore row 24.
- Round-trip tests for all 10 kinds: `transition(from, to, kind)` → `encode()` → decode helper → assert the resulting 1000-cell screen equals `to` (excluding ignored row 24 for partial-height kinds).
- Column-first test: each active block-column has its own row bitmap.
- Full-animation equivalence: at measurement time, re-running `encode_cube` regenerated all 103 debug PNGs with the same aggregate SHA-256. This exercises the evaluator path for the block opcodes on real frames. The current animation has 202 debug PNGs (see §7.5).

## 7. Measured gain for the used animation

At measurement time the animation was `resources/render/001/*.png` (Act 1, 100 frames) and `resources/render/002/*.png` (Act 2, 1 frame), encoded by `cargo run -p c64 --example encode_cube`. The repository now also has 100 Act 2 frames on disk; current totals are in §7.5.

The numbers below are **measured from the implemented encoder and Rust decoder**, not estimates. `encode_cube` reports the baseline (existing screen-update commands only) and the result after enabling all 10 block candidates on every frame. Baseline = the smallest existing screen-update candidate for that frame, including opcode bytes; structural frame headers and charset updates are excluded from the screen-byte totals but included in the stream totals.

### 7.1 Baseline and measured result

| Act | Frames | Stream before | Stream after | Screen bytes before | Screen bytes after |
|---|---:|---:|---:|---:|---:|
| 1 | 100 | 11,624 B | **10,405 B** | 7,881 B | **6,662 B** |
| 2 | 1 | 368 B | 368 B | 283 B | 283 B |

The whole Act-1 stream shrinks by **1,219 B (10.49%)**; the entire gain comes from screen updates. As a fraction of screen-update bytes, the gain is **15.47%**.

Exact Act-1 command split after the change (including opcode bytes): `SetPalette4` 3 B, `UpdateCharsRangedU16` 371 B, `UpdateCharsU16` 3,167 B, `UpdateScreenCharsRLE` 684 B, `UpdateScreenBlocks(col2x3)` 2,452 B, `UpdateScreenBlocks(col4x4)` 3,526 B, headers 202 B. Charset updates are still 3,538 B; screen updates are now 6,662 B; headers remain 202 B.

54 of the 100 Act-1 frames have a non-empty screen update (frame 1 and frames 27–79); the other 46 are no-op frames that still cost 2 B each for the command-count header. The Act-1 average is now 104.1 B/frame overall, down from 116.2 B/frame.

Changed screen cells, comparing decoded frames: **4,561 total**; per active frame min 22, median 78, mean 84.5, max 189. Existing RLE uses 3,572 packets over 43,122 covered cells, of which 37,740 are skip cells. Measured horizontal changed-run lengths: 1,627 runs, mean 2.8, most commonly 1–4 cells. The per-cell mask is what makes block commands competitive with that short-run structure.

Timing context: the engine waits three vblanks per encoded frame, so 100 frames play in ~6.0 s PAL (50 Hz) / ~5.0 s NTSC (60 Hz), i.e. ~16.7 / ~20 new frames per second. Against the streaming budget in `.designs/streaming.md` (~80 B/frame at 4 KB/s PAL), neither the current 104.1 B/frame average nor the peak frame fits without buffering/frame holds; the block format alone will not close that gap.

### 7.2 Implemented mask payload

Totals if **every changed frame used one block command of a single variant** (opcode included; summed over frames where that variant emitted a command):

| Variant | Act 1 total | Δ vs 7,881 B | Act 2 total | Δ vs 283 B |
|---|---:|---:|---:|---:|
| row 2×3 | 8,335 | +454 | 923 | +640 |
| row 4×3 | 8,552 | +671 | 917 | +634 |
| row 2×5 | 8,891 | +1,010 | 877 | +594 |
| row 4×5 | 8,484 | +603 | 834 | +551 |
| col 2×3 | 7,950 | +69 | 924 | +641 |
| col 4×3 | 8,232 | +351 | 914 | +631 |
| col 2×5 | 8,809 | +928 | 884 | +601 |
| col 4×5 | 8,366 | +485 | 835 | +552 |
| row 4×4 | 8,020 | +139 | 885 | +602 |
| **col 4×4** | **7,824** | **−57** | 886 | +603 |

These totals include the row-24 fallback described in §5.2: **12 of 540** Act-1 block candidates (and **6 of 10** Act-2 candidates) needed it. Only column-first 4×4 beats the baseline screen commands when used alone; the other variants still lose in aggregate, but many win on individual frames.

Actual encoder behaviour (pick the cheapest of all candidates per frame):

| Act | Baseline screen bytes | With block selection | Gain | % of screen bytes | Stream before → after |
|---|---:|---:|---:|---:|---:|
| 1 | 7,881 | 6,662 | **1,219 B** | **15.47%** | 11,624 → 10,405 B |
| 2 | 283 | 283 | **0 B** | 0% | 368 → 368 B |

**49 of 54 changed frames** benefit in Act 1: 23 choose `col2x3` and 26 choose `col4x4`. The other 5 changed frames keep `UpdateScreenCharsRLE`; the 46 unchanged frames keep the no-op. The gain is large enough to justify the format technically, but the C64 decoder remains a separate, explicitly deferred decision (§8).

### 7.3 Variants tried and rejected

The original full-block payload (one byte per cell, no intra-block mask) saved only 30 B / 0.38% on Act 1. Adding the intra-block mask to the same eight sizes was simulated at 1,153 B saved; adding 4×4 as well produced the measured 1,219 B above.

A 3×3 variant was simulated in both orientations, with and without the mask. It needed a two-byte mask (9 cells) and coverage of only 39×24, and added **zero** benefit once the other variants were present. It was not implemented.

An independent square-block estimate also found 4×4 to be the sweet spot; finer grids drown in mask/index overhead and larger blocks include too many unchanged cells.

### 7.4 Measured gain

The per-frame gain tables were removed from `encode_cube`; the numbers below were recorded before that removal and remain the measurement of the block-selection change.

| Act | Strategy | Baseline screen B | With blocks B | Gain B | Changed frames | Block wins | Candidates | Row-24 fallbacks | Stream B |
|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | initial | 7,881 | 6,662 | **1,219** | 54 | 49 | 540 | 12 | 10,405 |
| 2 | one_charmap | 283 | 283 | 0 | 1 | 0 | 10 | 6 | 368 |

The full-animation equivalence check passed: regenerating all 103 debug PNGs produced the same aggregate SHA-256 as before block commands were enabled (`806825d59821b76f6767731f53f940e4226e0d916bf8e632991b8b46e85e28ad`).

### 7.5 All-acts command and redundancy report

`print_command_summary()` in `c64-encoder-reporting/src/reports/command_report.rs` prints three Markdown tables: per-act stream totals, emitted command counts/bytes by opcode, and charset-upload redundancy. With all 100 Act 2 frames on disk, the current stream totals are:

| Act | Strategy | Frames | Commands | Stream bytes |
|---:|---|---:|---:|---:|
| 1 | initial | 100 | 105 | 10,405 |
| 2 | one_charmap | 100 | 47 | 1,956 |
| **total** |  | **200** | **152** | **12,361** |

The remaining tables in this section are the original one-frame Act 2 measurement. Its final stream totals were:

| Act | Strategy | Frames | Commands | Stream bytes |
|---:|---|---:|---:|---:|
| 1 | initial | 100 | 105 | 10,405 |
| 2 | one_charmap | 1 | 3 | 368 |
| **total** |  | **101** | **108** | **10,773** |

Non-zero emitted commands (the report also lists all zero-count catalog entries):

| Opcode | Command | Count | Bytes |
|---:|---|---:|---:|
| 1 | clear_screen_chars | 1 | 2 |
| 2 | set_palette4 | 1 | 3 |
| 16 | update_chars_u16 | 49 | 3,167 |
| 17 | update_chars_ranged_u16 | 2 | 452 |
| 34 | rle | 6 | 965 |
| 39 | block:col2x3 | 23 | 2,452 |
| 44 | block:col4x4 | 26 | 3,526 |

Charset redundancy totals:

| Act | Uploads | Unique glyphs | First uploads | Reuploads after eviction | Duplicates while resident | Redundant uploads | Payload B | Redundant B | Redundant % |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 1,207 | 846 | 845 | 361 | 1 | 362 | 3,437 | 1,085 | 31.57% |
| 2 | 39 | 39 | 38 | 0 | 1 | 1 | 78 | 2 | 2.56% |
| **total** | 1,246 | - | 883 | 361 | 2 | 363 | 3,515 | 1,087 | 30.92% |

Definitions and caveats are documented separately in `.designs/charset-redundancy.md`.

## 8. Implementation phases

### Phase 0 — design (this document)

✅ Completed by creating this file.

### Phase 1 — encoder only

✅ Completed.

1. Added `BlockKind`, `UpdateScreenBlocks`, the `Encoder` implementation, and 10 opcodes (§5.3).
2. Wired the 10 candidates into `choose_best_text_mode_update_with_report`.
3. Added encoder unit tests (§6). `all_decoder_modules()` was **not** touched.
4. Existing commands and defaults remain intact; block commands are a pure addition selected by size.

**Gate passed:** `cargo test -p c64-encoder --lib` passes (8 tests); `cargo fmt --all` passes.

### Phase 2 — Rust decoder + measurement

✅ Completed.

1. Extended `evaluator::evaluate()` with the 10 opcodes via `decode_block_payload()` (§6).
2. Added round-trip tests for all 10 kinds, the column-first row-bitmap test, and the row-24 tests.
3. Added the final command summary and the charset-redundancy report (§7.5). The per-frame gain tables were subsequently removed; their recorded measurements remain in §7.4.
4. Updated §7 with the exact measured numbers and command usage.

**Gate passed:** regenerated debug PNGs are byte-identical to the previous run, `cargo test -p c64-encoder --lib` passes (8 tests), and `encode_cube` builds and runs. The report confirms a 1,219 B / 15.47% screen-byte gain. This completed the "check where it is used and how much benefit it has" step.

### Phase 3 — C64 decoder

✅ Completed.

Phase 2 showed a 1,219 B (15.47%) screen-byte gain, which justified implementing the C64 decoder.

1. `src/c64-encoder/src/command/modules/update_screen_blocks.rs` implements one shared, config-driven 6502 decoder, `UpdateScreenBlocksDecoder`, in `update_screen_blocks__process`. It reads the outer bitmap, then for each active band reads that band's inner bitmap, then for each present block reads its mask and changed characters. Writes go through `(SCREEN_CHAR_PTR),Y`; only mask-selected cells are written.
2. Ten thin wrappers from `modules::decoder_module(kind)` skip the opcode byte, load geometry into zero-page scratch (`BLOCK_*` in `0xA0..=0xC3`), and tail-jump into the shared decoder. Row-first uses outer step `H*40` and inner step `W`; column-first uses outer step `W` and inner step `H*40`. `BLOCK_CELL_MASK = W-1` and `BLOCK_ROW_WRAP = 41-W` make cell advancement wrap at the right screen column regardless of the block's starting `x`.
3. `X` is saved with `TXA/PHA` and restored with `PLA/TAX` in `update_screen_blocks__process_block` and `update_screen_blocks__process_outer_band`. Those routines use `X` as a bitmap/mask byte index and call nested routines that would otherwise clobber it. Separate counters (`BLOCK_OUTER_BIT_COUNT`, `BLOCK_INNER_BIT_COUNT`, `BLOCK_MASK_BIT_COUNT`) prevent nested calls from sharing a count.
4. `all_decoder_modules()` registers opcodes 35–44, so `command_dispatcher()` emits all ten `cmp/bne/jmp` branches. The existing full-screen opcode 32 was also given a `DecoderModule` and registered; previously the dispatcher silently skipped it.
5. `src/c64-encoder/tests/decode_update_screen_blocks.rs` runs the generated 6502 code in the `mos6502` emulator. It covers all ten kinds, multi-byte outer/inner bitmaps, multi-byte masks, multiple active bands, row-24 exclusion for heights 3 and 4, row-24 coverage for height 5, final `CURRENT_PTR`, opcode registration, engine generation, and decoding through the full engine dispatcher.
6. `src/c64-encoder/tests/decode_update_text_mode_screen.rs` verifies the newly added opcode 32 decoder.

**Gate passed:** all 6 block decoder tests and the full-screen decoder test pass; `cargo test --workspace`, `cargo fmt --all -- --check`, and `cargo build -p c64 --example encode_cube` pass. The checked-in embedded streams in `src/builder/src/applications/intro.rs` and `engine.rs` were not regenerated by this change; `encode_cube` writes debug PNGs, not those source arrays, so regenerating them is a separate packaging step. Real VICE playback was validated separately.

## 9. Validation checklist

- [x] `byte_size()` equals the number of bytes written by `encode()` for all 10 variants.
- [x] Bit order: bit 0 = topmost/leftmost band; padding bits zero.
- [x] Heights 3 and 4 never claim row 24; the encoder adds a row-24 fallback and tests verify the exclusion.
- [x] Mask and character payloads are in row-major order; tests cover row-first and column-first blocks.
- [x] Evaluator output equals target screen for synthetic and real frames.
- [x] Full-animation debug PNGs unchanged after enabling block commands.
- [x] Command-summary and charset-redundancy reports recorded in §7 with exact numbers; measured gain retained in §7.4.
- [x] 6502 emulator tests pass for all ten block opcodes, including dispatcher integration.
- [x] Real VICE playback validated.

## 10. Risks & constraints

- **Opcode space:** 35–44 are allocated to the block commands and all have registered 6502 modules; `all_decoder_modules()` must continue to have a decoder for every emitted opcode or the dispatcher will silently skip it.
- **Screen visibility:** like the existing RLE, block writes happen while the previous frame is visible. Writing the mask-selected cells is safe (values are from the target frame) but must not be combined with a screen clear.
- **Row 24:** do not silently drop changes; either cover with a height-5 variant or leave those cells to an existing command and account for its bytes in the size comparison.
- **Bandwidth:** the improved Act-1 average is still about 104 B/frame, above the ~80 B/frame streaming budget in `.designs/streaming.md`. Buffering or frame holds remain necessary.
- **Charset churn:** 361 Act-1 glyph reuploads cost 1,085 B, but only 256 glyphs can be resident. See `.designs/charset-redundancy.md` before treating that number as recoverable.

## 11. Decisions and remaining status

1. **Payload granularity — resolved.** The implemented format is an intra-block mask plus one byte per set bit (§4). The original full-block payload saved only 30 B; the mask payload saves 1,219 B.
2. **3×3 variants — rejected.** They added no benefit once the other nine variants were available.
3. **Bit order — resolved.** LSB-first is implemented and covered by unit tests.
4. **Opcode range — resolved.** 35–44 are allocated for the 10 commands.
5. **Row-24 handling — resolved.** Heights 3 and 4 are paired with a row-24-only partial update, whose bytes count toward candidate size.
6. **Redundancy reporting — resolved.** The scanner and `CommandSummary` live in the `c64-encoder-reporting` crate; the Markdown report is printed by `encode_cube`. Definitions and measurements are in `.designs/charset-redundancy.md`.
7. **C64/6502 decoder — resolved.** Phase 2 measured a meaningful benefit, Phase 3 was explicitly requested, and the shared decoder plus ten wrappers are implemented, registered, and emulator-tested.
8. **Embedded stream data — unchanged.** `encode_cube` does not write `src/builder/src/applications/intro.rs`, `engine.rs`, or `development.D64`; those artifacts still need a separate regeneration step before they contain block commands.
