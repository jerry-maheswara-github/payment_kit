use thiserror::Error;

#[derive(Error, Debug)]
pub enum PaymentError {
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Callback validation failed: {0}")]
    InvalidCallback(String),

    #[error("Gateway error: {0}")]
    Gateway(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Processing error: {0}")]
    ProcessingError(String),

    #[error("Service unavailable: {0}")]
    Unavailable(String),

    #[error("Unknown error occurred")]
    Unknown,
}
