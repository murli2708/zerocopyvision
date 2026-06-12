# ZeroCopyVision — Rust ↔ Dart zero-copy camera pipeline

> Phase 3 starter. *Move 30 frames a second through a Rust inference engine without copying a single pixel more than you have to.*

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

## License

MIT.
