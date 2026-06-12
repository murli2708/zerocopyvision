//! ZeroCopyVision Rust core.
//!
//! The whole point of this crate is the *boundary*: Dart passes a raw pointer to a
//! camera frame plus its geometry, and we run inference by borrowing that buffer —
//! we never copy it across the FFI line. Keep this surface tiny and stable.

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    Bgra8888,
    Yuv420,
}

#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub model_path: String,
    pub num_threads: usize,
}

/// Opaque handle to a loaded engine. Dart holds this across frames.
pub struct EngineHandle {
    config: EngineConfig,
    // TODO: store the loaded model/session (ort, tract, or candle).
}

#[derive(Debug, Clone)]
pub struct Detection {
    pub label: u32,
    pub score: f32,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Debug, Clone)]
pub struct FrameResult {
    pub detections: Vec<Detection>,
    pub inference_ms: f32,
}

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("model failed to load: {0}")]
    ModelLoad(String),
    #[error("unsupported pixel format")]
    UnsupportedFormat,
    #[error("inference failed: {0}")]
    Inference(String),
}

/// Load the model once and return a handle.
pub fn init_engine(config: EngineConfig) -> EngineHandle {
    // TODO (Phase 3): load the model session here using `config`.
    EngineHandle { config }
}

/// Run inference on a borrowed camera frame. The caller owns `ptr`; we must NOT
/// keep it past this call. `stride` is bytes-per-row (often != width * channels).
///
/// # Safety
/// `ptr` must point to at least `stride * height` valid, readable bytes that stay
/// alive for the duration of this call.
pub unsafe fn process_frame(
    handle: &EngineHandle,
    ptr: usize,
    width: u32,
    height: u32,
    stride: u32,
    format: PixelFormat,
) -> Result<FrameResult, EngineError> {
    let _ = (handle, ptr, width, height, stride);
    match format {
        PixelFormat::Bgra8888 | PixelFormat::Yuv420 => {
            // TODO (Phase 3): build a borrowed view over the buffer (no copy),
            // preprocess, run the model, return detections + timing.
            Err(EngineError::Inference("not implemented".into()))
        }
    }
}

/// Free the engine. After this the handle must not be used again.
pub fn dispose(handle: EngineHandle) {
    drop(handle);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_and_dispose_roundtrip() {
        let h = init_engine(EngineConfig { model_path: "model.onnx".into(), num_threads: 2 });
        assert_eq!(h.config.num_threads, 2);
        dispose(h);
    }
}
