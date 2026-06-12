# ZeroCopyVision — Rust ↔ Dart zero-copy camera pipeline

![Phase](https://img.shields.io/badge/Phase%203-The%20Eyes-7C3AED)
![Status](https://img.shields.io/badge/status-scaffold%20%C2%B7%20building%20in%20public-orange)
![Rust](https://img.shields.io/badge/core-Rust-000000?logo=rust&logoColor=white)
![Flutter](https://img.shields.io/badge/app-Flutter-02569B?logo=flutter&logoColor=white)
![Bridge](https://img.shields.io/badge/bridge-flutter__rust__bridge%20%C2%B7%20FFI-DEA584)
![License](https://img.shields.io/badge/license-MIT-green)

> Phase 3 · The Eyes. *Move 30 frames a second through a Rust inference engine without copying a single pixel more than you have to.*

> **Status — honest:** scaffold + engineering plan. The FFI contract and KPI targets below
> are the design this repo commits to; benchmark numbers are filled in on real hardware.

## Problem

Real-time on-device vision lives or dies on the camera→inference→display path. Every
needless buffer copy is dropped frames and a hot battery. ZeroCopyVision defines a clean
FFI boundary where Dart hands Rust a pointer to the camera frame and gets results back —
**no per-frame allocation across the bridge.**

## Constraints

- 30 FPS sustained on a mid-range phone.
- Zero per-frame heap copies across the FFI boundary (pass pointer + stride + format).
- The Rust core is platform-agnostic; all camera/UI glue lives in Flutter.

## Architecture

```
 Flutter (Dart)                     flutter_rust_bridge                Rust core
 ─────────────                      ──────────────────                ─────────
 CameraImage  ──ptr,w,h,stride──▶   process_frame(handle, ...)  ──▶   inference engine
 results overlay  ◀──FrameResult──  (borrows the buffer, no copy)     (ort / tract / candle)
```

## FFI boundary contract (the part that matters)

```rust
// rust/src/lib.rs  — keep this surface tiny and stable.
pub fn init_engine(config: EngineConfig) -> EngineHandle;
pub fn process_frame(
    handle: &EngineHandle,
    ptr: usize, width: u32, height: u32, stride: u32, format: PixelFormat,
) -> Result<FrameResult, EngineError>;
pub fn dispose(handle: EngineHandle);
```

## Key design decisions & tradeoffs

- **Pointers cross the bridge, pixels don't.** The camera buffer stays in native memory; Dart hands Rust a `ptr + width + height + stride + format` and gets back a compact `FrameResult`. Zero image-byte copies per frame is the entire point of the project.
- **Narrow, coarse-grained FFI surface (3 functions).** `init / process_frame / dispose` — one call per frame, not many chatty calls. Each FFI crossing has overhead, so minimize crossings.
- **Drop-on-busy backpressure.** Always process the *latest* frame and drop stale ones (queue depth ≤ 1–2). A smooth 30 FPS on fresh frames beats a laggy 60 on stale ones.
- **Pre-allocate once, reuse forever.** All scratch/input/output buffers are allocated at engine init inside Rust; the hot path allocates nothing. A `NativeFinalizer` makes cleanup deterministic.

## Quickstart

```bash
# Rust core
cd rust && cargo test

# Flutter app (after `cargo install flutter_rust_bridge_codegen`)
flutter_rust_bridge_codegen generate
flutter run
```

## KPI table

| Metric                | Target                                  |
| --------------------- | --------------------------------------- |
| Sustained throughput  | 30 FPS                                   |
| Per-frame copies      | 0 across the FFI boundary                |
| Boundary surface      | 3 functions (init / process / dispose)   |
| Memory                | Flat over a 10-minute session (no leak)  |

## Failure modes & what I'd change next

- **Lifetime bugs**: the borrowed buffer must outlive `process_frame`. Document who owns
  the frame and never store the raw pointer past the call.
- **Format drift**: YUV420 vs. BGRA varies by platform — `PixelFormat` is explicit on purpose.
- **Bridge chattiness**: batch results into one `FrameResult` struct instead of many small calls.

## Where this fits

Part of the **[Edge AI Architect roadmap](https://github.com/murli2708/edge-ai-roadmap)** — a 6-month, 6-project build-in-public series.

**Phase 3 of 6** · ← prev **[EdgeQuant](https://github.com/murli2708/edgequant)** · next → **[LocalMind](https://github.com/murli2708/localmind)** (Phase 4 · offline memory & RAG)

## License

MIT © Murli — see [LICENSE](LICENSE).
