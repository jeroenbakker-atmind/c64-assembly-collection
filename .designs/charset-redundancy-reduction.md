# Charset Redundancy Reduction — Plan

## 1. Goal

Reduce charset-upload churn in the final animation stream, especially in Act 1, without changing the stream format or the rendered frames.

“Frequently used” must not mean only frame count. A glyph with one long run cannot be reuploaded. The glyphs worth protecting are those with many on-screen cells, several separated runs, and short repeat gaps.

## 2. Current baseline

From `.designs/charset-redundancy.md`:

| Act | Uploads | Unique glyphs | Reuploads after eviction | Redundant B | Redundant % | Stream B |
|---:|---:|---:|---:|---:|---:|---:|
| 1 | 1,207 | 846 | 361 | 1,085 | 31.57% | 10,405 |
| 2 | 170 | 170 | 0 | 2 | 0.59% | 1,956 |
| **total** | 1,377 | - | 361 | 1,087 | 28.78% | 12,361 |

Act 2 was measured with all 100 frames on disk and is already near optimal. The work should focus on `build_frames_initial()` in `src/c64/examples/encode_cube.rs`.

## 3. Why churn happens

`build_frames_initial()`:

1. builds `usages: HashMap<BitEncodedChar, Vec<usize>>`,
2. sorts glyphs by `(frame count, glyph bits)`,
3. assigns each glyph to the first slot whose frames are free,
4. reserves one extra frame after a run ends so a slot is not changed while its old glyph is still visible,
5. dilates gap frames so every slot has a value.

The first-fit step reuses a slot during a gap between two runs of the same glyph. When that glyph reappears, the slot must be written again. That is a reupload after eviction.

The fact that Act 1 has 846 distinct glyphs does **not** by itself prove that 590 reuploads are unavoidable. If the active intervals of glyphs fit within 256 slots, a better interval-coloring allocator can keep every glyph in one slot for its whole life. Phase 2 measures this.

## 4. Phase 1 — Diagnose before changing behavior

No encoder behavior change in this phase.

### 4.1 Build per-glyph usage records

In `encode_cube.rs`, create a usage record containing:

- encoded glyph value
- total cell occurrences across all frames
- number of frames used
- consecutive run list `(start, end)`
- guard frame after each run
- reupload count and redundant bytes from the final stream

Use deterministic tie-breakers from glyph bits.

### 4.2 Extend the redundancy checker

Extend `src/c64-encoder-reporting/src/checks/char_redundancy.rs` with an optional detail mode:

- top reuploaded glyphs by redundant bytes
- their cell count and frame count
- their repeat-gap distribution
- whether they qualify as “hot” by cell usage

This is the explicit check that frequently used glyphs are not being overwritten.

### 4.3 Add per-strategy comparison

Add a table to `src/c64-encoder-reporting/src/reports/command_report.rs`:

| strategy | uploads | reuploads | redundant B | redundant % | stream B |
|---|---:|---:|---:|---:|---:|

Compare `initial`, `one_charmap`, and `static_with_dynamic_charset` directly.

## 5. Phase 2 — Interval-aware slot assignment

Replace first-fit-by-usage-count with interval coloring.

### 5.1 Build extended intervals

For each glyph, build intervals from its usage runs plus the guard frame after each run. The guard frame keeps a slot unchanged while the old glyph is still visible.

### 5.2 Best-fit assignment

Process intervals by start frame. For each interval, choose the free slot whose last used frame is latest, instead of the first free slot in allocation order.

This minimizes the number of simultaneously occupied slots and avoids unnecessary slot reuse.

### 5.3 Measure overlap

Report:

- maximum number of extended intervals covering any frame
- number of slots actually used
- reupload count and redundant bytes

If `max_overlap <= 256`, every glyph can keep one slot for its whole life. Reuploads should fall to zero.

If `max_overlap > 256`, the excess is the real capacity limit and Phase 3 is required.

### 5.4 Dilation

Once slot ownership is explicit, simplify dilation where possible. Unused slots can stay zero. Do not emit charset writes for unused slots.

## 6. Phase 3 — Pin hot glyphs and add a cache policy

Only needed if Phase 2 shows genuine over-capacity.

### 6.1 Pinning

Select glyphs from Phase 1 with the highest avoidable reupload cost and highest cell usage.

A pinned glyph gets one dedicated slot for the whole animation. It is uploaded once and never evicted.

### 6.2 Belady-style allocation for the rest

For the remaining glyphs, process frames in order:

- use a free slot when one exists,
- otherwise evict the resident glyph whose next use is farthest away,
- never evict a glyph whose slot is still visible in the previous frame.

This keeps short-gap glyphs resident and pushes out glyphs that will not be needed soon.

### 6.3 Pin-budget sweep

Try a deterministic set of pin budgets, for example:

- `0`
- `16`
- `32`
- `64`
- `96`

For each budget, record stream bytes and redundancy. Choose the smallest stream. Break ties with lower redundant bytes.

### 6.4 Add a strategy variant

Add `Strategy::PinnedInitial` so the existing `initial` strategy remains a baseline. If the new strategy produces the smallest stream, `encode_act()` will select it automatically.

## 7. Phase 4 — Validation and documentation

### 7.1 Commands

Run:

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo run -p c64 --example encode_cube
```

Then check the debug PNG aggregate hash:

```bash
find resources/render -name 'debug.*.png' -type f -print0 | sort -z | xargs -0 shasum -a 256 | shasum -a 256
```

Expected hash for the current 202 debug PNGs:

```text
476e1fa29fd2bcdcfc833daf10c114e2f323623b59991c89c5fd8c3813d677cd
```

Also run VICE playback, because slot numbers change even when rendered output is identical.

### 7.2 Acceptance criteria

- Act 1 redundant bytes drop below the current `1,085 B / 31.57%`.
- Final chosen stream does not grow beyond `12,361 B`.
- All 202 debug PNGs remain identical (`476e1fa29fd2bcdcfc833daf10c114e2f323623b59991c89c5fd8c3813d677cd`).
- VICE playback remains correct.

### 7.3 Documentation

Update `.designs/charset-redundancy.md` with:

- the interval-coloring and pinning algorithm,
- hot-glyph definitions,
- before/after measurements,
- the remaining unavoidable churn caused by 256-slot capacity.

## 8. Risks

- Too many pinned glyphs can starve short-lived glyphs. Measure the pin budget.
- Ranged charset updates benefit from contiguous slot changes. The new allocator may fragment them; measure command overhead separately from payload.
- Different slot assignments change screen command bytes even when frames look identical. The debug PNG hash is the main equivalence check.
- The checker measures uploaded glyph values, not visual correctness. VICE playback must still be checked.

## 9. Touchpoints

- `src/c64/examples/encode_cube.rs` — usage records, allocator, strategy variant.
- `src/c64-encoder-reporting/src/checks/char_redundancy.rs` — detailed churn diagnostics.
- `src/c64-encoder-reporting/src/reports/command_report.rs` — per-strategy comparison table.
- `.designs/charset-redundancy.md` — measurements and caveats.
- `.designs/charset-redundancy-reduction.md` — this plan.
