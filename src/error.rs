use thiserror::Error;

/// Common error variants that can occur in the payment system.
#[derive(Error, Debug)]
pub enum PaymentError {
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Callback validation failed: {0}")]
    InvalidCallback(String),

    #[error("Gateway error: {0}")]
    GatewayError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Processing error: {0}")]
    ProcessingError(String),

    #[error("Service unavailable: {0}")]
    Unavailable(String),

    #[error("Unknown error occurred: {0}")]
    Unknown(String),
}
