use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompositorError {
    #[error("no suitable GPU adapter found")]
    NoAdapter,

    #[error("failed to request GPU device: {0}")]
    DeviceRequest(String),

    #[error("render error: {0}")]
    RenderError(String),
}
