use thiserror::Error;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Custom error: `{0}`")]
    Custom(String),
    #[error("Unknown error in engine.")]
    Unknown,
}
