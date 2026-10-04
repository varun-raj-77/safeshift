# SS-002 Target-Selection Research

**STATUS: TARGET RECOMMENDATION ONLY — SS-002 REPRODUCTION NOT STARTED**

Research date: 2026-10-03. Preserved for the project pause on 2026-10-04.
This is a research/planning note, not executable evidence, an adopted migration
target, or proof of SafeShift additionality. No target was cloned, downloaded,
built, or tested during this selection work.

Before resuming after a substantial pause, re-check upstream releases, prior art,
SACTOR/current migration tooling, and target suitability. Do not assume this
October 2026 selection remains current indefinitely.

## Method and candidate comparison

The five governing instruction/research/README documents were read in full.
Upstream source, APIs, build definitions, tests, callers, releases, Rust
replacements, and primary migration research were inspected remotely. Nine
serious C components were considered. Source-size estimates and build suitability
were inspection judgments; no local target reproduction or benchmark was run.

Scores used twelve 0–5 dimensions: relevance, boundary, statefulness, build/tests,
differential suitability, contract discoverability, callers, regression seeds,
performance observability, additionality, manageable scope, and maintenance.
Totals were decision aids, not measured success probabilities.

| Candidate | Score / 60 | Selection judgment |
|---|---:|---|
| libogg complete framing subsystem | 52 | First finalist: compact, rich lifecycle and ownership behavior; substantial direct prior art |
| LZ4 framing/decompression | 51 | Second finalist: streaming progress/history and performance are valuable; dependencies and existing safe Rust implementation raise the bar |
| MPack buffered reader | 48 | Third finalist: realistic buffering/error callbacks; permitted nonlocal callback exits complicate a safe boundary |
| zlib inflate | 52 | Strong infrastructure, but an existing Rust implementation with C-facing interfaces makes additionality difficult to establish |
| libyaml event parser | 45 | Existing unsafe and safe ports; compatibility details and broader parser-test integration enlarge scope |
| Expat streaming XML parser | 49 | Rich tests/state, but entities, encodings, callbacks and suspension make the first boundary too large |
| libpng progressive reader | 47 | Transforms, interlace, error recovery and zlib enlarge the boundary; substantial Rust replacement work exists |
| llhttp generated parser | 49 | Real C runtime, but authoritative generator source complicates migration and maintenance comparisons |
| bzip2 streaming decompressor | 46 | Plausible boundary; substantial Rust replacement and limited maintenance-change supply weaken first-target value |

Higher totals did not override scope or provenance disadvantages. These are
first-target judgments, not claims that the other projects lack research value.

## Conditional recommendation

- Project: [Xiph libogg](https://github.com/xiph/ogg).
- Component: complete framing subsystem centered on `src/framing.c`, including
  encode/decode framing, synchronization, lifecycle, helpers, `src/crctable.h`,
  and relevant public headers. Original reproduction builds upstream libogg
  unchanged, including the separate `src/bitwise.c` implementation.
- Release: [v1.3.6](https://github.com/xiph/ogg/releases/tag/v1.3.6).
- Commit: `be05b13e98b048f0b5a0f5fa8ce514d56db5f822`.
- Expected original self-test executables: `test_framing` and `test_bitwise`.
- Expected initial build route: upstream CMake on Windows, with actual tools,
  versions, options and environment recorded during reproduction.
- Confidence: **MEDIUM**.

Published SHA-256 for the official `libogg-1.3.6.tar.gz` release archive:

```text
83e6704730683d004d20e21b8f7f55dcb3383cdf84c0daedf30bde175f774638
```

This checksum was identified from upstream release information. The archive has
**NOT** been locally downloaded or independently checked. A different source
artifact must not be assumed to have this checksum.

Libogg was preferred for its unusually compact implementation relative to
behavioral richness: stateful framing, packet/page continuation, sequencing,
buffering, reset/reuse, damaged-stream recovery, exposed/public state, and
borrowed buffer/lifetime obligations. Real callers include libvorbis decoding,
seeking and encoding. A study must retain those obligations rather than replacing
the real interface with a convenient one-shot transform.

The recommendation was strong enough to justify baseline reproduction, **not**
to establish novelty, adopt the target unconditionally, or lock a migration method.

## Reproduction cautions and acceptance gates

The inspected v1.3.6 CMake file creates the two self-test executables when
`BUILD_TESTING=ON`, but does not explicitly include CTest or call
`enable_testing()`. Run the actual original executables; a zero-test `ctest`
result is not success. These self-tests compile source independently and do not
establish downstream ABI compatibility of the built library.

Before adoption, verify source identity/checksum, a repeatable unmodified local
build, actual successful original test execution, and the exact environment.
Confirm a manageable dependency scope, usable original API, realistic callers,
meaningful state, applicable license terms, and a credible intended-behavior scope.
Reassess whether an existing workflow already solves the practical question.
Pending gates are not passed gates. Stop SS-002 if substantive source repair is
required; do not turn reproduction into porting, contracts, or custom harness work.

## Strongest baseline and additionality warning

**Direct prior art is substantial.** [RustAudio/ogg](https://github.com/RustAudio/ogg)
exists, and [SACTOR directly evaluated libogg](https://aclanthology.org/2026.acl-long.28/).
[C2Rust](https://github.com/immunant/c2rust) and competent manual migration remain
relevant. Retaining or sandboxing the original C is a legitimate alternative;
[Mozilla documented RLBox use for Ogg](https://hacks.mozilla.org/2021/12/webassembly-and-back-again-fine-grained-sandboxing-in-firefox-95/).
SafeShift has **NOT demonstrated additionality over these workflows**.

The strongest later comparison must include existing replacements, upstream and
caller-level tests, differential/property testing, fuzzing, sanitizers, and
applicable existing verification tools. SACTOR results were read, not reproduced.
Kani, CBMC, SAW/Crux, VERT, &inator and
[CRISP & CLEAR](https://www.galois.com/project/crisp-clear) remain relevant;
tool limitations must not be converted into unsupported novelty claims.
For the other finalists, assess [lz4_flex](https://github.com/PSeitz/lz4_flex) and
[Rust MessagePack implementations](https://github.com/3Hren/msgpack-rust) before
assuming another port is necessary.

The possible research question is whether one specific addition reduces total
human migration and maintenance cost while preserving equal or better assurance
about caller-sensitive state and ownership. Translation or passing tests alone
cannot answer that question. Keep PROVED, BOUNDED, TESTED, REFUTED, UNSUPPORTED and
ASSUMED distinct. Already-inspected historical bugs are calibration material,
not unseen held-out regressions.

## Fast-fail conditions

- Original upstream reproduction fails or requires substantive source repair.
- The component boundary becomes artificial, or realistic callers are excluded.
- Prior art already solves the practical problem economically.
- Intended behavior cannot be distinguished from undefined behavior or ambiguity.
- Unsafe adapter obligations dominate the claimed safety result.
- Maintenance-change evidence is inadequate or contrived to favor SafeShift.
- Performance, copying, allocation or latency makes a replacement unacceptable.

## Selected primary evidence

- [Pinned upstream build/test definitions](https://raw.githubusercontent.com/xiph/ogg/v1.3.6/CMakeLists.txt).
- [Packet output and borrowed lifetime](https://xiph.org/ogg/doc/libogg/ogg_stream_packetout.html).
- [libvorbis decode/seek caller](https://raw.githubusercontent.com/xiph/vorbis/master/lib/vorbisfile.c).
- [libvorbis encoder example](https://raw.githubusercontent.com/xiph/vorbis/master/examples/encoder_example.c).
- [Upstream change history](https://raw.githubusercontent.com/xiph/ogg/v1.3.6/CHANGES).

These links support the dated selection record; moving branches and current
documentation must be rechecked and pinned where needed on resumption.
