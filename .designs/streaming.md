# Streaming Animation from Disk — Detailed Plan

## 1. Goal & constraints

**Goal:** Play the animation while loading its data from disk, so the full animation stream does not need to reside in C64 RAM at once.

**Constraints (agreed):**
- **Software-only** fast loading on stock hardware (C64 + 1541), no JiffyDOS/parallel cable.
- **Loading during animation is mandatory** → requires an **IRQ-capable loader** (animation continues while bytes transfer over IEC).
- **Tracks/sectors will be prepared/mastered** on disk for streaming (not general-purpose DOS file access).
- **No sector link bytes** (raw 256-byte payload sectors); **sector ranges/sequence hardcoded** in the build.
- **Streaming is command-driven within the demo/frame stream:** a frame chunk can issue a **request to read a range of sectors** (async, serviced by the IRQ loader) and later issue a **wait** until that request is fully read. **Bytes read are stored after that wait command** (so the data becomes available to the decoder/engine at that point in the stream).
- **RAM buffer:** stream data uses a **cyclic (ring) buffer of ~16 KB** in RAM.

**Current baseline (measured, `encode_cube` with all frames on disk):** 12,361 B total stream for 200 encoded frames, i.e. 61.8 B/frame overall. Act 1 is 10,405 B / 100 frames; Act 2 is 1,956 B / 100 frames. The older Act-1-only baseline was 11,624 B / 100 frames (116.2 B/frame, 2–590 B/frame, payload split ~67.8% screen RLE, ~32.1% charset updates, ~1.7% structural headers) and the older Act 2 was 368 B / 1 frame; those figures are historical. This plan assumes streaming of the encoded command stream as-is or compressed.

## 2. Throughput & timing budget

| Loader scenario (1541, software-only, mastered) | Sustained throughput (plan value) | Bytes / 20 ms frame (50 Hz, PAL) | Bytes / 16.7 ms frame (60 Hz, NTSC) |
|---|---:|---:|---:|
| Conservative / safe | **2 KB/s** | ~40 B | ~33 B |
| Realistic target | **4 KB/s** | ~80 B | ~67 B |
| Optimized best-case (tuned interleave, linear) | **6 KB/s** | ~120 B | ~100 B |

**Feasibility relation (must hold with margin):**
- `avg(chunk_bytes) / chunk_duration_frames <= sustained_Bps / fps`
- and buffer must absorb `max(chunk_bytes)` and worst-case I/O gaps (track step, speed-zone change, rotation miss).

**Against the historical Act 1 baseline (11,624 B, 100 frames, avg 116 B/frame, peak frame 590 B; current stream is 10,405 B):**
- At 4 KB/s end-to-end stream time ≈ 2.9 s (budget headroom in aggregate) but **per-frame average (116 B) exceeds 80 B/frame @50 Hz** and peaks (590 B) far exceed it.
- **Conclusion:** streaming-at-frame-rate requires (a) **ahead buffering sized to bursts**, (b) **hold/repeat frames** when data not ready, and/or (c) **reduced bytes/frame** via compression/encoding improvements and/or fewer changed glyphs/cells. Design the pipeline to the conservative rate and add smoothing.

## 3. Disk layout (mastered, raw)

### 3.1 Sector format
- **Raw 256-byte payload per sector**, no DOS 2-byte track/sector link (bytes 0–1 not reserved for chaining). +2 B/sector (+0.78%) vs DOS-chained 254 B payload.
- **Sequencing owned by loader:** next sector determined by hardcoded order/interleave, not by on-disk links.
- **End-of-data:** store `total_bytes` (or total sectors + used bytes in last sector) in init/constants or an in-stream terminator so the decoder stops exactly (last sector typically partially filled).

### 3.2 1541 geometry & zones (for addressing/budget)
| Tracks | Sectors/track | Zone speed |
|---:|---:|---|
| 1–17 | 21 | fastest / most capacity |
| 18–24 | 19 | |
| 25–30 | 18 | |
| 31–35 | 17 | slowest |

Total 683 sectors (170 KB). Prefer placing the stream on **lower-numbered (faster) tracks**; account for sectors/track when spanning zones.

### 3.3 Interleave & ordering
- **Linear, forward-only read order** matching the loader’s expected sector stepping: `next = (current + interleave) mod sectors_per_track`, advance track in forward direction at boundary (exact rule per loader).
- **No backward seeks, no fragmentation, no foreign data** interleaved into the stream path.
- Stream occupies a contiguous reserved region in mastered order; directory/BAM must not conflict with used sectors.

### 3.4 Hardcoded sector ranges
- Encode the stream location/sequence as build-time constants (no per-sector metadata on disk):
  - **Minimal:** `start_track`, `start_sector`, `total_sectors` (+ `interleave`/step rule) if the stream is one contiguous run in mastered order.
  - **Run list (if needed):** small table of spans `(track, first_sector_index, count)` for gaps/zone transitions.
- **Build regenerates** these constants whenever stream content, size, location, or interleave changes. Add a build-time assert that the encoded sequence covers exactly the mastered sectors in order.

## 4. Loader & transfer architecture

### 4.1 Loader
- **IRQ fastloader, software-only, 1541 drivecode upload** (e.g. Krill’s loader or equivalent) configured for **raw track/sector streaming** (not DOS file-chain LOAD) with the chosen interleave.
- Loader runs under IRQ so the CPU can animate/decode between serviced transfers; respect IRQ timing (raster/NMI interactions) and budget stolen cycles.
- Confirm 1541-only scope (1571/1581/other drives need different routines/geometry).

### 4.2 Command-driven streaming (request / wait)
Streaming is controlled explicitly by commands embedded in the demo/frame command stream, scheduled by the encoder/build so loading overlaps animation work.

- **Request-read command (async):** part of a demo frame/chunk; issues a request to start reading a **range of sectors** (hardcoded/mastered range) into the stream buffer at the position that follows the corresponding wait point. Returns control immediately so animation/decoding of already-available commands continues while the IRQ loader services the transfer in the background.
- **Wait-read command (sync point):** blocks/holds progression until the **last request is fully read** (transfer complete). The engine must not consume bytes that depend on that data before the wait completes.
- **Data placement:** **bytes read are stored after that wait command** in stream order. With a **cyclic ~16 KB buffer**, this means the loader writes the payload into the ring immediately following the wait command’s position in the buffer; on completion the decoder continues sequentially from there (wrapping at the buffer end back to the start as needed). The logical contract is “data available immediately after wait” in the command stream.
- **Pipelining:** requests can be issued ahead of the point where data is needed (e.g. early in a frame/chunk, or N frames ahead) so the wait is ideally a no-op or short; if not yet complete, the wait holds (repeat/hold frame or stall as defined) until ready.
- **Cyclic buffer management:** producer (IRQ loader write position) and consumer (decoder/command read position) advance monotonically modulo ~16 KB. Free space is reclaimed as the decoder consumes past regions; **scheduling must ensure the producer never overtakes unread data** and that a requested range (plus wrap) fits available space at issue/wait time. Chunking is aligned to these request/wait boundaries.

### 4.3 CPU & VIC timing
- **Decode/update at safe times:** perform screen and charset writes in vblank/stable raster windows; never rewrite a char slot while it is visible in a way that violates display correctness (engine/encoder already assume slot-liveness constraints).
- **Budget split per frame:** IRQ/transfer overhead + decoder/command execution + animation/game logic must fit within frame time with slack.
- **Worst-case gaps:** track step and speed-zone boundary can stall delivery for a rotation+; chunk/buffer sizing must cover this without starving playback.

## 5. Memory map (resident, indicative)

| Region | Purpose | Notes |
|---|---|---|
| Loader + drive protocol host stub | IRQ transfer, sector stepping | resident for duration |
| Decoder / command engine | interpret streamed command bytes, incl. read-request/wait commands (current engine in `src/engine.asm`/command modules) | resident |
| Active VIC state | screen RAM, charset (and bitmap/colors as used) | resident, mutated by commands |
| **Cyclic stream/command buffer** | ring buffer holding the active command/data window; loaded sector data is placed **after the wait command** and consumed sequentially (with wrap) | **~16 KB** fixed |
| Read-request / ring state | pending range (start/range id), producer/write and consumer/read positions (or offsets), wrap/end markers, completion flag/status for “last request” | few bytes |
| Stack/zeropage/workspace | runtime | as required |

**Peak streaming RAM = resident fixed + ~16 KB cyclic buffer + small state.** The ~16 KB ring bounds resident stream memory regardless of total animation size (compare to non-streaming: ~11.6 KB resident for Act 1 stream alone, larger for longer animations). Correctness requires that unread data (distance between consumer and producer, modulo buffer size) is never overwritten and that each requested range fits contiguously in the available ring space at the point it is written (or is handled across the wrap boundary by the loader/decoder convention).

## 6. Stream format & engine integration

- **Payload:** the existing encoded demo/command byte stream (frames/commands as produced by `c64-encoder`), consumed sequentially, extended by **streaming commands**.
- **Streaming commands (new, part of the demo/frame command set):**
  - **`RequestReadRange` (name TBD):** parameters identify a **range of sectors** to read (e.g. encoded start/range id or hardcoded-range selector, and/or destination/length as required). Issues an async read serviced by the IRQ loader; execution continues with following commands while loading proceeds.
  - **`WaitRead` (name TBD):** synchronization point that waits until the **last request is fully read**. On completion, the **bytes read are stored immediately after this wait command** in the stream/buffer, and the decoder/engine proceeds to consume that loaded data/commands in sequence.
  - *Semantics to finalize:* single in-flight request vs. queued/multiple; how the range is encoded (index into hardcoded table vs. inline t/s/count); whether destination/size is implicit (derived from range and ring write position) or explicit; and the precise ring-buffer contract for “stored after wait” — write offset relative to the wait command, behavior when a range crosses the ~16 KB wrap boundary (split write/contiguous reservation), and how the consumer read pointer advances past wrap.
- **Chunk/frame integration:** range-to-read requests are placed within demo frame chunks at points chosen to overlap I/O with animation (issue early, wait at/just before data is needed). Chunk boundaries align with request/wait and the resulting in-memory layout so the decoder never reads beyond available data.
- **Encoding-time insertion & timing check:** the encoder inserts `RequestReadRange`/`WaitRead` commands into the stream and **verifies timing per request** (see §7.2). A request is only valid if its data is guaranteed (within the assumed budget) to be complete by the corresponding wait.
- **Init:** load/bootstrap initial resident command region; first `RequestReadRange` (and pre-roll) issued early enough that the first `WaitRead` is satisfied on time.
- **End:** final chunk/frame completes with no further pending request; hold/end per animation design.
- **Compatibility:** denser command encoding (e.g. 1A packed RLE, B1 coalesced charset updates) reduces bytes per loaded range/frame and lowers throughput/buffer pressure — transparent if decoder and stream share the format. Scheduling uses the chosen throughput/fps budget and must satisfy the per-request timing check.

## 7. Build & mastering pipeline

1. **Encode animation** → base command stream bytes (per-frame commands/sizes) using `c64-encoder`/builder.
2. **Plan ranges & insert streaming commands:** partition the stream into sector ranges that fit the ~16 KB cyclic buffer/window, then insert `RequestReadRange`/`WaitRead` commands at chosen issue/wait points (frame/chunk boundaries) so each range is requested ahead of its wait and data is placed after the wait in stream order. Emit the hardcoded range/run list (`start_t/s`, count, interleave/step params) and stream constants.
3. **Timing check & report (per request):** during/after insertion, compute and **report for each request** the **timing needed** vs **timing expected** (see §7.2). Fail or warn when a request cannot complete by its wait under the assumed budget.
4. **Lay out sectors:** map stream bytes (including streaming commands and the payload layout they imply) to raw 256 B sectors in mastered linear/interleaved order; generate disk image (`.D64`) or write track/sector data with tooling (`cc1541`/`c1541`/loader packer), ensuring no overlap with system areas and correct interleave.
5. **Assemble/link runtime:** include loader drivecode/host stub, decoder/engine, hardcoded range/step constants, and cyclic-buffer configuration.
6. **Emit artifacts:** disk image + runnable prg/bootstrap as needed, plus the per-request timing report and measured/expected throughput assumptions.

### 7.2 Per-request timing analysis & report
For every `RequestReadRange` → `WaitRead` pair the encoder computes and reports:

- **Request identity:** request index/id, target range (track/sector start, sector count), payload bytes (`sectors × 256`), and buffer placement (ring offset/span, wrap if any).
- **Issue/wait placement:** frame/command position of request and of wait; **expected time available** = lead time between request issue and wait point, expressed in frames/time (e.g. `frames_lead × frame_duration` at target fps/region).
- **Timing needed:** estimated transfer time to complete the range under the assumed loader model — derived from payload size and **sustained throughput assumption** (e.g. 2/4/6 KB/s) plus modeled overhead/gaps where known (sector/rotation cost, interleave, track/zone steps). Report the assumption(s) used.
- **Timing expected (margin):** `expected_available − needed` (and/or ratio). Mark each request as **OK**, **tight**, or **underrun risk/fail** against a configurable safety margin.
- **Ring/buffer check:** confirm the range fits available cyclic-buffer space at issue/wait (no overwrite of unread data) and note wrap behavior.

**Output:** a per-request report (e.g. table/log emitted alongside build artifacts) listing at minimum request id/range, bytes, issue→wait lead time (expected), transfer time (needed), margin/status, and assumptions (fps, B/s, overhead model). The build should surface failures/warnings so request placement or range sizing can be adjusted before mastering.

## 8. Validation & acceptance

- **Correctness:** decoded/played output matches non-streamed playback for the same stream (frame-by-frame equivalence where applicable).
- **Encode-time timing report:** review the per-request “timing needed vs. timing expected” report (§7.2); resolve any tight/underrun-flagged requests (adjust range sizes, request placement/lead time, or compression) before mastering.
- **True-drive timing:** verify on **VICE with true drive emulation** and on **real 1541** hardware.
- **Throughput & gaps:** measure sustained B/s and worst-case stall (track/zone step) under IRQ + animation; confirm no underrun at target fps and that actual transfer times align with the report’s assumptions.
- **Cyclic buffer:** validate no overwrite of unread data, correct wrap behavior, and that “stored after wait” placement/consumption matches the contract.
- **Robustness:** clean end-of-stream handling, correct behavior on partially filled last sector, and deterministic behavior given fixed mastering.

## 9. Open decisions / next inputs
- **Loader selection:** Krill’s IRQ loader (and its exact streaming/raw-t/s mode) vs another — fixes interleave convention and integration details.
- **Streaming command semantics:** opcode names/encoding for `RequestReadRange`/`WaitRead`; single in-flight vs. queued requests; range parameter form (index into hardcoded range table vs. inline params) and whether destination/size are implicit or explicit.
- **Cyclic buffer details (~16 KB):** exact buffer base/size (power-of-two vs. 16 KB exactly) and index/mask arithmetic; ring producer/consumer pointer representation; write offset relative to the wait command for “stored after wait”; handling of ranges that cross the wrap boundary (contiguous reservation/split copy) and alignment; invariant/scheduling rules to prevent overwrite of unread data.
- **Target fps & region:** PAL 50 Hz vs NTSC 60 Hz (affects bytes/frame budget).
- **Chunk/range policy:** frame-aligned ranges vs. variable, max in-flight range size vs. ~16 KB window, and how request/wait placement is scheduled relative to burstiness (Act 1 peak 590 B) and lead-time budget.
- **Timing model & reporting:** throughput/overhead assumptions for “timing needed” (conservative/realistic/best-case B/s, track/zone-step penalties), safety-margin threshold for OK/tight/fail, and report format/destination.
- **Pre-roll & underrun policy:** lead time for first/ongoing requests; behavior on wait-not-ready (hold-frame vs. stall) and acceptable latency.
- **Compression scope in force:** whether 1A/A2/B1 (or content/strategy changes) land before finalizing range sizes and scheduling margins.
- **Drive compatibility:** 1541-only vs also 1571/1581.

## 10. Example implementations & references

### IRQ loaders / streaming frameworks (software-only, 1541 drivecode)
- **Krill’s Loader** (IRQ loader, install/resident; widely used for demos/streaming, high sustained throughput on mastered disks): CSDB release — https://csdb.dk/release/?id=226124 ; repository — https://github.com/bbohl/krill-loader
- **Spindle** (IRQ loader + trackmo/link framework by Linus Åkesson): CSDB — https://csdb.dk/release/?id=147458 ; homepage/docs — https://www.linusakesson.net/programming/spindle/
- **Bitfire** (modern trackmo loader/framework by Axis): repository — https://github.com/bboxy/bitfire ; CSDB group/releases — https://csdb.dk/group/?id=8665
- **DreamLoad** (IRQ fastloader, long-standing): CSDB — https://csdb.dk/release/?id=152965
- **Covert Bitops Loader** (Cadaver’s IRQ loader, source + docs): https://cadaver.github.io/rants/irqload.html ; tools/repo — https://github.com/cadaver/c64tools
- **Sparkle** (IRQ loader / trackmo build system): https://github.com/spartaomg/Sparkle ; CSDB — https://csdb.dk/release/?id=222256

### Disk mastering / raw sector & image tooling
- **cc1541** (create `.D64` images with control over file/sector layout): https://bitbucket.org/PTV_Claus/cc1541
- **c1541** (VICE disk image utility, scriptable): https://vice-emu.sourceforge.io/vice_14.html
- **VICE** emulator (use true drive emulation for timing-accurate loader tests): https://vice-emu.sourceforge.io/
- **D64/D71/D81 format reference** (sector counts, layout): https://vice-emu.sourceforge.io/vice_17.html#SEC396

*Notes:* verify the exact raw track/sector streaming mode, interleave configuration, and 1541-only assumptions in the chosen loader’s docs before relying on specific throughput figures. Some projects/mirrors move over time; CSDB entries are stable indexes to releases and docs.

## 11. Summary
A software-only, IRQ-driven loader reads **raw 256 B sectors** in a **hardcoded, mastered linear/interleaved** sequence, controlled by **in-stream commands**: a frame chunk issues a **request to read a sector range** (async), later a **wait** ensures the last request is fully read, and the **loaded bytes are stored after the wait command** so decoding continues into them within a **~16 KB cyclic (ring) buffer** in RAM. With mastered 1541 layout plan for **~4 KB/s sustained** (~80 B/frame @50 Hz), request/wait pipelining and ring-buffered flow control against bursts/gaps—combined with payload reduction (screen/charset encoding, chunk smoothing)—make load-during-animation feasible with bounded resident stream memory (~16 KB) instead of holding the full ~12.4 KB (and larger) animation stream in RAM.